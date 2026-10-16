# 代码管理命令手册

`yunxiao codeup` 命令用于管理代码仓库、代码组、分支、提交、合并请求、文件和代码比较。

> **ID 参数约定**：仓库与代码组相关的 ID 参数（`--repo-id`、`--group-id`、`--namespace-id`、`--parent-id`、`--template-id`）均接受纯数字 ID 或完整路径（`path_with_namespace`）。含 `/` 的完整路径在拼入 URL 路径段时需百分号编码；多数命令由 CLI 自动编码，少数命令（帮助中注明 "URL-encoded full path"）需自行传入已编码的路径（如 `60de7a6852743a5162b5f957%2FDemoRepo`）。

---

## 命令列表

### 仓库管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup repos list` | 列出代码仓库（支持关键词、排序、归档状态过滤） |
| `yunxiao codeup repos get` | 查看仓库详情 |
| `yunxiao codeup repos create` | 创建代码库（支持从模板导入） |
| `yunxiao codeup repos templates list` | 查询模板仓库 |
| `yunxiao codeup repos update` | 更新代码库元数据（部分更新） |
| `yunxiao codeup repos archive` | 归档代码库 |
| `yunxiao codeup repos transfer` | 转移代码库到其他代码组 |
| `yunxiao codeup repos delete` | 删除代码库（双重确认） |

### 代码组管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup groups list` | 列出代码组（及根空间） |
| `yunxiao codeup groups get` | 查看代码组或根空间详情 |
| `yunxiao codeup groups create` | 创建代码组 |
| `yunxiao codeup groups update` | 更新代码组元数据（部分更新） |
| `yunxiao codeup groups repos list` | 列出代码组内的代码库 |

### 分支管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup branches list` | 列出分支 |
| `yunxiao codeup branches get` | 查看分支详情 |
| `yunxiao codeup branches create` | 创建分支 |
| `yunxiao codeup branches delete` | 删除分支 |

### 提交管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup commits list` | 列出提交记录 |
| `yunxiao codeup commits get` | 查看提交详情 |

### 代码比较

| 命令 | 说明 |
|------|------|
| `yunxiao codeup compare` | 比较两个分支/提交的差异 |

### 合并请求

| 命令 | 说明 |
|------|------|
| `yunxiao codeup mr list` | 列出合并请求 |
| `yunxiao codeup mr get` | 查看合并请求详情 |
| `yunxiao codeup mr create` | 创建合并请求 |
| `yunxiao codeup mr update` | 更新合并请求标题或描述 |
| `yunxiao codeup mr merge` | 合并合并请求 |
| `yunxiao codeup mr review` | 评审合并请求 |
| `yunxiao codeup mr close` | 关闭合并请求 |
| `yunxiao codeup mr reopen` | 重新打开合并请求 |
| `yunxiao codeup mr person` | 更新评审人或订阅人 |
| `yunxiao codeup mr labels list` | 查看合并请求标签 |
| `yunxiao codeup mr labels attach` | 关联标签到合并请求 |
| `yunxiao codeup mr tree` | 查看变更文件树 |
| `yunxiao codeup mr comments list` | 查看评论 |
| `yunxiao codeup mr comments create` | 创建评论 |
| `yunxiao codeup mr comments update` | 更新评论 |
| `yunxiao codeup mr comments delete` | 删除评论 |
| `yunxiao codeup mr patchsets` | 查看 Patchset 列表 |

### 文件管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup files list` | 查看文件列表 |
| `yunxiao codeup files get` | 查看文件内容 |
| `yunxiao codeup files create` | 创建新文件 |
| `yunxiao codeup files update` | 更新文件内容 |
| `yunxiao codeup files delete` | 删除文件 |

---

## 列出代码仓库

### 基本用法

```bash
yunxiao codeup repos list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |
| `--search` | 按关键词模糊匹配仓库路径 | 否 |
| `--order-by` | 排序字段：`created_at`、`name`、`path`、`last_activity_at` | 否 |
| `--sort` | 排序方向：`asc`、`desc` | 否 |
| `--archived` | 按归档状态过滤：`true`/`false`；省略则包含两者 | 否 |

### 示例

```bash
# 列出全部仓库
yunxiao codeup repos list --org-id org-xxxxxxxx --output json

