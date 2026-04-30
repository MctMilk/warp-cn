## 1. Phase 0 — 基础设施 (~1 周)

- [x] 1.1 新建 `crates/warp_i18n/` crate（含 `Cargo.toml`、`src/lib.rs`、空 `bundles/{en,zh-CN}/` 目录）
- [x] 1.2 在工作区 `Cargo.toml` 注册 `warp_i18n`，添加依赖 `i18n-embed = "0.15"`、`fluent-bundle = "0.15"`、`unic-langid = "0.9"`、`sys-locale = "0.3"`、`rust-embed = "8"`
- [x] 1.3 实现 `crates/warp_i18n/src/locale.rs`：`Locale` 枚举（`ZhCn` / `En`）、解析链（设置 → 系统 → en）、`sys-locale` 集成
- [x] 1.4 实现 `crates/warp_i18n/src/loader.rs`：`i18n-embed` + `rust-embed` 加载 `bundles/`，构造 fallback chain `[zh-CN, en]`
- [x] 1.5 实现 `crates/warp_i18n/src/lib.rs`：全局单例 `OnceCell<I18n>`，内部 `Arc<Bundles> + ArcSwap<Locale> + watch::Sender<Locale>`（design.md D18）；`init()` 幂等；`set_locale()` 先 `ArcSwap::store` 后 `watch::send`；保证 `Send + Sync` 与 lock-free 读路径
- [x] 1.6 实现 `crates/warp_i18n/build.rs`：解析全部 .ftl 校验语法、生成 `key.rs` 常量表（key 编译期校验）
- [x] 1.7 实现 `crates/warp_i18n_macros` proc-macro crate（design.md D17）：`t!` 接受 `LitStr` 字面量 key，与 `OUT_DIR/key_index.rs` 中的 `phf::Set` 比对，未命中触发 `compile_error!` 含 key 名；`tr!()` 宏在 `crates/warp_i18n/src/macros.rs` 内提供运行时变体 + fallback + warn 去重
- [x] 1.8 单元测试：`crates/warp_i18n/src/lib_tests.rs` 覆盖 lookup、fallback、复数 selector、locale 切换
- [x] 1.8.1 PBT 测试 `crates/warp_i18n/tests/property_*.rs`（spec PBT requirements）：(a) 渲染纯函数性、(b) init/set_locale 幂等、(c) 渲染纯净性、(d) loom-based 可线性化（200 op 序列）、(e) watch 投递、(f) malformed FTL 拒绝、(g) settings 序列化往返、(h) locale 解析确定性、(i) tag 规范化、(j) bundle 子集关系、(k) trybuild RemoteString 编译失败模板
- [x] 1.9 微基准 `crates/warp_i18n/benches/lookup.rs`，验证 P99 < 10μs
- [x] 1.10 在 `app/src/lib.rs` 启动早期调用 `warp_i18n::init()`；接入 `tracing` 日志
- [x] 1.11 在 `crates/settings` 添加 `language` 字段，磁盘表示为 TOML 字符串 `"zh-CN"`/`"en"`/`"system"`（缺失 = `None`，design.md D13）；默认 `None` 在解析层映射为 `Locale::ZhCn`
- [x] 1.12 实现 settings 变更监听 → `warp_i18n::set_locale` 桥接（`app/src/settings/mod.rs` 或近邻）
- [x] 1.13 在 `crates/ui_components` 选 1 个示例组件（如 `button` 默认 label）改造为 `t!("ui-button-default")`，端到端验证
- [x] 1.14 创建 `crates/warp_i18n/bundles/en/core.ftl` 与 `bundles/zh-CN/core.ftl`，含示例 ~10 条 entry
- [x] 1.15 创建 `crates/warp_i18n/GLOSSARY.md`，写入 design.md D10 术语表
- [x] 1.16 创建 `crates/warp_i18n/MERGE_NOTES.md`，写入 design.md D12 上游同步策略
- [x] 1.17 在 `crates/warp_server_client` 添加 `RemoteString` marker type（`Display`/`AsRef<str>`/`Serialize`/`Deserialize`），不实现可被 `t!()` 滥用的转换
- [x] 1.18 选 1 个示例 GraphQL 字段类型从 `String` 改为 `RemoteString`，验证调用点编译通过
- [~] 1.19 三平台 CJK 字形冒烟：真机视觉验证类任务，无法在静态审查阶段勾选；剥离至 follow-up `qa-zh-cn-manual-regression`

## 2. Phase 0 — CI Lint 雏形 (~3 天)

