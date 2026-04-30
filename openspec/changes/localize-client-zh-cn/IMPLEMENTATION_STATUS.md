# 实施状态备忘 — localize-client-zh-cn

## 给下次会话的核心提醒

**OpenSpec `tasks.md` 里 `[x]` 的状态严重失实。** 多个声称完成的 task 实际只做了一部分或完全没做。开始任何工作前，**先用 `cargo build --bin warp-oss -p warp` 验证现状**，不要相信 task 勾选。

---

## 当前真实状态（2026-04-30 第二会话末）

### 已让 fork 编过 + 跑起来
- `cargo build --bin warp-oss -p warp` exit 0（仅 2 个无关 warning：`std::sync::LazyLock` unused、`header_text` dead_code，**勿删除** —— 都是别处遗留）
- 二进制 `target/debug/warp-oss` 启动正常，首启即中文
- `Locale::DEFAULT = ZhCn`，无 settings 时 fallback 也走 ZhCn
- `git status --short | wc -l` = **162** 文件改动，**全部未 commit**

### 当前各设置页 t!() 调用数（从原始 → 现在）
| 文件 | 起点 | 目前 |
|---|---|---|
| `features_page.rs` | 1 | **92** |
| `ai_page.rs` | 32 | **139** |
| `appearance_page.rs` | 6 | **50** |
| `mod.rs`（settings_view） | — | 33 |

### 本会话补做的内容（在 working tree 里，未 commit）

| 区块 | 文件 | 备注 |
|---|---|---|
| 编译修复 56 处错 | `app/src/{lib,menu,language_settings}.rs`、`app/src/settings_view/**`、`crates/warpui/src/platform/mac/app.rs`、`app/src/view_components/filterable_dropdown.rs` 等 | 中文智能引号、`tracing::` 路径遮蔽、feature flag const 误转 fn、字段类型 `&'static str` ↔ `String` 不兼容、unsafe 块漏写。详见 git diff |
| 新增 intern helper | `crates/warp_i18n/src/intern.rs` + `crates/warp_i18n/src/macros.rs` 中加 `t_static!` 宏 | leak-once 缓存，给老 widget API 接 `&'static str` 处用 |
| Onboarding 全 crate 汉化 | `crates/onboarding/src/slides/{intro,intention,agent,free_user_no_ai,theme_picker,project,customize,third_party,toggle_card,two_line_button}_slide.rs` + `lib.rs` | task 5.6 原本声称完成实际未做；本会话补完 |
| Login slide 汉化 | `app/src/auth/login_slide.rs` | onboarding 入口 + Disable Warp Drive/AI 弹窗 + 浏览器登录页 + Privacy 页 |
| 右上角用户菜单 12 项 | `app/src/workspace/view.rs` | 在 `view.rs:8268-8344` 附近，`MenuItemFields::new("...")` |
| Bundle entry 新增 ~80 条 | `crates/warp_i18n/bundles/{en,zh-CN}/onboarding.ftl`、`menu.ftl` | onboarding-* / menu-user-* |
| `AI_FEATURES` / `WARP_DRIVE_FEATURES` 改为 key 数组 | `crates/onboarding/src/lib.rs` | 调用点 `intention_slide.rs` 与 `app/src/auth/login_slide.rs` 改 `tr!(item)` |

---

## 还没做的（按工作量降序）

仅 grep 长字面量（含大写开头 + 8 字符以上）的粗扫数。真实数 +/- 30%。

| 文件 | 估计未改字面量 | 用户截图覆盖 |
|---|---|---|
| ~~`app/src/settings_view/features_page.rs`~~ | ~~~176~~ → 大部分主可见字面量已迁完（2026-04-30 本会话）。剩 init_actions_from_parent_view 命令面板 ~30 处、`Active Screen` 等 dropdown 内部 name | 截图 11 通用/会话 |
| ~~`app/src/settings_view/ai_page.rs`~~ | ~~~108~~ → 大部分主可见 UI 已迁完（2026-04-30 同会话续）。t!() 调用从 32 → **139**。剩余命令面板 description_suffix、`render_ai_setting_subsection_header_static` 静态字符串若干 | 截图 4/5/6（AI/Knowledge / Third Party / Codebase） |
| ~~`app/src/settings_view/appearance_page.rs`~~ | ~~~60~~ → 主可见 UI 已迁完。t!() 调用 6 → **50**。剩余命令面板 description_suffix 几处 | 截图 10 主题/外观 |
| ~~`app/src/settings_view/mod.rs`~~ | ~~~52~~ → 头部 "Settings" + Split/Close 上下文菜单已做；剩内部 ContextFlag 标识符 ~30（不应汉化） | 设置页主入口 / 工厂 |
| `app/src/settings_view/code_page.rs` | ~45 | 截图 7 Editor and Code Review |
| `app/src/settings_view/environments_page.rs` | ~26 | 截图 8 Environments（顶部说明 / Quick setup / Use the agent） |
| `app/src/settings_view/keybindings.rs` | ~6（**但**注册中心命令名 ~150+ 在别处） | 截图 12 |
| `app/src/settings_view/main_page.rs` | ~5 | |
| `app/src/settings_view/platform_page.rs` | ~3 | 截图 9 Teams |
| `app/src/menu.rs` | ~6 | 上下文菜单基础设施 |
| MCP Servers / Profiles / Warp Drive 等子页 | 未单独 grep | 截图 2/3 |
| 命令面板 / 全部命令名注册中心 | 未确认 | 截图 12 命令名 |

