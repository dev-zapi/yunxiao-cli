# YunXiao CLI Skill

## 用途

阿里云云效（YunXiao）DevOps 平台的命令行工具，用于通过终端管理完整的 DevOps 生命周期。

## 能做什么

- **认证管理** - 登录、查看用户信息、登出
- **组织管理** - 查看组织信息、成员、部门、角色
- **项目协作** - 管理工作项（需求/任务/缺陷）、迭代、版本
- **代码管理** - 查看仓库、分支、提交代码、发起合并请求
- **流水线** - 查看流水线、触发构建、查看构建日志
- **应用交付** - 管理应用、环境变量、创建部署
- **制品库** - 查看制品仓库和构建产物
- **测试管理** - 查看测试用例、测试计划、测试结果
- **效率洞察** - 查看效能分析数据
- **知识库** - 查看文档和页面

## 使用前提

1. 登录云效控制台获取个人访问令牌（PAT）
2. 使用以下方式之一配置认证：
   - 环境变量: `export YUNXIAO_CLI_TOKEN=<your_token>`
   - 配置文件: `~/.config/yunxiao/config.toml`

## 命令参考

### 认证相关

```bash
# 查看当前登录用户
yunxiao auth whoami

# 查看认证状态
yunxiao auth status

# 登出（清除本地 token）
yunxiao auth logout
```

### 配置管理

```bash
# 查看所有配置
yunxiao config list

# 获取配置项
yunxiao config get token
yunxiao config get default_output
yunxiao config get organization_id

# 设置配置项
yunxiao config set default_output table
yunxiao config set timeout 60
yunxiao config set organization_id <ORG_ID>

# 删除配置项
yunxiao config delete organization_id

# 查看配置文件路径
yunxiao config path
```

### 组织与成员

```bash
# 列出所有组织
yunxiao org list

# 查看组织详情
yunxiao org info --org-id <ORG_ID>

# 列出成员
yunxiao org members list --org-id <ORG_ID>

# 搜索成员
yunxiao org members search --query "alice" --org-id <ORG_ID>

# 查看成员详情
yunxiao org members get --user-id <USER_ID> --org-id <ORG_ID>

# 列出部门
yunxiao org departments list --org-id <ORG_ID>

# 列出角色
yunxiao org roles list --org-id <ORG_ID>
```

### 项目协作（Projex）

```bash
# 搜索项目
yunxiao projex projects search --keyword "demo" --org-id <ORG_ID>

# 查看项目详情
yunxiao projex projects get --project-id <PROJECT_ID> --org-id <ORG_ID>

# 搜索工作项（Req=需求, Task=任务, Bug=缺陷）
yunxiao projex workitems search --space-id <PROJECT_ID> --category Req --org-id <ORG_ID>

# 查看工作项详情
yunxiao projex workitems get --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID>

# 创建工作项
yunxiao projex workitems create --space-id <PROJECT_ID> --category Req --subject "新功能" --org-id <ORG_ID>

# 列出迭代
yunxiao projex sprints list --space-id <PROJECT_ID> --org-id <ORG_ID>

# 创建迭代
yunxiao projex sprints create --space-id <PROJECT_ID> --name "迭代1" --org-id <ORG_ID>

# 列出版本
yunxiao projex versions list --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 代码管理（Codeup）

```bash
# 列出代码仓库
yunxiao codeup repos list --org-id <ORG_ID>

# 查看仓库详情
yunxiao codeup repos get --repo-id <REPO_ID> --org-id <ORG_ID>

# 列出分支
yunxiao codeup branches list --repo-id <REPO_ID> --org-id <ORG_ID>

# 创建分支
yunxiao codeup branches create --repo-id <REPO_ID> --branch feature/new --ref main --org-id <ORG_ID>

# 列出合并请求
yunxiao codeup mr list --repo-id <REPO_ID> --org-id <ORG_ID>

# 创建合并请求
yunxiao codeup mr create --repo-id <REPO_ID> --source feature/new --target main --title "添加新功能" --org-id <ORG_ID>

# 查看文件列表
yunxiao codeup files list --repo-id <REPO_ID> --org-id <ORG_ID>

