# AGENTS.md - YunXiao CLI 开发指南

本文档为 AI 编码助手提供项目背景、架构设计和开发规范。

---

## 项目概述

**YunXiao CLI** 是阿里云云效（DevOps）平台的命令行工具，基于 Rust 开发，提供对云效 API 的完整访问能力。

### 核心信息

| 属性 | 值 |
|-----|---|
| 语言 | Rust 1.75+ |
| 架构 | 异步 Tokio + Clap CLI |
| API 版本 | devops-2021-06-25 |
| 端点 | `https://openapi-rdc.aliyuncs.com` |
| 认证 | Personal Access Token (PAT) |

---

## 架构设计

### 目录结构

```
src/
├── main.rs              # 程序入口
├── lib.rs               # 库导出
├── cli/
│   ├── mod.rs           # CLI 根模块，参数解析
│   └── commands/        # 子命令实现
│       ├── mod.rs       # 命令路由
│       ├── auth.rs      # 认证管理
│       ├── codeup.rs    # 代码管理（95个API）
│       ├── flow.rs      # 流水线（62个API）
│       ├── projex.rs    # 项目管理（40个API）
│       ├── org.rs       # 组织成员
│       ├── appstack.rs  # 应用交付
│       ├── insight.rs   # 效能洞察
│       ├── testhub.rs   # 测试管理
│       ├── packages.rs  # 制品库
│       ├── thoughts.rs  # 知识库
│       ├── config_cmd.rs # 配置管理
│       ├── budget.rs    # 预算管理
│       └── completion.rs # Shell 补全
├── client/
│   └── mod.rs           # HTTP 客户端封装
├── config/
│   ├── mod.rs           # 配置管理逻辑
│   └── types.rs         # 配置类型定义
├── output/
│   └── mod.rs           # 输出格式化（JSON/Table/Text）
├── error/
│   └── mod.rs           # 错误类型定义
├── cache/
│   └── mod.rs           # 缓存机制
└── auth/
    └── mod.rs           # 认证逻辑
```

### 数据流

```
CLI 参数 → clap 解析 → 命令路由 → API 客户端 → 云效 API
                             ↓
                      配置解析（四层优先级）
                             ↓
                      输出格式化 → 终端
```

---

## 编码规范

### 1. 代码风格

- **格式化**: 使用 `cargo fmt`
- **Lint**: 使用 `cargo clippy --all-targets`，禁止有警告
- **文档**: 所有公共 API 必须有 rustdoc 注释
- **命名**: 遵循 Rust 命名规范（snake_case, CamelCase）

### 2. 错误处理

使用 `thiserror` 定义错误类型：

```rust
// src/error/mod.rs
#[derive(Error, Debug)]
pub enum CliError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, CliError>;
```

**规范**:
- 所有异步函数返回 `crate::error::Result<T>`
- API 错误需包含完整 URL 和状态码信息
- 用户输入错误使用 `CliError::Config` 或 `CliError::Auth`

### 3. API 路径规范

所有 API 调用使用统一前缀：

```
/oapi/v1/{module}/{资源}
```

示例：
- `/oapi/v1/projex/organizations/{oid}/projects` - 项目列表
- `/oapi/v1/codeup/organizations/{oid}/repositories` - 代码库列表
- `/oapi/v1/platform/organizations/{oid}/members` - 组织成员

### 4. HTTP 方法语义

| 操作 | HTTP 方法 | 示例 |
|-----|----------|------|
| 查询 | GET | `client.get("/projects", &[("page", "1")])` |
| 创建 | POST | `client.post("/projects", &body)` |
| 更新 | PUT | `client.put("/projects/{id}", &body)` |
| 删除 | DELETE | `client.delete("/projects/{id}", &[])` |

---

## 如何添加新命令

### 步骤 1：定义命令参数

在 `src/cli/commands/{module}.rs` 中添加：

```rust
use clap::{Args, Subcommand};

/// Arguments for the `example` subcommand.
#[derive(Debug, Args)]
pub struct ExampleArgs {
    #[command(subcommand)]
    pub command: ExampleCommands,
}

#[derive(Debug, Subcommand)]
pub enum ExampleCommands {
    /// List all examples.
    List(ExampleListArgs),
    /// Get a specific example.
    Get(ExampleGetArgs),
}

#[derive(Debug, Args)]
pub struct ExampleListArgs {
    /// Filter by keyword.
    #[arg(long)]
    pub keyword: Option<String>,
    /// Page number.
    #[arg(long, default_value = "1")]
    pub page: u32,
}

#[derive(Debug, Args)]
pub struct ExampleGetArgs {
    /// Example ID.
    pub id: String,
}
```

### 步骤 2：实现执行逻辑

