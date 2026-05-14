use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::Serialize;
use tracing::{info, warn};

use crate::core::asar::Asar;
use crate::core::error::{AppError, Result};
use crate::core::feishu_locator::InstallInfo;
use crate::core::patch_source::PatchSet;
use crate::core::patcher::{apply_patches, PatchReport};
use crate::core::state::{self, FeishuState, PatchSource, PatchesState, State};

#[derive(Debug, Clone, Serialize)]
pub struct ApplyReport {
    pub patches_attempted: u32,
    pub patches_hit: u32,
    pub files_modified: Vec<PathBuf>,
    pub backup_path: PathBuf,
    pub hash_before: String,
    pub hash_after: String,
}

pub fn apply_patch(
    install: &InstallInfo,
    patches: &PatchSet,
    state_dir: &Path,
    work_root: &Path,
) -> Result<ApplyReport> {
    let asar_path = &install.asar_path;
    let bak_path = asar_path.with_extension("asar.bak");
    let bak_tmp = asar_path.with_extension("asar.bak.tmp");
    let new_tmp = asar_path.with_extension("asar.new");
    let unpack_dir = work_root.join("unpacked");

    if bak_path.exists() {
        return Err(AppError::BackupExists(bak_path));
    }

    info!(target: "patch", "compute hash before");
    let hash_before = sha256_file(asar_path)?;

    info!(target: "patch", "copy .asar -> .asar.bak.tmp");
    fs::copy(asar_path, &bak_tmp).map_err(AppError::Io)?;

    info!(target: "patch", "unpack .asar -> {:?}", unpack_dir);
    if unpack_dir.exists() {
        fs::remove_dir_all(&unpack_dir).map_err(AppError::Io)?;
    }
    let res: Result<PatchReport> = (|| {
        Asar::extract(asar_path, &unpack_dir)?;
        info!(target: "patch", "apply patches");
        apply_patches(&unpack_dir, &patches.patches)
    })();
    let report = match res {
        Ok(r) => r,
        Err(e) => {
            cleanup(&[&bak_tmp, &new_tmp, &unpack_dir]);
            return Err(e);
        }
    };

    info!(target: "patch", "repack -> {:?}", new_tmp);
    if let Err(e) = Asar::pack(&unpack_dir, &new_tmp) {
        cleanup(&[&bak_tmp, &new_tmp, &unpack_dir]);
        return Err(e);
    }

    info!(target: "patch", "rename bak.tmp -> bak, new -> asar");
    if let Err(e) = fs::rename(&bak_tmp, &bak_path) {
        cleanup(&[&bak_tmp, &new_tmp, &unpack_dir]);
        return Err(AppError::Io(e));
    }
    if let Err(e) = fs::rename(&new_tmp, asar_path) {
        // 至此 .bak 已经在,但 .asar 还是原始的(因为 rename 失败,原文件未被覆盖)
        // 把 .bak 还原回去,保持一致
        let _ = fs::rename(&bak_path, asar_path);
        cleanup(&[&new_tmp, &unpack_dir]);
        return Err(AppError::Io(e));
    }

    let hash_after = sha256_file(asar_path)?;

    let next_state = State {
        schema: 1,
        feishu: Some(FeishuState {
            path: install.install_path.clone(),
            version_seen: install.version.clone().unwrap_or_default(),
            asar_hash_before: hash_before.clone(),
            asar_hash_after: hash_after.clone(),
            patched_at: Utc::now(),
        }),
        patches: Some(PatchesState {
            active_version: patches.version.clone(),
            active_source: PatchSource::Builtin,
            remote_pubkey_fingerprint: None,
        }),
    };
    state::save(state_dir, &next_state)?;

    if let Err(e) = fs::remove_dir_all(&unpack_dir) {
        warn!(target: "patch", "清理 unpack_dir 失败(忽略):{e}");
    }

    Ok(ApplyReport {
        patches_attempted: report.patches_attempted,
        patches_hit: report.patches_hit,
        files_modified: report.files_modified,
        backup_path: bak_path,
        hash_before,
        hash_after,
    })
}

pub fn restore_backup(install: &InstallInfo, state_dir: &Path) -> Result<()> {
    let asar_path = &install.asar_path;
    let bak_path = asar_path.with_extension("asar.bak");
    if !bak_path.exists() {
        return Err(AppError::BackupMissing(bak_path));
    }
    fs::rename(&bak_path, asar_path).map_err(AppError::Io)?;

    let mut s = state::load(state_dir)?;
    s.feishu = None;
    s.patches = None;
    state::save(state_dir, &s)?;
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).map_err(AppError::Io)?;
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(format!("sha256:{:x}", h.finalize()))
}

fn cleanup(paths: &[&Path]) {
    for p in paths {
        if p.is_dir() {
            let _ = fs::remove_dir_all(p);
        } else if p.exists() {
            let _ = fs::remove_file(p);
        }
    }
}