# 查看文件内容
yunxiao codeup files get --repo-id <REPO_ID> --path src/main.rs --org-id <ORG_ID>
```

### 流水线（Flow）

```bash
# 列出流水线
yunxiao flow pipelines list --org-id <ORG_ID>

# 查看流水线详情
yunxiao flow pipelines get --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# 触发流水线运行
yunxiao flow runs create --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# 列出运行记录
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# 查看最新运行
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>

# 查看任务日志
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID>
```

### 应用交付（Appstack）

```bash
# 列出应用
yunxiao appstack apps list --org-id <ORG_ID>

# 查看应用详情
yunxiao appstack apps get --app-name <APP_NAME> --org-id <ORG_ID>

# 列出环境变量
yunxiao appstack vars list --org-id <ORG_ID>

# 创建部署
yunxiao appstack deploy create --app-name <APP_NAME> --org-id <ORG_ID>
```

### 制品库（Packages）

```bash
# 列出制品仓库
yunxiao packages repos list --org-id <ORG_ID>

# 列出版本
yunxiao packages artifacts list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 测试管理（Testhub）

```bash
# 搜索测试用例
yunxiao testhub cases search --space-id <PROJECT_ID> --org-id <ORG_ID>

# 列出测试计划
yunxiao testhub plans list --space-id <PROJECT_ID> --org-id <ORG_ID>

# 查看测试结果
yunxiao testhub plans results list --space-id <PROJECT_ID> --plan-id <PLAN_ID> --org-id <ORG_ID>
```

### 效率洞察（Insight）

```bash
# 查看效能指标（具体参数根据实际 API 调整）
yunxiao insight metrics --org-id <ORG_ID>
```

### 知识库（Thoughts）

```bash
# 列出文档
yunxiao thoughts documents list --org-id <ORG_ID>

# 查看页面
yunxiao thoughts pages list --org-id <ORG_ID>
```

### 生成 Shell 补全

```bash
# Bash
yunxiao completion generate --shell bash > ~/.local/share/bash-completion/completions/yunxiao

# Zsh
yunxiao completion generate --shell zsh > ~/.zfunc/_yunxiao

# PowerShell
yunxiao completion generate --shell powershell > yunxiao.ps1
```

## 全局参数

所有命令都支持以下全局参数：

| 参数 | 环境变量 | 说明 |
|------|---------|------|
| `-o, --output` | `YUNXIAO_CLI_OUTPUT` | 输出格式：json, text, table, markdown |
| `--timeout` | `YUNXIAO_CLI_TIMEOUT` | API 请求超时时间（秒） |
| `--log-level` | `YUNXIAO_CLI_LOG_LEVEL` | 日志级别：debug, info, warn, error |
| `--token` | `YUNXIAO_CLI_TOKEN` | 个人访问令牌 |
| `--endpoint` | `YUNXIAO_CLI_ENDPOINT` | API 端点地址 |
| `--org-id` | `YUNXIAO_CLI_ORG_ID` | 组织 ID |

## 配置文件

### 配置文件路径

- Linux/macOS: `~/.config/yunxiao/config.toml`
- Windows: `%APPDATA%\yunxiao\config.toml`

### 配置文件示例

```toml
# 个人访问令牌
token = "pt_xxxxxxxxxxxxxxxxxxxx"

# API 端点域名（默认：openapi-rdc.aliyuncs.com）
domain = "openapi-rdc.aliyuncs.com"

# 默认组织 ID
organization_id = "org-xxxxxxxx"

# 默认输出格式：json | text | table | markdown
default_output = "table"

# 日志级别：debug | info | warn | error
log_level = "warn"

# 请求超时时间（秒，默认 30）
timeout = 30
```

### 配置优先级

配置按以下优先级从高到低生效：

1. **命令行参数** - 如 `--output json`, `--timeout 60`
2. **环境变量** - 如 `YUNXIAO_CLI_TOKEN`, `YUNXIAO_CLI_OUTPUT`
3. **配置文件** - `~/.config/yunxiao/config.toml`
4. **默认值** - JSON 输出，30秒超时，warn 日志级别

## 获取个人访问令牌

1. 登录 [云效控制台](https://devops.console.aliyun.com/)
2. 点击右上角头像 → **个人设置** → **个人访问令牌**
3. 点击 **创建令牌**
4. 复制令牌（仅显示一次）