# 关键词搜索 + 按名称升序
yunxiao codeup repos list --search demo --order-by name --sort asc --org-id org-xxxxxxxx

# 仅列出未归档仓库
yunxiao codeup repos list --archived false --org-id org-xxxxxxxx
```

---

## 查看仓库详情

### 基本用法

```bash
yunxiao codeup repos get --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID；也接受完整路径 `path_with_namespace`（含 `/` 时需 URL 编码） | 是 |

### 示例

```bash
yunxiao codeup repos get --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 创建代码库

### 基本用法

```bash
yunxiao codeup repos create --name <NAME> --path <PATH> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--name` | 代码库名称 | 是 |
| `--path` | 代码库路径 | 是 |
| `--namespace-id` | 父代码组空间 ID，可传纯数字 ID 或全路径；省略则创建在组织根路径下 | 否 |
| `--description` | 代码库描述 | 否 |
| `--visibility` | 可见性，仅 `private` 或 `internal` | 否 |
| `--readme-type` | 自动创建 README 类型：`EMPTY` 或 `USER_GUIDE` | 否 |
| `--tags` | 代码库标签，逗号分隔（如 `tag1,tag2`） | 否 |
| `--avatar-url` | 代码库头像地址 | 否 |
| `--template-id` | 模板库 ID（可传纯数字 ID 或全路径），用于从模板导入 | 否 |
| `--template-type` | 模板类型：`1` 自定义模板，`2` 系统预置模板 | 否 |
| `--sync-all-branches` | 导入模板库所有分支 | 否 |
| `--create-parent-path` | 自动创建缺失的父路径 | 否 |

未传的可选参数不会进入请求体。

### 示例

```bash
# 创建空代码库
yunxiao codeup repos create --name demo-repo --path demo-repo --org-id org-xxxxxxxx

# 指定父代码组、可见性并自动生成 README
yunxiao codeup repos create --name demo-repo --path demo-repo \
  --namespace-id 2813489 --visibility private --readme-type USER_GUIDE \
  --org-id org-xxxxxxxx

# 从模板导入，并同步模板全部分支
yunxiao codeup repos create --name scaffold --path scaffold \
  --template-id 7 --template-type 2 --sync-all-branches \
  --org-id org-xxxxxxxx
```

---

## 查询模板仓库

查询可用的模板仓库，用于 `repos create --template-id` 导入。

### 基本用法

```bash
yunxiao codeup repos templates list --template-type <1|2> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--template-type` | 模板类型：`1` 自定义模板，`2` 系统预置模板 | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
# 查询系统预置模板
yunxiao codeup repos templates list --template-type 2 --org-id org-xxxxxxxx --output json

# 查询自定义模板
yunxiao codeup repos templates list --template-type 1 --org-id org-xxxxxxxx --output json
```

---

## 更新代码库

部分更新代码库元数据，仅传入的参数会进入请求体。

### 基本用法

```bash
yunxiao codeup repos update --repo-id <REPO_ID> [OPTIONS] --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID 或完整路径 `path_with_namespace`（CLI 自动百分号编码） | 是 |
| `--name` | 新仓库名称 | 否 |
| `--description` | 新仓库描述（最长 65535 字符） | 否 |
| `--default-branch` | 默认分支 | 否 |
| `--visibility` | 可见性：`private` 或 `internal` | 否 |
| `--tags` | 仓库标签，逗号分隔 | 否 |

### 示例

```bash
# 重命名并更新描述
yunxiao codeup repos update --repo-id repo-xxxxxxxx \
  --name demo-repo-v2 --description "更新后的说明" \
  --org-id org-xxxxxxxx

# 使用完整路径标识仓库
yunxiao codeup repos update --repo-id "60de7a6852743a5162b5f957/DemoRepo" \
  --default-branch develop --org-id org-xxxxxxxx
```

---

## 归档代码库

归档代码库（变为只读）。

### 基本用法

```bash
yunxiao codeup repos archive --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID 或完整路径 `path_with_namespace`（CLI 自动百分号编码） | 是 |

