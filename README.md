<p align="center">
  <img src="https://img.shields.io/github/license/your-org/yunxiao-cli?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/version-0.1.0-007EC6?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macos-6C757D?style=for-the-badge" alt="Platform">
</p>

<h1 align="center">云效 CLI</h1>

<p align="center">
  <b>阿里云云效 DevOps 平台的现代化极速命令行工具</b>
</p>

<p align="center">
  <a href="#特性">特性</a> •
  <a href="#安装">安装</a> •
  <a href="#快速开始">快速开始</a> •
  <a href="#命令列表">命令列表</a> •
  <a href="#预算管理">预算管理</a> •
  <a href="#配置">配置</a>
</p>

---

## ✨ 特性

- 🚀 **完整 API 覆盖** — 12 个业务域，100+ 个命令
- 💻 **跨平台支持** — Windows、Linux、macOS
- 📊 **多种输出格式** — JSON、纯文本、表格、Markdown
- ⚙️ **智能配置** — 四层优先级系统
- 🐚 **Shell 自动补全** — Bash、Zsh、Fish、PowerShell
- 🔒 **默认安全** — Unix 权限控制 (0600)，支持环境变量
- ⚡ **极速运行** — 基于 Rust + Tokio 异步运行时构建

## 📦 安装

### 从源码安装

```bash
# 克隆仓库
git clone https://github.com/your-org/yunxiao-cli.git
cd yunxiao-cli

# 构建发布版本
cargo build --release

# 安装到 PATH
cp target/release/yunxiao-cli ~/.local/bin/
```

### 前提条件

- Rust 1.75+（通过 [rustup](https://rustup.rs/) 安装）
- 从云效控制台获取个人访问令牌

## 🚀 快速开始

### 1. 认证

从 [云效控制台](https://devops.console.aliyun.com/) 获取令牌：

```bash
yunxiao-cli auth login --token <YOUR_TOKEN>
```

或使用环境变量：

```bash
export YUNXIAO_CLI_TOKEN=<YOUR_TOKEN>
```

### 2. 验证设置

```bash
yunxiao-cli auth whoami
yunxiao-cli org list
```

### 3. 常用命令

```bash
# 列出项目
yunxiao-cli projex projects search --keyword "my-project"

# 查看代码库
yunxiao-cli codeup repos list

# 查看流水线
yunxiao-cli flow pipelines list

# 搜索工作项
yunxiao-cli projex workitems search --space-id <PROJECT_ID> --category Req
```

## 📋 命令列表

| 命令 | 描述 |
|---------|-------------|
| `auth` | 登录、登出、状态查看、个人信息 |
| `config` | 配置管理 |
| `org` | 组织与成员管理 |
| `projex` | 项目、工作项、迭代 |
| `codeup` | 代码库、分支、合并请求 |
| `flow` | 流水线与运行记录 |
| `appstack` | 应用交付与部署 |
| `packages` | 制品仓库管理 |
| `testhub` | 测试管理 |
| `insight` | 效能分析指标 |
| `thoughts` | 知识库 |
| `completion` | Shell 补全脚本生成 |
| `budget` | 💰 成本追踪与预算管理 |

## 💰 预算管理

追踪和管理您的云效资源成本：

### 查看预算状态

```bash
# 查看当月预算
yunxiao-cli budget status --org-id <ORG_ID>

# 查看预算历史
yunxiao-cli budget history --months 6 --org-id <ORG_ID>

# 获取详细分解
yunxiao-cli budget breakdown --org-id <ORG_ID>
```

### 设置预算告警

```bash
# 配置月度预算上限
yunxiao-cli budget set-limit --amount 5000 --currency CNY --org-id <ORG_ID>

# 设置告警阈值
yunxiao-cli budget set-alert --threshold 80 --org-id <ORG_ID>
```

### 成本分析

```bash
# 按服务查看成本
yunxiao-cli budget by-service --org-id <ORG_ID>

# 按项目查看成本
yunxiao-cli budget by-project --org-id <ORG_ID>

# 导出成本报表
yunxiao-cli budget export --format csv --month 2024-01 --org-id <ORG_ID>
```

### 预算配置

添加到配置文件（`~/.config/yunxiao-cli/config.toml`）：

```toml
[budget]
monthly_limit = 5000
currency = "CNY"
alert_threshold = 80  # 使用率达到 80% 时告警
alert_email = "admin@example.com"
```

## ⚙️ 配置

### 配置文件位置

```
~/.config/yunxiao-cli/config.toml
```

### 配置示例

```toml
# 认证
token = "pt_xxxxxxxxxxxxxxxxxxxx"

# API 设置
domain = "openapi-rdc.aliyuncs.com"
organization_id = "org-xxxxxxxx"

# 输出与日志
default_output = "table"
log_level = "warn"
timeout = 30

# 预算设置
[budget]
monthly_limit = 10000
alert_threshold = 85
```

### 优先级顺序

1. **CLI 参数** — `--output json`, `--timeout 60`
2. **环境变量** — `YUNXIAO_CLI_TOKEN`
3. **配置文件** — `~/.config/yunxiao-cli/config.toml`
4. **默认值** — JSON, 30秒超时, warn 日志级别

### 环境变量

| 变量 | 描述 |
|----------|-------------|
| `YUNXIAO_CLI_TOKEN` | 个人访问令牌 |
| `YUNXIAO_CLI_OUTPUT` | 输出格式 |
| `YUNXIAO_CLI_TIMEOUT` | 请求超时时间 |
| `YUNXIAO_CLI_ORG_ID` | 默认组织 ID |
| `YUNXIAO_CLI_LOG_LEVEL` | 日志级别 |

## 🛠️ 开发

```bash
# 构建
cargo build

# 运行测试
cargo test

# 代码检查与格式化
cargo clippy --all-targets
cargo fmt

# 构建发布版本
cargo build --release
```

## 📖 API 参考

- **文档**: [云效 OpenAPI](https://help.aliyun.com/zh/yunxiao/developer-reference/)
- **端点**: `https://openapi-rdc.aliyuncs.com`
- **认证**: 通过 `x-devops-pat` 请求头传递个人访问令牌

## 📄 许可协议

MIT 许可协议 — 详情请参见 [LICENSE](LICENSE)。

---

<p align="center">
  使用 Rust 精心打造 ❤️
</p>
