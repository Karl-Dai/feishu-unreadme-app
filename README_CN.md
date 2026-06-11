<div align="center">

# 🙈 feishu-unreadme-app

**屏蔽飞书已读回执 —— 跨平台桌面补丁工具,一键打补丁、一键恢复、自动更新。**

[![Release](https://img.shields.io/github/v/release/Karl-Dai/feishu-unreadme-app?label=release&color=2ea043)](https://github.com/Karl-Dai/feishu-unreadme-app/releases)
[![Downloads](https://img.shields.io/github/downloads/Karl-Dai/feishu-unreadme-app/total?color=1f6feb)](https://github.com/Karl-Dai/feishu-unreadme-app/releases)
[![Stars](https://img.shields.io/github/stars/Karl-Dai/feishu-unreadme-app?color=e3b341)](https://github.com/Karl-Dai/feishu-unreadme-app/stargazers)
[![License: MIT](https://img.shields.io/badge/License-MIT-lightgrey.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20·%20macOS%20·%20Linux-informational)]()

基于 **Rust** · **Tauri 2** · **Svelte 5**

[English](README.md) · **中文**

![主界面:locate、补丁状态、更新器与日志抽屉](docs/screenshots/main.png)

</div>

---

## 为什么做这个

飞书会把"已读"状态回传给发送方,而且没有任何开关可以关掉。本工具直接给飞书桌面客户端打补丁,让已读回执永远不会发出:

- 🖱️ **一键补丁** —— 自动定位飞书安装位置,就地修改 `messenger.asar`,几秒完成。
- 🔍 **状态感知** —— 自动识别当前飞书是未补丁、已补丁、被升级覆盖,还是规则不兼容。
- 🛟 **安全可回退** —— 首次打补丁自动备份原始 `messenger.asar`,一键恢复(或两条命令手动恢复)。
- 🔄 **应用内自更新** —— 检查 GitHub Releases,minisign 签名校验通过后自动安装并重启。
- 🌏 **双语终端风界面** —— 简体中文 / English,运行时随时切换。

## 目录

- [下载](#下载)
- [快速上手](#快速上手)
- [状态对照](#状态对照)
- [飞书升级后](#飞书升级后)
- [出问题恢复](#出问题恢复)
- [自更新与日志](#自更新与日志)
- [从源码构建](#从源码构建)
- [架构](#架构)
- [与原仓库的关系](#与原仓库的关系)
- [参与贡献](#参与贡献)
- [更新日志](#更新日志)
- [macOS 首次启动](#macos-首次启动)
- [License](#license)

## 下载

各平台安装包见 **[Releases 页面](https://github.com/Karl-Dai/feishu-unreadme-app/releases)**。

| 平台 | 文件 | 备注 |
|---|---|---|
| macOS Apple Silicon | `feishu-unreadme-app_*_aarch64.dmg` | M1/M2/M3 |
| macOS Intel | `feishu-unreadme-app_*_x64.dmg` | |
| Windows | `feishu-unreadme-app_*_x64-setup.exe` | 推荐,NSIS 安装器 |
| Windows (企业部署) | `feishu-unreadme-app_*_x64_en-US.msi` | |
| Linux (Debian/Ubuntu) | `feishu-unreadme-app_*_amd64.deb` | |
| Linux (Fedora/RHEL) | `feishu-unreadme-app-*.x86_64.rpm` | |

macOS 用户首次启动需要[多做一步](#macos-首次启动)。

### 国内镜像

国内访问 GitHub Releases 可能不稳定,推荐通过镜像直接下载安装包:

- <https://ghfast.top/https://github.com/Karl-Dai/feishu-unreadme-app/releases/latest>

## 快速上手

从安装到屏蔽已读回执,只需四步。

### 1 · 彻底退出飞书

- **macOS**:顶部菜单 → 退出飞书,或 `Cmd+Q`。**直接关闭窗口不算**,飞书会留后台进程,占用 `messenger.asar`。
- **Windows**:系统托盘飞书图标右键 → 退出。

### 2 · 启动 feishu-unreadme-app

`feishu/locate` 一栏会自动扫描安装位置:

- 显示 `[ 就绪 ]`:成功识别,继续下一步。
- 显示 `[ 未找到 ]`:点 `[ 选择目录 ]`,手动定位 `Feishu.app` 或飞书安装目录。

### 3 · 一键补丁

点 `[ 一键补丁 ]`。`patch/status` 会从 `[ 未补丁 ]` → `[ 处理中 ]` → `[ 已补丁 ]`,详情显示 `飞书 <版本> · 规则 <日期>`。

### 4 · 重启飞书

未读消息小红点不再出现。

## 状态对照

`patch/status` 状态徽章及对应操作:

| 徽章 | 含义 | 该做什么 |
|---|---|---|
| `[ 未补丁 ]` | 飞书原始状态 / 从未打过补丁 | 点 `[ 一键补丁 ]` |
| `[ 已补丁 ]` | 当前飞书已生效补丁 | 无需操作 |
| `[ 需重跑 ]` | 飞书升级覆盖了之前的补丁 | 重新点 `[ 一键补丁 ]` |
| `[ 不兼容 ]` | 飞书新版本规则未命中 | 等待本工具发新版,或自行提 issue |
| `[ 处理中 ]` | 正在写入 | 等几秒 |
| `[ 探测中 ]` | 初次启动尚未读取状态 | 等几秒 |

## 飞书升级后

飞书会自动后台更新,任意一次更新都会覆盖补丁。重启飞书后回到 feishu-unreadme-app,如果显示 `[ 需重跑 ]`,再点一次 `[ 一键补丁 ]` 即可,无需重装。

## 出问题恢复

### 在应用内恢复

`patch/status` 卡片下方点 `[ 恢复备份 ]`,会把首次打补丁时备份的 `messenger.asar.bak` 还原为 `messenger.asar`。

### 手动恢复(飞书打不开时)

```bash
# macOS
cd "/Applications/Feishu.app/Contents/Frameworks/Lark Framework.framework/Resources/webcontent/"
mv messenger.asar messenger.asar.patched
mv messenger.asar.bak messenger.asar

# Windows (PowerShell)
cd "$env:LOCALAPPDATA\Feishu\resources"
Move-Item messenger.asar messenger.asar.patched
Move-Item messenger.asar.bak messenger.asar
```

## 自更新与日志

- **更新器** —— 底部 `updater/check` 卡片自动检查 GitHub Releases 上的 `latest.json`。`[ 有新版 ]` 时显示 `当前 v0.1.x → 远端 v0.1.y`,点 `[ 下载并安装 ]`,minisign 签名校验通过后自动安装并重启。
- **日志** —— 主界面底部 `tail var/log/feishu-unreadme` 抽屉可展开,实时显示 `INFO` / `WARN` / `ERROR` 三级日志。提 issue 时请附上 `ERROR` 行截图。
- **语言** —— 顶部 meta 行末尾 `语言 中 | EN`,点击切换,选择写入 localStorage,关闭重开记住。

## 从源码构建

### 前置依赖

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 18+ 与 [pnpm](https://pnpm.io/)

### 步骤

```bash
pnpm install
pnpm tauri dev
```

### 跑测试

```bash
cd src-tauri && cargo test
```

## 架构

```
feishu-unreadme-app/
├── src-tauri/src/
│   ├── core/
│   │   ├── asar.rs            # asar 解析/解包/重打包
│   │   ├── patcher.rs         # 多锚点倒序 payload 插入
│   │   ├── patch_source.rs    # 内置 patches.builtin.json 加载
│   │   ├── feishu_locator.rs  # 飞书安装位置探测
│   │   ├── state.rs           # state.json schema v1 读写
│   │   ├── orchestrator.rs    # apply_patch / restore_backup 原子序列
│   │   └── error.rs           # AppError 枚举(IPC 可序列化)
│   └── commands.rs            # Tauri command 层
└── src/                       # Svelte 5 + SvelteKit static 前端
```

| 层 | 技术栈 |
|---|---|
| 后端 | Rust, Tauri 2 |
| 前端 | Svelte 5, SvelteKit (static), TypeScript, Vite |
| 更新器 | Tauri updater 插件,minisign 签名 |

完整设计文档:[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) · [docs/spec.md](docs/spec.md)

## 与原仓库的关系

由 Python CLI 版的 [`feishu-unreadme`](https://github.com/starccy/feishu-unreadme) 演化而来,原仓库继续可用但不再新增功能。

## 参与贡献

欢迎 issue 和 PR。提交代码改动前请确保 `cd src-tauri && cargo test` 通过。

## 更新日志

见 [CHANGELOG.md](CHANGELOG.md) 或 [Releases 页面](https://github.com/Karl-Dai/feishu-unreadme-app/releases)。

## macOS 首次启动

安装包**未经 Apple 公证**(没有付费开发者账号),首次启动可能被 Gatekeeper 拦截。执行一次:

```bash
xattr -dr com.apple.quarantine /Applications/feishu-unreadme-app.app
```

或走图形界面:打开 *系统设置 → 隐私与安全性*,滚动到底部,点 *仍要打开*。之后的启动不再拦截。

## License

[MIT](LICENSE)
