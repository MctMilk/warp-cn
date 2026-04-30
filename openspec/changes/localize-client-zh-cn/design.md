## Context

Warp 是一个 Rust 编写的 1.24M LOC、3119 个源文件的桌面终端应用。本次 change 在中文社区 fork 上一次性建立 i18n 基础设施并完成全量汉化。已知关键事实：

- **无既有 i18n 基础**：搜索显示 0 个 fluent/gettext/sys-locale/cargo-i18n 依赖；`crates/languages/` 是 TreeSitter 编程语言语法 crate（与本地化无关）。
- **字符串分布**：~7000+ 条用户可见英文字符串以 `&str`/`String` 字面量形式 inline 在 ~1400 个 `.rs` 文件中。典型样本（`app/src/app_menus.rs`）：`"New Window"`、`"Preferences"`、`"Set Warp as Default Terminal"`。
- **UI 框架**：自研 `crates/warpui_core`（基于 `winit`），文本渲染走 `elements::text::Text`，字符串作为构造参数透传，UI 层无 locale 概念。
- **macOS 顶部菜单**：`app/src/app_menus.rs` 程序构造 `Menu::new("Warp", ...)`，再走 AppKit FFI；不使用 .lproj 资源 bundle 路径。
- **服务端文案**：通过 `crates/graphql` 与 `crates/warp_server_client` 拉取，类型上目前与本地字符串无区分。
- **AI 内容**：`crates/ai`、`app/src/ai` 处理模型回复，输出文本来自远端模型，不可在客户端预翻译。
- **字体回退**：`crates/warpui/src/windowing/winit/fonts*.rs` 已实现回退链，但 CJK 字形覆盖未经验证。
- **fork 维护现实**：上游 `warpdev/warp` 持续演进，本 fork 必须能定期 merge upstream。

利益相关方：中文社区用户（最终受众）、本 fork 的少数维护者（同时承担翻译与代码改造）、可能的下游贡献者。

## Goals / Non-Goals

**Goals:**

- 建立可复用的 i18n 运行时（`warp_i18n` crate），架构上不与"中文"绑定，未来可扩展第三语言。
- 100% 覆盖客户端持有的 UI 字符串：菜单、命令面板、设置、Modal/Dialog、Tooltip、Banner、Toast、错误提示、Onboarding、Block actions、Resource Center 等。
- 默认 zh-CN，自动检测系统 locale，用户可在设置内切换；切换不需要重启进程。
- CI 在 Phase 5 切换为硬阻断：任何新增 inline 英文 UI 字符串必须经 `t!()`，否则 presubmit 失败。
- 翻译术语一致：建立 glossary 锁定关键术语（pane/block/tab/agent/workflow/...）的中文译法。
- 与上游可合并：每个 .rs 文件改动尽量在一处集中（避免散点修改），便于 `git rerere` 与人工合并。

**Non-Goals:**

- 不汉化终端命令输出（用户数据，绝对不动）。
- 不汉化服务端拉取的字符串（GraphQL 字段、通知、计费页文案、横幅、营销内容）。
- 不接管 AI 模型输出（保持模型行为；仅汉化包裹 AI 的客户端 UI 框架）。
- 不汉化 `resources/bundled/skills/*.md`、`.warp/` 下 AI 系统 prompt（驱动 AI 行为，改动会污染模型）。
- 不汉化 `README.md`/`FAQ.md`/`CONTRIBUTING.md`/`SECURITY.md`/`about.toml`（仓库元数据，面向贡献者）。
- 不汉化 log/panic/`tracing!`/`debug!`/test 字符串（开发者面，无用户价值）。
- 不汉化键位标记 `Cmd+N` / `⌘+S` / `Ctrl+T` 等国际通用约定。
- 不引入 RTL（阿拉伯语/希伯来语等）布局支持——本 fork 仅 zh + en。
- 不替换底层渲染、字体加载、布局系统——仅在文本进入渲染管线之前完成 lookup。
- 不向上游 `warpdev/warp` 提交 PR（本 fork 自治）。

## Decisions

