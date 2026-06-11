<div align="center">

# 🙈 feishu-unreadme-app

**Block Feishu / Lark read receipts — a cross-platform desktop patcher with one-click apply, restore and auto-update.**

[![Release](https://img.shields.io/github/v/release/Karl-Dai/feishu-unreadme-app?label=release&color=2ea043)](https://github.com/Karl-Dai/feishu-unreadme-app/releases)
[![Downloads](https://img.shields.io/github/downloads/Karl-Dai/feishu-unreadme-app/total?color=1f6feb)](https://github.com/Karl-Dai/feishu-unreadme-app/releases)
[![Stars](https://img.shields.io/github/stars/Karl-Dai/feishu-unreadme-app?color=e3b341)](https://github.com/Karl-Dai/feishu-unreadme-app/stargazers)
[![License: MIT](https://img.shields.io/badge/License-MIT-lightgrey.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20·%20macOS%20·%20Linux-informational)]()

Built with **Rust** · **Tauri 2** · **Svelte 5**

**English** · [中文](README_CN.md)

![Main UI with locate, patch status, updater and log drawer](docs/screenshots/main.png)

</div>

---

## Why this project

Feishu (Lark) tells the sender whether you have read a message — and there is no setting to turn it off. This app patches the Feishu desktop client so the read receipt is never sent:

- 🖱️ **One-click patch** — auto-locates your Feishu install, patches `messenger.asar` in place, done in seconds.
- 🔍 **Status awareness** — knows whether the running Feishu is unpatched, patched, overwritten by an upgrade, or incompatible with the current rules.
- 🛟 **Safe by design** — the original `messenger.asar` is backed up on first patch; restore it in one click (or two shell commands).
- 🔄 **In-app auto-update** — checks GitHub Releases, verifies the minisign signature, installs and restarts.
- 🌏 **Bilingual terminal-style UI** — full English / 简体中文, switchable at runtime.

## Table of Contents

- [Download](#download)
- [Quick Start](#quick-start)
- [Status Reference](#status-reference)
- [After a Feishu Upgrade](#after-a-feishu-upgrade)
- [Recovery](#recovery)
- [Self-Update & Logs](#self-update--logs)
- [Build from Source](#build-from-source)
- [Architecture](#architecture)
- [Relationship to the Original CLI](#relationship-to-the-original-cli)
- [Contributing](#contributing)
- [Changelog](#changelog)
- [macOS First Launch](#macos-first-launch)
- [License](#license)

## Download

Pre-built installers for every platform are on the **[Releases page](https://github.com/Karl-Dai/feishu-unreadme-app/releases)**.

| Platform | Installer | Notes |
|----------|-----------|-------|
| macOS Apple Silicon | `feishu-unreadme-app_*_aarch64.dmg` | M1/M2/M3 |
| macOS Intel | `feishu-unreadme-app_*_x64.dmg` | |
| Windows | `feishu-unreadme-app_*_x64-setup.exe` | Recommended, NSIS installer |
| Windows (enterprise) | `feishu-unreadme-app_*_x64_en-US.msi` | |
| Linux (Debian/Ubuntu) | `feishu-unreadme-app_*_amd64.deb` | |
| Linux (Fedora/RHEL) | `feishu-unreadme-app-*.x86_64.rpm` | |

macOS users need [one extra step on first launch](#macos-first-launch).

### China mirror

Users in mainland China may have unstable access to GitHub Releases. Recommended mirror for direct installer downloads:

- <https://ghfast.top/https://github.com/Karl-Dai/feishu-unreadme-app/releases/latest>

## Quick Start

Four steps from install to no more read receipts.

### 1 · Quit Feishu completely

- **macOS**: menu bar → Quit Feishu, or `Cmd+Q`. Closing the window is **not enough** — Feishu keeps a background process that locks `messenger.asar`.
- **Windows**: right-click the tray icon → Quit.

### 2 · Launch feishu-unreadme-app

The `feishu/locate` card auto-scans known install locations:

- `[ 就绪 ]` (Ready) — install detected, move on.
- `[ 未找到 ]` (Not found) — click `[ 选择目录 ]` (Choose directory) and point it at `Feishu.app` or the Feishu install folder.

### 3 · One-click patch

Click `[ 一键补丁 ]` (Patch). The `patch/status` badge walks through `[ 未补丁 ]` → `[ 处理中 ]` → `[ 已补丁 ]`, and the detail line shows `飞书 <version> · 规则 <date>` (Feishu version · rule date).

### 4 · Restart Feishu

Unread-message red dots no longer appear for the sender.

## Status Reference

The `patch/status` badge and what to do about it:

| Badge | Meaning | Action |
|-------|---------|--------|
| `[ 未补丁 ]` Unpatched | Pristine Feishu / never patched | Click `[ 一键补丁 ]` |
| `[ 已补丁 ]` Patched | Patch is active | Nothing |
| `[ 需重跑 ]` Re-run needed | A Feishu upgrade overwrote the patch | Click `[ 一键补丁 ]` again |
| `[ 不兼容 ]` Incompatible | New Feishu version, rules don't match | Wait for a new release, or file an issue |
| `[ 处理中 ]` Working | Writing the patch | Wait a few seconds |
| `[ 探测中 ]` Probing | First launch, status not read yet | Wait a few seconds |

## After a Feishu Upgrade

Feishu auto-updates in the background, and any update overwrites the patch. After restarting Feishu, come back to feishu-unreadme-app — if it shows `[ 需重跑 ]`, click `[ 一键补丁 ]` once more. No reinstall needed.

## Recovery

### In-app restore

Click `[ 恢复备份 ]` (Restore backup) under the `patch/status` card — it restores the `messenger.asar.bak` saved on first patch.

### Manual restore (if Feishu won't start)

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

## Self-Update & Logs

- **Updater** — the `updater/check` card polls `latest.json` on GitHub Releases. `[ 有新版 ]` (New version) shows `current v0.1.x → remote v0.1.y`; click `[ 下载并安装 ]` (Download & install) and the bundle is minisign-verified, installed and relaunched.
- **Logs** — the `tail var/log/feishu-unreadme` drawer at the bottom streams live `INFO` / `WARN` / `ERROR` logs. When filing an issue, include a screenshot of the `ERROR` lines.
- **Language** — `语言 中 | EN` at the end of the top meta row toggles the UI language; the choice persists via localStorage.

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 18+ and [pnpm](https://pnpm.io/)

### Steps

```bash
pnpm install
pnpm tauri dev
```

### Run tests

```bash
cd src-tauri && cargo test
```

## Architecture

```
feishu-unreadme-app/
├── src-tauri/src/
│   ├── core/
│   │   ├── asar.rs            # asar parse / unpack / repack
│   │   ├── patcher.rs         # multi-anchor reverse-order payload insertion
│   │   ├── patch_source.rs    # built-in patches.builtin.json loading
│   │   ├── feishu_locator.rs  # Feishu install discovery
│   │   ├── state.rs           # state.json schema v1 read/write
│   │   ├── orchestrator.rs    # apply_patch / restore_backup atomic sequences
│   │   └── error.rs           # AppError enum (IPC-serializable)
│   └── commands.rs            # Tauri command layer
└── src/                       # Svelte 5 + SvelteKit static frontend
```

| Layer | Stack |
|-------|-------|
| Backend | Rust, Tauri 2 |
| Frontend | Svelte 5, SvelteKit (static), TypeScript, Vite |
| Updater | Tauri updater plugin, minisign-signed bundles |

Full design docs: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) · [docs/spec.md](docs/spec.md)

## Relationship to the Original CLI

Evolved from the Python CLI [`feishu-unreadme`](https://github.com/starccy/feishu-unreadme). The original repo still works but no longer gains new features.

## Contributing

Issues and pull requests are welcome. For a code change, please make sure `cd src-tauri && cargo test` passes before opening a PR.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) or the [Releases page](https://github.com/Karl-Dai/feishu-unreadme-app/releases).

## macOS First Launch

The bundles are **not Apple-notarized** (no paid Developer Program), so Gatekeeper may block the first launch. Run once:

```bash
xattr -dr com.apple.quarantine /Applications/feishu-unreadme-app.app
```

Or via the GUI: open *System Settings → Privacy & Security*, scroll to the bottom, click *Open Anyway*. Subsequent launches go straight through.

## License

[MIT](LICENSE)
