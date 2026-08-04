---
name: yunxiao-cli
description: 云效命令行工具。用于查询云效数据、管理项目、操作代码仓库、触发流水线或查看测试结果。务必在用户提及云效、阿里云DevOps、项目管理、代码仓库、流水线、工作项、迭代、版本、测试、制品库、需求、任务、缺陷、工时、标签、合并请求、分支等关键词时触发此技能，即使用户未明确提及"yunxiao"或"CLI"。适用于Agent编程与自动化脚本场景。
cli_version: ">=0.1.0"
---

# YunXiao CLI Skill

云效（阿里云 DevOps）平台的命令行工具，提供完整的 API 访问能力。

---

## 项目上下文配置（YUNXIAO.md）

**执行任何云效命令前，先检查项目根目录是否存在 `YUNXIAO.md` 文件。**

这个文件包含预定义的项目信息，可以避免重复查询 ID。如果文件存在，读取并使用其中的配置；如果不存在，按照常规流程查询。

### YUNXIAO.md 文件格式

```markdown
# 云效项目配置

## 基本信息
- 组织 ID: org-xxxxxxxx
- 项目 ID: proj-xxxxxxxx
- 项目名称: 示例项目

## 常用 ID
- 需求类型 ID: type-xxx
- 任务类型 ID: type-yyy
- 缺陷类型 ID: type-zzz
- 当前迭代 ID: sprint-xxx

## 仓库信息
- 主仓库 ID: repo-xxxxxxxx
- 默认分支: main

## 其他配置
- 优先级 P0 ID: priority-xxx
- 状态"进行中" ID: status-xxx
```

### 使用规则

1. **优先使用 YUNXIAO.md 中的值**：如果文件中定义了某个 ID，直接使用，无需再查询
2. **缺失时再查询**：如果文件中没有需要的 ID，再使用 CLI 命令查询
3. **可以补充文件**：如果查询到了新的常用 ID，可以建议用户更新 YUNXIAO.md

---

## Agent Quick Reference

Agent 编写脚本或自动化任务时的关键规则：

### 0. 优先检查 YUNXIAO.md

**在执行任何查询前，先检查项目根目录的 `YUNXIAO.md` 文件。** 该文件包含预定义的项目 ID 和配置，可以直接使用，避免重复查询。详见上方"项目上下文配置"章节。

### 1. 输出格式

**Agent 场景必须使用 `--output json`**，这是 Agent 解析结果的唯一可靠方式。

```bash
yunxiao projex projects search --org-id org-xxx --output json | jq -r '.[0].id'
```

- JSON 输出格式稳定、结构化，便于 jq 或代码解析
- table/plain 输出格式可能变化，不适合程序解析
- 人类使用时可省略，默认 table 格式更易读

### 2. 全局标志

几乎所有命令都需要以下全局标志：

| 标志 | 环境变量 | 说明 |
|------|---------|------|
| `--org-id <ID>` | `YUNXIAO_CLI_ORG_ID` | 组织 ID，**绝大多数命令必需** |
| `-o, --output json` | `YUNXIAO_CLI_OUTPUT` | 输出格式 |
| `--timeout <秒>` | `YUNXIAO_CLI_TIMEOUT` | API 超时（默认 30） |
| `--token <TOKEN>` | `YUNXIAO_CLI_TOKEN` | 个人访问令牌 |

### 3. ID 链路查询流程

Agent 查询数据时需要按以下链路逐步获取 ID：

```
org_id (组织ID)
    ↓
[projects search] → space_id (项目ID)
    ↓                      ↓
[workitems types]     [workitems search] → workitem_id
    ↓                      ↓
type_id            [workitems get]
    ↓
[workitems flow] → status_id
    ↓
[workitems fields] → fieldIdentifier
```

### 4. 核心 ID 获取速查

| 需要 ID | 使用命令 |
|--------|---------|
| `project_id` (space_id) | `yunxiao projex projects search --keyword <kw>` |
| `workitem_id` | `yunxiao projex workitems search --space-id <SPACE_ID>` |
| `type_id` | `yunxiao projex workitems types --space-id <SPACE_ID>` |
| `sprint_id` | `yunxiao projex sprints list --space-id <SPACE_ID>` |
| `version_id` | `yunxiao projex versions list --space-id <SPACE_ID>` |
| `label_id` | `yunxiao projex labels list --space-id <SPACE_ID>` |
| `priority_id` | `yunxiao projex workitems fields --project-id <ID> --type-id <TYPE_ID>` |
| `status_id` | `yunxiao projex workitems flow --space-id <SPACE_ID> --type-id <TYPE_ID>` |
| `user_id` | `yunxiao org members list --org-id <ORG_ID>` |
| `repo_id` | `yunxiao codeup repos list --org-id <ORG_ID>` |
| `pipeline_id` | `yunxiao flow pipelines list --org-id <ORG_ID>` |

### 5. 参数风格差异

