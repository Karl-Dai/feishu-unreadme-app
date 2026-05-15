# Changelog

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与 [SemVer](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [0.1.2] - 2026-05-15

### Fixed
- **asar 重打包 header 损坏** — `pack()` 的 `header_pickle_size` / `header_string_pickle_size` 漏算 readString 末尾的 4-byte 对齐 padding,Electron 严格按 sizeBuf 读 inner pickle 时算到的 string_len 落到 payload 区,解出离谱长度(GB 级),`messenger.asar` 直接判损坏,飞书消息列表/会话内容全部渲染失败。Rust 端读路径同样漏校验,所以 roundtrip 测试自洽不爆,只有真实 Electron 加载才暴露
- **patch regex 在链式调用上注入位错** — `\w+\.info\(...)` 在 `l.Ay.info(...)` 这种成员链上只匹配 `Ay.info(...)`,m.start() 落在 `Ay`,注入字符串切坏链,产物变 `l.<INJECT>Ay.info(...)` 或 `S.window.__feishuAllowMeReadCount=1,Ay.info(...)`(运行时 `S.window` undefined → TypeError)。改用 `[\w.]+\.info\(...)` 贪婪吃下整条前缀
- 内置补丁规则版本提升到 `2026-05-15`,飞书 131.x 实测点击消息可正常显示

### Added
- `pack_writes_header_pickle_size_including_string_padding` 集成测试,直接按 byte 校验 sizeBuf 字段,防 asar header 长度回归
- `matches_full_member_chain_with_dot_in_charset` 单元测试,防 regex 链式调用注入位错回归

## [0.1.1] - 2026-05-15

### Fixed
- macOS 飞书自动探测扩充 `Feishu.app` / `飞书.app` 路径,中国版用户不再需要手动选目录
- CI release 配置补全 minisign 签名 secret,产物正确生成 `latest.json`,修复客户端"Could not fetch a valid release JSON"自动更新报错

### Changed
- CI/release workflow runner `macos-13` → `macos-15-intel`,解决 Intel 档长时间排队问题

## [0.1.0] - 2026-05-14

### Added
- Tauri + Svelte 跨平台 GUI 骨架
- Rust 实现 asar 解析、重打包、正则插入式补丁、原子写
- 内置补丁规则覆盖飞书 7.31.x
- macOS `/Applications/Lark.app` 自动探测,其它平台手动选目录
- GitHub Releases 自更新(`tauri-plugin-updater`)
- 4 平台 CI 构建矩阵 + 签名发布流水线