```rust
pub async fn execute(
    args: &ExampleArgs,
    format: &OutputFormat,
    cli_token: Option<&str>,
    cli_endpoint: Option<&str>,
    cli_timeout: Option<u64>,
    cli_org_id: Option<&str>,
) -> Result<()> {
    let token = config::resolve_token(cli_token)?;
    let endpoint = config::resolve_endpoint(cli_endpoint);
    let timeout = config::resolve_timeout(cli_timeout);
    let org_id = config::resolve_org_id(cli_org_id);
    let client = ApiClient::new(&token, &endpoint, timeout)?;

    match &args.command {
        ExampleCommands::List(l) => {
            let oid = require_org(&org_id)?;
            let mut params: Vec<(&str, &str)> = vec![];
            if let Some(ref kw) = l.keyword {
                params.push(("keyword", kw));
            }
            let data = client
                .get(&format!("/oapi/v1/example/organizations/{oid}/items"), &params)
                .await?;
            output::print_output(&data, format)?;
        }
        ExampleCommands::Get(g) => {
            let oid = require_org(&org_id)?;
            let data = client
                .get(&format!("/oapi/v1/example/organizations/{oid}/items/{}", g.id), &[])
                .await?;
            output::print_output(&data, format)?;
        }
    }
    Ok(())
}

/// Helper: require org ID for org-scoped endpoints.
fn require_org(org_id: &Option<String>) -> Result<&str> {
    org_id.as_deref().ok_or_else(|| {
        crate::error::CliError::Config(
            "Organization ID required. Set via --org-id, YUNXIAO_CLI_ORG_ID, or config.".into(),
        )
    })
}
```

### 步骤 3：注册命令

在 `src/cli/commands/mod.rs` 中：

```rust
pub mod example;

#[derive(Debug, Subcommand)]
pub enum Commands {
    // ... existing commands
    /// Example management.
    Example(example::ExampleArgs),
}
```

在 `src/cli/mod.rs` 中路由：

```rust
Commands::Example(e) => {
    commands::example::execute(e, &output_format, cli.token.as_deref(), ...).await
}
```

---

## API 客户端使用规范

### 基本用法

```rust
use crate::client::ApiClient;

let client = ApiClient::new(&token, &endpoint, timeout)?;

// GET 请求
let data = client.get("/path", &[("param", "value")]).await?;

// POST 请求
let body = json!({ "key": "value" });
let data = client.post("/path", &body).await?;

// PUT 请求
let data = client.put("/path/{id}", &body).await?;

// DELETE 请求
let data = client.delete("/path/{id}", &[]).await?;
```

### 需要响应头时使用

```rust
let resp = client.post_with_headers("/path", &body).await?;
// resp.headers - HeaderMap
// resp.body - serde_json::Value
```

### 构建查询条件

```rust
// 使用条件组构建复杂查询
let mut conditions = Vec::new();
conditions.push(json!({
    "fieldIdentifier": "subject",
    "operator": "CONTAINS",
    "value": [keyword],
    "className": "string",
    "format": "input"
}));

let body = json!({
    "page": page,
    "perPage": per_page,
    "conditions": json!({ "conditionGroups": [conditions] }).to_string(),
});
```

---

## 配置系统

### 四层优先级（从高到低）

1. **CLI 参数** `--token`, `--output`, `--timeout`
2. **环境变量** `YUNXIAO_CLI_TOKEN`, `YUNXIAO_CLI_OUTPUT`
3. **配置文件** `~/.config/yunxiao-cli/config.toml`
4. **默认值** 

### 配置文件格式

```toml
# ~/.config/yunxiao-cli/config.toml
token = "pt_xxxxxxxx"
domain = "openapi-rdc.aliyuncs.com"
organization_id = "org-xxxx"
default_output = "table"
timeout = 30
log_level = "info"
```

### 配置解析函数

```rust
use crate::config;

let token = config::resolve_token(cli_token)?;
let endpoint = config::resolve_endpoint(cli_endpoint);
let timeout = config::resolve_timeout(cli_timeout);
let org_id = config::resolve_org_id(cli_org_id);
```

---

## 输出格式化

### 使用统一输出接口

```rust
use crate::output;
use crate::config::types::OutputFormat;

// 在 execute 函数中
output::print_output(&data, format)?;
```

### 支持的格式

| 格式 | CLI 参数 | 说明 |
|-----|---------|------|
| JSON | `--output json` | 机器可读，完整数据 |
| 表格 | `--output table` | 人类可读，列对齐 |
| 纯文本 | `--output plain` | 简洁文本输出 |
| Markdown | `--output markdown` | 文档友好格式 |

---

## 测试规范

### 单元测试

在模块底部添加测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_projects() {
        let client = create_mock_client().await;
        // ...
    }
}
```

### 集成测试

使用 `assert_cmd` 和 `predicates`：

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("yunxiao-cli").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains("云效"));
}
```

---

## 常用开发命令

```bash
# 构建
cargo build

# 运行（开发）
cargo run -- projex projects list --org-id <ORG_ID>

# 测试
cargo test
cargo test -- --nocapture  # 显示打印输出

# 代码检查
cargo clippy --all-targets
cargo fmt -- --check

# 发布构建
cargo build --release

# 生成 Shell 补全
./target/release/yunxiao-cli completion bash > /tmp/yunxiao.bash
```

---

## API 实现检查清单

添加新 API 时检查：

- [ ] API 路径符合 `/oapi/v1/{module}/...` 规范
- [ ] HTTP 方法使用正确（GET/POST/PUT/DELETE）
- [ ] 必要的 org_id 参数已处理
- [ ] 请求体/查询参数符合云效 API 文档
- [ ] 错误处理包含上下文信息
- [ ] 响应数据使用 `output::print_output` 输出
- [ ] 分页参数支持（page, per_page）
- [ ] 添加了 rustdoc 注释
- [ ] 更新了命令帮助文本

---

## 参考资料

- [云效 OpenAPI 文档](https://help.aliyun.com/zh/yunxiao/developer-reference/)
- [API 列表](./docs/yunxiao_api_list.md)
- [实现报告](./docs/projex_api_implementation_report.md)
