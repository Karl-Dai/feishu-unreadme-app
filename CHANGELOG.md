# Changelog

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与 [SemVer](https://semver.org/lang/zh-CN/)。

## [Unreleased]

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
