## Why

Warp 客户端目前完全无 i18n 基础设施：~7000+ 条用户可见字符串作为字面量散落在 ~1400 个 `.rs` 文件中，自定义 UI 框架（`crates/warpui_core::elements::text`）和 macOS 原生菜单构造（`app/src/menu.rs`、`app/src/app_menus.rs`）均不感知 locale。中文社区 fork 需要一次性建立 i18n 框架并完成全量汉化，以让中文用户能在母语环境下使用 Warp 终端，同时通过 CI 硬阻断防止后续英文 PR 合入时漏译。

## What Changes

- **新增** crate `warp_i18n`：基于 `i18n-embed` + `fluent-bundle` + `unic-langid` + `arc-swap` 的 i18n 运行时，lock-free 读路径，`Send + Sync`，提供 Fluent bundle 加载、locale 切换 API。
- **新增** crate `warp_i18n_macros`（独立 proc-macro crate）：`t!()` 宏接受 `LitStr` key，`build.rs` 生成 `phf::Set<&'static str>` 进行编译期校验；`tr!()` 宏在 `warp_i18n` 内提供运行时变体 + en fallback + warn 去重。（design.md D17）
- **新增** `crates/warp_i18n/bundles/{en,zh-CN}/*.ftl`：按 namespace 拆分的 ~30-50 个 Fluent 资源文件，覆盖菜单、命令面板、设置、终端 UI、对话框、Tooltip 等所有客户端持有的 UI 文案。
- **新增** locale 解析链（design.md D13）：`settings.language = Some(L) → L`；`= None → Locale::ZhCn`（fork 默认强制中文）；`= "system" → sys-locale` 映射。`language` 字段接入 `crates/settings`，磁盘表示 `"zh-CN"`/`"en"`/`"system"`/`<missing>`。
- **修改** `app/src/app_menus.rs`、`app/src/menu.rs`、`app/src/command_palette.rs`、`app/src/settings_view/**`、`app/src/terminal/**` 以及全部含 UI 字符串的源文件：将 inline 英文字面量替换为 `t!("namespace-key")` 调用。
- **新增** `RemoteString` 标记类型（`crates/warp_server_client` 与 `crates/graphql`）：标记从服务端拉取的字段，禁止误用 `t!()`；`trybuild` 编译失败模板覆盖类型级隔离不变量。
- **新增** `cargo xtask check-i18n`：基于 `syn` AST 扫描的 CI lint，识别 UI 构造点的字符串字面量；exclude_globs 含 `crates/warp_cli/**`（CLI 不汉化，design.md D14）；维护 allowlist（schema 见 design.md D21）；新增独立 workflow `.github/workflows/i18n-lint.yml`（design.md D20）。
- **新增** 桌面通知三平台截断函数 `warp_i18n::notification::truncate`（macOS ≤256 / Win title ≤64 body ≤240 / Linux ≤256，按 grapheme cluster；design.md D15）；客户端生成的 title/body 走 `t!()`，远端 `RemoteString` 不变。
- **新增** 键位渲染 `display_for_locale(accel, locale, platform)`：macOS 输出 `⌘⇧⌥⌃` 符号，Win/Linux 输出 `Ctrl/Shift/Alt` 文字（design.md D16）。
- **修改** `crates/warpui/src/windowing/winit/fonts*.rs`：补充 macOS/Windows/Linux 三平台 CJK 字体回退验证（仅验证，不改回退算法）。
- **不变** 终端命令输出、服务端 GraphQL 字段、AI 模型回复、`resources/bundled/skills/*.md`、`README/FAQ/CONTRIBUTING/about.toml`、`crates/warp_cli/**`、键位字母与修饰键名（`N`/`Ctrl`/`Shift`/`Alt`/`Cmd` 等）保持英文不动。

## Capabilities

### New Capabilities

- `i18n-runtime`：`warp_i18n` crate 的运行时能力——Fluent bundle 加载、`t!()`/`tr!()` 宏 API、参数化与复数（Fluent selector）、locale 解析链与 fallback、运行时切换。
- `i18n-locale-settings`：用户可见的语言偏好——`settings.language` 字段、设置页 UI、首启系统 locale 检测、切换后立刻生效（无需重启）。
- `i18n-ci-gate`：`cargo xtask check-i18n` 子命令——AST 扫描、UI 构造点识别、allowlist、presubmit 集成、Phase 5 切换为硬阻断。
- `client-string-localization`：客户端 UI 字符串本地化的覆盖契约——哪些字符串必须经 `t!()`、哪些豁免（log/panic/debug/test/键位/远程文案）、按 Phase 0-5 的覆盖范围与字符串类别。

### Modified Capabilities

<!-- 当前 openspec/specs/ 为空，无既有 capability 需修改 -->

## Impact

- **新增依赖**：`i18n-embed = "0.15"`、`fluent-bundle = "0.15"`、`unic-langid = "0.9"`、`sys-locale = "0.3"`、`rust-embed = "8"`、`syn = "2"`（xtask 内）。约束在 `warp_i18n` 与新建 `xtask` crate 内，不污染主依赖图。
- **代码改造面**：~1400 个 `.rs` 文件、~7000+ 条字符串字面量替换。分 5 Phase 推进，单 PR 控制在 1-3 个 Phase 子模块以内。
- **二进制体积**：新增 ~30-50 个 .ftl 文件经 `rust-embed` 嵌入，估算 +200-500 KB（en + zh-CN 双 bundle）。可接受。
- **启动性能**：bundle 加载发生在首次 `t!()` 调用前，估算 <5ms。无可测影响。
- **GraphQL 客户端层**：`crates/graphql`、`crates/warp_server_client` 引入 `RemoteString` marker type，影响所有消费服务端字符串字段的调用点（数十处）。需配合 `Display`/`AsRef<str>` 让现有渲染路径无缝兼容。
- **macOS 顶部菜单**：构造点改造，无 .lproj bundle 路径，AppKit 接收的 `NSString` 由 `t!()` 运行时产出。
- **CI 流水线**：presubmit 增加 ~5-10s 的 AST 扫描成本（首版 warning，Phase 5 升级为 hard fail）。
- **测试**：现有快照测试若锁定英文文案需更新；新增 `warp_i18n` 单元测试 + xtask 自检测试。
- **下游 fork 同步**：与上游 `warpdev/warp` 合并时冲突显著（每个含 UI 字符串的文件都被改），需在 fork 维护者侧建立合并策略文档（design.md 中说明）。
