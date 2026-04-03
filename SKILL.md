# YunXiao CLI Skill

## Overview

This skill provides comprehensive instructions for developing and using the **YunXiao CLI** - a command-line interface for the Alibaba Cloud YunXiao (DevOps) platform built with Rust.

## Purpose

Manage the entire DevOps lifecycle through the command line: authentication, organizations, projects, work items, code repositories, pipelines, deployments, packages, tests, and analytics.

## Key Commands

### Build & Development

```bash
# Build the CLI
cargo build --release

# Run with debug logging
YUNXIAO_CLI_LOG_LEVEL=debug cargo run -- <command>

# Run tests
cargo test

# Lint
cargo clippy --all-targets

# Generate documentation
cargo doc --open
```

### Authentication Commands

```bash
# Login with token
yunxiao auth login --token <PERSONAL_ACCESS_TOKEN>

# Check current user
yunxiao auth whoami

# Check auth status
yunxiao auth status

# Logout
yunxiao auth logout
```

### Configuration Commands

```bash
# List all config
yunxiao config list

# Get specific config value
yunxiao config get token
yunxiao config get default_output
yunxiao config get organization_id

# Set config value
yunxiao config set default_output table
yunxiao config set timeout 60
yunxiao config set organization_id <ORG_ID>

# Delete config key
yunxiao config delete organization_id

# Show config file path
yunxiao config path
```

### Organization Commands

```bash
# List organizations
yunxiao org list

# Get organization info
yunxiao org info --org-id <ORG_ID>

# List members
yunxiao org members list --org-id <ORG_ID>

# Get specific member
yunxiao org members get --user-id <USER_ID> --org-id <ORG_ID>

# Search members
yunxiao org members search --query "alice" --org-id <ORG_ID>

# List departments
yunxiao org departments list --org-id <ORG_ID>

# List roles
yunxiao org roles list --org-id <ORG_ID>
```

### Project (Projex) Commands

```bash
# Search projects
yunxiao projex projects search --keyword "demo" --org-id <ORG_ID>

# Get project details
yunxiao projex projects get --project-id <PROJECT_ID> --org-id <ORG_ID>

# Search work items (requirements)
yunxiao projex workitems search --space-id <SPACE_ID> --category Req --org-id <ORG_ID>

# Get work item
yunxiao projex workitems get --space-id <SPACE_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID>

# Create work item
yunxiao projex workitems create --space-id <SPACE_ID> --category Req --subject "New feature" --org-id <ORG_ID>

# List sprints
yunxiao projex sprints list --space-id <SPACE_ID> --org-id <ORG_ID>

# Create sprint
yunxiao projex sprints create --space-id <SPACE_ID> --name "Sprint 1" --org-id <ORG_ID>

# List versions
yunxiao projex versions list --space-id <SPACE_ID> --org-id <ORG_ID>
```

### Code Management (Codeup) Commands

```bash
# List repositories
yunxiao codeup repos list --org-id <ORG_ID>

# Get repository
yunxiao codeup repos get --repo-id <REPO_ID> --org-id <ORG_ID>

# List branches
yunxiao codeup branches list --repo-id <REPO_ID> --org-id <ORG_ID>

# Create branch
yunxiao codeup branches create --repo-id <REPO_ID> --branch feature/new --ref main --org-id <ORG_ID>

# List merge requests
yunxiao codeup mr list --repo-id <REPO_ID> --org-id <ORG_ID>

# Create merge request
yunxiao codeup mr create --repo-id <REPO_ID> --source feature/new --target main --title "Add feature" --org-id <ORG_ID>

# List files
yunxiao codeup files list --repo-id <REPO_ID> --org-id <ORG_ID>

# Get file content
yunxiao codeup files get --repo-id <REPO_ID> --path src/main.rs --org-id <ORG_ID>
```

### Pipeline (Flow) Commands

```bash
# List pipelines
yunxiao flow pipelines list --org-id <ORG_ID>

# Get pipeline
yunxiao flow pipelines get --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# Create run
yunxiao flow runs create --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# List runs
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# Get latest run
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# Get job logs
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID>
```

### Application Delivery (Appstack) Commands

```bash
# List applications
yunxiao appstack apps list --org-id <ORG_ID>

# Get application
yunxiao appstack apps get --app-name <APP_NAME> --org-id <ORG_ID>

# List variables
yunxiao appstack vars list --org-id <ORG_ID>

# Create deployment
yunxiao appstack deploy create --app-name <APP_NAME> --org-id <ORG_ID>
```