### D1. i18n 框架选型：`i18n-embed` + `fluent-bundle`

- **选定**：`i18n-embed = "0.15"` + `fluent-bundle = "0.15"` + `unic-langid = "0.9"` + `rust-embed = "8"`。
- **替代方案**：
  - `rust-i18n` (json/toml 后端)：键值对模型简单，但无 ICU MessageFormat 复数/选择器，不适配 ~7000 字符串规模。
  - `gettext-rs`：依赖系统 libgettext，跨平台（特别是 Windows）部署痛苦。
  - 自研：维护成本高，没必要重造轮子。
- **理由**：Fluent 是 Mozilla 维护的 ICU MessageFormat 2 风格语法，原生支持复数/选择器/参数化/嵌套消息；`i18n-embed` 提供 bundle 加载、fallback chain、`rust-embed` 集成；业界 Rust 桌面应用（Fractal、Pop!_OS Cosmic、Lapce）均用此组合。

### D2. Key 命名风格：路径式 (`menu-file-new-window`)

- **选定**：kebab-case 路径式 key，按 namespace 分组在不同 .ftl 文件。
- **替代方案**：
  - Hash 式 (`t!("New Window")`)：英文文案微调（大小写/标点）即断 key，重复原文会合并（语义可能冲突）。
  - 编号式 (`M001`)：源码不可读，无法 grep。
- **理由**：Fluent 官方推荐；与英文文案解耦后修订英文不会打断翻译；可按 namespace（menu/settings/terminal/dialog/...）拆 .ftl 文件，便于按 Phase 推进。

### D3. Bundle 文件结构

```
crates/warp_i18n/
├── Cargo.toml
├── build.rs                 # 编译时 .ftl 语法校验 + key 索引生成
├── bundles/
│   ├── en/
│   │   ├── core.ftl         # 应用名、通用按钮 (确定/取消/保存)
│   │   ├── menu.ftl         # 顶部菜单 + 上下文菜单
│   │   ├── command_palette.ftl
│   │   ├── settings.ftl
│   │   ├── settings_appearance.ftl
│   │   ├── settings_features.ftl
│   │   ├── ... (settings 子页按页拆 ~15 个)
│   │   ├── terminal.ftl     # Block actions / 状态徽章
│   │   ├── dialog.ftl       # Modal / Confirmation
│   │   ├── notification.ftl # Toast / Banner (客户端侧)
│   │   ├── onboarding.ftl
│   │   ├── ai_ui.ftl        # AI 包裹 UI（不含模型输出）
│   │   ├── voice.ftl
│   │   ├── error.ftl
│   │   └── ... (~30-50 个 .ftl 文件)
│   └── zh-CN/
│       └── ... (与 en 一一镜像)
└── src/
    ├── lib.rs               # I18n 全局单例、locale 切换
    ├── loader.rs            # i18n-embed loader + fallback chain
    ├── locale.rs            # locale 解析链（设置 → 系统 → en）
    └── macros.rs            # t!() / tr!() 宏
```

- **理由**：按 namespace 拆 .ftl 让每个 Phase 集中改动一组文件；bundle 经 `rust-embed` 嵌入二进制，零运行时 IO 失败；`build.rs` 在编译期解析 .ftl 并生成 key 常量表，让 `t!("menu-file-new-window")` 可以编译期校验 key 存在（不存在则编译失败）。

### D4. 宏 API：`t!()` 与 `tr!()`

```rust
// 静态字符串（无参数）
let label = t!("menu-file-new-window");
// → "新建窗口"

// 带参数
let msg = t!("tabs-close-confirm", n: tab_count);
// → "关闭 3 个标签页"

// 带显式 locale（罕见，仅测试用）
let s = tr!(Locale::En, "menu-file-new-window");
```

- `t!()`：编译期校验 key、运行时按当前 locale lookup、返回 `String`（Fluent 渲染产物）。
- `tr!()`：动态 key 或显式 locale 路径，运行时校验。
- 编译期校验依靠 `build.rs` 生成 `key.rs` 常量表 + 宏内部 `concat!` 校验。
- **替代方案**：函数式 `i18n.tr("key", &args)`——失去编译期校验，调用点冗长。

