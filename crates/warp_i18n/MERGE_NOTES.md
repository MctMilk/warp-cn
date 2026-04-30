# 与上游 `warpdev/warp` 同步策略

本 fork 长期维护中文社区版，需要定期 merge 上游变更。汉化改动覆盖 ~1400 个 `.rs` 文件，必然冲突。
本文档锁定合并工作流，确保翻译数据与代码改动可分别推进。

## 提交策略：两步 split

每个 Phase 内的改造严格分两类提交：

- **Step A — 抽 key**：把 inline 英文字符串替换为 `t!("namespace-key")`，并在 `bundles/en/*.ftl`
  同步追加 entry。属于"纯结构改动"，与上游英文 PR 容易合并。
- **Step B — 翻译**：在 `bundles/zh-CN/*.ftl` 加翻译；不触碰英文与代码。属于"纯翻译改动"，与上游
  完全无冲突。

PR 描述需明确标注 Step A / Step B；CI 校验：Step A 改动如包含 `bundles/zh-CN/**` 必须 reject。

## merge 上游 PR 时

1. `git rerere` 已启用（仓库设置）。冲突大概率落在被汉化的 .rs 文件中。
2. 大多数冲突形如：
   ```
   <<<<<<< HEAD (ours)
       .label(t!("settings-foo-label"))
   =======
       .label("Foo Label v2")  // upstream renamed
   >>>>>>> upstream
   ```
   解决：取本地 `t!()` 调用形式，更新 `bundles/en/*.ftl` 中对应 entry 的英文文案为 v2。
3. 如上游新增字符串：在本地 `t!()` 替换 + 同步 `bundles/en/*.ftl` + 标记 `bundles/zh-CN/*.ftl`
   对应 entry 为 `# TODO(translation):`，由后续 review 人工补齐。
4. CI 的 `cargo xtask check-i18n --check-parity` 会拒绝 en/zh-CN key 集合不一致；merge 后必须
   立即修齐才能 push 到 master。

## 上游若改某条英文文案

仅需更新 `bundles/en/*.ftl` 中对应 entry，并在 `bundles/zh-CN/*.ftl` 同 entry 头注释加
`# TODO(translation): upstream rephrased on YYYY-MM-DD, please review`。代码无须改动。

## 不向上游提 PR

本 fork 自治；汉化基础设施不向上游 `warpdev/warp` 反向贡献（design.md Non-Goal）。

## allowlist 维护

`crates/warp_i18n/lint_allowlist.toml` 在 merge 时如有冲突，按"取并集"原则保留两侧 entries，但
随后跑 `cargo xtask check-i18n --mode warning` 重新核对，剔除已不再命中的 entries，使列表保持
最小集。
