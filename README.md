# YunXiao CLI

A comprehensive command-line client for the **Alibaba Cloud YunXiao (DevOps)** platform, built with Rust.

Manage projects, code repositories, pipelines, deployments, tests, and more -- entirely from your terminal.

## Features

- **Full API coverage** across all YunXiao business domains (12 top-level modules)
- **Cross-platform** -- Windows 11, Linux (Ubuntu/CentOS/Debian), macOS 10.12+
- **Multiple output formats** -- JSON, plain text, ASCII table, Markdown
- **Configurable** -- layered priority system (CLI args > env vars > config file > defaults)
- **Shell completion** -- Bash, Zsh, Fish, PowerShell, Elvish
- **Secure** -- config file permissions restricted to owner-only on Unix (0600)

## Quick Start

### Installation

```bash
# Build from source
git clone https://github.com/your-org/yunxiao-cli.git
cd yunxiao-cli
cargo build --release

# The binary is at target/release/yunxiao-cli
# Optionally copy to a directory in your PATH:
cp target/release/yunxiao-cli ~/.local/bin/
```

### Authentication

1. Obtain a **Personal Access Token** from the YunXiao web console:
   - Log in to [YunXiao](https://devops.console.alibabacloud.com/)
   - Click your avatar > **Personal Settings** > **Personal Access Token** > **Create Token**
   - Copy the token (it is only shown once)

2. Save the token:

```bash
yunxiao-cli auth login --token <YOUR_TOKEN>
```

Or set it as an environment variable:

```bash
export YUNXIAO_CLI_TOKEN=<YOUR_TOKEN>
```

3. Verify:

```bash
yunxiao-cli auth whoami
```

### Basic Usage

```bash
# View current user info
yunxiao-cli auth whoami

# List organizations
yunxiao-cli org list

# Search projects
yunxiao-cli projex projects search --keyword "my-project" --org-id <ORG_ID>

# List code repositories
yunxiao-cli codeup repos list --org-id <ORG_ID>

# List pipelines
yunxiao-cli flow pipelines list --org-id <ORG_ID>

# Search work items (requirements)
yunxiao-cli projex workitems search --space-id <PROJECT_ID> --category Req --org-id <ORG_ID>
```

## Command Reference

### Top-Level Commands

| Command      | Description                                      |
|-------------|--------------------------------------------------|
| `auth`      | Authentication -- login, logout, status, whoami   |
| `config`    | Local configuration management                    |
| `org`       | Organization and member management                |
| `projex`    | Project collaboration -- work items, sprints, versions |
| `codeup`    | Code management -- repos, branches, merge requests |
| `flow`      | Pipeline management -- create, run, query          |
| `appstack`  | Application delivery -- apps, environments, deploys |
| `packages`  | Package repository management                     |
| `testhub`   | Test management -- cases, plans, results           |
| `insight`   | Efficiency analytics and metrics                   |
| `thoughts`  | Knowledge base -- documents, pages                 |
| `completion`| Generate shell completion scripts                  |

### Global Flags

| Flag               | Env Variable             | Description                     |
|-------------------|--------------------------|---------------------------------|
| `-o, --output`     | `YUNXIAO_CLI_OUTPUT`     | Output format: json/text/table/markdown |
| `--timeout`        | `YUNXIAO_CLI_TIMEOUT`    | API request timeout (seconds)   |
| `--log-level`      | `YUNXIAO_CLI_LOG_LEVEL`  | Log level: debug/info/warn/error |
| `--token`          | `YUNXIAO_CLI_TOKEN`      | Personal access token           |
| `--domain`         | `YUNXIAO_CLI_DOMAIN`     | API endpoint domain             |
| `--org-id`         | `YUNXIAO_CLI_ORG_ID`     | Organization ID                 |

### Subcommand Examples

#### Auth

```bash
yunxiao-cli auth login --token pt_xxxxxxxxxxxx
yunxiao-cli auth logout
yunxiao-cli auth status
yunxiao-cli auth whoami
```

#### Config

```bash
yunxiao-cli config list
yunxiao-cli config get token
yunxiao-cli config set default_output table
yunxiao-cli config set timeout 60
yunxiao-cli config delete organization_id
yunxiao-cli config path
```

#### Organization

```bash
yunxiao-cli org info --org-id <ORG_ID>
yunxiao-cli org list
yunxiao-cli org members list --org-id <ORG_ID>
yunxiao-cli org members get --user-id <UID> --org-id <ORG_ID>
yunxiao-cli org members search --query "alice" --org-id <ORG_ID>
yunxiao-cli org departments list --org-id <ORG_ID>
yunxiao-cli org roles list --org-id <ORG_ID>
```

#### Project Collaboration (Projex)

```bash
# Projects
yunxiao-cli projex projects search --keyword "demo" --org-id <ORG_ID>
yunxiao-cli projex projects get --project-id <PID> --org-id <ORG_ID>

# Work Items
yunxiao-cli projex workitems search --space-id <SID> --category Req --org-id <ORG_ID>
yunxiao-cli projex workitems get --space-id <SID> --workitem-id <WID> --org-id <ORG_ID>
yunxiao-cli projex workitems create --space-id <SID> --category Req --subject "New feature" --org-id <ORG_ID>

# Sprints
yunxiao-cli projex sprints list --space-id <SID> --org-id <ORG_ID>
yunxiao-cli projex sprints create --space-id <SID> --name "Sprint 1" --org-id <ORG_ID>

# Versions
yunxiao-cli projex versions list --space-id <SID> --org-id <ORG_ID>
```

#### Code Management (Codeup)

```bash
# Repositories
yunxiao-cli codeup repos list --org-id <ORG_ID>
yunxiao-cli codeup repos get --repo-id <RID> --org-id <ORG_ID>

# Branches
yunxiao-cli codeup branches list --repo-id <RID> --org-id <ORG_ID>
yunxiao-cli codeup branches create --repo-id <RID> --branch feature/new --ref main --org-id <ORG_ID>

# Merge Requests
yunxiao-cli codeup mr list --repo-id <RID> --org-id <ORG_ID>
yunxiao-cli codeup mr create --repo-id <RID> --source feature/new --target main --title "Add feature" --org-id <ORG_ID>

# Files
yunxiao-cli codeup files list --repo-id <RID> --org-id <ORG_ID>
yunxiao-cli codeup files get --repo-id <RID> --path src/main.rs --org-id <ORG_ID>
```

#### Pipeline (Flow)

```bash
yunxiao-cli flow pipelines list --org-id <ORG_ID>
yunxiao-cli flow pipelines get --pipeline-id <PID> --org-id <ORG_ID>
yunxiao-cli flow runs create --pipeline-id <PID> --org-id <ORG_ID>
yunxiao-cli flow runs list --pipeline-id <PID> --org-id <ORG_ID>
yunxiao-cli flow runs latest --pipeline-id <PID> --org-id <ORG_ID>
yunxiao-cli flow jobs log --pipeline-id <PID> --run-id <RID> --job-id <JID> --org-id <ORG_ID>
```

#### Application Delivery (Appstack)

```bash
yunxiao-cli appstack apps list --org-id <ORG_ID>
yunxiao-cli appstack apps get --app-name my-service --org-id <ORG_ID>
yunxiao-cli appstack vars list --org-id <ORG_ID>
yunxiao-cli appstack deploy create --app-name my-service --org-id <ORG_ID>
```

#### Packages

```bash
yunxiao-cli packages repos list --org-id <ORG_ID>
yunxiao-cli packages artifacts list --repo-id <RID> --org-id <ORG_ID>
```

#### Test Management (Testhub)

```bash
yunxiao-cli testhub cases search --space-id <SID> --org-id <ORG_ID>
yunxiao-cli testhub plans list --space-id <SID> --org-id <ORG_ID>
yunxiao-cli testhub plans results list --space-id <SID> --plan-id <PID> --org-id <ORG_ID>
```

#### Shell Completion

```bash
# Generate Bash completion
yunxiao-cli completion generate --shell bash > ~/.local/share/bash-completion/completions/yunxiao-cli

# Generate Zsh completion
yunxiao-cli completion generate --shell zsh > ~/.zfunc/_yunxiao-cli

# Generate PowerShell completion
yunxiao-cli completion generate --shell powershell > yunxiao-cli.ps1
```

## Configuration

### Config File

Location: `~/.config/yunxiao-cli/config.toml`

```toml
# Personal access token
token = "pt_xxxxxxxxxxxxxxxxxxxx"

# API endpoint domain (default: openapi-rdc.aliyuncs.com)
domain = "openapi-rdc.aliyuncs.com"

# Active organization ID
organization_id = "org-xxxxxxxx"

# Default output format: json | text | table | markdown
default_output = "json"

# Default log level: debug | info | warn | error
log_level = "warn"

# HTTP request timeout in seconds (default: 30)
timeout = 30
```

### Configuration Priority

Settings are resolved in this order (highest priority first):

1. **CLI flags** -- e.g. `--output json`, `--timeout 60`
2. **Environment variables** -- e.g. `YUNXIAO_CLI_TOKEN`, `YUNXIAO_CLI_OUTPUT`
3. **Config file** -- `~/.config/yunxiao-cli/config.toml`
4. **Built-in defaults** -- JSON output, 30s timeout, warn log level

### Cache Directory

Runtime cache data is stored at `~/.shell/yunxiao-cli/`:
- Token-bound user/organization info
- Temporary API response caches
- Runtime intermediate data

Clear the cache:

```bash
# Via the config subcommand or by deleting the directory
rm -rf ~/.shell/yunxiao-cli/
```

## Architecture

```
src/
  main.rs              Entry point
  lib.rs               Library root
  error/mod.rs         Unified error types (CliError)
  config/              Configuration layer (load/save/resolve)
    mod.rs             Config management with 4-tier priority
    types.rs           OutputFormat, LogLevel, CliConfig
  cache/mod.rs         File-based JSON cache
  auth/mod.rs          Token management (save/get/clear/require)
  client/mod.rs        HTTP client (reqwest + x-devops-pat auth)
  output/mod.rs        Output formatting (JSON/text/table/markdown)
  cli/
    mod.rs             Root CLI definition with global flags
    commands/
      mod.rs           Commands enum (12 subcommands)
      auth.rs          login, logout, status, whoami
      config_cmd.rs    get, set, list, delete, path
      org.rs           Organization & member management
      projex.rs        Projects, work items, sprints, versions
      codeup.rs        Repositories, branches, MRs, files
      flow.rs          Pipelines, runs, jobs, connections
      appstack.rs      Apps, vars, orchestrations, deployments
      packages.rs      Package repos, artifacts
      testhub.rs       Test cases, plans, results
      insight.rs       Efficiency analytics (placeholder)
      thoughts.rs      Knowledge base (placeholder)
      completion.rs    Shell completion generation
```

### Design Principles

- **Layered architecture** -- config, cache, auth, HTTP client, output, and CLI layers are fully decoupled
- **Modular commands** -- each business domain is an independent module, no cross-dependencies
- **Testable** -- core logic separated from CLI interaction; external dependencies support mock injection
- **Cross-platform** -- no platform-specific dependencies; Unix permissions applied conditionally

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy --all-targets

# Generate docs
cargo doc --open

# Run with debug logging
YUNXIAO_CLI_LOG_LEVEL=debug cargo run -- auth status
```

## API Reference

This CLI wraps the [Alibaba Cloud YunXiao OpenAPI](https://help.aliyun.com/zh/yunxiao/developer-reference/).

- **API Endpoint**: `https://openapi-rdc.aliyuncs.com` (central version)
- **Authentication**: Personal Access Token via `x-devops-pat` HTTP header
- **Token Management**: [Personal Access Token Docs](https://help.aliyun.com/zh/yunxiao/developer-reference/obtain-personal-access-token)

## License

MIT