### D5. Locale 解析链

```
[启动 / 设置变更]
    │
    ▼
① 用户设置 settings.language: Option<Locale>
    │ Some(zh-CN) → 锁定
    │ None
    ▼
② 系统 locale (sys-locale::get_locale)
    │ "zh-CN" / "zh-Hans" / "zh*" → zh-CN
    │ 其他 → en
    ▼
③ Fluent fallback chain: [zh-CN, en]
   - zh-CN bundle 缺 key → 自动回落 en bundle
   - en bundle 缺 key → 编译期已阻止（build.rs 校验）
```

- **重要**：设置切换走 `tokio::sync::watch::Sender<Locale>`，UI 层订阅；切换后 `presenter` 触发全局重渲染（Warp 自研 UI 已有 dirty/repaint 机制）。无需进程重启。

### D6. 服务端字符串边界：`RemoteString` marker type

```rust
// crates/warp_server_client/src/remote_string.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteString(String);

impl RemoteString {
    pub fn as_str(&self) -> &str { &self.0 }
}
impl Display for RemoteString { ... }
impl AsRef<str> for RemoteString { ... }
```

- GraphQL 反序列化将所有字符串字段类型从 `String` 改为 `RemoteString`（按字段，非全替换；通过 codegen 或手工标注）。
- `t!()` 宏只接受 `&'static str` key，编译器自动阻止 `t!(remote.as_str())` 误用。
- CI lint（D7）补充检查：禁止对 `RemoteString` 字段做翻译查找。
- **范围**：Phase 0 实现 marker type 与 1-2 个示例字段；其余字段在 Phase 3-4 被对应 UI 层改造时同步标注。

### D7. CI 硬阻断：`cargo xtask check-i18n`

- 新建 `xtask` crate（如已有则扩展），基于 `syn = "2"` 解析 AST。
- 识别策略（白名单 vs 黑名单组合）：
  - **目标调用点**（必须经 `t!()`）：`Text::new(<lit>)`、`Button::new(<lit>)`、`.label(<lit>)`、`.tooltip(<lit>)`、`.title(<lit>)`、`Menu::new(<lit>, ...)`、`MenuItem::new(<lit>, ...)`、`Dialog::message(<lit>)`、Toast/Banner 构造等。完整名单在 `xtask/i18n_lint/sites.rs`。
  - **豁免**：`tracing::*!`、`log::*!`、`println!`、`eprintln!`、`panic!`、`assert!`、`debug_assert!`、`format!` 配合 log；`#[cfg(test)]` 与 `*_test.rs`、`*_tests.rs` 模块；`fmt::Debug`/`Display` 实现内字面量；以 `_` 开头的内部常量。
  - **allowlist**：`crates/warp_i18n/lint_allowlist.toml`，逐条登记现有合法英文字面量（如 keystroke 解析 `Keystroke::parse("cmd-n")`、URL、env var 名）。
- 阶段：
  - Phase 0-4：lint 输出 warning，不阻断 CI（"baseline 模式"）。
  - Phase 5：lint 切换为硬失败，`exit 1`；同时校验 allowlist 不增长（防止偷加豁免）。
- 接入：`.github/workflows/presubmit.yml` 增加 `cargo xtask check-i18n` step。

### D8. macOS 原生菜单处理

- 不走 `.lproj` 资源 bundle 路径（Warp 不打包成标准 macOS app bundle 形态供本地化）。
- `app/src/app_menus.rs` 与 `app/src/menu.rs` 在程序构造 `Menu::new(t!("menu-app-name"), ...)` 时即用 `t!()`，得到 `String`，传入 AppKit FFI 转 `NSString`。
- locale 切换时需重建菜单树并通知 AppKit（`NSApplication::setMainMenu:`），通过现有 `app/src/menu.rs` 重建路径。

### D9. 字体与 CJK 渲染

