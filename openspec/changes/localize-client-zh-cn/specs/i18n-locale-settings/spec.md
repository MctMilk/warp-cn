## ADDED Requirements

### Requirement: 用户 locale 偏好设置项

系统 SHALL 在用户设置中提供 `language` 偏好项，类型为 `Option<Locale>`，可取值 `Some(Locale::ZhCn)`、`Some(Locale::En)`、`None`（语义为「fork 默认」）。默认值 MUST 为 `None`，在本 fork 中**强制等价 `Locale::ZhCn`**（design.md D13）。

#### Scenario: 全新安装首次启动强制中文

- **WHEN** 全新安装的 Warp 首次启动且 `settings.language = None`
- **THEN** 生效 locale MUST 等于 `Locale::ZhCn`
- **AND** 系统 locale MUST 不被读取（不影响生效结果）

#### Scenario: 用户在设置页选择 zh-CN

- **WHEN** 用户在设置页"语言"下拉中选择"简体中文"
- **THEN** `settings.language` 被持久化为 `Some(Locale::ZhCn)`
- **AND** UI 立即开始用中文渲染（无需重启）

### Requirement: 启动时按解析链确定生效 locale

系统启动时 MUST 按以下优先级链解析当前 locale：

1. `settings.language = Some(L)` → 锁定 `L`。
2. `settings.language = None` → 强制 `Locale::ZhCn`（fork 默认，design.md D13）。
3. 仅当用户在设置页选择「跟随系统」时（写入设置文件特殊值 `"system"`）→ 调用 `sys-locale::get_locale`；语言子标签 `zh` → `ZhCn`，其余 → `En`；不可用时 → `En`。

设置文件中 `language` 的磁盘表示为以下三种 TOML 字符串之一：`"zh-CN"`、`"en"`、`"system"`，或字段缺失（视为 `None`）。

#### Scenario: 缺省 None 强制中文

- **WHEN** `settings.language = None`（字段缺失）
- **THEN** 生效 locale MUST 为 `Locale::ZhCn`
- **AND** `sys_locale::get_locale` MUST 不被调用

#### Scenario: 显式跟随系统且系统为 zh-Hans-CN

- **WHEN** `settings.language = "system"`
- **AND** `sys_locale::get_locale()` 返回 `Some("zh-Hans-CN")`
- **THEN** 生效 locale MUST 为 `Locale::ZhCn`

#### Scenario: 显式跟随系统且系统为 ja-JP

- **WHEN** `settings.language = "system"`
- **AND** `sys_locale::get_locale()` 返回 `Some("ja-JP")`
- **THEN** 生效 locale MUST 为 `Locale::En`

#### Scenario: 显式跟随系统但系统 locale 不可用

- **WHEN** `settings.language = "system"`
- **AND** `sys_locale::get_locale()` 返回 `None`
- **THEN** 生效 locale MUST 为 `Locale::En`

#### Scenario: 显式选择 English

- **WHEN** `settings.language = Some(Locale::En)`
- **THEN** 生效 locale MUST 为 `Locale::En`，无视系统 locale

### Requirement: 运行时切换 locale 必须立即生效

系统 SHALL 提供 `warp_i18n::set_locale(locale)` API。调用后所有后续 `t!()` 查找 MUST 返回新 locale 的字符串，且 UI 层 MUST 在下一帧完成重绘，无需进程重启。

#### Scenario: 用户切换语言后 UI 立即更新

- **WHEN** 用户在设置页将语言从"English"切换到"简体中文"
- **THEN** `warp_i18n::set_locale(Locale::ZhCn)` 被调用
- **AND** 顶部菜单、命令面板、设置页本身在下一帧内全部以中文渲染
- **AND** macOS 顶部菜单 MUST 通过 `NSApplication::setMainMenu:` 重建以反映新语言

### Requirement: locale 变更必须广播给订阅者

系统 SHALL 通过 `tokio::sync::watch::Receiver<Locale>` 对外暴露 locale 变更通知。需要响应 locale 变更的子系统（如 macOS 菜单重建、UI 重绘触发）MUST 订阅此通道。

#### Scenario: 菜单子系统订阅并响应

- **WHEN** locale 切换为 zh-CN
- **THEN** `app/src/menu.rs` 监听器被唤醒
- **AND** 主菜单树被重建并提交给 AppKit

### Requirement: 设置页 UI 中提供语言选择器

系统 SHALL 在设置页（`app/src/settings_view`）的"外观"或"通用"分类下提供语言选择下拉，选项依次为「简体中文 (zh-CN)」「English」「跟随系统」。

#### Scenario: 选择"跟随系统"

- **WHEN** 用户在语言下拉中选择"跟随系统"
- **THEN** `settings.language` 字段被持久化为 TOML 字符串 `"system"`
- **AND** 生效 locale MUST 立即按解析链重新计算

### Requirement: locale 解析为纯函数

`resolve_locale(settings, sys_locale_lookup)` MUST 是纯函数：相同输入产生相同输出，无 I/O、无随机性、无环境变量读取。

#### Scenario: Property — Locale 解析确定性

- **WHEN** 对任意 `s ∈ Option<Locale | "system">` 与任意 `tag ∈ Option<String>`，调用 `resolve_locale(s, tag)` 1000 次
- **THEN** 所有返回值相等且 ∈ `{Locale::En, Locale::ZhCn}`
- **AND** Falsification: `prop_oneof![None, Some(En), Some(ZhCn), System] × prop::option::of(any::<String>())`

### Requirement: Locale tag 规范化

`normalize_sys_locale(tag)` MUST 满足：`tag.lowercase().starts_with("zh")` → `ZhCn`；`starts_with("en")` → `En`；其他 → `En`。该函数 MUST 接受 `_` 与 `-` 两种分隔符，并对大小写不敏感。

#### Scenario: Property — Tag 规范化

- **WHEN** 对任意 `tag` 调用 `normalize_sys_locale(tag)`
- **THEN** 结果与 `lowercase` 后前缀比对的判定一致
- **AND** Falsification: `prop_oneof![Just("zh"), Just("zh-CN"), Just("zh_Hans_CN"), Just("zh_Hant_TW"), Just("en"), Just("en-US"), Just("ZH"), "[a-zA-Z]{0,12}([-_][a-zA-Z0-9]{0,12}){0,4}"]`

### Requirement: settings.language 序列化往返

`Settings { language: x }` MUST 满足 `deserialize(serialize(s)) == s`，其中 `x ∈ {None, Some(En), Some(ZhCn), System}` 对应磁盘表示 `<missing>`/`"en"`/`"zh-CN"`/`"system"`。未知字符串值 MUST 被视为 `None`，并记录 `tracing::warn!`。

#### Scenario: Property — 设置往返

- **WHEN** 对任意 `x` 写盘后再读取
- **THEN** 读取值与原值完全相等
- **AND** Falsification: `prop_oneof![Just(None), Just(Some(En)), Just(Some(ZhCn)), Just(System)]` 配合 `prop::collection::btree_map(any field name, any value, 0..50)` 注入无关 settings 字段
