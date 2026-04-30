## ADDED Requirements

### Requirement: 客户端 UI 字符串覆盖范围

系统 SHALL 将所有"客户端持有的 UI 字符串"通过 `t!()` 宏本地化。"客户端持有的字符串"定义为：字符串值由本仓库代码生成（字面量或 `format!`/常量），最终通过 UI 元素（菜单项、按钮、标签、Tooltip、对话框、Toast、Banner、错误提示、设置项 description/label 等）展示给最终用户。

#### Scenario: 顶部菜单项字符串

- **WHEN** Phase 1 完成后检查 `app/src/app_menus.rs`
- **THEN** 文件内不再含 `MenuItem::new("...")` 形式的 inline 英文字面量（除 allowlist 项外）
- **AND** 对应字符串均通过 `t!("menu-...")` 提供

#### Scenario: 设置页字段标签

- **WHEN** Phase 2 完成后检查 `app/src/settings_view/**`
- **THEN** 所有字段 `label` / `description` / 分组标题均通过 `t!()` 提供

#### Scenario: Block actions 与 Tooltip

- **WHEN** Phase 3 完成后检查 `app/src/terminal/**` 中 UI 部分
- **THEN** Block 头部按钮、状态徽章、Tooltip 文案均通过 `t!()` 提供
- **AND** 终端命令输出本身（用户数据）MUST 不被处理

### Requirement: 排除范围必须明确

系统 MUST NOT 本地化以下内容（视为契约性排除）：

1. 终端命令输出（用户数据）
2. 服务端 GraphQL 返回的字段（`RemoteString` 类型）
3. AI 模型的回复内容
4. `resources/bundled/skills/*.md` 与 `.warp/` 下 AI 系统 prompt
5. `README.md` / `FAQ.md` / `CONTRIBUTING.md` / `SECURITY.md` / `CODE_OF_CONDUCT.md` / `about.toml`
6. `tracing!` / `log!` / `panic!` / `assert!` / `debug_assert!` / `println!` / `eprintln!` 字符串
7. 测试代码中的字符串
8. 键位字母与字面量本身（`N`/`T`/`S`），但 modifier 显示规则按 design.md D16：
   - macOS：渲染为符号 `⌘⇧⌥⌃`，不汉化
   - Windows / Linux：渲染为英文文字 `Ctrl`/`Shift`/`Alt`，不汉化
9. URL、env var 名、产品标识符（"Warp"、"Anthropic"、"OpenAI" 等）
10. CLI crates `crates/warp_cli/**` 与 `crates/warp_cli_*/**` 的全部字符串（design.md D14）

#### Scenario: 服务端字段保持英文

- **WHEN** GraphQL 返回 `Notification { title: "Important update available" }`
- **AND** UI 渲染该 notification
- **THEN** title 在中文 locale 下 MUST 仍渲染为英文 "Important update available"

#### Scenario: AI Agent 回复保持原样

- **WHEN** AI Agent 用模型默认行为回复 "I'll help you debug this..."
- **THEN** 回复内容 MUST 不被 `t!()` 处理
- **AND** 包裹该回复的 UI 框架（"复制" 按钮、"重试" 按钮、状态徽章）MUST 已汉化

#### Scenario: README 不被翻译

- **WHEN** Phase 5 完成后检查仓库根目录
- **THEN** `README.md`、`FAQ.md`、`CONTRIBUTING.md` 内容 MUST 与上游一致（保持英文）

### Requirement: 提供 `RemoteString` marker type

`crates/warp_server_client` SHALL 导出 `RemoteString` 类型用于标记所有从服务端拉取的字符串字段。该类型 MUST 实现 `Display`、`AsRef<str>`、`Serialize`、`Deserialize`，但 MUST NOT 实现 `Into<&'static str>` 或暴露任何允许其作为 `t!()` 参数的转换。

#### Scenario: GraphQL 字段类型迁移

- **WHEN** Phase 0 完成后检查 `crates/graphql` 与 `crates/warp_server_client` 的至少一个示例字段
- **THEN** 字段类型为 `RemoteString` 而非 `String`
- **AND** 该字段在 UI 调用点直接通过 `Display`/`AsRef<str>` 渲染

### Requirement: Phase 0-5 必须按顺序覆盖以下范围

系统 SHALL 按以下 Phase 顺序覆盖 UI 字符串。每个 Phase 的覆盖范围 MUST 在该 Phase 完成时通过 `cargo xtask check-i18n` 的 warning 数下降验证。

- **Phase 0**：`warp_i18n` crate + `t!()` 宏 + locale 解析链 + `ui_components` crate 端到端示例（约 10 条字符串）
- **Phase 1**：`app/src/app_menus.rs` + `app/src/menu.rs` + `app/src/command_palette.rs` 与命令注册点（约 500 条）
- **Phase 2**：`app/src/settings_view/**` 全部子页（约 1500 条）
- **Phase 3**：终端 UI 框架——Block actions / Tooltip / Modal / Dialog / Banner / Toast / Onboarding / Resource Center（约 2000 条）
- **Phase 4**：AI 包裹 UI / Voice / 调试 / 隐藏菜单等长尾（约 2000 条）
- **Phase 5**：CI 切换为 `--mode hard` + 三平台 CJK 字形冒烟 + UI 截图回归

#### Scenario: 每个 Phase 完成时 warning 数下降

