use std::path::{Path, PathBuf};

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;

use crate::core::error::{AppError, Result};
use crate::core::feishu_locator::{self, InstallInfo};
use crate::core::orchestrator::{self, ApplyReport};
use crate::core::patch_source::{self, PatchSet};
use crate::core::state::{self, State as PersistedState};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PatchUiState {
    Unpatched,
    Patched { feishu_version: String, patch_version: String },
    Stale { feishu_version_seen: String },
    Unknown,
    Incompatible,
}

fn ctx_paths(app: &AppHandle) -> Result<(PathBuf, PathBuf)> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(format!("无法解析 app_data_dir:{e}")))?;
    let state_dir = path.join("state");
    let work_root = path.join("work");
    std::fs::create_dir_all(&state_dir).map_err(AppError::Io)?;
    std::fs::create_dir_all(&work_root).map_err(AppError::Io)?;
    Ok((state_dir, work_root))
}

#[tauri::command]
pub fn detect_feishu_install() -> Result<InstallInfo> {
    feishu_locator::detect()
}

#[tauri::command]
pub fn validate_feishu_path(path: String) -> Result<InstallInfo> {
    feishu_locator::validate(Path::new(&path))
}

#[tauri::command]
pub fn get_patch_state(app: AppHandle, install: InstallInfo) -> Result<PatchUiState> {
    let (state_dir, _) = ctx_paths(&app)?;
    let persisted = state::load(&state_dir)?;
    Ok(derive_patch_state(&install, &persisted))
}

fn derive_patch_state(install: &InstallInfo, persisted: &PersistedState) -> PatchUiState {
    let bak = install.asar_path.with_extension("asar.bak");
    let bak_exists = bak.exists();
    let Some(f) = persisted.feishu.as_ref() else {
        return if bak_exists { PatchUiState::Unknown } else { PatchUiState::Unpatched };
    };
    let Ok(current_hash) = sha256_file(&install.asar_path) else {
        return PatchUiState::Unknown;
    };
    if current_hash == f.asar_hash_after
        && install.version.as_deref() == Some(f.version_seen.as_str())
    {
        let v = persisted.patches.as_ref().map(|p| p.active_version.clone()).unwrap_or_default();
        PatchUiState::Patched {
            feishu_version: f.version_seen.clone(),
            patch_version: v,
        }
    } else if bak_exists {
        PatchUiState::Stale {
            feishu_version_seen: f.version_seen.clone(),
        }
    } else {
        PatchUiState::Unknown
    }
}

fn sha256_file(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let bytes = std::fs::read(path).map_err(AppError::Io)?;
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(format!("sha256:{:x}", h.finalize()))
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> Result<String> {
    let pkg_info = app.package_info();
    Ok(pkg_info.version.to_string())
}

#[tauri::command]
pub fn apply_patch_cmd(app: AppHandle, install: InstallInfo) -> Result<ApplyReport> {
    let (state_dir, work_root) = ctx_paths(&app)?;
    let patches = patch_source::load_builtin()?;
    orchestrator::apply_patch(&install, &patches, &state_dir, &work_root)
}

#[tauri::command]
pub fn restore_backup_cmd(app: AppHandle, install: InstallInfo) -> Result<()> {
    let (state_dir, _) = ctx_paths(&app)?;
    orchestrator::restore_backup(&install, &state_dir)
}

#[tauri::command]
pub fn get_builtin_patches() -> Result<PatchSet> {
    patch_source::load_builtin()
}

#[derive(Debug, Clone, Serialize)]
pub struct AppUpdateInfo {
    pub version: String,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn check_app_update(app: AppHandle) -> Result<Option<AppUpdateInfo>> {
    let updater = app
        .updater()
        .map_err(|e| AppError::UpdaterFailed(e.to_string()))?;
    let res = updater
        .check()
        .await
        .map_err(|e| AppError::UpdaterFailed(e.to_string()))?;
    Ok(res.map(|u| AppUpdateInfo {
        version: u.version.to_string(),
        notes: u.body.clone(),
    }))
}

#[tauri::command]
pub async fn trigger_app_update(app: AppHandle) -> Result<()> {
    let updater = app
        .updater()
        .map_err(|e| AppError::UpdaterFailed(e.to_string()))?;
    let update = updater
        .check()
        .await
        .map_err(|e| AppError::UpdaterFailed(e.to_string()))?
        .ok_or_else(|| AppError::UpdaterFailed("没有可用更新".into()))?;
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| AppError::UpdaterFailed(e.to_string()))?;
    Ok(())
}