- 不修改字体加载/回退算法本身（出 scope）。
- 在 Phase 0 任务中加入"三平台 CJK 字形冒烟"：在 macOS（默认 PingFang SC）、Windows（Microsoft YaHei）、Linux（Noto Sans CJK SC）下渲染中文 UI 截图，目视验证字形覆盖。
- 若发现回退链遗漏 CJK 字体，登记为 follow-up issue（不在本 change 范围）。
- Warp 终端等宽假设：终端字符不汉化，仅 UI 层（Block 头、按钮等）使用 CJK 字体——UI 层本来就支持 proportional 渲染，无对齐问题。

### D10. 翻译术语 Glossary（必须提前锁定，避免后期返工）

| 英文 | 中文 | 备注 |
|------|------|------|
| Pane | 窗格 | 不译"面板"（panel） |
| Block | 区块 | Warp 特有概念，保留首字母大写感 |
| Tab | 标签页 | 不译"选项卡" |
| Window | 窗口 | |
| Workflow | 工作流 | |
| Agent | 智能体 | 不译"代理"（避免与网络代理冲突） |
| Notebook | 笔记本 | |
| Drive | 云盘 | Warp Drive 译"Warp 云盘" |
| Warp | Warp | 产品名不译 |
| Settings / Preferences | 设置 / 偏好设置 | 顶层用"设置"，菜单项用"偏好设置..." |
| Command Palette | 命令面板 | |
| Theme | 主题 | |
| Subshell | 子 Shell | |
| Onboarding | 新手引导 | |
| Voice | 语音 | |
| AI / LLM | AI / 大模型 | "AI"保留英文；"LLM"译"大模型" |
| Prompt | 提示词 | AI 上下文；shell prompt 译"提示符" |

完整术语表随 Phase 推进增补，记录在 `crates/warp_i18n/GLOSSARY.md`。

### D11. 翻译执行：AI Agent 全量产出

- 翻译由 AI Agent（即本提案的执行者）按 namespace 批量产出，不引入外部翻译流程。
- 每个 .ftl 文件生成时：参照 GLOSSARY 锁定术语；遇到歧义在 .ftl 内 `# TODO(translation):` 注释，由后续 review 人工裁定。
- 不引入 weblate / crowdin 等翻译平台（成本不匹配）。

### D12. 与上游同步策略（fork 维护）

