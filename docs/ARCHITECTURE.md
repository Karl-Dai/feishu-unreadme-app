# 架构

完整设计见 [spec.md](spec.md)。本文档只列实施层补充。

## 模块清单

| 模块 | 路径 | 职责 |
|---|---|---|
| core::asar | `src-tauri/src/core/asar.rs` | 解析/解包/重打包 asar |
| core::patcher | `src-tauri/src/core/patcher.rs` | 多锚点倒序 payload 插入 |
| core::patch_source | `src-tauri/src/core/patch_source.rs` | 内置 patches.builtin.json 加载;v0.2 加远程拉取 |
| core::feishu_locator | `src-tauri/src/core/feishu_locator.rs` | macOS MVP:`/Applications/Lark.app` + Info.plist;v0.2 补 Win/Linux/Spotlight |
| core::state | `src-tauri/src/core/state.rs` | state.json schema v1 读写 |
| core::orchestrator | `src-tauri/src/core/orchestrator.rs` | `apply_patch` / `restore_backup` 原子序列 |
| core::error | `src-tauri/src/core/error.rs` | `AppError` 枚举(IPC 可序列化) |
| commands | `src-tauri/src/commands.rs` | Tauri command 层 |
| ui | `src/` | Svelte 5 + SvelteKit static |

## Crate 命名

Tauri 2 脚手架要求 lib 与 bin 不同名,所以:

- bin: `feishu-unreadme-app`(产生最终可执行)
- lib: `feishu_unreadme_app_lib`(`run()` 入口与所有 core 模块)
- 测试代码引用:`use feishu_unreadme_app_lib::core::...`

## IPC 契约

见 spec §5.1 与 `src/lib/ipc.ts`。前端只能通过 `ipc.*` 函数发起调用,不允许直接 `invoke`。

## v0.2 待办占位

- `core::patch_source::fetch_remote` 远程拉取 + minisign 验签
- `core::feishu_locator` 补全 Windows 注册表 / Linux 多路径 / macOS Spotlight 兜底
- 飞书进程检测(`sysinfo`)+ 运行中弹窗
- Playwright 端到端测试
- 验证真实飞书 minified 代码的 regex 段数,可能需要把 builtin patches 的 `\w+\.info\(...` 升级回 `\w+\.\w+\.info\(...`