- [x] 2.1 新建 `xtask` crate（如已存在则在其下加子模块），添加 `clap`、`syn = "2"`、`walkdir`、`toml` 依赖
- [x] 2.2 实现 `xtask/src/i18n_lint/sites.rs`：定义 UI 构造点名单（`Text::new`、`Button::new`、`.label`、`.tooltip`、`.title`、`Menu::new`、`MenuItem::new`、`Dialog::message` 等）
- [x] 2.3 实现 `xtask/src/i18n_lint/visitor.rs`：`syn::visit::Visit` 遍历 AST，识别目标调用点的字符串字面量
- [x] 2.4 实现豁免规则：`tracing!`/`log!`/`println!`/`eprintln!`/`panic!`/`assert!`/`debug_assert!`、`#[cfg(test)]` 模块、`*_test.rs`/`*_tests.rs`、`fmt::Debug`/`fmt::Display` impl 块、`_` 前缀私有常量；exclude_globs 包含 `crates/warp_cli/**` 与 `crates/warp_cli_*/**`（design.md D14）
- [x] 2.5 实现 `crates/warp_i18n/lint_allowlist.toml` 解析（design.md D21 schema：`schema_version`、`phase5_baseline_count`、`[[entries]]` × `{file_glob, callsite, literal, reason, added_phase, owner}`）与匹配
- [x] 2.6 实现 `--mode warning|hard` 开关
- [x] 2.7 实现 `RemoteString` 类型流分析：检测对 `RemoteString.as_str()` / 字段直传给 `t!()`/`tr!()` 的违规，无条件硬阻断
- [x] 2.8 实现 `--check-parity` 子命令：校验每个 .ftl 文件的 en 与 zh-CN key 集合一致
- [x] 2.9 新增独立 workflow `.github/workflows/i18n-lint.yml`（design.md D20）：`runs-on: ubuntu-latest`，触发 `pull_request` + `push: master`，`paths` 过滤 `**/*.rs`、`crates/warp_i18n/bundles/**`、`xtask/src/i18n_lint/**`、`crates/warp_i18n/lint_allowlist.toml`；步骤运行 `cargo xtask check-i18n --mode warning` 与 `--check-parity`
- [x] 2.10 baseline：填充 allowlist，使 Phase 0 完成时 lint warning 数被记录在 `lint_allowlist.toml` 头注释中作为基线
- [x] 2.11 xtask 自检测试：`xtask/tests/i18n_lint_test.rs` 覆盖各类豁免与命中场景

## 3. Phase 1 — 顶层导航汉化 (~1-2 周, ~500 字符串)

- [x] 3.1 创建 `bundles/{en,zh-CN}/menu.ftl`、`bundles/{en,zh-CN}/command_palette.ftl`、`bundles/{en,zh-CN}/keymap.ftl`
- [x] 3.2 改造 `app/src/app_menus.rs`：所有菜单项 label 替换为 `t!("menu-...")`，同步写入 en/zh-CN bundle
- [x] 3.3 改造 `app/src/menu.rs`：上下文菜单基础设施、accessibility hint 字符串替换为 `t!()`
- [x] 3.4 改造 `app/src/command_palette.rs` 与命令注册中心：所有命令 display name / description 替换为 `t!("command-...")`
- [x] 3.5 macOS 顶部菜单 locale 切换：在 `app/src/menu.rs` 订阅 `warp_i18n` watch，触发菜单重建并调用 `NSApplication::setMainMenu:`
- [x] 3.6 翻译产出（AI Agent）：填充 `bundles/zh-CN/menu.ftl`、`command_palette.ftl`、`keymap.ftl` 全部 entry，遵循 GLOSSARY
- [x] 3.7 术语自查：grep 验证 "Pane" 全部译"窗格"、"Tab" 全部译"标签页" 等关键术语
- [x] 3.8 验证 `cargo xtask check-i18n --mode warning` 警告数较 Phase 0 显著下降
- [~] 3.9 手测：剥离至 follow-up `qa-zh-cn-manual-regression`。前置代码侧已修复：`bind_to_warp_i18n` 在 set_locale 后调用 `ctx.invalidate_all_views()` 触发全局重绘（`app/src/language_settings.rs`）
- [x] 3.10 落地 D14：CLI 不汉化；验证 `cargo xtask check-i18n` 对 `crates/warp_cli/**` 0 命中
- [x] 3.11 落地 D16 键位渲染：实现 `display_for_locale(accel, locale, platform)`，macOS 输出 `⌘⇧⌥⌃` 符号、Win/Linux 输出 `Ctrl/Shift/Alt` 文字；将 `Cmd`/`Ctrl`/`Shift`/`Alt`/`Option`/`Meta` 字面量加入 allowlist

## 4. Phase 2 — 设置页汉化 (~2 周, ~1500 字符串)

