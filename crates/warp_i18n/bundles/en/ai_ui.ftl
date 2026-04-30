# AI / Agent UI wrapper strings (non-model-response user-facing text).
# Keys MUST start with `ai-ui-`.

ai-ui-thinking = Thinking...
ai-ui-generating = Generating...
ai-ui-cancelled = Cancelled
ai-ui-error-occurred = An error occurred
ai-ui-retry = Retry
ai-ui-accept = Accept
ai-ui-reject = Reject

# Suggestion chip tooltips
ai-ui-tooltip-add-rule = Add rule: { $content }
ai-ui-tooltip-suggested-prompt =
    Suggested prompt:
    { $prompt }

# Todos popup
ai-ui-tasks-header = Tasks

# Agent type selector
ai-ui-choose-agent = Choose your agent
ai-ui-suggested-badge = Suggested

# Cloud setup guide
ai-ui-cloud-setup-title = Getting started with Oz cloud agents
ai-ui-cloud-setup-intro = Start Oz cloud agents directly in Warp from an integration (Linear, Slack), with an event (GitHub, built-in schedule), or programmatically with the Oz SDK or CLI.
ai-ui-cloud-setup-manual-section = Manual setup: Create a Slack or Linear integration with the Oz CLI
ai-ui-cloud-setup-create-env = Create an environment
ai-ui-cloud-setup-env-first = First, set up an environment to create an integration.
ai-ui-cloud-setup-custom-image = Or, supply your own existing docker image.
ai-ui-cloud-setup-create-integration = Create an integration

# Agent input footer
ai-ui-using-default-model = Now using Full Terminal Agent's default model.

# Zero state block
ai-ui-zero-isolated-cloud = Run your agent task in an isolated cloud environment.
ai-ui-zero-recent-activity = RECENT ACTIVITY
ai-ui-view-changelog = View changelog

# Common AI badges and statuses
ai-ui-recommended = Recommended
ai-ui-queued = Queued
ai-ui-check-now-suffix = { " · " }Check now
ai-ui-invalid-api-key = Provided API key is not valid
ai-ui-debug-output = Debug output

# AWS Bedrock credentials
ai-ui-aws-creds-error = AWS credentials expired or missing
ai-ui-always-auto-run = Always run automatically

# Code diff view
ai-ui-file-renamed-no-change = File renamed without changes

# Requested command
ai-ui-permission-always-ask = Your profile is set to always ask for permission to execute commands.

# Conversation details panel
ai-ui-conversation-error = Error
ai-ui-conversation-status = Status
ai-ui-conversation-harness = Harness
ai-ui-conversation-artifacts = Artifacts
ai-ui-conversation-env-setup = Environment setup commands
ai-ui-conversation-env-details = Environment details
ai-ui-conversation-credits-used = Credits used

# Execution profile editor
ai-ui-profile-name = Name
ai-ui-plan-auto-sync = Plan auto-sync
ai-ui-plan-auto-sync-desc = The plans this agent creates will be automatically added and synced to Warp Drive.
ai-ui-call-web-tools = Call web tools
ai-ui-call-web-tools-desc = The agent may use web search when helpful for completing tasks.

# Ambient (cloud) agent UI
ai-ui-ambient-failed = Failed
ai-ui-ambient-start-cloud-agent = Start a new Oz cloud agent
ai-ui-ambient-cloud-env-intro = Cloud agents require an environment that they'll run in to get their task done. Create your first environment below. You'll be able to edit the environment later, or add new environments when you need them.
ai-ui-ambient-free-credits = Free credits
ai-ui-ambient-failed-start-env = Failed to start environment
ai-ui-ambient-github-auth-required = GitHub Authentication Required
ai-ui-ambient-github-auth-msg = Please authenticate with GitHub to continue
ai-ui-ambient-cancelled-title = Cloud Agent Run Cancelled
ai-ui-ambient-no-cloud-env = No cloud environment was started

# Conversation list / Codex modal
ai-ui-no-conversations = No conversations yet
ai-ui-codex-new = New
