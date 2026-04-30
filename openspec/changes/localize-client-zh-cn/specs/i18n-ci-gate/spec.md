## ADDED Requirements

### Requirement: 提供 `cargo xtask check-i18n` 子命令

系统 SHALL 提供 `cargo xtask check-i18n` 子命令，使用 `syn` 解析全部生产代码 AST，识别 UI 构造点（D7 列出的调用名单）的字符串字面量。命令 MUST 输出违规位置（文件:行号 + 字面量内容）。

#### Scenario: 命令存在且可执行

- **WHEN** 在仓库根目录运行 `cargo xtask check-i18n --help`
- **THEN** 退出码为 0 且输出包含子命令使用说明

#### Scenario: 检测到未本地化的 UI 字符串

- **WHEN** `app/src/menu.rs` 包含 `MenuItem::new("Untranslated Item", ...)`
- **AND** "Untranslated Item" 不在 allowlist 中
- **THEN** `cargo xtask check-i18n` MUST 输出违规：`app/src/menu.rs:<line>: untranslated string "Untranslated Item" at MenuItem::new`

### Requirement: lint 必须豁免开发者面字符串

`check-i18n` MUST 自动豁免以下场景的字符串字面量：`tracing::*!`、`log::*!`、`println!`、`eprintln!`、`panic!`、`assert*!`、`debug_assert!`、`format!`/`write!` 在 log/panic 上下文内、`#[cfg(test)]` 模块、文件名以 `_test.rs`/`_tests.rs` 结尾的测试文件、`fmt::Debug`/`fmt::Display` impl 块内字面量、以 `_` 开头的私有常量。

#### Scenario: tracing 调用不被标记

- **WHEN** 代码包含 `tracing::info!("Loading user config")`
- **THEN** `cargo xtask check-i18n` MUST 不报告该字符串

#### Scenario: 测试文件不被标记

- **WHEN** `*_test.rs` 中包含 `Button::new("Click me")`
- **THEN** `cargo xtask check-i18n` MUST 不报告该字符串

### Requirement: 提供 allowlist 处理合法的英文字面量

系统 SHALL 在 `crates/warp_i18n/lint_allowlist.toml` 维护 allowlist。文件 schema：

```toml
schema_version = 1
phase5_baseline_count = 0  # Phase 5 启动时固化条目数

[[entries]]
file_glob = "crates/keymap/src/**/*.rs"      # globset 语法，仓库相对路径
callsite = "Keystroke::parse"                # syn::Path 末段或全限定名（精确匹配，非正则）
literal = "cmd-n"                             # 转义后的字符串字面量精确值
reason = "keystroke parser DSL"              # 必填，>= 10 字符
added_phase = 0                               # 0..=5
owner = "i18n-team"                           # 自由字符串
```

匹配语义为 entry 集合的成员谓词，对集合排列 invariant（commutative）。

#### Scenario: allowlist 命中

- **WHEN** 代码包含 `Keystroke::parse("cmd-n")` 且 allowlist 含对应条目
- **THEN** `cargo xtask check-i18n` MUST 不报告

#### Scenario: allowlist 在 Phase 5 后必须冻结

- **WHEN** Phase 5 之后的 PR 试图新增 allowlist 条目
- **AND** 该 PR 未带 `i18n-allowlist-grow` label
- **THEN** CI MUST 阻断（presubmit fail）

### Requirement: lint 在 Phase 5 切换为硬阻断模式

`check-i18n` MUST 提供 `--mode warning|hard` 开关。Phase 0-4 期间 CI 以 `--mode warning` 运行（违规仅警告，CI 通过）；Phase 5 起切换为 `--mode hard`（违规直接 fail CI）。

#### Scenario: warning 模式下违规但 CI 通过

- **WHEN** 以 `--mode warning` 运行且检测到 5 处违规
- **THEN** 退出码为 0
- **AND** stderr 输出违规清单 + 总数

#### Scenario: hard 模式下违规直接 fail

- **WHEN** 以 `--mode hard` 运行且检测到 1 处违规
- **THEN** 退出码非 0
- **AND** GitHub Actions step 标红

### Requirement: 接入 GitHub Actions

系统 SHALL 在 `.github/workflows/i18n-lint.yml`（独立 workflow，不混入既有 presubmit）中定义 `i18n-lint` job：

- `runs-on: ubuntu-latest`
- 步骤：checkout → setup-rust（与 workspace toolchain 一致）→ `cargo xtask check-i18n --mode <warning|hard>` → `cargo xtask check-i18n --check-parity`
- 触发：`pull_request` 与 `push: master`
- `paths` 过滤：`**/*.rs`、`crates/warp_i18n/bundles/**`、`xtask/src/i18n_lint/**`、`crates/warp_i18n/lint_allowlist.toml`

Phase 0 落地时为 `--mode warning`，Phase 5 完成时切换为 `--mode hard` 并加入 branch protection 必过检查。

#### Scenario: PR 触发时执行 lint

- **WHEN** 任意 PR 推送至 fork master 分支且改动命中 `paths` 过滤
- **THEN** workflow `i18n-lint.yml` MUST 触发
- **AND** 该 job 运行 `cargo xtask check-i18n --mode <current>` 与 `--check-parity` 并报告结果

#### Scenario: paths 不命中时跳过

- **WHEN** PR 仅修改 `README.md` 等非代码、非 bundle 文件
- **THEN** workflow MUST 不执行（节省 CI 资源）

### Requirement: 禁止对 `RemoteString` 类型调用 `t!()`

`check-i18n` MUST 检测对 `RemoteString` 字段或其 `.as_str()` 结果的 `t!()`/`tr!()` 调用，并在任意模式下硬阻断（不依赖 warning/hard 模式）。

#### Scenario: 误用 RemoteString 调用 t!()

- **WHEN** 代码包含 `t!(notification.title.as_str())` 且 `title` 为 `RemoteString`
- **THEN** `cargo xtask check-i18n` MUST 退出码非 0
- **AND** 输出明确指示 "RemoteString cannot be passed to t!/tr!"

### Requirement: CLI crate 全量豁免

`check-i18n` MUST 排除 globs `crates/warp_cli/**` 与 `crates/warp_cli_*/**`，不扫描 CLI 字符串字面量。CLI 字符串保持英文，不进 allowlist（避免 allowlist 噪声）。

#### Scenario: warp_cli 字面量不被报告

- **WHEN** `crates/warp_cli/src/main.rs` 含 `Command::new("warp").about("Terminal of the future")`
- **THEN** `cargo xtask check-i18n` MUST 不报告该字符串

### Requirement: lint 行为为纯函数

`lint(source, allowlist, sites_table)` MUST 是纯函数：相同输入产生相同输出，与文件系统遍历顺序、环境变量、时区无关；对 allowlist entries 的排列具备 commutativity。

#### Scenario: Property — Lint 确定性与 allowlist commutativity

- **WHEN** 对同一 `(source, allowlist, sites_table)` 重复运行 lint，并对 allowlist 任意置换 50 次
- **THEN** 所有运行的违规集合（按文件:行号:literal 排序后）相等
- **AND** Falsification: `prop::collection::vec(entry_strategy, 0..100)` × `SliceRandom::shuffle` × 多次 lint 比对