### 示例

```bash
yunxiao codeup repos archive --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 转移代码库

将代码库转移到另一个代码组。

### 基本用法

```bash
yunxiao codeup repos transfer --repo-id <REPO_ID> --group-id <GROUP_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID 或完整路径 `path_with_namespace` | 是 |
| `--group-id` | 目标代码组 ID 或完整路径 `path_with_namespace` | 是 |

### 示例

```bash
# 转移到数字 ID 代码组
yunxiao codeup repos transfer --repo-id repo-xxxxxxxx --group-id 2813489 --org-id org-xxxxxxxx

# 转移到全路径代码组
yunxiao codeup repos transfer --repo-id repo-xxxxxxxx \
  --group-id "60de7a6852743a5162b5f957/DemoGroup" --org-id org-xxxxxxxx
```

---

## 删除代码库

删除代码库需要**双重确认**，防止误删。

### 基本用法

```bash
yunxiao codeup repos delete --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID 或已 URL 编码的完整路径 `path_with_namespace`（如 `60de...%2FDemoRepo`） | 是 |
| `--yes` | 跳过交互式确认（用于自动化/脚本） | 否 |

### 双重确认流程

默认情况下（未传 `--yes`）分两步交互确认：

1. **第一次确认**：提示 `This will permanently delete repository '<repo-id>' and all its contents. Continue? [y/N]`，输入 `y`/`yes` 继续，其他输入取消。
2. **第二次确认**：提示 `Type the repository ID or full path to confirm deletion:`，必须**逐字输入与 `--repo-id` 完全一致的 ID 或全路径**（去除首尾空白后精确匹配），否则取消。

任一步骤取消都会打印 `Deletion cancelled.` 并安全退出，不会调用删除接口。

### 自动化场景（--yes）

脚本/CI 等非交互环境必须传 `--yes` 跳过两次确认：

```bash
yunxiao codeup repos delete --repo-id repo-xxxxxxxx --yes --org-id org-xxxxxxxx
```

**注意**：当标准输入不是 TTY 且未传 `--yes` 时，命令会直接报错拒绝执行：
`Refusing to delete in non-interactive mode. Pass --yes to confirm.`

### 示例

```bash
# 交互式删除（终端中逐字确认）
yunxiao codeup repos delete --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx

# 自动化删除（脚本/CI）
yunxiao codeup repos delete --repo-id repo-xxxxxxxx --yes --org-id org-xxxxxxxx
```

---

## 代码组管理

`codeup groups` 用于管理代码组（代码库与子组的容器）。底层云效 API 以 `namespaces` 资源暴露代码组，组织根空间也是空间而非代码组；CLI 按 [ADR-0005](../../../docs/adr/0005-cli-command-surface-decoupled-from-api-resources.md) 将命令面与 API 资源名解耦——`groups list` / `groups get` 底层调用 namespaces 接口，但对外统一呈现为"代码组 / 根空间"一个概念，用户无需区分 namespace 与 group。

### 列出代码组

```bash
yunxiao codeup groups list [OPTIONS] --org-id <ORG_ID>
```

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--parent-id` | 父代码组或根空间 ID，或完整路径；省略则列出当前用户的代码组空间 | 否 |
| `--search` | 搜索关键词 | 否 |
| `--order-by` | 排序字段：`created_at` 或 `updated_at`（默认 `updated_at`） | 否 |
| `--sort` | 排序方向：`asc` 或 `desc`（默认 `desc`） | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

```bash
yunxiao codeup groups list --org-id org-xxxxxxxx --output json
yunxiao codeup groups list --search demo --parent-id "60de.../DemoGroup" --org-id org-xxxxxxxx
```

### 查看代码组详情

```bash
yunxiao codeup groups get --group-id <GROUP_ID> --org-id <ORG_ID>
```

`--group-id` 接受代码组 ID、组织根空间 ID 或完整路径（含 `/` 时 CLI 自动百分号编码）。

```bash
yunxiao codeup groups get --group-id 2813489 --org-id org-xxxxxxxx --output json
yunxiao codeup groups get --group-id "60de7a6852743a5162b5f957/DemoGroup" --org-id org-xxxxxxxx
```

