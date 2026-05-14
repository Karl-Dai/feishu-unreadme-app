# feishu-unreadme-app 设计文档

- **日期:** 2026-05-14
- **作者:** Changyu Dai
- **状态:** Approved (brainstorming → 待写实施计划)
- **目标仓库:** 新建 GitHub 仓库 `feishu-unreadme-app`(旧 CLI 仓库 `feishu-unreadme` 保留并在 README 指向新仓)

## 1. 背景与目标

现有 `feishu-unreadme` 是一个 Python CLI 脚本,通过解包/正则插入/重打包 `messenger.asar` 来屏蔽飞书前端的已读回执上报。两个核心痛点:

1. CLI 对非开发者用户门槛高。
2. 飞书更新会覆盖修改,且补丁锚点会随版本失效;每次都要用户手动重跑、手动从 GitHub 拉新版脚本。

本项目把它做成一个跨平台 GUI 应用,提供:

- 一键打补丁 / 恢复备份;
- 自动探测飞书安装路径与运行状态;
- 飞书升级后自动提示重跑;
- **应用自身**通过 GitHub Releases 自更新;
- **补丁规则** 通过 GitHub Releases 增量下发(无需重发 app 版本)。

## 2. 技术选型(已敲定)

| 维度 | 决策 |
|---|---|
| 桌面框架 | **Tauri 2.x**(包体小、内置 GitHub Releases updater) |
| 前端 | **Svelte 5 + SvelteKit static adapter** |
| 后端 | **Rust**,补丁核心逻辑(asar 解析/重打包/正则插入)用 Rust 重写,不再依赖 Python |
| 目标平台 | Windows x64、macOS arm64、macOS x64、Linux x64(macOS 两个架构分别构建独立产物,不用 universal binary,避免包体翻倍) |
| UI 语言 | 简体中文(不做 i18n) |
| 更新签名 | Tauri minisign 私钥;**应用更新包**与**远程补丁包**共用同一密钥对,公钥编进 binary,私钥存 GitHub Actions secret |
| 补丁规则交付 | 内置 + 启动时拉远程(签名校验通过才采用,否则 fallback) |
| 自动更新 UX | 启动检测 → 提示横幅 → 用户点击「下载并安装」 |
| 进程冲突处理 | 主动检测飞书进程,运行中则提示用户手动退出(不自动 kill) |

## 3. 架构总览

### 3.1 模块边界

| 模块 | 职责 | 主要依赖 |
|---|---|---|
| `core::asar` | asar 文件的解析、解包、重打包 | std::fs, byteorder |
| `core::patcher` | 加载 patches、匹配锚点、应用 payload、备份/恢复 | core::asar, regex |
| `core::feishu_locator` | 探测飞书安装路径(各平台默认位置)、读取飞书版本号、检测飞书进程 | sysinfo, plist/winreg |
| `core::patch_source` | 内置 patches.json + 启动时拉远程并做签名校验 | reqwest, minisign-verify |
| `core::state` | 本地状态持久化(已补丁的 asar hash、飞书版本号、上次远程 patches 版本) | serde_json, dirs |
| `app::commands` | Tauri command 层,把上述能力暴露给前端,做参数校验和错误归一化 | tauri |
| `ui (Svelte)` | 仪表盘视图、状态轮询、操作触发 | svelte |
| `updater` | 直接复用 Tauri 官方 updater 插件 | tauri-plugin-updater |

### 3.2 进程模型

- **前端(WebView):** Svelte UI,只负责展示与用户交互,**不直接碰文件系统、不直接发网络请求**。
- **Rust core:** 所有特权操作(路径扫描、进程检测、asar 解包/重打包、写文件、本地状态持久化、远程补丁包下载与签名校验、updater 触发)。
- 二者通过 Tauri 的 `invoke` IPC 通信,所有写操作均从前端发起、由 Rust 校验后执行。

## 4. UI / 仪表盘

单页面,纵向四个卡片 + 底部可折叠日志抽屉:

