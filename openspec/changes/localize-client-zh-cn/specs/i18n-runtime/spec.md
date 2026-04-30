## ADDED Requirements

### Requirement: 提供 `warp_i18n` crate 作为客户端 i18n 运行时

系统 SHALL 提供 `warp_i18n` crate，封装 Fluent bundle 加载、locale 解析、字符串查找与回退逻辑。所有客户端 UI 字符串的本地化查找 MUST 通过此 crate 完成。

#### Scenario: 主程序初始化时加载 i18n 运行时

- **WHEN** Warp 主程序启动
- **THEN** `warp_i18n::init()` 被调用，加载 en 与 zh-CN 两个 Fluent bundle
- **AND** 后续任意 `t!()` 调用 MUST 不再触发 bundle 加载

#### Scenario: bundle 加载失败时崩溃而非沉默回退

- **WHEN** 嵌入的 .ftl 资源解析失败（例如打包缺失 / 语法错误）
- **THEN** `warp_i18n::init()` MUST panic 并附带具体的 .ftl 文件名与错误位置
- **AND** 这种情况 MUST 在 `build.rs` 编译期被阻止，运行期 panic 仅作为最后防线

### Requirement: 提供 `t!()` 宏用于静态 key 的字符串查找

系统 SHALL 提供 `t!(key, ...)` 宏，接受 `&'static str` 类型的 key 字面量与可选命名参数，返回 `String`。Key MUST 在编译期被校验为 en bundle 中存在的有效 key。

#### Scenario: 已知 key 的查找

- **WHEN** 代码调用 `t!("menu-file-new-window")` 且当前 locale 为 zh-CN
- **THEN** 宏返回 `String::from("新建窗口")`

#### Scenario: 不存在的 key 在编译期被阻止

- **WHEN** 代码包含 `t!("nonexistent-key")` 且 en bundle 中无该 key
- **THEN** `cargo build` MUST 编译失败，错误信息包含 key 名与缺失提示

#### Scenario: 带命名参数的查找

- **WHEN** 代码调用 `t!("tabs-close-confirm", n: 3)` 且当前 locale 为 zh-CN
- **AND** zh-CN bundle 含 `tabs-close-confirm = 关闭 {$n} 个标签页`
- **THEN** 宏返回 `String::from("关闭 3 个标签页")`

#### Scenario: 复数（Fluent selector）

- **WHEN** zh-CN bundle 含 `tabs-close-confirm = { $n -> [one] 关闭 1 个标签页 *[other] 关闭 {$n} 个标签页 }`
- **AND** 代码调用 `t!("tabs-close-confirm", n: 1)`
- **THEN** 宏返回 `String::from("关闭 1 个标签页")`

### Requirement: 提供 `tr!()` 宏用于动态 key 或显式 locale

系统 SHALL 提供 `tr!()` 宏作为 `t!()` 的运行时变体，支持非字面量 key 与显式 locale 参数。Key 不存在时 MUST fallback 到 en；en 也缺则返回 key 字符串本身并记录 `tracing::error!`。

#### Scenario: 显式 locale 的查找

- **WHEN** 代码调用 `tr!(Locale::En, "menu-file-new-window")`
- **THEN** 宏忽略当前 locale，返回 en bundle 中的 `"New Window"`

#### Scenario: zh-CN 缺失 key 自动回退到 en

- **WHEN** 代码调用 `tr!("only-in-en-key")` 且 zh-CN bundle 无该 key 但 en bundle 有
- **THEN** 宏返回 en bundle 的内容
- **AND** `tracing::warn!` 记录 zh-CN 缺失该 key（用于后续翻译补全）

### Requirement: Fluent fallback chain 必须包含 en 作为最后兜底

系统的 Fluent bundle fallback chain MUST 配置为 `[当前 locale, en]`。en bundle 是真实来源（source of truth），所有 key 在 en bundle 中 MUST 存在。

#### Scenario: 三平台启动后 fallback chain 一致

- **WHEN** 在 macOS / Windows / Linux 上以 zh-CN locale 启动 Warp
- **THEN** `warp_i18n::current_fallback_chain()` 返回 `[zh-CN, en]`

### Requirement: 字符串查找性能必须满足 UI 响应预算

`t!()` 调用的 P99 延迟 MUST < 10μs（命中 zh-CN 路径），以满足 60fps UI 重绘预算。

#### Scenario: 微基准测试

- **WHEN** 运行 `cargo bench -p warp_i18n` 的 lookup 基准
- **THEN** P99 < 10μs
- **AND** 基准测试 MUST 在 CI 中以非阻断方式运行（仅记录回归）

### Requirement: `t!()` 必须由 proc-macro 实现并在编译期校验 key