### 创建代码组

```bash
yunxiao codeup groups create --name <NAME> --path <PATH> --org-id <ORG_ID>
```

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--name` | 代码组显示名称 | 是 |
| `--path` | 代码组路径段 | 是 |
| `--parent-id` | 父代码组或根空间 ID；省略则在组织根下创建 | 否 |
| `--description` | 描述（最长 65535 字符） | 否 |
| `--visibility` | 可见性：`private`（仅组内成员）或 `internal`（组织内可见） | 否 |
| `--avatar` | 头像 URL | 否 |

```bash
# 在组织根下创建代码组
yunxiao codeup groups create --name DemoGroup --path demo-group --org-id org-xxxxxxxx

# 在父代码组下创建
yunxiao codeup groups create --name SubGroup --path sub-group \
  --parent-id 2813489 --visibility private --org-id org-xxxxxxxx
```

### 更新代码组

部分更新代码组元数据，仅传入的参数会进入请求体。

```bash
yunxiao codeup groups update --group-id <GROUP_ID> [OPTIONS] --org-id <ORG_ID>
```

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--group-id` | 代码组 ID 或完整路径（含 `/` 时 CLI 自动百分号编码） | 是 |
| `--name` | 新显示名称 | 否 |
| `--path` | 新路径段 | 否 |
| `--description` | 新描述 | 否 |
| `--visibility` | 新可见性：`private` 或 `internal` | 否 |
| `--avatar` | 新头像 URL | 否 |

```bash
yunxiao codeup groups update --group-id 2813489 \
  --name "DemoGroup 2.0" --description "更新后的代码组" \
  --org-id org-xxxxxxxx
```

### 列出代码组内的代码库

```bash
yunxiao codeup groups repos list --group-id <GROUP_ID> [OPTIONS] --org-id <ORG_ID>
```

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--group-id` | 代码组或根空间 ID，或完整路径 | 是 |
| `--search` | 搜索关键词（仓库名称） | 否 |
| `--include-subgroups` | 包含子组内的代码库 | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

```bash
yunxiao codeup groups repos list --group-id 2813489 --org-id org-xxxxxxxx --output json
yunxiao codeup groups repos list --group-id "60de.../DemoGroup" \
  --include-subgroups --org-id org-xxxxxxxx
```

---

## 列出分支

### 基本用法

```bash
yunxiao codeup branches list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao codeup branches list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 查看分支详情

### 基本用法

```bash
yunxiao codeup branches get --repo-id <REPO_ID> --branch <branch> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--branch` | 分支名称 | 是 |

### 示例

```bash
yunxiao codeup branches get --repo-id repo-xxxxxxxx --branch main --org-id org-xxxxxxxx --output json
```

---

## 创建分支

### 基本用法

```bash
yunxiao codeup branches create --repo-id <REPO_ID> --branch <branch> --ref <ref> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--branch` | 新分支名称 | 是 |
| `--ref` | 来源分支/提交 | 是 |

### 示例

```bash
# 从 main 创建分支
yunxiao codeup branches create --repo-id repo-xxxxxxxx --branch feature/new --ref main --org-id org-xxxxxxxx

# 从其他分支创建
yunxiao codeup branches create --repo-id repo-xxxxxxxx --branch bugfix/login --ref develop --org-id org-xxxxxxxx

# 从特定提交创建
yunxiao codeup branches create --repo-id repo-xxxxxxxx --branch hotfix/issue-123 --ref abc123def --org-id org-xxxxxxxx
```

---

## 删除分支

### 基本用法

```bash
yunxiao codeup branches delete --repo-id <REPO_ID> --branch <branch> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--branch` | 分支名称 | 是 |

### 示例

```bash
yunxiao codeup branches delete --repo-id repo-xxxxxxxx --branch feature/old --org-id org-xxxxxxxx
```

---

## 列出提交记录

### 基本用法

```bash
yunxiao codeup commits list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--ref` | 分支或标签（默认主分支） | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
# 查看主分支提交
yunxiao codeup commits list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx --output json