### 2026-04-30 第二会话 ai_page.rs 具体改动

**bundle 增量**（`crates/warp_i18n/bundles/{en,zh-CN}/settings_ai.ftl` 50 → 150 行），新增 ~100 个 `settings-ai-*` key：
- 权限区块完整（`section-permissions`、`perm-apply-diffs`、`perm-read-files`、`perm-execute`、`perm-interact`、`perm-managed-by-workspace`）
- Command/Directory denylist & allowlist 两两描述都搬迁
- Base model 区（标题 + 长 desc）、Codebase Context、`learn-more`
- MCP 子区块（`mcp-call-servers`、`mcp-allowlist-desc`、`mcp-denylist-desc`、`mcp-add-server`、`mcp-manage`、`mcp-autospawn` + desc、`mcp-supported-providers`、`mcp-select-servers-header`）
- 输入相关（`input-hint-text`、`show-agent-tips`、`include-agent-cmds-history`、`autodetect-prompts-terminal`、`autodetect-cmds-agent`、`natural-lang-detection` + desc、`natural-lang-denylist` + desc、`let-us-know`）
- 规则 / 知识库（`rules-help-desc`、`suggested-rules` + desc、`warp-drive-context` + desc、`knowledge`、`manage-rules`）
- 语音输入（`voice-input` + desc、`voice-input-key` + tip）
- 工具栏（`show-conv-history`、`thinking-display` + desc、`existing-conv-layout`、`show-coding-toolbar` + desc、`third-party-cli-agents`、`rich-input-auto-show/-open/-dismiss`、`requires-plugin`、`toolbar-regex-desc`、`toolbar-layout`、`show-model-picker`、`toolbar-cmds-enable-span`）
- Agent Attribution（toggle / section / desc）
- Cloud computer use（`cloud-computer-use` + desc、`experimental` 标签）、Orchestration（+ desc）
- BYOK（`byok-desc`、`openai-key`、`anthropic-key`、`google-key`、`upgrade-build`、`credit-fallback`）
- AWS Bedrock（`bedrock-managed-by-org`、`bedrock-toggle`、`bedrock-login-cmd`、`bedrock-auto-run-login` + desc）
- 顶部状态（`active-ai`、`next-command`、`split-pane`、`read-only-permission`、`supervised-permission`、`restricted-billing`、`unlimited`）
- 区块标题（`prompt-suggestions`、`suggested-code-banners`、`natural-lang-autosuggestions`、`shared-block-title-gen`、`warp-agent`、`create-account-prompt`、`profiles`、`set-boundaries-desc`）
- 其它（`other`、`select-coding-agent`、`resets-after = Resets {$time}` Fluent 占位符）

### 2026-04-30 第二会话 appearance_page.rs 具体改动

**bundle 增量**（`settings_appearance.ftl` 12 → 80 行），新增 ~55 个 `settings-appearance-*` key：
- 输入位置 4 个命令面板项（`input-start-top`、`input-pin-top`、`input-pin-bottom`、`toggle-input-mode`）
- 标签栏配对项（`show-code-review-tab`/`hide-code-review-tab`、`tab-always-show`、`tab-hide-fullscreen`、`tab-hover-only`）
- `fullscreen-apps`（用 `t_static!()` 因为 `Category::new` 要 `&'static str`）
- 主题/图标（`create-custom-theme`、`sync-with-os`、`customize-app-icon`、`restart-icon-hint`）
- 窗口区（`custom-window-size`、`window-opacity`、`transparency-not-supported`、`graphics-no-transparent`、`use-window-blur`、`hardware-no-transparent`、`tools-panel-consistent`）
- 输入/面板（`input-type`、`input-position`、`dim-inactive`、`focus-follows-mouse`、`compact-mode`、`show-jump-bottom`、`show-block-dividers`、`agent-font`、`terminal-font`、`thin-strokes`、`min-contrast`、`show-ligatures`、`cursor-type`、`blinking-cursor`）
- 标签（`tab-close-button-pos`、`show-tab-indicators`、`show-code-review-btn`、`preserve-tab-color`、`vertical-tab-layout`、`prompt-as-tab-title`、`header-toolbar-layout`、`show-tab-bar`、`custom-alt-screen-padding`）

