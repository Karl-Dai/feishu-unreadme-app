use std::fs;
use std::path::PathBuf;

use feishu_unreadme_app_lib::core::asar::Asar;
use feishu_unreadme_app_lib::core::error::AppError;
use feishu_unreadme_app_lib::core::feishu_locator::InstallInfo;
use feishu_unreadme_app_lib::core::orchestrator::{apply_patch, restore_backup};
use feishu_unreadme_app_lib::core::patch_source::load_builtin;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn fake_install(asar_path: PathBuf) -> InstallInfo {
    InstallInfo {
        install_path: asar_path.parent().unwrap().to_path_buf(),
        asar_path,
        version: Some("7.31.5-fake".into()),
        is_running: false,
    }
}

#[test]
fn happy_path_applies_and_writes_state() {
    let tmp = tempfile::tempdir().unwrap();
    let asar = tmp.path().join("messenger.asar");
    fs::copy(fixture("happy.asar"), &asar).unwrap();
    let install = fake_install(asar.clone());
    let patches = load_builtin().unwrap();
    let state_dir = tmp.path().join("state");
    let work_root = tmp.path().join("work");

    let report = apply_patch(&install, &patches, &state_dir, &work_root).unwrap();
    assert_eq!(report.patches_attempted, 2);
    assert_eq!(report.patches_hit, 2);
    assert!(report.backup_path.exists());

    // 重新打开新 asar,确认 preload.js 里有 payload 字面量
    let mut after = Asar::open(&asar).unwrap();
    let body = String::from_utf8(after.read_file("renderer/preload.js").unwrap()).unwrap();
    assert!(body.contains("__feishuAllowMeReadCount"));
}

#[test]
fn mismatch_fixture_aborts_with_patch_no_hit_and_no_change_to_original() {
    let tmp = tempfile::tempdir().unwrap();
    let asar = tmp.path().join("messenger.asar");
    fs::copy(fixture("mismatch.asar"), &asar).unwrap();
    let original_bytes = fs::read(&asar).unwrap();
    let install = fake_install(asar.clone());
    let patches = load_builtin().unwrap();
    let state_dir = tmp.path().join("state");
    let work_root = tmp.path().join("work");

    let err = apply_patch(&install, &patches, &state_dir, &work_root).unwrap_err();
    assert!(matches!(err, AppError::PatchNoHit { .. }));
    // 原 asar 字节级未变
    assert_eq!(fs::read(&asar).unwrap(), original_bytes);
    // .bak 不应残留
    assert!(!asar.with_extension("asar.bak").exists());
    assert!(!asar.with_extension("asar.bak.tmp").exists());
    assert!(!asar.with_extension("asar.new").exists());
}

#[test]
fn restore_backup_brings_original_back() {
    let tmp = tempfile::tempdir().unwrap();
    let asar = tmp.path().join("messenger.asar");
    fs::copy(fixture("happy.asar"), &asar).unwrap();
    let original_bytes = fs::read(&asar).unwrap();
    let install = fake_install(asar.clone());
    let patches = load_builtin().unwrap();
    let state_dir = tmp.path().join("state");
    let work_root = tmp.path().join("work");

    apply_patch(&install, &patches, &state_dir, &work_root).unwrap();
    let after_patch = fs::read(&asar).unwrap();
    assert_ne!(after_patch, original_bytes);

    restore_backup(&install, &state_dir).unwrap();
    let restored = fs::read(&asar).unwrap();
    assert_eq!(restored, original_bytes);
    assert!(!asar.with_extension("asar.bak").exists());
}

#[test]
fn existing_backup_blocks_apply() {
    let tmp = tempfile::tempdir().unwrap();
    let asar = tmp.path().join("messenger.asar");
    fs::copy(fixture("happy.asar"), &asar).unwrap();
    fs::write(asar.with_extension("asar.bak"), b"pre-existing").unwrap();
    let install = fake_install(asar.clone());
    let patches = load_builtin().unwrap();
    let state_dir = tmp.path().join("state");
    let work_root = tmp.path().join("work");

    let err = apply_patch(&install, &patches, &state_dir, &work_root).unwrap_err();
    assert!(matches!(err, AppError::BackupExists(_)));
}
