use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Node {
    Dir { files: BTreeMap<String, Node> },
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
        Ok(Self { file, header, payload_offset })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    /// 按虚拟路径(如 "renderer/preload.js")读取文件字节
    pub fn read_file(&mut self, virtual_path: &str) -> Result<Vec<u8>> {
        let node = find_node(&self.header.files, virtual_path)
            .ok_or_else(|| AppError::AsarMalformed(format!("找不到文件:{virtual_path}")))?;
        let (offset, size) = match node {
            Node::File { offset: Some(o), size, .. } => {
                let off: u64 = o.parse().map_err(|_| AppError::AsarMalformed(format!("非法 offset:{o}")))?;
                (off, *size)
            }
            _ => return Err(AppError::AsarMalformed(format!("不是文件:{virtual_path}"))),
        };
        self.file.seek(SeekFrom::Start(self.payload_offset + offset)).map_err(AppError::Io)?;
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
        let next = if prefix.is_empty() { name.clone() } else { format!("{prefix}/{name}") };
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
        return Err(AppError::AsarMalformed(format!("非预期外层 pickle size:{outer}")));
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