- **WHEN** Phase N 的 PR 合入主分支
- **THEN** `cargo xtask check-i18n --mode warning` 报告的违规数 MUST 严格小于 Phase N-1 完成时的违规数

#### Scenario: Phase 5 完成态

- **WHEN** Phase 5 PR 合入
- **THEN** `cargo xtask check-i18n --mode hard` 退出码为 0
- **AND** allowlist 条目数与 Phase 5 启动时一致或更少

### Requirement: 翻译术语遵循 GLOSSARY

所有翻译 MUST 遵循 `crates/warp_i18n/GLOSSARY.md` 中锁定的术语对照（design.md D10）。Phase 完成后 MUST 对译文做术语 grep 自查，发现不一致时回滚或修订。

#### Scenario: "Pane" 统一译为"窗格"

- **WHEN** Phase 1 完成
- **THEN** `bundles/zh-CN/**/*.ftl` 中 grep "面板" MUST 不出现在 pane 相关 entry
- **AND** 对应 entry 译文为"窗格"

### Requirement: Bundle 文件按 namespace 拆分

`crates/warp_i18n/bundles/{en,zh-CN}/` 下 SHALL 按 namespace 拆分 .ftl 文件，每文件聚焦单一 UI 模块。namespace 命名 MUST 与 key 前缀一致（例如 `menu.ftl` 内所有 key 以 `menu-` 开头）。

#### Scenario: namespace 与 key 前缀一致

- **WHEN** 检查 `bundles/en/menu.ftl`
- **THEN** 文件内每个 key MUST 以 `menu-` 开头

#### Scenario: en 与 zh-CN bundle key 集合相同

- **WHEN** Phase 4 完成时
- **THEN** 对每个 .ftl 文件，en 与 zh-CN 版本的 key 集合 MUST 完全一致
- **AND** CI MUST 提供 `cargo xtask check-i18n --check-parity` 子命令验证此条件

### Requirement: 桌面通知（client-side）汉化与三平台截断

由本仓库代码生成的桌面通知 `title` / `body`（`app/src/notification.rs` 等）MUST 经 `t!()` 汉化；并 MUST 通过 `warp_i18n::notification::truncate(s, max_chars)`（按 grapheme cluster 安全截断）适配三平台限制：

- macOS（`UNUserNotificationCenter`）：建议 ≤ 256 字符截断
- Windows（`ToastNotificationManager`）：title ≤ 64、body ≤ 240 字符截断
- Linux（`notify-rust`）：无硬限，应用 256 字符上限

服务端推送的通知（`RemoteString`）MUST 不被 `t!()` 处理。

#### Scenario: 客户端通知汉化

- **WHEN** Phase 3 完成且 zh-CN 用户触发本地通知 `t!("notification-update-available")`
- **THEN** 系统通知中心显示中文 title/body
- **AND** Windows 平台 body 长度 MUST ≤ 240 grapheme clusters

#### Scenario: 服务端通知保持英文

- **WHEN** GraphQL 返回 `Notification { title: RemoteString("Important update available") }`
- **AND** 客户端将其展示为系统通知
- **THEN** title MUST 保持原英文

### Requirement: 键位提示按平台规则渲染

系统 SHALL 提供 `display_for_locale(accel, locale, platform)` 路径，按 design.md D16 渲染：

- macOS：`⌘⇧⌥⌃` 符号 + 字母（如 `⌘⇧N`）
- Windows / Linux：英文文字 `Ctrl+Shift+Alt+N`

修饰键名字面量（`Cmd`/`Ctrl`/`Shift`/`Alt`/`Option`/`Meta`）MUST 进入 lint allowlist。

#### Scenario: macOS 渲染符号

- **WHEN** 在 macOS zh-CN locale 下渲染快捷键 `Cmd+Shift+N`
- **THEN** UI 显示 `⌘⇧N`

#### Scenario: Windows 渲染英文文字

- **WHEN** 在 Windows zh-CN locale 下渲染同一快捷键
- **THEN** UI 显示 `Ctrl+Shift+N`（修饰键不汉化）

### Requirement: Bundle parity 方向性

系统 MUST 满足 `keys(bundles/zh-CN/N.ftl) ⊆ keys(bundles/en/N.ftl)`（en 是 source of truth）。Phase 0-3 期间 zh-CN 缺 key 触发 `tracing::warn!` 并 fallback；Phase 4 完成时双向集合 MUST 完全相等。

#### Scenario: Property — Bundle 子集关系

- **WHEN** 对每个 namespace `N`，比较两侧 .ftl 的 key 集合
- **THEN** zh-CN keys ⊆ en keys
- **AND** Falsification: `prop::collection::btree_set("[a-z][a-z0-9-]{0,63}", 0..500)` 生成 en_keys 与 zh_keys 子集，验证 parity 校验仅拒绝非子集情形

### Requirement: RemoteString 类型级隔离

`RemoteString` MUST NOT 暴露任何转换路径将其内容作为 `t!()` 的 `&'static str` 参数。`tr!(remote.as_str())` 不论 lint 模式 MUST 被硬阻断（见 i18n-ci-gate spec）。

#### Scenario: Property — RemoteString 不可被 t!() 接受

- **WHEN** 通过 `trybuild` 注入候选模板 `t!(remote.as_str())`、`t!(&*remote.to_string())`、`t!(&format!("{}", remote))`
- **THEN** 全部 MUST 编译失败或被 lint 硬阻断
- **AND** Falsification: `\PC{0,1024}` 生成 RemoteString payload 填入模板