```
┌─ feishu-unreadme ─────────────────────[设置]─[×]┐
│                                                │
│ ┌─ 飞书安装 ────────────────────────────────┐  │
│ │ 路径:/Applications/Lark.app          [改] │  │
│ │ 版本:7.31.5                                │  │
│ │ 进程状态:● 运行中  [打开飞书 / 退出飞书]   │  │
│ └────────────────────────────────────────────┘  │
│                                                │
│ ┌─ 补丁状态 ────────────────────────────────┐  │
│ │ ● 已补丁(对应飞书 7.31.5)                 │  │
│ │ 补丁规则版本:patches@2026-05-10            │  │
│ │ [一键补丁]   [恢复备份]                    │  │
│ └────────────────────────────────────────────┘  │
│                                                │
│ ┌─ 补丁规则更新 ────────────────────────────┐  │
│ │ 远程:patches@2026-05-12 ●新              │  │
│ │ [拉取最新规则]                              │  │
│ └────────────────────────────────────────────┘  │
│                                                │
│ ┌─ 应用更新 ────────────────────────────────┐  │
│ │ 当前 v0.3.1   GitHub: v0.4.0 ●新           │  │
│ │ [下载并安装]                                │  │
│ └────────────────────────────────────────────┘  │
│                                                │
│ 日志(底部可折叠抽屉,纯文本流)               │
└────────────────────────────────────────────────┘
```

### 4.1 补丁状态机

| State | 含义 | 触发条件 |
|---|---|---|
| `Unpatched` | 未补丁 | 检测不到 .bak,且当前 .asar 没有插桩特征 |
| `Patched` | 已补丁且飞书未升级 | 当前 .asar hash == state 中记录的"补丁后 hash" |
| `Stale` | 已补丁但飞书已升级 | .bak 存在,但 .asar hash 与 state 不符,且飞书版本号变化 |
| `Unknown` | 状态文件丢失或异常 | state 缺失但 .bak 存在 |
| `Incompatible` | 飞书新版本规则未命中 | 一键补丁尝试时命中数为 0 |

### 4.2 关键交互

- 启动即触发四件事并发:路径探测 + 状态评估 + 规则远程检查 + app 远程检查,UI 边查边渲染。
- 「一键补丁」前若检测到飞书进程在运行,弹模态:"飞书正在运行,请退出后继续"(不主动 kill),用户退出后弹窗自动消失。
- 「恢复备份」前弹二次确认。
- 日志抽屉显示 Rust 端 `tracing` 输出的人类可读条目。

## 5. 数据流与持久化

### 5.1 Tauri command 契约

```rust
// 同步、便宜的查询
detect_feishu_install() -> InstallInfo { path, version, is_running }
get_patch_state()       -> PatchState  { state, hash, patch_version, feishu_version }
get_app_version()       -> { current: String }

// 网络查询(异步,有 timeout)
check_remote_patches()  -> Option<RemotePatchInfo { version, url, signature }>
check_app_update()      -> Option<AppUpdateInfo  { version, notes, download_url }>

// 写操作(均需前端用户主动触发,Rust 端再次校验前置条件)
apply_patch()           -> Result<ApplyReport>
restore_backup()        -> Result<()>
fetch_remote_patches()  -> Result<PatchSetVersion>
trigger_app_update()    -> Result<()>

// 流式
subscribe_logs()        -> tauri::Event stream
```

`ApplyReport` 包含 `patches_attempted / patches_hit / files_modified / backup_path`,前端在卡片下方折叠显示。

### 5.2 本地状态文件

路径:`$APPDATA/feishu-unreadme-app/state.json`(各平台用 Tauri 的 `app_data_dir`)。

```json
{
  "schema": 1,
  "feishu": {
    "path": "/Applications/Lark.app/Contents/Resources/...",
    "version_seen": "7.31.5",
    "asar_hash_before": "sha256:abc...",
    "asar_hash_after":  "sha256:def...",
    "patched_at": "2026-05-14T08:30:00Z"
  },
  "patches": {
    "active_version": "2026-05-12",
    "active_source":  "remote",
    "remote_pubkey_fingerprint": "..."
  }
}
```

### 5.3 补丁包结构(patches.json)

```json
{
  "version": "2026-05-12",
  "min_app_version": "0.3.0",
  "patches": [
    {
      "id":      "block-updateMessagesMeRead",
      "regex":   "\\w+\\.\\w+\\.info\\(\"updateMessagesMeRead\"",
      "payload": "(window.__feishuAllowMeReadCount>0?window.__feishuAllowMeReadCount--:t.messageIds=[]),",
      "required_hits": 1
    },
    {
      "id":      "allow-on-send-success",
      "regex":   "\\w+\\.\\w+\\.info\\(\"MessageService::sendMessage:onSendMessageSuccess:\"",
      "payload": "window.__feishuAllowMeReadCount=1,",
      "required_hits": 1
    }
  ]
}
```

### 5.4 远程补丁分发(安全模型)