# 查看特定分支提交
yunxiao codeup commits list --repo-id repo-xxxxxxxx --ref feature/new --org-id org-xxxxxxxx --output json
```

---

## 查看提交详情

### 基本用法

```bash
yunxiao codeup commits get --repo-id <REPO_ID> --sha <SHA> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--sha` | 提交 SHA | 是 |

### 示例

```bash
yunxiao codeup commits get --repo-id repo-xxxxxxxx --sha abc123def456 --org-id org-xxxxxxxx --output json
```

---

## 比较代码差异

### 基本用法

```bash
yunxiao codeup compare --repo-id <REPO_ID> --from <from> --to <to> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--from` | 来源分支/标签/提交 | 是 |
| `--to` | 目标分支/标签/提交 | 是 |

### 示例

```bash
# 比较两个分支
yunxiao codeup compare --repo-id repo-xxxxxxxx --from main --to develop --org-id org-xxxxxxxx --output json

# 比较两个提交
yunxiao codeup compare --repo-id repo-xxxxxxxx --from abc123 --to def456 --org-id org-xxxxxxxx --output json

# 比较分支与标签
yunxiao codeup compare --repo-id repo-xxxxxxxx --from v1.0.0 --to main --org-id org-xxxxxxxx --output json
```

---

## 列出合并请求

### 基本用法

```bash
yunxiao codeup mr list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID（可选过滤器） | 否 |
| `--state` | 状态过滤：opened、closed、merged、all | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
# 列出所有 MR
yunxiao codeup mr list --org-id org-xxxxxxxx --output json

# 按仓库过滤
yunxiao codeup mr list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx --output json

# 按状态过滤
yunxiao codeup mr list --repo-id repo-xxxxxxxx --state opened --org-id org-xxxxxxxx --output json
```

---

## 创建合并请求

### 基本用法

```bash
yunxiao codeup mr create --repo-id <REPO_ID> --source <source> --target <target> --title <title> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--source` | 来源分支 | 是 |
| `--target` | 目标分支 | 是 |
| `--title` | MR 标题 | 是 |
| `--description` | MR 描述 | 否 |
| `--workitem-id` | 关联工作项 ID，可多次传入 | 否 |
| `--source-project-id` | 源仓库 ID（默认使用 repo_id） | 否 |
| `--target-project-id` | 目标仓库 ID（默认使用 repo_id） | 否 |

### 示例

```bash
# 同库 MR: feature/new -> main（简化用法）
yunxiao codeup mr create --repo-id repo-xxxxxxxx --source feature/new --target main --title "添加新功能" --org-id org-xxxxxxxx

# 同库 MR: bugfix/login -> develop
yunxiao codeup mr create --repo-id repo-xxxxxxxx --source bugfix/login --target develop --title "修复登录问题" --org-id org-xxxxxxxx

# 跨库 MR: 从 fork 仓库合并到主仓库
yunxiao codeup mr create --repo-id fork-repo-xxx \
    --source feature/new --target main --title "添加新功能" \
    --source-project-id fork-repo-xxx --target-project-id main-repo-xxx \
    --org-id org-xxxxxxxx

# 创建 MR 并关联多个工作项
yunxiao codeup mr create --repo-id repo-xxxxxxxx \
    --source feature/login --target main --title "登录功能" \
    --workitem-id wi-xxxxxxxx --workitem-id wi-yyyyyyyy \
    --org-id org-xxxxxxxx
