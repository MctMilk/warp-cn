# 终端 / 区块界面字符串。
# Keys MUST start with `terminal-`.

terminal-filter-placeholder = 搜索区块输出
terminal-context-lines-tooltip = 显示匹配项的上下文行
terminal-regex-tooltip = 正则表达式切换
terminal-case-sensitive-tooltip = 区分大小写搜索
terminal-invert-filter-tooltip = 反向筛选
terminal-type-search-phrase = 输入搜索内容。
terminal-press-escape = 按 Escape 退出
terminal-tag-agent = 标记智能体以获取帮助
terminal-save-as-workflow = 保存为工作流
terminal-workflow-secrets-warning = 包含密钥的区块无法保存。
terminal-conversation-restored = 对话已恢复
terminal-previous-session = 上一会话
terminal-copy = 复制
terminal-share-block = 分享区块

# 链接提示气泡（富内容 + 网格链接）
terminal-link-open-folder = 打开文件夹
terminal-link-open-file = 打开文件
terminal-link-open-link = 打开链接

# 零态 / SSH / 共享会话 / 初始化环境 / 区块引导
terminal-zero-new-session = 新建终端会话
terminal-zero-dont-show-again = 不再显示
terminal-ssh-dont-ask-again = 不再询问
# SSH 远程服务端启动失败横幅（按失败种类提供标题与描述）
terminal-ssh-failed-banner-title-binary-check = SSH 扩展无法验证
terminal-ssh-failed-banner-title-binary-install = SSH 扩展无法安装
terminal-ssh-failed-banner-title-launch = SSH 扩展无法启动
terminal-ssh-failed-banner-desc-binary-check = 无法在远程主机上验证 SSH 扩展二进制文件。虽然文件浏览、代码审查等高级功能暂不可用，但 Warpify 体验的其他部分仍完全可用。
terminal-ssh-failed-banner-desc-binary-install = 二进制文件无法写入或在远程主机上执行。虽然文件浏览、代码审查等高级功能暂不可用，但 Warpify 体验的其他部分仍完全可用。
terminal-ssh-failed-banner-desc-launch = 无法在远程主机上启动 SSH 扩展。虽然文件浏览、代码审查等高级功能暂不可用，但 Warpify 体验的其他部分仍完全可用。
terminal-ssh-failed-banner-error-line = 错误：{$detail}
terminal-shared-snapshot-title = 你正在查看快照
terminal-shared-snapshot-desc = 此共享会话显示打开时的状态。若智能体仍在运行，请刷新以查看最新进度。
terminal-init-env-mode-prompt = 选择你希望如何设置环境
terminal-init-env-suggested = 建议
terminal-thinking = 思考中...

# 终端输入
terminal-no-results = 无结果
terminal-billed-to-api = 按 API 计费
terminal-or-separator = { " 或 " }

# 输入建议模式 placeholder 文案（terminal/input.rs InputSuggestionsMode）
terminal-input-placeholder-search-queries = 搜索查询
terminal-input-placeholder-search-queries-rewind = 搜索可回退到的查询
terminal-input-placeholder-search-conversations = 搜索会话
terminal-input-placeholder-search-skills = 搜索技能
terminal-input-placeholder-search-models = 搜索模型
terminal-input-placeholder-search-profiles = 搜索配置文件
terminal-input-placeholder-search-commands = 搜索命令
terminal-input-placeholder-search-prompts = 搜索提示词
terminal-input-placeholder-search-indexed-repos = 搜索已索引仓库
terminal-input-placeholder-search-plans = 搜索计划