托管位置:`patches.json` 与 `patches.json.minisig` 作为附件,附在**独立的 patches release** 上(以 `patches-YYYY-MM-DD` 作为 tag,与 app 版本 `vX.Y.Z` 互不干扰)。客户端通过 GitHub Releases API 取最新满足 `tag_name` 前缀 `patches-` 的 release 的资源 URL,再下载:

```
GET https://api.github.com/repos/<owner>/feishu-unreadme-app/releases
→ 过滤 tag 前缀为 "patches-" 的最新一项
→ 下载该 release 的 patches.json + patches.json.minisig
```

之所以不用 `releases/latest/download/...`:GitHub 的 "latest" 指的是 release 列表里的最新一项,会被 app release 抢占;独立的 tag 命名空间让 app 发版与补丁发版完全解耦。

**强制规则:**

- 客户端必须用打包时编进 binary 的 minisign 公钥校验 `.minisig`,与 Tauri updater 共用**同一对密钥**。
- 校验失败 → 丢弃下载内容,fallback 到内置 patches,UI 红色提示"远程补丁签名校验失败"。**不静默兜底为未签名版本。**
- 版本回退保护:`new.version <= state.active_version` 时忽略。
- payload 字段做简单白名单/黑名单正则审查(纵深防御,不替代签名)。

### 5.5 飞书路径自动探测的平台默认值

| 平台 | 探测顺序 |
|---|---|
| macOS | `/Applications/Lark.app`,`~/Applications/Lark.app`,然后 bundle id `com.electron.lark` |
| Windows | `%LOCALAPPDATA%\Lark`,`%PROGRAMFILES%\Lark`,注册表 `HKCU\Software\Lark\InstallLocation` |
| Linux | `/opt/bytedance/lark`,`/usr/lib/lark`,`~/.local/share/lark` |

探测失败 → 退化为"手动选目录"。

## 6. 错误处理

### 6.1 错误模型

```rust
pub enum AppError {
    FeishuNotFound(Vec<PathBuf>),
    FeishuRunning,
    AsarLocked(PathBuf),
    AsarMalformed(String),
    BackupExists(PathBuf),
    BackupMissing(PathBuf),
    PatchNoHit { patch_id: String },
    PatchVersionIncompatible { required: String, current: String },
    RemoteFetchFailed(String),
    RemoteSignatureInvalid,
    RemoteVersionRegression,
    PermissionDenied(PathBuf),
    UpdaterFailed(String),
    Io(std::io::Error),
}
```

### 6.2 核心原则

1. **任何写操作失败必须可回滚。** `apply_patch` 执行序列:

   ```
   detect_running → 拒绝
   compute_hash_before
   copy .asar → .asar.bak.tmp
   unpack .asar → temp_unpack_dir
   apply patches → 验证每条 required_hits 满足
   repack temp_unpack_dir → .asar.new
   atomic_rename .asar.bak.tmp → .asar.bak
   atomic_rename .asar.new    → .asar
   compute_hash_after → 写 state.json
   清理 temp_unpack_dir
   ```

   任何一步失败都不污染原 `.asar`;失败时清理 `.asar.bak.tmp` / `.asar.new` / `temp_unpack_dir`。

2. **不静默兜底。** `RemoteSignatureInvalid` 不退化为"使用未签名版本";`PatchNoHit` 不退化为"少打一条"。两者均直接 abort 并把 UI 状态卡到 `Incompatible` / 错误条。

3. **远程操作严格 timeout 与重试:** 拉 `patches.json` 总超时 10s,重试 1 次。失败不阻塞 UI 启动(仅卡片显示离线)。

4. **日志分级:** Rust 用 `tracing`,`INFO` 走日志抽屉,`DEBUG` 落到 `$APPDATA/.../logs/app.log` 滚动(单文件 5MB,保留 3 份)。UI 只显示人类可读文案 + "复制诊断信息"按钮。

5. **不收集遥测、不上报。** 出错信息只在本机日志,用户手动附 issue。

6. **前端层错误兜底:** 所有 `invoke` 用统一 wrapper,把 `Err` 转成顶层 toast + 卡片 inline 错误,不让异常导致页面白屏。

## 7. 测试策略

### 7.1 Rust 单元测试

| 模块 | 关注点 |
|---|---|
| `asar` | round-trip(unpack→repack 结构等价)、损坏 header 报 `AsarMalformed`、中文文件名条目正常 |
| `patcher` | 多锚点倒序插入正确 offset、`required_hits` 不满足返回 `PatchNoHit`、payload 含非 ASCII 字节正常 |
| `feishu_locator` | 版本号从 `package.json`/`Info.plist`/registry 三种结构正确解析(mock 文件系统) |
| `patch_source` | 远程签名校验通过/失败/版本回退三种路径;载入畸形 JSON 报错而非 panic |
| `state` | schema v1 读写、缺字段时给默认、forward-compatible(未知字段保留) |