```

---

## 查看合并请求详情

### 基本用法

```bash
yunxiao codeup mr get --repo-id <REPO_ID> --mr-id <MR_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--mr-id` | 合并请求局部 ID | 是 |

---

## 更新合并请求

### 基本用法

```bash
yunxiao codeup mr update --repo-id <REPO_ID> --mr-id <MR_ID> --title <TITLE> --description <DESC> --org-id <ORG_ID>
```

`--title` 和 `--description` 至少传入一个。

---

## 合并合并请求

### 基本用法

```bash
yunxiao codeup mr merge --repo-id <REPO_ID> --mr-id <MR_ID> --merge-type <TYPE> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--merge-type` | 合并方式：`ff-only`、`no-fast-forward`、`squash`、`rebase` | 是 |
| `--merge-message` | 合并提交信息 | 否 |
| `--remove-source-branch` | 合并后删除源分支 | 否 |

---

## 评审合并请求

### 基本用法

```bash
yunxiao codeup mr review --repo-id <REPO_ID> --mr-id <MR_ID> --opinion <PASS|NOT_PASS> --comment <CONTENT> --org-id <ORG_ID>
```

`--opinion`、`--comment`、`--draft-comment-id` 至少传入一个。`--draft-comment-id` 可重复传入，也支持逗号分隔。

---

## 关闭和重新打开合并请求

```bash
yunxiao codeup mr close --repo-id <REPO_ID> --mr-id <MR_ID> --org-id <ORG_ID>
yunxiao codeup mr reopen --repo-id <REPO_ID> --mr-id <MR_ID> --org-id <ORG_ID>
```

---

## 更新合并请求干系人

### 基本用法

```bash
yunxiao codeup mr person --repo-id <REPO_ID> --mr-id <MR_ID> --type <REVIEWER|SUBSCRIBER> --user-id <USER_ID> --org-id <ORG_ID>
```

`--user-id` 可重复传入，也支持逗号分隔；兼容 `--user-ids`。

---

## 管理合并请求标签

```bash
yunxiao codeup mr labels list --repo-id <REPO_ID> --mr-id <MR_ID> --org-id <ORG_ID>
yunxiao codeup mr labels attach --repo-id <REPO_ID> --mr-id <MR_ID> --label-id <LABEL_ID> --org-id <ORG_ID>
```

`--label-id` 可重复传入，也支持逗号分隔；兼容 `--label-ids`。

---

## 查看合并请求变更文件树

```bash
yunxiao codeup mr tree --repo-id <REPO_ID> --mr-id <MR_ID> --from-patchset <BASE_PATCHSET_ID> --to-patchset <HEAD_PATCHSET_ID> --org-id <ORG_ID>
```

---

## 管理合并请求评论

```bash
yunxiao codeup mr comments list --repo-id <REPO_ID> --mr-id <MR_ID> --org-id <ORG_ID>
yunxiao codeup mr comments create --repo-id <REPO_ID> --mr-id <MR_ID> --content <CONTENT> --org-id <ORG_ID>
yunxiao codeup mr comments update --repo-id <REPO_ID> --mr-id <MR_ID> --comment-id <COMMENT_ID> --content <CONTENT> --resolved true --org-id <ORG_ID>
yunxiao codeup mr comments delete --repo-id <REPO_ID> --mr-id <MR_ID> --comment-id <COMMENT_ID> --org-id <ORG_ID>
```

`comments update` 的 `--content` 和 `--resolved` 至少传入一个。

---

## 查看文件列表

### 基本用法

```bash
yunxiao codeup files list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--ref` | 分支/提交引用 | 否（默认主分支） |
| `--path` | 目录路径 | 否（默认根目录） |

### 示例

```bash
# 查看根目录文件
yunxiao codeup files list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx --output json

# 查看特定目录
yunxiao codeup files list --repo-id repo-xxxxxxxx --path src --org-id org-xxxxxxxx --output json

# 查看特定分支
yunxiao codeup files list --repo-id repo-xxxxxxxx --ref develop --org-id org-xxxxxxxx --output json
```

---

## 查看文件内容

### 基本用法

```bash
yunxiao codeup files get --repo-id <REPO_ID> --path <path> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--path` | 文件路径 | 是 |
| `--ref` | 分支/提交引用 | 否（默认主分支） |

### 示例

```bash
# 查看文件内容
yunxiao codeup files get --repo-id repo-xxxxxxxx --path src/main.rs --org-id org-xxxxxxxx --output json