系统 SHALL 通过独立 crate `warp_i18n_macros` 提供 `t!` proc-macro。`build.rs` 在 `warp_i18n` 解析 `bundles/en/**/*.ftl` 后生成 `OUT_DIR/key_index.rs`，内部为 `phf::Set<&'static str>`。proc-macro 接受 `LitStr` 字面量 key，与该 phf 集合比对；非字面量 key MUST 触发 `compile_error!`，未命中 key MUST 触发包含 key 名的 `compile_error!`。

#### Scenario: 非字面量 key 编译失败

- **WHEN** 代码包含 `let k = "menu-file-new-window"; t!(k);`
- **THEN** 编译失败，错误信息包含 `t! requires a string literal key; use tr! for dynamic keys`

#### Scenario: 缺失 key 编译失败信息可追踪

- **WHEN** 代码包含 `t!("typo-key")` 且 en bundle 不存在 `typo-key`
- **THEN** 编译错误包含 key 字符串 `typo-key` 与提示 `Add it before referencing.`
- **AND** 错误指向调用点行号

### Requirement: 全局状态必须支持任意线程并发访问

`warp_i18n` 全局单例 MUST 实现 `Send + Sync` 并支持 lock-free 读路径。`t!()`/`tr!()` MUST 可从任意线程（含非 tokio runtime 线程）调用，不得 panic。`set_locale(L)` MUST 先发布 locale 状态、后通知 watcher，保证 happens-before 序。

#### Scenario: 多线程并发查找无需锁

- **WHEN** 100 个工作线程并发调用 `t!("menu-file-new-window")` 1000 次
- **THEN** 无任何 panic、deadlock 或数据竞争
- **AND** 所有返回值与单线程结果一致

#### Scenario: init() 幂等

- **WHEN** `warp_i18n::init()` 被连续调用 3 次
- **THEN** 仅首次执行 bundle 加载
- **AND** 后续调用直接返回同一 `&'static I18n` 引用

### Requirement: `t!()` 渲染纯函数性

固定 `(locale, key, args)` 输入，`t!()` 调用 MUST 返回 bit-wise 相等的 `String`，不得修改全局状态、不得读取环境变量、不得依赖时间。

#### Scenario: Property — 渲染纯函数性

- **WHEN** 对任意 `L ∈ {En, ZhCn}`、任意有效 key K 与命名参数 args，重复调用 `render(L, K, args)` 1000 次
- **THEN** 所有返回值相等
- **AND** `current_locale()`、bundle 内容、args 均未被修改
- **AND** Falsification: proptest 用 `prop::sample::select(keys)` × `btree_map(arg-name, FluentValue)` 生成 (key, args) 进行 1000 轮快照对比

### Requirement: locale 切换的可线性化语义

并发 `Set(L)` 与 `Lookup(K, A)` 操作的执行 MUST 等价于某种保留每线程顺序的串行序；任一 `Lookup` 观察到的 fallback chain MUST 是切换前或切换后的完整 chain，不得是混合状态。

#### Scenario: Property — Locale 切换可线性化

- **WHEN** `loom` 或 `tokio::test(start_paused = true)` 交错执行 200 个 `Set` / `Lookup` 操作
- **THEN** 每个 `Lookup` 结果 MUST 属于「切换前」或「切换后」locale 的有效输出
- **AND** Falsification: `prop::collection::vec(prop_oneof![Set(any::<Locale>()), Lookup(key, args)], 0..200)` 加 loom interleaving

### Requirement: locale 切换必须广播到 watcher

每次 `set_locale(L)`（其中 L ≠ 当前 locale）MUST 向所有活跃 `watch::Receiver<Locale>` 至少投递一次 `changed()`，且观察序列保留时间序。重复 `set_locale(L)`（L 等于当前）MAY coalesce。

#### Scenario: Property — Watch 投递

- **WHEN** 顺序执行 100 次 `set_locale(L_i)`，其中相邻 L 互异
- **THEN** 每个 receiver 观察到的 distinct locale 子序列 MUST 与输入序列相同
- **AND** Falsification: `prop::collection::vec(prop_oneof![Just(En), Just(ZhCn)], 0..100)` × 0..20 个 receiver

### Requirement: malformed FTL 在 build.rs 期被全量拒绝

build.rs MUST 解析所有 `bundles/{en,zh-CN}/**/*.ftl`；任意一个文件语法错误 MUST 让编译失败，错误信息包含文件路径与至少一个 `(line, col)` 源位置。

#### Scenario: Property — 全量拒绝 malformed

- **WHEN** 在 fixtures 中注入任一 `prop_oneof![unterminated_selector, missing_eq, duplicate_id, invalid_escape]`
- **THEN** `cargo build -p warp_i18n` 退出非 0
- **AND** stderr 包含错误文件路径与位置
- **AND** Falsification: `\\PC{0,512}.prop_filter("must be invalid", |s| fluent_syntax::parser::parse(s).is_err())` 注入
