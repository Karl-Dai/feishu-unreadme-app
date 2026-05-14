# feishu-unreadme-app

跨平台 GUI 版的飞书已读回执屏蔽工具,用 Tauri + Svelte + Rust 实现,支持 GitHub Releases 自更新。

## 使用

1. 从 [Releases](https://github.com/Karl-Dai/feishu-unreadme-app/releases) 下载对应平台的安装包。
2. macOS 首次启动可能被 Gatekeeper 拦截,执行一次:
   ```bash
   xattr -dr com.apple.quarantine /Applications/feishu-unreadme-app.app
   ```
3. 启动应用,**先彻底退出飞书**,点「一键补丁」。
4. 重启飞书。
5. 如出现异常,在应用中点「恢复备份」,或手动把 `messenger.asar.bak` 重命名回 `messenger.asar`。

## 开发

```bash
pnpm install
pnpm tauri dev
```

跑测试:

```bash
cd src-tauri && cargo test
```

## 项目结构

详见 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md);完整设计见 [docs/spec.md](docs/spec.md)。

## 与原仓库的关系

由 Python CLI 版的 [`feishu-unreadme`](https://github.com/starccy/feishu-unreadme) 演化而来,原仓库继续可用但不再新增功能。

## License

MIT