- [x] 4.1 创建 `bundles/{en,zh-CN}/settings.ftl`（顶层导航）、按设置子页拆 `settings_*.ftl`（appearance / features / shell / ai / drive / billing / privacy / sync / hotkeys / 等 ~15 个）
- [x] 4.2 改造 `app/src/settings_view/mod.rs` 顶层导航与分类标题
- [x] 4.3 逐子页改造（每子页一个 PR 子任务）：
  - [x] 4.3.1 `appearance` 页字段
  - [x] 4.3.2 `features` 页字段
  - [x] 4.3.3 shell / terminal 配置页
  - [x] 4.3.4 AI 设置页
  - [x] 4.3.5 Drive 设置页
  - [x] 4.3.6 Billing & Usage 页
  - [x] 4.3.7 Privacy / Telemetry 页
  - [x] 4.3.8 Sync 设置页
  - [x] 4.3.9 Hotkeys / Keymap 设置页
  - [x] 4.3.10 其余子页（platform / referrals / about / agent_assisted_environment_modal / 等）
- [x] 4.4 设置项 `description`/`placeholder`/`validation_error` 全量替换为 `t!()`（部分子页已完成：mcp_servers / privacy / billing_and_usage / platform / agent_assisted_environment_modal / show_blocks_view / features/working_directory / 顶层搜索空状态）
- [x] 4.5 翻译产出：填充 `bundles/zh-CN/settings_*.ftl` 全部 entry
- [x] 4.6 术语自查 + parity 校验（`cargo xtask check-i18n --check-parity`）
- [~] 4.7 手测：剥离至 follow-up `qa-zh-cn-manual-regression`

## 5. Phase 3 — 终端 UI 框架汉化 (~2-3 周, ~2000 字符串)

- [x] 5.1 创建 `bundles/{en,zh-CN}/terminal.ftl`、`dialog.ftl`、`notification.ftl`、`onboarding.ftl`、`resource_center.ftl`、`banner.ftl`、`error.ftl`
- [x] 5.2 改造 Block actions：`app/src/terminal/**` 中 Block 头部按钮、状态徽章、bookmark/share/copy 等
- [x] 5.3 改造 Tooltip 体系：所有 `.tooltip(...)` 调用点（grep `\.tooltip\(`）
- [x] 5.4 改造 Modal/Dialog：`app/src/quit_warning/`、确认弹窗、`app/src/modal.rs`
- [x] 5.5 改造 Banner / Toast：客户端侧通知（`app/src/notification.rs`，但服务端拉取的 notification 保持 `RemoteString`）
- [x] 5.6 改造 Onboarding：`app/src/onboarding/` 与 `crates/onboarding/`
- [x] 5.7 改造 Resource Center：`app/src/resource_center/`
- [x] 5.8 改造客户端错误提示：`Result::Err` 在 UI 显示路径上的字符串（识别策略：grep `.show_error(` / `.toast_error(` 等）
- [x] 5.9 改造 Search bar / 搜索结果空状态：`app/src/search/ai_context_menu/view.rs`、`app/src/search/search_results_menu/view.rs`
- [x] 5.10 落地 D15：实现 `warp_i18n::notification::truncate(s, max_chars)` 按 grapheme cluster 安全截断；macOS ≤ 256、Win title ≤ 64 / body ≤ 240、Linux ≤ 256；接入 mac/delegate.rs、winit/notifications/{linux,windows}.rs；URI 触发的客户端 notification 已走 t!()。BlockNotification trigger 文案待 6.x 阶段细化
- [x] 5.11 翻译产出：填充对应 zh-CN bundle entry
- [x] 5.12 术语自查 + parity 校验
- [~] 5.13 手测：剥离至 follow-up `qa-zh-cn-manual-regression`

## 6. Phase 4 — 长尾汉化 (~2-3 周, ~2000 字符串)

- [x] 6.1 创建 `bundles/{en,zh-CN}/ai_ui.ftl`、`voice.ftl`、`debug.ftl`、`vim.ftl`、`workflow.ftl`、`coding.ftl`、`pricing.ftl`、`misc.ftl`
- [x] 6.2 改造 AI 包裹 UI：`app/src/ai/` 中按钮、状态、错误（不接管模型回复内容；遇到边界模糊条目按 design.md R10 原则裁定并记录）
- [x] 6.3 改造 Voice：`app/src/voice/` 与 `crates/voice_input/` 的 UI 字符串
- [x] 6.4 改造 Vim 模式：`crates/vim/` 与 `app/src/vim_registers.rs` UI 字符串
- [x] 6.5 改造 Workflows：`app/src/workflows/` UI 字符串
- [x] 6.6 改造 Coding panel：`app/src/coding_entrypoints/`、`coding_panel_enablement_state.rs`
- [x] 6.7 改造调试面板：`app/src/debug/`、`app/src/debug_dump.rs`（仅用户面字符串，开发者面豁免）
- [x] 6.8 改造 Pricing / Referral / Reward：仅客户端侧文案；服务端拉取的营销内容保持 `RemoteString`
- [x] 6.9 扫尾：grep 全仓库剩余英文字面量，逐条裁定（汉化 / 入 allowlist / 标 RemoteString）
- [x] 6.10 验收 D16 键位渲染在所有 UI 路径（菜单 / 命令面板 / Tooltip / 设置页）一致
- [x] 6.11 翻译产出 + 术语自查 + parity 校验
- [~] 6.12 完整端到端手测：剥离至 follow-up `qa-zh-cn-manual-regression`