**关键代码模式注意**（**给下次会话避免再踩**）：
- `Text::new_inline(text, ...)` 接 `impl Into<Cow<'static, str>>`：传 `t!()` owned 值 OK，但 `&t!()` 是 `&String` 不能 → `Cow<'static, str>`，**要么 owned 要么 `t_static!()`**。
- `render_dropdown_item(label: &str, ...)`、`render_ai_list(header: &str, desc: &str, ...)`、`render_full_pane_width_ai_button(text: &str, ...)`、`ToggleSettingActionPair::new(suffix: &str, ...)`、`render_execution_profile_dropdown(header: &str, ...)`、`Hoverable::new` 等多数 `&str` 参数：用 `&warp_i18n::t!(...)`。
- `render_api_key_input(label: &'static str, ...)`、`render_input(label: &'static str, ...)`、`Category::new(title: &'static str, ...)`、`set_menu_header_to_static(...)`：必须 `t_static!()`。
- `render_ai_setting_description(impl Into<Cow<'static, str>>)`、`paragraph(impl Into<Cow<...>>)`：传 owned `t!()` 即可，**不要加 `&`**（会变 `&String` 触发 E0277）。
- `Cow::Borrowed(t!(...))` 不行（t! 返回 owned String），改 `Cow::Owned(t!(...))` 并显式标 `Cow<'_, str>`。

### 2026-04-30 本会话 features_page.rs 具体改动

新增 ~80 个 bundle entry（`settings-features-*`）覆盖：
- 全部 50+ 个 `render_body_item` 标题
- 9 个 `Category::new()` 分组标题 (用 `t_static!`)
- 4 个 `tooltip_override_text`（open-links-desktop / mouse-wheel / linux-clipboard / wayland）
- Pin 4 方向 / NewTabPlacement 2 项 / Tab key behavior 等 dropdown 文案
- Quake 模式提示（autohide / 长时通知前缀 / 设置全局快捷键 / 按下新快捷键 / 修改快捷键）
- 通知开关三项（agent completed / needs attention / play sound）+ Toast 前缀
- Vim 子项（unnamed register / status bar）
- 会话恢复 Wayland 提示 + "查看文档" + "Wayland 不支持" 文案
- macOS / non-mac `Start Warp at login` 双语版本
- `Make Warp the default terminal` / `Warp is the default terminal` / `takes-effect-new-sessions`

跳过的（命令面板专用、Setting 页本体不显示）：
- `init_actions_from_parent_view` 内 `ToggleSettingActionPair::new(suffix, ...)` 中的 `description_suffix` 字面量约 30 处（前缀 `Enable `/`Disable ` 由调用端硬拼）
- `EXTRA_META_KEYS_LEFT_TEXT` / `RIGHT_TEXT` 静态常量保留（命令面板用），UI 渲染走新增的 `extra_meta_keys_left_text()` / `right_text()` 函数

**合计估算 1000+ 处。** 完整做完是十几小时连续工作。

### 已知 follow-up（OpenSpec tasks.md `[ ]` 的）
- **8.4** `&'static str` 占位符字段族重构（9 处位置在 `tasks.md:124` 显式列出）。本会话用 `t_static!` intern 绕过，仅是临时补丁；正确做法是把那些字段改为 `String`/`Cow<'static, str>` + 接 invalidate。
- **8.5** `qa-zh-cn-manual-regression` 提案：真机视觉回归三平台。

---

## 重启工作的命令清单

```bash
# 看当前 diff（确认上轮工作还在）
git status --short | wc -l   # 应该有 ~150 行

# 验证编译
cargo build --bin warp-oss -p warp

# 启动看效果
pkill -f "target/debug/warp-oss"; /Users/liji/warp/target/debug/warp-oss &

# 跑某个 crate 的快速 check
cargo check -p onboarding
cargo check --bin warp-oss -p warp

# 扫一个文件还有多少未改字面量（粗）
grep -nE '"[A-Z][a-z][a-zA-Z\s\-,\.\:]{8,}"' app/src/settings_view/<page>.rs \
  | grep -vE 'warp_i18n::|^\s*//|tracing!|log::|cfg|track_caller'
```

---