# 查看特定分支的文件
yunxiao codeup files get --repo-id repo-xxxxxxxx --path README.md --ref develop --org-id org-xxxxxxxx --output json
```

---

## 创建新文件

### 基本用法

```bash
yunxiao codeup files create --repo-id <REPO_ID> --path <path> --content <content> --branch <branch> --message <message> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--path` | 文件路径 | 是 |
| `--content` | 文件内容 | 是 |
| `--branch` | 目标分支 | 是 |
| `--message` | 提交信息 | 是 |

### 示例

```bash
yunxiao codeup files create --repo-id repo-xxxxxxxx --path docs/README.md \
    --content "# 说明文档\n\n这是项目说明。" \
    --branch main --message "添加说明文档" \
    --org-id org-xxxxxxxx
```

---

## 更新文件内容

### 基本用法

```bash
yunxiao codeup files update --repo-id <REPO_ID> --path <path> --content <content> --branch <branch> --message <message> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--path` | 文件路径 | 是 |
| `--content` | 新文件内容 | 是 |
| `--branch` | 目标分支 | 是 |
| `--message` | 提交信息 | 是 |

### 示例

```bash
yunxiao codeup files update --repo-id repo-xxxxxxxx --path README.md \
    --content "# 项目名\n\n更新后的说明。" \
    --branch main --message "更新 README" \
    --org-id org-xxxxxxxx
```

---

## 删除文件

### 基本用法

```bash
yunxiao codeup files delete --repo-id <REPO_ID> --path <path> --branch <branch> --message <message> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |
| `--path` | 文件路径 | 是 |
| `--branch` | 目标分支 | 是 |
| `--message` | 提交信息 | 是 |

### 示例

```bash
yunxiao codeup files delete --repo-id repo-xxxxxxxx --path docs/old.md \
    --branch main --message "删除旧文档" \
    --org-id org-xxxxxxxx
```

---

## 常见用法

### 创建分支并发起 MR

```bash
# 1. 列出仓库
yunxiao codeup repos list --org-id org-xxx --output json

# 2. 创建分支
yunxiao codeup branches create --repo-id repo-xxx --branch feature/new --ref main --org-id org-xxx

# 3. 创建合并请求
yunxiao codeup mr create --repo-id repo-xxx --source feature/new --target main --title "新功能" --org-id org-xxx
```

### 查看项目代码结构

```bash
# 查看仓库列表
yunxiao codeup repos list --org-id org-xxx --output json

# 选择仓库
REPO_ID=$(yunxiao codeup repos list --org-id org-xxx --output json | jq -r '.[0].id')

# 查看分支
yunxiao codeup branches list --repo-id $REPO_ID --org-id org-xxx --output json

# 查看提交历史
yunxiao codeup commits list --repo-id $REPO_ID --org-id org-xxx --output json

# 查看文件结构
yunxiao codeup files list --repo-id $REPO_ID --org-id org-xxx --output json

# 查看文件内容
yunxiao codeup files get --repo-id $REPO_ID --path README.md --org-id org-xxx --output json
```

### 查看代码变更

```bash
# 列出合并请求
yunxiao codeup mr list --repo-id repo-xxx --org-id org-xxx --output json

# 查看特定分支的文件
yunxiao codeup files get --repo-id repo-xxx --path src/main.rs --ref feature/new --org-id org-xxx --output json

# 比较两个分支
yunxiao codeup compare --repo-id repo-xxx --from main --to feature/new --org-id org-xxx --output json
```

---

## 故障排查

### "Repository not found"

**原因**: 仓库 ID 错误或无权限

**解决方案**:
```bash
# 搜索仓库
yunxiao codeup repos list --org-id org-xxx --output json
```

### "Branch not found"

**原因**: 分支名称错误

**解决方案**:
```bash
# 列出所有分支
yunxiao codeup branches list --repo-id repo-xxx --org-id org-xxx --output json
```

### "File not found"

**原因**: 文件路径错误或分支引用错误

**解决方案**:
```bash
# 查看文件列表
yunxiao codeup files list --repo-id repo-xxx --org-id org-xxx --output json

# 确认分支引用
yunxiao codeup files list --repo-id repo-xxx --ref main --org-id org-xxx --output json
```

### "Commit not found"

**原因**: 提交 SHA 错误

**解决方案**:
```bash
# 列出提交记录
yunxiao codeup commits list --repo-id repo-xxx --org-id org-xxx --output json
```