- 每个 Phase 的 commit 严格按"先抽 key、后翻译"两步 split：
  - Step A: 把 inline 字符串替换为 `t!("...")`，bundles/en/*.ftl 同步加 entry——纯结构改动，与上游英文 PR 容易合并。
  - Step B: bundles/zh-CN/*.ftl 加翻译——纯翻译改动，不触碰英文与代码。
- 上游若改某条英文文案，本 fork merge 时只需更新 `bundles/en/*.ftl`，并 mark `bundles/zh-CN/` 对应 entry 为待复核（CI 检测 en/zh-CN entry 数量与 key 集合差异）。
- 在 `crates/warp_i18n/MERGE_NOTES.md` 写明合并步骤。

## Risks / Trade-offs

- **[R1] 1400 文件改动与上游冲突**：每次 merge upstream 都会冲突。
  - **Mitigation**：D12 的"两步 split"提交策略；启用 `git rerere`；维护 `MERGE_NOTES.md`；翻译数据与代码改动分离。

- **[R2] CJK 字形覆盖不足导致渲染豆腐块**：`fonts*.rs` 回退链未经 CJK 验证。
  - **Mitigation**：Phase 0 三平台冒烟；若发现遗漏，回退到登记 follow-up issue（非本 change 阻塞项）。

- **[R3] CI lint 假阳/假阴**：AST 扫描难以 100% 准确识别 UI 字面量。
  - **Mitigation**：Phase 0-4 仅 warning 不阻断；逐 Phase 调优 sites 名单与 allowlist；Phase 5 切硬阻断时 allowlist 必须冻结（PR 内 allowlist 增长直接拒绝）。

- **[R4] 翻译质量与术语漂移**：~7000 条字符串由 AI Agent 单次翻译，可能存在不一致。
  - **Mitigation**：D10 GLOSSARY 提前锁核心术语；每 Phase 完成后做术语 grep 自查；用户在使用中发现不当译法走 issue 修缮（个人 fork 容忍迭代）。

- **[R5] Fluent build.rs 解析失败拖慢编译**：~30-50 个 .ftl 编译期解析。
  - **Mitigation**：实测 fluent-syntax 解析千条 entry 在 ~10ms 量级；可接受。bundle 改动不影响其他 crate 增量编译（`warp_i18n` 是叶子 crate）。

- **[R6] 服务端 `RemoteString` 改造影响面广**：所有 GraphQL 字符串字段需重新标注。
  - **Mitigation**：Phase 0 仅落 marker type 与机制，不要求全量改造；后续按 UI 路径推进时增量迁移；保留 `From<String>` 转换便于过渡。

- **[R7] 翻译 PR 巨大难 review**：单 Phase ~1500-2000 条字符串。
  - **Mitigation**：每 Phase 内再按 .ftl 文件拆 PR；翻译 PR 与代码改造 PR 分离（D12）。

- **[R8] 设置切换运行时一致性**：locale watch 信号到 UI 重绘之间可能短暂渲染英文。
  - **Mitigation**：locale 切换走 `tokio::sync::watch`，UI presenter 在下一帧拿到新值；可接受亚秒级闪烁；用户切换语言频率极低。

- **[R9] tracing/log 误改**：自动化替换可能错误地把 `tracing::info!("Loading config")` 误改为 `tracing::info!(t!(...))`。
  - **Mitigation**：D7 lint 明确豁免 tracing/log；改造工作严格"按调用点 sites 名单"机械执行，不做全文 grep 替换。

- **[R10] AI 包裹 UI 与 AI 输出边界模糊**：例如 Agent 错误提示 `"Failed to connect to model"` 是客户端字符串还是模型 SDK 字符串？
  - **Mitigation**：在 Phase 4 启动时由维护者逐条裁定边界，并写入 design.md 附录；裁定原则：**字符串值由本仓库代码生成 = 客户端字符串（汉化）；字符串值来自远端 = 远程文案（不汉化）**。

## Plan-phase Lock-in (Zero-decision Constraints)

下述决策在 Plan 阶段最终锁定，实现期不再裁定。原 Open Questions OQ1-OQ4 在此处全部消解。

### D13. 默认语言策略（替代 OQ Goals 矛盾）

- **锁定**：本 fork 是「中文社区版」，全新安装首启 `settings.language = None` 时**强制默认 zh-CN**，不再读取系统 locale。仅当用户显式选择「跟随系统」或「English」时才走系统/英文路径。
- **Locale 解析链**修订为：
  ```
  ① settings.language: Some(L) → 锁定 L
  ② settings.language: None → Locale::ZhCn（fork 默认）
  ③ 用户选「跟随系统」时（settings.language = None 由设置页特定路径写入空标记）→ sys-locale；zh* → ZhCn，en* → En，其他 → En
  ```
- **实现**：`settings.language` 类型保持 `Option<Locale>`；`None` 在 fork 语义里等价 `Some(Locale::ZhCn)`，`crates/warp_i18n::resolve_locale` 实现此映射，单测验证。

### D14. CLI 不汉化（OQ1 关闭）

- **锁定**：`crates/warp_cli` 与 `crates/warp_cli_*`、`clap` 派生的 `--help`/`--version`/`error.kind` 全部保持英文。
- **lint 处理**：`xtask check-i18n` 默认排除 glob `crates/warp_cli/**`、`crates/warp_cli_*/**`，写入 sites.rs 的 exclude_globs；不进 allowlist（避免 allowlist 被噪声膨胀）。
- **理由**：CLI 国际通用约定；脚本/CI/教程链接稳定性。

### D15. 桌面通知汉化（OQ2 关闭）

- **锁定**：`app/src/notification.rs` 中由本仓库代码生成的 `title` / `body` 字段经 `t!()` 汉化；远端拉取的（推送、营销）走 `RemoteString` 保持英文。
- **三平台细节**：
  - macOS（`UNUserNotificationCenter`）：UTF-8 直传，无长度硬限（建议 ≤ 256 字符做安全截断）。
  - Windows（`ToastNotificationManager`）：UTF-16，title ≤ 64、body ≤ 240 字符做截断。
  - Linux（`notify-rust` libnotify）：UTF-8，无硬限。
- **实现**：`warp_i18n::notification::truncate(s, max_chars)` 工具函数按 grapheme cluster 安全截断。Phase 3 任务 5.10 升级为「实现并测试三平台截断」。

### D16. 键位提示规则（OQ3 关闭）

- **锁定**：
  - **macOS**：渲染为符号 `⌘⇧⌥⌃` + 字母（如 `⌘⇧N`），不汉化修饰键名。
  - **Windows / Linux**：渲染为英文文字 `Ctrl+Shift+Alt+N`，修饰键名保持英文。
- **实现**：在 `crates/keymap` 或 `crates/warpui_core::accel` 增加 `display_for_locale(accel, locale, platform)` 路径；`Cmd`/`Ctrl`/`Shift`/`Alt`/`Option` 字面量进 allowlist。

### D17. `t!()` 编译期校验机制（消解原 D4 模糊处）

- **锁定**：proc-macro 实现，crate 名 `warp_i18n_macros`（不是 `macro_rules!`）。
  - `build.rs` 在 `warp_i18n` crate 解析 `bundles/en/**/*.ftl`，输出 `OUT_DIR/key_index.rs`：`pub static EN_KEY_INDEX: phf::Set<&'static str> = phf_set! { ... };`。
  - `warp_i18n_macros::t` proc-macro 接受 `LitStr` 字面量 key + 可选 `name: expr` 命名参数；将 key 与编译期常量 phf set 比对（通过运行 `include!(concat!(env!("OUT_DIR"), "/key_index.rs"))` 引用），未命中则 `compile_error!("warp_i18n: key `{}` not found in en bundle (bundles/en/*.ftl). Add it before referencing.")`。
  - 非字面量 key 调用 `t!()` → `compile_error!("warp_i18n::t! requires a string literal key; use tr! for dynamic keys")`。
