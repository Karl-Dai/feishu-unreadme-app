use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn can_open_and_list_fixture() {
    use feishu_unreadme_app_lib::core::asar::Asar;
    let path = fixture("happy.asar");
    if !path.exists() {
        eprintln!("跳过:fixture 还没生成");
        return;
    }
    let asar = Asar::open(&path).expect("open");
    let files = asar.list_files();
    assert!(!files.is_empty(), "fixture 应该至少包含一个文件");
}
