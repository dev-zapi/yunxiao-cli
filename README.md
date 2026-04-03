<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Alibaba%20Cloud-FF6A00?style=for-the-badge&logo=alibabadotcom&logoColor=white" alt="Alibaba Cloud">
  <img src="https://img.shields.io/badge/YunXiao-DevOps-blue?style=for-the-badge" alt="YunXiao">
</p>

<h1 align="center">YunXiao CLI</h1>

<p align="center">
  <b>A modern, blazing-fast command-line client for Alibaba Cloud YunXiao (DevOps)</b>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#commands">Commands</a> •
  <a href="#budget">Budget</a> •
  <a href="#configuration">Configuration</a>
</p>

---

## ✨ Features

- 🚀 **Full API Coverage** — 12 business domains, 100+ commands
- 💻 **Cross-Platform** — Windows, Linux, macOS
- 📊 **Multiple Output Formats** — JSON, text, table, Markdown
- ⚙️ **Smart Configuration** — 4-tier priority system
- 🐚 **Shell Completion** — Bash, Zsh, Fish, PowerShell
- 🔒 **Secure by Default** — Unix permissions (0600), env var support
- ⚡ **Blazing Fast** — Built with Rust + Tokio async runtime

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/your-org/yunxiao-cli.git
cd yunxiao-cli

# Build release binary
cargo build --release

# Install to PATH
cp target/release/yunxiao-cli ~/.local/bin/
```

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Personal Access Token from YunXiao console

## 🚀 Quick Start

### 1. Authentication

Get your token from [YunXiao Console](https://devops.console.alibabacloud.com/):

```bash
yunxiao-cli auth login --token <YOUR_TOKEN>
```

Or use environment variable:

```bash
export YUNXIAO_CLI_TOKEN=<YOUR_TOKEN>
```

### 2. Verify Setup

```bash
yunxiao-cli auth whoami
yunxiao-cli org list
```

### 3. Common Commands

```bash
# List projects
yunxiao-cli projex projects search --keyword "my-project"

# View repositories
yunxiao-cli codeup repos list

# Check pipelines
yunxiao-cli flow pipelines list

# Search work items
yunxiao-cli projex workitems search --space-id <PROJECT_ID> --category Req
```

## 📋 Commands

| Command | Description |
|---------|-------------|
| `auth` | Login, logout, status, whoami |
| `config` | Configuration management |
| `org` | Organization & members |
| `projex` | Projects, work items, sprints |
| `codeup` | Repositories, branches, MRs |
| `flow` | Pipelines & runs |
| `appstack` | App delivery & deployments |
| `packages` | Package repositories |
| `testhub` | Test management |
| `insight` | Analytics & metrics |
| `thoughts` | Knowledge base |
| `completion` | Shell completion scripts |
| `budget` | 💰 Cost tracking & budgets |

## 💰 Budget

Track and manage your YunXiao resource costs:

### View Budget Status

```bash
# Check current month's budget
yunxiao-cli budget status --org-id <ORG_ID>

# View budget history
yunxiao-cli budget history --months 6 --org-id <ORG_ID>

# Get detailed breakdown
yunxiao-cli budget breakdown --org-id <ORG_ID>
```

### Set Budget Alerts

```bash
# Configure monthly budget limit
yunxiao-cli budget set-limit --amount 5000 --currency CNY --org-id <ORG_ID>

# Set alert thresholds
yunxiao-cli budget set-alert --threshold 80 --org-id <ORG_ID>
```

### Cost Analysis

```bash
# View cost by service
yunxiao-cli budget by-service --org-id <ORG_ID>

# View cost by project
yunxiao-cli budget by-project --org-id <ORG_ID>

# Export cost report
yunxiao-cli budget export --format csv --month 2024-01 --org-id <ORG_ID>
```

### Budget Configuration

Add to your config file (`~/.config/yunxiao-cli/config.toml`):

```toml
[budget]
monthly_limit = 5000
currency = "CNY"
alert_threshold = 80  # Alert at 80% usage
alert_email = "admin@example.com"
```

## ⚙️ Configuration

### Config File Location

```
~/.config/yunxiao-cli/config.toml
```

### Example Configuration

```toml
# Authentication
token = "pt_xxxxxxxxxxxxxxxxxxxx"

# API Settings
domain = "openapi-rdc.aliyuncs.com"
organization_id = "org-xxxxxxxx"

# Output & Logging
default_output = "table"
log_level = "warn"
timeout = 30

# Budget Settings
[budget]
monthly_limit = 10000
alert_threshold = 85
```

### Priority Order

1. **CLI flags** — `--output json`, `--timeout 60`
2. **Environment variables** — `YUNXIAO_CLI_TOKEN`
3. **Config file** — `~/.config/yunxiao-cli/config.toml`
4. **Defaults** — JSON, 30s timeout, warn level

### Environment Variables

| Variable | Description |
|----------|-------------|
| `YUNXIAO_CLI_TOKEN` | Personal access token |
| `YUNXIAO_CLI_OUTPUT` | Output format |
| `YUNXIAO_CLI_TIMEOUT` | Request timeout |
| `YUNXIAO_CLI_ORG_ID` | Default organization |
| `YUNXIAO_CLI_LOG_LEVEL` | Logging level |

## 🛠️ Development

```bash
# Build
cargo build

# Run tests
cargo test

# Lint & format
cargo clippy --all-targets
cargo fmt

# Build release
cargo build --release
```

## 📖 API Reference

- **Documentation**: [YunXiao OpenAPI](https://help.aliyun.com/zh/yunxiao/developer-reference/)
- **Endpoint**: `https://openapi-rdc.aliyuncs.com`
- **Auth**: Personal Access Token via `x-devops-pat` header

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

<p align="center">
  Built with ❤️ using Rust
</p>
