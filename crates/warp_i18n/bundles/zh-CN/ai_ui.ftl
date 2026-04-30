# AI / Agent UI 包裹层字符串（非模型回复文本）。
# Keys MUST start with `ai-ui-`.

ai-ui-thinking = 思考中...
ai-ui-generating = 生成中...
ai-ui-cancelled = 已取消
ai-ui-error-occurred = 发生错误
ai-ui-retry = 重试
ai-ui-accept = 接受
ai-ui-reject = 拒绝

# 建议气泡的 Tooltip
ai-ui-tooltip-add-rule = 添加规则：{ $content }
ai-ui-tooltip-suggested-prompt =
    建议的提示词：
    { $prompt }

# Todos 弹层
ai-ui-tasks-header = 任务

# Agent 类型选择
ai-ui-choose-agent = 选择智能体
ai-ui-suggested-badge = 建议

# 云端 Agent 配置向导
ai-ui-cloud-setup-title = 开始使用 Oz 云智能体
ai-ui-cloud-setup-intro = 直接在 Warp 中通过集成（Linear、Slack）、事件（GitHub、内置计划任务），或通过 Oz SDK / CLI 以编程方式启动 Oz 云智能体。
ai-ui-cloud-setup-manual-section = 手动配置：使用 Oz CLI 创建 Slack 或 Linear 集成
ai-ui-cloud-setup-create-env = 创建环境
ai-ui-cloud-setup-env-first = 首先配置一个环境，以便创建集成。
ai-ui-cloud-setup-custom-image = 或者，提供你自己已有的 Docker 镜像。
ai-ui-cloud-setup-create-integration = 创建集成

# Agent 输入区底栏
ai-ui-using-default-model = 当前使用全终端智能体的默认模型。

# 零态区块
ai-ui-zero-isolated-cloud = 在隔离的云环境中运行你的智能体任务。
ai-ui-zero-recent-activity = 近期活动
ai-ui-view-changelog = 查看更新日志

# 通用 AI 徽标与状态
ai-ui-recommended = 推荐
ai-ui-queued = 排队中
ai-ui-check-now-suffix = { " · " }立即检查
ai-ui-invalid-api-key = 提供的 API 密钥无效
ai-ui-debug-output = 调试输出

# AWS Bedrock 凭据
ai-ui-aws-creds-error = AWS 凭据已过期或缺失
ai-ui-always-auto-run = 始终自动运行

# 代码 Diff 视图
ai-ui-file-renamed-no-change = 文件仅重命名，内容未改

# 命令请求
ai-ui-permission-always-ask = 你的执行档已设置为每次执行命令前都需获得授权。

# 会话详情面板
ai-ui-conversation-error = 错误
ai-ui-conversation-status = 状态
ai-ui-conversation-harness = 框架
ai-ui-conversation-artifacts = 制品
ai-ui-conversation-env-setup = 环境配置命令
ai-ui-conversation-env-details = 环境详情
ai-ui-conversation-credits-used = 已用额度

# 执行档编辑器
ai-ui-profile-name = 名称
ai-ui-plan-auto-sync = 计划自动同步
ai-ui-plan-auto-sync-desc = 该智能体创建的计划将自动添加并同步至 Warp 云盘。
ai-ui-call-web-tools = 调用网页工具
ai-ui-call-web-tools-desc = 该智能体在完成任务有需要时，可使用网页搜索。

# 云端（ambient）智能体 UI
ai-ui-ambient-failed = 失败
ai-ui-ambient-start-cloud-agent = 启动新的 Oz 云智能体
ai-ui-ambient-cloud-env-intro = 云智能体需要一个运行环境来完成任务。请先创建第一个环境。之后你可以编辑环境，或在需要时添加新环境。
ai-ui-ambient-free-credits = 免费额度
ai-ui-ambient-failed-start-env = 启动环境失败
ai-ui-ambient-github-auth-required = 需要 GitHub 授权
ai-ui-ambient-github-auth-msg = 请使用 GitHub 授权以继续
ai-ui-ambient-cancelled-title = 云智能体运行已取消
ai-ui-ambient-no-cloud-env = 未启动云环境

# 会话列表 / Codex 弹窗
ai-ui-no-conversations = 暂无会话
ai-ui-codex-new = 新建