## 关键陷阱 / 反复踩坑的点

1. **菜单顶层不在 `app_menus.rs`**。截图 1 那个右上角下拉菜单在 `app/src/workspace/view.rs:8268` 附近的 `MenuItemFields::new("...")`，不是 macOS 顶部菜单。
2. **`tracing::error!` 在 `app` crate 内被本地 `app/src/tracing.rs` 模块遮蔽**，必须用绝对路径 `::tracing::error!`。
3. **很多 widget API 字段是 `&'static str`**（`PageType`、`Category`、`ActionButton::label` 接 `Cow<'static, str>` 还好，但 `set_menu_header_to_static`、`Text::new_inline` 接受 `Cow<'static, str>` 但写 `.as_str()` 借用临时 String 会 E0716）。两条路：
   - 把字段改 `String`（推荐：`filterable_dropdown::static_menu_header` 已这样改）
   - 用 `warp_i18n::t_static!("key")` 走 intern（应急）
4. **`Hoverable::new` 闭包是 `FnOnce`**，可以 move owned String 进去；`Hoverable::on_click` 等是 `Fn`，需要 clone。
5. **`log::warn!`/`error!` 第一参数必须字面量字符串**，不能直传 String。改 `log::warn!("{}", warp_i18n::t!(...))`。
6. **`t!()` 命名参数语法是 `key = value`**，不是 `key: value`（Rust struct field shorthand 容易写错）。
7. **`AI_FEATURES` / `WARP_DRIVE_FEATURES` 现在是 key 数组**（不是 UI 文案数组）。所有消费点要 `warp_i18n::tr!(key)` 渲染。
8. **macOS Metal Toolchain 单独装**：`xcodebuild -downloadComponent MetalToolchain`，不装 build 就 panic。
9. **`protoc` 必须装**：`brew install protobuf`，否则 `warp_multi_agent_api` build 失败。
10. **`warp` bin 需要 `warp-channel-config`**（internal 私有 repo）。OSS fork 应当跑 `warp-oss` bin（`oss.rs` 入口，硬编码 ChannelConfig）。
11. **不在 workspace 根目录跑 `cargo build --bin warp-oss` 会报"not found"**。当前目录在子 crate 时必须加 `-p warp`。
12. **`crates/warp_i18n/wip/` 是 .gitignore 的产物**，但 cargo workspace `members = ["crates/*"]` glob 会扫到它。如果 stash 了 untracked，`crates/warp_i18n/` 壳还在，cargo 会报 manifest 找不到。临时 mv 走即可。

---

## 推荐下一步

**按用户使用频率优先排序**做剩余设置子页。建议每会话锁定 1-2 个子页彻底做完（含 bundle entry + 调用点 + cargo check 多轮 + 真机验证），不要拍脑袋一次全揽。

**强烈建议**先做完命令面板 / keybindings 命令名注册中心（找到注册点先），那块涉及面广、影响所有设置页里出现的命令名。

### 下次会话立即可做的清单（按优先级）

1. **`code_page.rs`（~45 处）** —— 截图 7 Editor and Code Review 子页。`settings_code.ftl` 现有 198 行 bundle，估计需追加 ~60 个 key。
2. **`environments_page.rs`（~26 处）** —— 截图 8 Environments，包括顶部说明、Quick setup、Use the agent 文案。
3. **`menu.rs`（~6 处）** —— 上下文菜单基础设施，影响面较广。
4. **MCP Servers / Profiles / Warp Drive 子页** —— 单独 grep 看每个文件内未改字面量。
5. **命令面板 / keybindings 注册中心** —— 涉及 ~150+ 命令名，需先找注册点。当前看到的 `init_actions_from_parent_view` 内 `ToggleSettingActionPair::new(suffix, ...)` 有数十处，得统一处理。

### 给下次会话的工作模板

```bash
# 1. 进入项目
cd /Users/liji/warp

# 2. 验证现状
git status --short | wc -l  # 应仍为 162 左右
cargo build --bin warp-oss -p warp 2>&1 | tail -3

# 3. 选定一个文件，扫剩余字面量
grep -nE '^[[:space:]]+("[A-Z][a-z][a-zA-Z\s\-,\.\:\?\!\(\)/]{6,}")[\.,]' \
  app/src/settings_view/code_page.rs \
  | grep -vE 'warp_i18n::|^[[:space:]]*//|action:|::storage_key|::sync_to_cloud'

# 4. 先批量加 ftl key，再批量替换调用点
# 5. cargo build 反复验证（每 5-10 次 Edit 一次 build）
# 6. pkill warp-oss; ./target/debug/warp-oss & 视觉验证
```
