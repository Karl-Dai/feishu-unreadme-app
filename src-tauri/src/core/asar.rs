use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Node {
    Dir {
        files: BTreeMap<String, Node>,
    },
    File {
        #[serde(default)]
        offset: Option<String>,
        size: u64,
        #[serde(default)]
        executable: bool,
        #[serde(default)]
        unpacked: bool,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    pub files: BTreeMap<String, Node>,
}

pub struct Asar {
    file: File,
    header: Header,
    payload_offset: u64,
}

impl Asar {
    pub fn open(path: &Path) -> Result<Self> {
        let mut file = File::open(path).map_err(AppError::Io)?;
        let (header, payload_offset) = read_header(&mut file)?;
        Ok(Self {
            file,
            header,
            payload_offset,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    /// 按虚拟路径(如 "renderer/preload.js")读取文件字节
    pub fn read_file(&mut self, virtual_path: &str) -> Result<Vec<u8>> {
        let node = find_node(&self.header.files, virtual_path)
            .ok_or_else(|| AppError::AsarMalformed(format!("找不到文件:{virtual_path}")))?;
        let (offset, size) = match node {
            Node::File {
                offset: Some(o),
                size,
                ..
            } => {
                let off: u64 = o
                    .parse()
                    .map_err(|_| AppError::AsarMalformed(format!("非法 offset:{o}")))?;
                (off, *size)
            }
            _ => return Err(AppError::AsarMalformed(format!("不是文件:{virtual_path}"))),
        };
        self.file
            .seek(SeekFrom::Start(self.payload_offset + offset))
            .map_err(AppError::Io)?;
        let mut buf = vec![0u8; size as usize];
        self.file.read_exact(&mut buf).map_err(AppError::Io)?;
        Ok(buf)
    }

    /// 列出包内所有文件的虚拟路径(深度优先)
    pub fn list_files(&self) -> Vec<String> {
        let mut out = Vec::new();
        walk(&self.header.files, "", &mut out);
        out
    }

    pub fn payload_offset(&self) -> u64 {
        self.payload_offset
    }
}

fn walk(map: &BTreeMap<String, Node>, prefix: &str, out: &mut Vec<String>) {
    for (name, node) in map {
        let next = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        match node {
            Node::Dir { files } => walk(files, &next, out),
            Node::File { .. } => out.push(next),
        }
    }
}

fn find_node<'a>(map: &'a BTreeMap<String, Node>, path: &str) -> Option<&'a Node> {
    let mut cur = map;
    let parts: Vec<&str> = path.split('/').collect();
    for (i, p) in parts.iter().enumerate() {
        let node = cur.get(*p)?;
        if i == parts.len() - 1 {
            return Some(node);
        }
        match node {
            Node::Dir { files } => cur = files,
            _ => return None,
        }
    }
    None
}

fn read_u32_le(f: &mut File) -> Result<u32> {
    let mut b = [0u8; 4];
    f.read_exact(&mut b).map_err(AppError::Io)?;
    Ok(u32::from_le_bytes(b))
}

fn read_header(f: &mut File) -> Result<(Header, u64)> {
    // 跳过外层 pickle:第一个 u32 永远是 4(指示 header pickle size 字段的字节数)
    let outer = read_u32_le(f)?;
    if outer != 4 {
        return Err(AppError::AsarMalformed(format!(
            "非预期外层 pickle size:{outer}"
        )));
    }
    let _header_pickle_size = read_u32_le(f)?;
    let _header_string_pickle_size = read_u32_le(f)?;
    let header_string_length = read_u32_le(f)? as usize;

    let mut buf = vec![0u8; header_string_length];
    f.read_exact(&mut buf).map_err(AppError::Io)?;
    let header: Header = serde_json::from_slice(&buf)
        .map_err(|e| AppError::AsarMalformed(format!("JSON 解析失败:{e}")))?;

    // 对齐到 4 字节
    let mut pos = 16u64 + header_string_length as u64;
    let padding = (4 - (header_string_length % 4)) % 4;
    pos += padding as u64;
    f.seek(SeekFrom::Start(pos)).map_err(AppError::Io)?;

    Ok((header, pos))
}

impl Asar {
    /// 把整个 asar 解包到一个目录
    pub fn extract(src: &Path, dst_dir: &Path) -> Result<()> {
        let mut asar = Asar::open(src)?;
        fs::create_dir_all(dst_dir).map_err(AppError::Io)?;
        let files = asar.list_files();
        for vp in files {
            let bytes = asar.read_file(&vp)?;
            let target = dst_dir.join(&vp);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(AppError::Io)?;
            }
            fs::write(&target, &bytes).map_err(AppError::Io)?;
        }
        Ok(())
    }

    /// 把目录打包成 asar
    pub fn pack(src_dir: &Path, dst: &Path) -> Result<()> {
        // 1. 收集 (虚拟路径, 字节内容),并构建 header tree
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();
        collect(src_dir, src_dir, &mut files)?;
        files.sort_by(|a, b| a.0.cmp(&b.0));

        let mut tree: BTreeMap<String, Node> = BTreeMap::new();
        let mut cursor: u64 = 0;
        for (vp, bytes) in &files {
            let size = bytes.len() as u64;
            insert_node(
                &mut tree,
                vp,
                Node::File {
                    offset: Some(cursor.to_string()),
                    size,
                    executable: false,
                    unpacked: false,
                },
            );
            cursor += size;
        }
        let header = Header { files: tree };

        // 2. 序列化 header,计算 padding
        let header_str = serde_json::to_string(&header)
            .map_err(|e| AppError::Internal(format!("header 序列化失败:{e}")))?;
        let header_bytes = header_str.as_bytes();
        let header_len = header_bytes.len();
        let padding = (4 - (header_len % 4)) % 4;

        // 3. 写文件
        let mut out = fs::File::create(dst).map_err(AppError::Io)?;
        let header_string_pickle_size = header_len as u32 + 4;
        let header_pickle_size = header_string_pickle_size + 8;
        out.write_all(&4u32.to_le_bytes()).map_err(AppError::Io)?;
        out.write_all(&header_pickle_size.to_le_bytes())
            .map_err(AppError::Io)?;
        out.write_all(&header_string_pickle_size.to_le_bytes())
            .map_err(AppError::Io)?;
        out.write_all(&(header_len as u32).to_le_bytes())
            .map_err(AppError::Io)?;
        out.write_all(header_bytes).map_err(AppError::Io)?;
        out.write_all(&vec![0u8; padding]).map_err(AppError::Io)?;
        for (_, bytes) in &files {
            out.write_all(bytes).map_err(AppError::Io)?;
        }
        out.flush().map_err(AppError::Io)?;
        Ok(())
    }
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(AppError::Io)? {
        let entry = entry.map_err(AppError::Io)?;
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, out)?;
        } else if path.is_file() {
            let vp = path
                .strip_prefix(root)
                .map_err(|e| AppError::Internal(format!("strip_prefix:{e}")))?
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = fs::read(&path).map_err(AppError::Io)?;
            out.push((vp, bytes));
        }
    }
    Ok(())
}

fn insert_node(tree: &mut BTreeMap<String, Node>, virtual_path: &str, leaf: Node) {
    let parts: Vec<&str> = virtual_path.split('/').collect();
    let mut cur = tree;
    for (i, p) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            cur.insert(p.to_string(), leaf.clone());
            return;
        }
        let entry = cur.entry(p.to_string()).or_insert_with(|| Node::Dir {
            files: BTreeMap::new(),
        });
        match entry {
            Node::Dir { files } => cur = files,
            _ => return,
        }
    }
}