- **`tr!()`**：宏接受任意 `impl AsRef<str>` key 与可选 locale；运行时校验，缺 key 走 fallback。

### D18. 全局并发模型

- **锁定**：
  - 单例：`static I18N: OnceCell<I18n>`，`I18n` 内含 `Arc<Bundles>`（不可变快照）+ `arc_swap::ArcSwap<Locale>` + `tokio::sync::watch::Sender<Locale>`。
  - `t!()`/`tr!()`：lock-free 读路径，`ArcSwap::load()` 拿当前 locale，从 `Arc<Bundles>` 选 bundle 并渲染；MUST `Send + Sync`，可在任意线程（含非 tokio runtime）调用。
  - `set_locale(L)`：`ArcSwap::store(Arc::new(L))` + `watch::Sender::send(L)`；调用顺序保证「先发布 locale，后通知 watcher」（happens-before 序）。
  - `init()`：幂等；二次调用直接返回，不重新加载 bundle。
  - 渲染路径不分配除返回值 `String` 外的额外堆内存。

### D19. 依赖与 MSRV 锁定

| crate | 版本 | features |
|------|------|----------|
| `i18n-embed` | `=0.15.4` | `fluent-system`, `desktop-requester` 关 |
| `fluent-bundle` | `=0.15.3` | default |
| `unic-langid` | `=0.9.6` | `macros` |
| `sys-locale` | `=0.3.2` | default |
| `rust-embed` | `=8.5.0` | `compression` 关 |
| `phf` | `=0.11.2` | `macros` |
| `arc-swap` | `=1.7.1` | default |
| `syn` | `=2.0.79` | `full`, `visit`, `extra-traits` |
| `globset` | `=0.4.14` | default |
| `walkdir` | `=2.5.0` | default |

- MSRV 跟随 workspace 现状，不下调；若 i18n-embed 0.15 要求更高，单独在 `warp_i18n/Cargo.toml` 设 `rust-version`，主 workspace 不改。
- 升级规则：minor 升级允许（PR 内单独 commit），major 升级走新 OpenSpec proposal。