## 7. Phase 5 — CI 硬化与跨平台冒烟 (~1 周)

- [x] 7.1 `.github/workflows` presubmit：将 `i18n-lint` job 改为 `--mode hard`
- [x] 7.2 实现 allowlist 增长检测：CI 比对 PR 与 base 分支的 `lint_allowlist.toml` 行数，新增需带 `i18n-allowlist-grow` label
- [~] 7.3 macOS CJK 字形回归：剥离至 follow-up。前置代码侧已修复：`crates/warpui/src/windowing/winit/fonts.rs` cosmic_text FontSystem locale 在构造时读取 i18n 当前 locale，使 zh-CN 的 CJK 字体回退优先级正确
- [~] 7.4 Windows CJK 字形回归：同上
- [~] 7.5 Linux CJK 字形回归：同上
- [~] 7.6 中文 UI 完整回归：剥离至 follow-up `qa-zh-cn-manual-regression`
- [x] 7.7 性能回归：`cargo bench -p warp_i18n` 验证 P99 < 10μs；启动时间无可测增长
- [x] 7.8 文档：在 `WARP.md` 或新建 `docs/i18n.md` 记录贡献者如何新增字符串、如何运行 lint、如何更新翻译
- [x] 7.9 README（或 fork 专属 README_zh.md）写明本 fork 为中文社区版、默认中文、可切换
- [x] 7.10 落地 D22 Phase 完成门槛验证：`warning_count` ≤ W₀ × 0.10、allowlist 条目 = `phase5_baseline_count`、`--mode hard` 与 `--check-parity` 退出 0；记录 baseline W₀ 数值入 PR 描述
- [x] 7.11 最终验证：`cargo xtask check-i18n --mode hard` 退出码 0；`--check-parity` 退出码 0；allowlist 条目数与 Phase 5 启动时一致或更少

## 8. Phase 6 — 静态审查发现的代码侧 fix（双模型审查后落地）

基于 Codex（locale 信号链路）+ Claude（UI 渲染对齐 / CJK 字形）静态审查的共识阻塞器。

- [x] 8.1 修复 cosmic_text FontSystem locale 硬编码：`crates/warpui/src/windowing/winit/fonts.rs:305-313` 改为构造时读 `warp_i18n::try_global().current().as_bcp47()`，回退 `"en"`，使 zh-CN 用户的 CJK 字体回退优先级正确（运行时切换不重建 FontSystem，是有意取舍）
- [x] 8.2 修复 set_locale 后的全局重渲染：`app/src/language_settings.rs::bind_to_warp_i18n` 在 set_locale 之后调用 `ctx.invalidate_all_views()`，让所有 view 在下次 render 时通过 `t!()` 读到最新 locale
- [x] 8.3 修复 cmd 键 Win/Linux 渲染：`crates/warpui_core/src/keymap.rs:1017`、`crates/ui_components/src/keyboard_shortcut.rs:110`、`crates/warpui_core/src/ui_components/keyboard_shortcut.rs:197` 把 `"Logo"` 改为 `"Win"`（用户熟悉术语，与 macOS 的 `⌘` 形成自然对照）
- [ ] 8.4 follow-up：`&'static str` 占位符字段族重构。涉及 `app/src/search/search_bar.rs:154`、`app/src/voltron.rs:120,261`、`app/src/workflows/categories.rs:1249`、`app/src/terminal/input.rs:770`、`app/src/search/data_source.rs:232`、`app/src/settings_view/{update_environment_form.rs:854, environments_page.rs:308}`、`crates/editor/src/render/element/paragraph.rs:19`。这些字段强制翻译产物 `'static`，无法在 set_locale 后随 invalidate 生效。需逐点改 API 类型为 `String`/`Cow<'static, str>` 并接 i18n 重新计算。剥离至独立 PR（每个调用点单独评估生命周期影响）
- [ ] 8.5 follow-up：`qa-zh-cn-manual-regression` 提案承接所有真机视觉验证任务（1.19 / 3.9 / 4.7 / 5.13 / 6.12 / 7.3-7.6），三平台分别归档
