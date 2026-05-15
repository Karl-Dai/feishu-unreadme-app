use std::path::PathBuf;

use feishu_unreadme_app_lib::core::asar::{Asar, Node};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn happy_fixture_lists_expected_files() {
    let asar = Asar::open(&fixture("happy.asar")).expect("open");
    let mut files = asar.list_files();
    files.sort();
    assert_eq!(files, vec!["renderer/logger.js", "renderer/preload.js"]);
}

#[test]
fn happy_fixture_reads_preload_content() {
    let mut asar = Asar::open(&fixture("happy.asar")).expect("open");
    let bytes = asar.read_file("renderer/preload.js").expect("read");
    let s = std::str::from_utf8(&bytes).expect("utf8");
    assert!(s.contains("updateMessagesMeRead"));
    assert!(s.contains("onSendMessageSuccess"));
}

#[test]
fn multihit_fixture_has_four_js_files() {
    let asar = Asar::open(&fixture("multihit.asar")).expect("open");
    let files = asar.list_files();
    assert_eq!(files.len(), 5, "三个 mod + logger + send");
}

#[test]
fn malformed_header_returns_error() {
    use std::io::Write;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    // 写一个非法的 header(全 0)
    tmp.write_all(&[0u8; 32]).unwrap();
    tmp.flush().unwrap();
    let err = Asar::open(tmp.path()).err().expect("应该报错");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("AsarMalformed") || msg.contains("Io"),
        "got: {msg}"
    );
}

#[test]
fn node_enum_shape_is_used() {
    // 编译期保证 Node 枚举存在(防止后续被 dead-code 删除)
    let _ = Node::File {
        offset: Some("0".into()),
        size: 0,
        executable: false,
        unpacked: false,
    };
}

#[test]
fn pack_writes_header_pickle_size_including_string_padding() {
    // 回归测试:历史上 pack() 把 header_pickle_size 写成 header_len + 12,
    // 漏算了 readString 末尾的对齐 padding。Rust 端自己读自己写的 asar 因为
    // 读路径直接跳 16 bytes 不校验,所以 roundtrip 不爆;但 Electron / @electron/asar
    // 严格按 sizeBuf 给出的长度读 inner pickle,bug 表现为读 string_len 时
    // 落到 payload 区,解出离谱长度(GB 级)直接判 asar 损坏,飞书加载不了。
    use std::fs;
    use std::io::Read;

    let tmp = tempfile::tempdir().unwrap();
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();
    // 故意把内容长度造成 header_len % 4 != 0,触发 padding > 0
    fs::write(src.join("a.js"), "x".repeat(7)).unwrap();
    fs::write(src.join("b.js"), "y".repeat(13)).unwrap();

    let dst = tmp.path().join("out.asar");
    feishu_unreadme_app_lib::core::asar::Asar::pack(&src, &dst).unwrap();

    let mut f = fs::File::open(&dst).unwrap();
    let mut head = [0u8; 16];
    f.read_exact(&mut head).unwrap();
    let outer = u32::from_le_bytes(head[0..4].try_into().unwrap());
    let header_pickle_size = u32::from_le_bytes(head[4..8].try_into().unwrap());
    let header_string_pickle_size = u32::from_le_bytes(head[8..12].try_into().unwrap());
    let string_len = u32::from_le_bytes(head[12..16].try_into().unwrap());

    assert_eq!(outer, 4);
    let padding = (4 - (string_len % 4)) % 4;
    // inner_payload_size = 4 (string_len 字段) + string_len bytes + padding
    assert_eq!(
        header_string_pickle_size,
        4 + string_len + padding,
        "string_pickle_size 漏 padding"
    );
    // inner_buf_size = 4 (inner_payload_size 字段) + inner_payload_size
    assert_eq!(
        header_pickle_size,
        4 + header_string_pickle_size,
        "header_pickle_size 漏 padding"
    );
}

#[test]
fn pack_unpack_roundtrip_preserves_content() {
    use std::fs;

    let src = fixture("happy.asar");
    let mut original = Asar::open(&src).unwrap();
    let preload_orig = original.read_file("renderer/preload.js").unwrap();
    let logger_orig = original.read_file("renderer/logger.js").unwrap();

    // 解包到临时目录
    let tmp = tempfile::tempdir().unwrap();
    let unpack_dir = tmp.path().join("unpacked");
    fs::create_dir_all(&unpack_dir).unwrap();
    Asar::extract(&src, &unpack_dir).unwrap();
    assert!(unpack_dir.join("renderer/preload.js").exists());

    // 重打包
    let repacked = tmp.path().join("repacked.asar");
    Asar::pack(&unpack_dir, &repacked).unwrap();
    assert!(repacked.exists());

    // 读出来字节应当相等
    let mut again = Asar::open(&repacked).unwrap();
    assert_eq!(
        again.read_file("renderer/preload.js").unwrap(),
        preload_orig
    );
    assert_eq!(again.read_file("renderer/logger.js").unwrap(), logger_orig);
}