### Package Management Commands

```bash
# List package repositories
yunxiao packages repos list --org-id <ORG_ID>

# List artifacts
yunxiao packages artifacts list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### Test Management (Testhub) Commands

```bash
# Search test cases
yunxiao testhub cases search --space-id <SPACE_ID> --org-id <ORG_ID>

# List test plans
yunxiao testhub plans list --space-id <SPACE_ID> --org-id <ORG_ID>

# List test results
yunxiao testhub plans results list --space-id <SPACE_ID> --plan-id <PLAN_ID> --org-id <ORG_ID>
```

### Shell Completion

```bash
# Generate Bash completion
yunxiao completion generate --shell bash > ~/.local/share/bash-completion/completions/yunxiao

# Generate Zsh completion
yunxiao completion generate --shell zsh > ~/.zfunc/_yunxiao

# Generate PowerShell completion
yunxiao completion generate --shell powershell > yunxiao.ps1
```

## Global Flags

All commands support these global flags:

| Flag | Environment Variable | Description |
|------|---------------------|-------------|
| `-o, --output` | `YUNXIAO_CLI_OUTPUT` | Output format: json, text, table, markdown |
| `--timeout` | `YUNXIAO_CLI_TIMEOUT` | API request timeout in seconds |
| `--log-level` | `YUNXIAO_CLI_LOG_LEVEL` | Log level: debug, info, warn, error |
| `--token` | `YUNXIAO_CLI_TOKEN` | Personal access token |
| `--endpoint` | `YUNXIAO_CLI_ENDPOINT` | API endpoint URL |
| `--org-id` | `YUNXIAO_CLI_ORG_ID` | Organization ID |

## Configuration

### Config File Location

- Linux/macOS: `~/.config/yunxiao/config.toml`
- Windows: `%APPDATA%\yunxiao\config.toml`

### Config File Format

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

1. **CLI flags** – e.g. `--output json`, `--timeout 60`
2. **Environment variables** – e.g. `YUNXIAO_CLI_TOKEN`, `YUNXIAO_CLI_OUTPUT`
3. **Config file** – `~/.config/yunxiao/config.toml`
4. **Built-in defaults** – JSON output, 30s timeout, warn log level

## Getting a Personal Access Token

1. Log in to [YunXiao Console](https://devops.console.aliyun.com/)
2. Click your avatar > **Personal Settings** > **Personal Access Token**
3. Click **Create Token**
4. Copy the token (shown only once)

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library root
├── error/mod.rs         # Unified error types (CliError)
├── config/              # Configuration layer
│   ├── mod.rs           # Config management with 4-tier priority
│   └── types.rs         # OutputFormat, LogLevel, CliConfig
├── cache/mod.rs         # File-based JSON cache
├── auth/mod.rs          # Token management
├── client/mod.rs        # HTTP client (reqwest + x-devops-pat auth)
├── output/mod.rs        # Output formatting
└── cli/
    ├── mod.rs           # Root CLI definition
    └── commands/
        ├── mod.rs       # Commands enum
        ├── auth.rs      # Authentication commands
        ├── config_cmd.rs # Configuration commands
        ├── org.rs       # Organization commands
        ├── projex.rs    # Project collaboration commands
        ├── codeup.rs    # Code management commands
        ├── flow.rs      # Pipeline commands
        ├── appstack.rs  # Application delivery commands
        ├── packages.rs  # Package commands
        ├── testhub.rs   # Test management commands
        ├── insight.rs   # Analytics commands
        ├── thoughts.rs  # Knowledge base commands
        └── completion.rs # Shell completion
```

## Common Development Tasks

### Adding a New Subcommand

1. Create a new file in `src/cli/commands/<module>.rs`
2. Define argument structs with clap derive macros
3. Implement the `execute` function
4. Add the module to `src/cli/commands/mod.rs`
5. Add the variant to the `Commands` enum

### Adding API Endpoints

1. Add request/response structs in the relevant command module
2. Implement the API call in the client or command module
3. Handle errors using the `CliError` type
4. Format output using the output module

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Best Practices

- Always use the `CliError` type for error handling
- Use the `Output` trait for formatting results
- Support all four output formats (json, text, table, markdown)
- Add help text to all CLI arguments
- Use environment variables for global flags
- Handle missing org-id gracefully with helpful error messages
- Cache frequently accessed data when appropriate