### 7.2 集成测试

`src-tauri/tests/` 下准备三份 mock `messenger.asar`:

- `feishu-7.31.5.asar`: happy path,两条规则各 1 次命中。
- `feishu-mismatch.asar`: 锚点改名,验证 `PatchNoHit`。
- `feishu-multihit.asar`: 同锚点在多个 .js 文件出现 3 次,验证全部插入且 offset 不错位。

`apply_patch` 端到端:临时目录 mock asar → 执行 → 重新解包 → 抓修改后 JS → 断言插入位置与字节正确。

**原子回滚测试:** 在 repack 前注入 mock writer panic,断言原始 `.asar` 字节级未变、`.bak` 也被清掉。

### 7.3 前端测试

不写 Svelte 组件单测。改用 Playwright 跑 Tauri 应用本身:

- 启动 → 主屏出现路径卡片;
- mock command 返回 `FeishuRunning` → 点"一键补丁"出弹窗;
- mock command 返回各 `PatchState` → 卡片显示对应文案。

Playwright fixture 用 Tauri 的 mock IPC,不需要真飞书。**v0.1.0 MVP 暂不实现,v0.2+ 引入。**

### 7.4 CI(GitHub Actions)

- 矩阵:Windows x64 / macOS arm64 / macOS x64 / Linux x64 共 4 个 job。
- 每个 job:`cargo test` → `cargo clippy -D warnings` → `cargo fmt --check` → `pnpm test` → `cargo tauri build`。
- Release job:tag 推送(`v*`)触发,build → minisign 签名 → `tauri-action` 上传到 release。
- `patches.json` 独立 workflow `patches-publish.yml`:手动触发,签名后上传 `patches.json` + `patches.json.minisig` 到 latest release。

### 7.5 手工冒烟清单(每个 release 必跑)

- macOS:`xattr -dr com.apple.quarantine` 后能启动。
- 真飞书未运行:补丁成功,重启飞书功能正常。
- 真飞书运行中:点"一键补丁"看到拒绝弹窗。
- 拉远程补丁:篡改 `.minisig` 一字节,客户端拒绝并 fallback。
- 应用更新:发一个 `v0.0.1-test` 的 prerelease,本地能检出并安装。

## 8. 项目结构

```
feishu-unreadme-app/
├── README.md
├── LICENSE
├── .gitignore
├── package.json
├── pnpm-workspace.yaml
├── tsconfig.json
├── vite.config.ts
├── svelte.config.js
├── src/                         # Svelte 前端
│   ├── routes/
│   │   ├── +layout.svelte
│   │   └── +page.svelte
│   ├── lib/
│   │   ├── ipc.ts
│   │   ├── stores/
│   │   │   ├── feishu.ts
│   │   │   ├── patch.ts
│   │   │   ├── remote_patches.ts
│   │   │   └── app_update.ts
│   │   └── components/
│   │       ├── FeishuCard.svelte
│   │       ├── PatchCard.svelte
│   │       ├── RemotePatchCard.svelte
│   │       ├── AppUpdateCard.svelte
│   │       └── LogDrawer.svelte
│   └── app.html
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   ├── resources/
│   │   ├── patches.builtin.json
│   │   └── updater_pubkey.minisig.pub
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   └── core/
│   │       ├── mod.rs
│   │       ├── error.rs
│   │       ├── asar.rs
│   │       ├── patcher.rs
│   │       ├── feishu_locator.rs
│   │       ├── patch_source.rs
│   │       └── state.rs
│   └── tests/
│       ├── fixtures/
│       │   ├── feishu-7.31.5.asar
│       │   ├── feishu-mismatch.asar
│       │   └── feishu-multihit.asar
│       └── apply_patch_e2e.rs
├── e2e/                              # v0.2+
│   ├── playwright.config.ts
│   └── tests/
│       └── dashboard.spec.ts
├── docs/
│   ├── ARCHITECTURE.md
│   ├── RELEASING.md
│   └── SECURITY.md
└── .github/
    └── workflows/
        ├── ci.yml
        ├── release.yml
        └── patches-publish.yml
```

### 8.1 Rust 依赖

