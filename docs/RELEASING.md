# 发版手册

## 准备

- 工作分支干净(`git status` 无未提交)。
- 跑通 `pnpm check`、`cd src-tauri && cargo test && cargo clippy --all-targets -- -D warnings`。

## bump 版本

```bash
pnpm bump 0.x.y
```

会同步以下文件:
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `CHANGELOG.md`(在 `[Unreleased]` 上方插入新段,需要手动填 `Added/Changed/Fixed` 内容)

编辑 `CHANGELOG.md` 把新版本段的内容填好,然后:

```bash
git add -A
git commit -m "chore: release v0.x.y"
git tag v0.x.y
git push origin main --tags
```

`release.yml` 会被 tag 触发,4 平台并行构建,产物作为 **draft release** 出现在 GitHub。

## 手测冒烟清单

在 release draft 出来后,从 release assets 下载对应平台的安装包,在真实机器上跑:

- [ ] macOS:`xattr -dr com.apple.quarantine` 后能启动
- [ ] 真飞书未运行:补丁成功,重启飞书功能正常
- [ ] 真飞书运行中:点「一键补丁」看到拒绝弹窗(v0.2 才有此能力,v0.1.0 跳过)
- [ ] 应用更新链路:用旧版本 app 启动,能检测到这次的新 release 并自动安装

冒烟全过后,在 GitHub release 页面点 **Publish release**。

## 首次发版前的一次性准备

如果是把这个项目首次推到 GitHub,需要先:

1. 在 GitHub 上创建空的 `feishu-unreadme-app` 仓库(**不**勾选 "Initialize with README"),把本地 `main` 分支推上去:
   ```bash
   git remote add origin https://github.com/<owner>/feishu-unreadme-app.git
   git push -u origin main
   ```
2. 在 GitHub repo Settings → Secrets and variables → Actions 添加两个 secret:
   - `TAURI_SIGNING_PRIVATE_KEY`:本地 `.secrets/updater.key` 文件的完整内容
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`:生成密钥时设置的密码(默认是 `feishu-unreadme-dev`)