不同命令的 ID 参数风格不同（这是云效 API 的设计差异）：

| 命令 | 参数风格 | 示例 |
|------|---------|------|
| `projects get` | **位置参数** | `yunxiao projex projects get <PROJECT_ID>` |
| `workitems get` | **flag 参数** | `yunxiao projex workitems get --workitem-id <ID>` |
| `sprints get` | **flag 参数** | `yunxiao projex sprints get --sprint-id <ID>` |

Agent 必须仔细区分，避免参数传递错误。

### 6. 创建/更新前先查询

- 创建工作项前：先 `workitems types` 获取 type_id
- 更新状态前：先 `workitems flow` 获取可用 status_id
- 设置字段前：先 `workitems fields` 获取 fieldIdentifier 和格式类型

---

## 典型工作流示例

### 查询项目并获取工作项

```bash
# 1. 搜索项目
PROJECT_ID=$(yunxiao projex projects search --keyword "demo" --org-id org-xxx --output json | jq -r '.[0].id')

# 2. 获取工作项类型 ID
TYPE_ID=$(yunxiao projex workitems types --space-id $PROJECT_ID --category Req --org-id org-xxx --output json | jq -r '.[0].id')

# 3. 查询工作项
yunxiao projex workitems search --space-id $PROJECT_ID -c Req --org-id org-xxx --output json
```

### 创建工作项完整流程

```bash
# 1. 获取类型 ID
TYPE_ID=$(yunxiao projex workitems types --space-id proj-xxx --category Req --org-id org-xxx --output json | jq -r '.[0].id')

# 2. 获取可用状态（可选）
yunxiao projex workitems flow --space-id proj-xxx --type-id $TYPE_ID --org-id org-xxx --output json

# 3. 创建工作项
yunxiao projex workitems create --space-id proj-xxx --type-id $TYPE_ID \
  --subject "新功能需求" --description "## 说明\n- 功能点1" \
  --org-id org-xxx
```

### 查看代码仓库并创建 MR

```bash
# 1. 列出仓库
REPO_ID=$(yunxiao codeup repos list --org-id org-xxx --output json | jq -r '.[0].id')

# 2. 创建分支
yunxiao codeup branches create --repo-id $REPO_ID --branch feature/new --ref main --org-id org-xxx

# 3. 创建合并请求
yunxiao codeup mr create --repo-id $REPO_ID --source feature/new --target main --title "新功能" --org-id org-xxx
```

---

## 详细命令手册

完整命令手册请参阅 [commands/](./commands/) 目录下的各模块文档：

| 模块 | 说明 | 文档 |
|------|------|------|
| projex | 项目、工作项、迭代、版本、工时、标签 | [projex.md](./commands/projex.md) |
| codeup | 仓库、分支、合并请求、文件 | [codeup.md](./commands/codeup.md) |
| flow | 流水线、运行、日志 | [flow.md](./commands/flow.md) |
| org | 组织、成员 | [org.md](./commands/org.md) |
| appstack | 应用交付 | [appstack.md](./commands/appstack.md) |
| packages | 制品库 | [packages.md](./commands/packages.md) |
| testhub | 测试管理 | [testhub.md](./commands/testhub.md) |
| insight | 效能洞察 | [insight.md](./commands/insight.md) |
| thoughts | 知识库 | [thoughts.md](./commands/thoughts.md) |
| auth | 认证管理 | [auth.md](./commands/auth.md) |
| config | 配置管理 | [config.md](./commands/config.md) |
| completion | Shell 补全 | [completion.md](./commands/completion.md) |

---

## Help 命令

如果本手册未找到所需信息，可使用 `--help` 获取命令帮助：

```bash
yunxiao --help                    # 查看全局帮助
yunxiao <command> --help          # 查看子命令帮助
yunxiao <command> <sub> --help    # 查看更深层级的帮助
```

---

## 配置说明

详细配置项、环境变量、优先级说明请参阅 [configuration.md](./configuration.md)。

### 快速配置

```bash
# 设置组织 ID（避免每次命令重复指定）
yunxiao config set organization_id org-xxxxxxxx

# 设置令牌
yunxiao config set token pt_xxxxxxxxxxxxxxxx

# 设置输出格式
yunxiao config set default_output json
```

---

## 故障排查

### "Organization not found" 或 "org-id required"

```bash
# 查看当前配置
yunxiao config list

# 设置组织 ID
yunxiao config set organization_id org-xxx
# 或使用环境变量
export YUNXIAO_CLI_ORG_ID="org-xxx"
```

### "Token 无效" 或 "401 Unauthorized"

```bash
# 重新设置令牌
yunxiao config set token pt_xxx
# 或使用环境变量
export YUNXIAO_CLI_TOKEN="pt_xxx"
```

### "工作项类型id不能为空"

创建工作项前必须先获取类型 ID：

```bash
yunxiao projex workitems types --space-id proj-xxx --category Req --org-id org-xxx --output json
```