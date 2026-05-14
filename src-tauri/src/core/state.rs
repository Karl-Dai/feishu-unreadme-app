use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub schema: u32,
    #[serde(default)]
    pub feishu: Option<FeishuState>,
    #[serde(default)]
    pub patches: Option<PatchesState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuState {
    pub path: PathBuf,
    pub version_seen: String,
    pub asar_hash_before: String,
    pub asar_hash_after: String,
    pub patched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchesState {
    pub active_version: String,
    pub active_source: PatchSource,
    #[serde(default)]
    pub remote_pubkey_fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PatchSource {
    Builtin,
    Remote,
}

pub fn state_file(state_dir: &Path) -> PathBuf {
    state_dir.join("state.json")
}

pub fn load(state_dir: &Path) -> Result<State> {
    let path = state_file(state_dir);
    if !path.exists() {
        return Ok(State { schema: 1, ..Default::default() });
    }
    let bytes = fs::read(&path).map_err(AppError::Io)?;
    let state: State = serde_json::from_slice(&bytes)
        .map_err(|e| AppError::Internal(format!("state.json 解析失败:{e}")))?;
    Ok(state)
}

pub fn save(state_dir: &Path, state: &State) -> Result<()> {
    fs::create_dir_all(state_dir).map_err(AppError::Io)?;
    let path = state_file(state_dir);
    let tmp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(state)
        .map_err(|e| AppError::Internal(format!("state 序列化失败:{e}")))?;
    fs::write(&tmp, &bytes).map_err(AppError::Io)?;
    fs::rename(&tmp, &path).map_err(AppError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_returns_empty_v1() {
        let tmp = tempfile::tempdir().unwrap();
        let s = load(tmp.path()).unwrap();
        assert_eq!(s.schema, 1);
        assert!(s.feishu.is_none());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let s = State {
            schema: 1,
            feishu: Some(FeishuState {
                path: PathBuf::from("/Applications/Lark.app"),
                version_seen: "7.31.5".into(),
                asar_hash_before: "sha256:before".into(),
                asar_hash_after: "sha256:after".into(),
                patched_at: Utc::now(),
            }),
            patches: Some(PatchesState {
                active_version: "2026-05-14".into(),
                active_source: PatchSource::Builtin,
                remote_pubkey_fingerprint: None,
            }),
        };
        save(tmp.path(), &s).unwrap();
        let loaded = load(tmp.path()).unwrap();
        assert_eq!(loaded.schema, 1);
        assert_eq!(loaded.feishu.as_ref().unwrap().version_seen, "7.31.5");
        assert_eq!(loaded.patches.as_ref().unwrap().active_source, PatchSource::Builtin);
    }

    #[test]
    fn forward_compatible_with_unknown_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let raw = r#"{
            "schema": 1,
            "unknown_top_field": "ignored",
            "feishu": {
                "path": "/Applications/Lark.app",
                "version_seen": "7.31.5",
                "asar_hash_before": "x",
                "asar_hash_after": "y",
                "patched_at": "2026-05-14T08:30:00Z",
                "future_field": 42
            }
        }"#;
        fs::write(state_file(tmp.path()), raw).unwrap();
        let s = load(tmp.path()).unwrap();
        assert_eq!(s.feishu.as_ref().unwrap().version_seen, "7.31.5");
    }
}