### D20. CI workflow 集成

- **锁定**：新增 `.github/workflows/i18n-lint.yml`（独立 workflow，不混入 presubmit），包含 job：
  - `i18n-lint`：runs-on `ubuntu-latest`；步骤 = checkout → setup-rust（与 workspace 一致 toolchain）→ `cargo xtask check-i18n --mode <warning|hard>` → `cargo xtask check-i18n --check-parity`。
  - 触发：`pull_request` 与 `push: master`；`paths` 包含 `**/*.rs`、`crates/warp_i18n/bundles/**`、`xtask/src/i18n_lint/**`、`crates/warp_i18n/lint_allowlist.toml`。
- branch protection：在 fork 仓库设置中将 `i18n-lint` 加入必过检查（Phase 5 完成后）。
- 不接入 macOS/Windows runner（CI 成本；CJK 字形冒烟由维护者本地完成）。

### D21. allowlist TOML schema

```toml
# crates/warp_i18n/lint_allowlist.toml
schema_version = 1
phase5_baseline_count = 0  # Phase 5 启动时固化条目数；CI 比对此值

[[entries]]
file_glob = "crates/keymap/src/**/*.rs"
callsite = "Keystroke::parse"
literal = "cmd-n"
reason = "keystroke parser DSL, not user-visible"
added_phase = 0
owner = "i18n-team"
```

- 字段语义：
  - `file_glob`：`globset` 语法，匹配仓库相对路径。
  - `callsite`：精确 path 匹配（`syn::Path` 末段或全限定名），不支持正则。
  - `literal`：必须严格等于源码中的字符串字面量（含转义后值）。
  - `reason`：必填，≥ 10 字符，PR review 检查项。
  - `added_phase`：0-5 数字。
  - `owner`：自由字符串，便于责任追溯。
- 匹配是 entry 集合的成员谓词（commutative），实现期 MUST 用 `BTreeSet` 或排序后比较以保证 lint 输出稳定性。
- Phase 5 冻结校验：CI 比 base 与 head 分支的 entries 集合差集（不是行数 diff），只有带 `i18n-allowlist-grow` label 的 PR 才允许新增。

### D22. Phase 完成门槛（数值化）

| Phase | 完成判定 |
|------|--------|
| 0 | `cargo xtask check-i18n --mode warning` 完成、warning 数 W₀ 记录入 `lint_allowlist.toml` 头注释；P99 lookup < 10μs；`--check-parity` 退出 0 |
| 1 | warning 数 ≤ W₀ × 0.93（≈ -7%，对应 ~500 字符串）；菜单 + 命令面板 100% 覆盖（`grep -nE "MenuItem::new\(\"[^\"]" app/src/app_menus.rs` 命中 0 条非 allowlist 项）|
| 2 | warning 数 ≤ W₀ × 0.72（≈ -28%）；`app/src/settings_view/**` 中 `Text::new("...")` / `.label("...")` / `.tooltip("...")` 命中 0 条非 allowlist 项 |
| 3 | warning 数 ≤ W₀ × 0.43（≈ -57%）|
| 4 | warning 数 ≤ W₀ × 0.10（≈ -90%）；剩余 warning 全部进 allowlist 或开 follow-up issue |
| 5 | `--mode hard` 退出 0；`--check-parity` 退出 0；allowlist 条目数 ≤ Phase 5 启动值 |

- 维护者审核：每 Phase 由 fork 维护者（仓库 Admin 角色）签字进入下一 Phase；签字记录写入对应 PR 描述。
- 不绑定截图回归数值阈值（视觉回归由维护者主观判断，记录在 PR 描述）。

### D23. tr_n! 不引入（OQ4 关闭）

- **锁定**：复数与选择器统一走 Fluent `selector`（D4）；不补充 `tr_n!` 显式宏。
- 理由：保持 API surface 最小；Fluent selector 表达力已足够；ergonomics 已通过 `t!("k", n: count)` 命名参数解决。

## Open Questions

（无；OQ1-OQ4 已在 Plan 阶段决议为 D13-D16、D23。）
