use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallInfo {
    pub install_path: PathBuf,
    pub asar_path: PathBuf,
    pub version: Option<String>,
    /// v0.1.0 占位,始终 false。v0.2 会接 sysinfo 真实检测
    pub is_running: bool,
}

pub fn detect() -> Result<InstallInfo> {
    let candidates = default_candidates();
    for c in &candidates {
        if let Some(info) = try_path(c)? {
            return Ok(info);
        }
    }
    Err(AppError::FeishuNotFound(candidates))
}

/// 用户在 UI 上手动选了一个目录后,验证它确实是飞书安装根目录
pub fn validate(path: &Path) -> Result<InstallInfo> {
    if let Some(info) = try_path(path)? {
        Ok(info)
    } else {
        Err(AppError::FeishuNotFound(vec![path.to_path_buf()]))
    }
}

fn default_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        vec![
            PathBuf::from("/Applications/Lark.app"),
            PathBuf::from("/Applications/Feishu.app"),
            PathBuf::from("/Applications/飞书.app"),
        ]
    }
    #[cfg(not(target_os = "macos"))]
    {
        Vec::new()
    }
}

fn try_path(root: &Path) -> Result<Option<InstallInfo>> {
    if !root.exists() {
        return Ok(None);
    }
    let asar = find_messenger_asar(root);
    let asar = match asar {
        Some(a) => a,
        None => return Ok(None),
    };
    let version = read_version(root);
    Ok(Some(InstallInfo {
        install_path: root.to_path_buf(),
        asar_path: asar,
        version,
        is_running: false,
    }))
}

fn find_messenger_asar(root: &Path) -> Option<PathBuf> {
    walk_for(root, "messenger.asar", 6)
}

fn walk_for(dir: &Path, target_filename: &str, depth: u32) -> Option<PathBuf> {
    if depth == 0 {
        return None;
    }
    let entries = std::fs::read_dir(dir).ok()?;
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(found) = walk_for(&p, target_filename, depth - 1) {
                return Some(found);
            }
        } else if p.file_name().and_then(|s| s.to_str()) == Some(target_filename) {
            return Some(p);
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn read_version(root: &Path) -> Option<String> {
    let plist_path = root.join("Contents/Info.plist");
    if !plist_path.exists() {
        return None;
    }
    let plist: plist::Value = plist::from_file(&plist_path).ok()?;
    let dict = plist.as_dictionary()?;
    dict.get("CFBundleShortVersionString")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string())
}

#[cfg(not(target_os = "macos"))]
fn read_version(_root: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_nonexistent_path_returns_not_found() {
        let err = validate(Path::new("/definitely/not/here")).unwrap_err();
        assert!(matches!(err, AppError::FeishuNotFound(_)));
    }

    #[test]
    fn validate_dir_without_messenger_asar_returns_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let err = validate(tmp.path()).unwrap_err();
        assert!(matches!(err, AppError::FeishuNotFound(_)));
    }

    #[test]
    fn validate_dir_with_messenger_asar_returns_info() {
        let tmp = tempfile::tempdir().unwrap();
        let webcontent = tmp.path().join("Contents/Resources/webcontent");
        std::fs::create_dir_all(&webcontent).unwrap();
        std::fs::write(webcontent.join("messenger.asar"), b"dummy").unwrap();
        let info = validate(tmp.path()).unwrap();
        assert!(info.asar_path.ends_with("webcontent/messenger.asar"));
        assert!(!info.is_running);
    }
}