```
tauri = "2"
tauri-plugin-updater = "2"
tauri-plugin-dialog  = "2"
tauri-plugin-shell   = "2"
tauri-plugin-process = "2"
serde / serde_json   = "1"
sha2                 = "0.10"
regex                = "1"
sysinfo              = "0.32"
reqwest              = "0.12"     # rustls TLS
minisign-verify      = "0.2"
tracing / tracing-subscriber = "0.3"
dirs                 = "5"
plist                = "1"
winreg               = "0.52"     # cfg(windows)
```

## 9. 发布流程

- 版本号统一三处:`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`。用一个 npm script `bump-version <ver>` 同步,参考已有的 `release` skill。
- 步骤:
  1. `pnpm bump 0.x.y` → 三处版本号 + `CHANGELOG.md`。
  2. `git commit -am "chore: release v0.x.y"` → `git tag v0.x.y` → `git push origin main --tags`。
  3. `release.yml` 在 tag 推送时启动,矩阵构建 4 平台 → `tauri-action` 生成 release 草稿 + 签名后产物 + updater 用的 `latest.json`。
  4. 手测冒烟清单通过后,GitHub 网页 Publish。
  5. 已发布的 app 下次启动检测 `latest.json` 并提示用户。
- `patches.json` 独立轨道:更新规则时改 `resources/patches.builtin.json`,手动触发 `patches-publish.yml`,workflow 会新建一个 `patches-YYYY-MM-DD` tag/release 并附上签名后的 `patches.json` + `patches.json.minisig`。客户端按 §5.4 描述的方式按 tag 前缀查询最新一项。

## 10. MVP 切片(v0.1.0)

**包含:**

- Tauri + Svelte 骨架。
- 手动选目录(自动探测仅 macOS `/Applications/Lark.app` 一处兜底)。
- Rust 实现 asar 解析/重打包 + patcher。
- 内置 patches.builtin.json。
- 补丁卡片 + 备份恢复卡片 + 应用更新卡片(三张卡)。
- updater 接入。
- 4 平台 release.yml 跑通。
- Rust 单元测试 + 集成测试。

**v0.2.0 再做:**

- 远程补丁拉取(`core::patch_source` + 远程卡片)。
- 飞书路径自动探测的注册表 / Spotlight / Linux 多路径兜底。
- Playwright e2e。
- 飞书运行进程检测 & 提示弹窗(若 MVP 来得及可前置)。

主干一刀切干净:**v0.1.0 把"能发版且能自更新"这条主干跑通**,后续增量上能力。

## 11. 与旧仓库的关系

- 新仓库 `feishu-unreadme-app` 独立开,沿用 MIT 许可证。
- 旧仓库 `feishu-unreadme` 保留可用(给纯 CLI 用户),在 README 顶部加一段指向新仓库;不再加新功能,仅修关键 bug。
- 新仓库 README 顶部声明"由 `feishu-unreadme` 演化而来",并附原仓库链接致谢。

## 12. 风险与待跟踪问题

| 风险 | 影响 | 缓解 |
|---|---|---|
| 远程补丁机制扩大攻击面 | 任何能拿到 minisign 私钥的人可向所有用户飞书注入任意 JS | 私钥仅存 GH Actions secret;payload 黑名单审查;仅允许版本单调递增;docs/SECURITY.md 写明威胁模型 |
| Tauri updater 在 macOS 未签名时下载后无法直接启动 | 用户拿到更新后失败 | 文档明确指引 `xattr -dr com.apple.quarantine`;或后期接 Apple Developer 签名 |
| Windows 反病毒可能误报未签名 exe | 用户安装失败 | 长期考虑代码签名证书;短期 docs 指引 |
| 飞书安装路径在新版本/新平台变更 | 自动探测失效 | 始终保留"手动选目录"兜底;探测路径走配置而非硬编码 |
| Linux 包格式发散(AppImage/deb/rpm) | 维护成本 | MVP 只发 AppImage + deb;rpm 用户走 AppImage |

## 13. 已敲定与未决

**已敲定(本文档全部以此为准):**

- 技术栈、平台、UI 范围、更新策略、补丁交付方式、仓库关系、签名方案、进程检测策略、UI 语言、错误处理原则。

**未决(留到实施计划阶段):**

- 具体 npm script `bump-version` 的实现细节。
- `tauri.conf.json` 中 updater endpoint 的具体配置 URL 形态(`latest.json` vs 直接走 GitHub API)。
- Linux 包格式最终发哪几个。
- minisign 公钥的轮换策略(暂不做,后期再设计)。
