# 代码管理命令手册

`yunxiao codeup` 命令用于管理代码仓库、分支、合并请求和文件。

---

## 命令列表

### 仓库管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup repos list` | 列出代码仓库 |
| `yunxiao codeup repos get` | 查看仓库详情 |

### 分支管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup branches list` | 列出分支 |
| `yunxiao codeup branches create` | 创建分支 |

### 合并请求

| 命令 | 说明 |
|------|------|
| `yunxiao codeup mr list` | 列出合并请求 |
| `yunxiao codeup mr create` | 创建合并请求 |

### 文件管理

| 命令 | 说明 |
|------|------|
| `yunxiao codeup files list` | 查看文件列表 |
| `yunxiao codeup files get` | 查看文件内容 |

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

### 示例

```bash
yunxiao codeup repos list --org-id org-xxxxxxxx
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
| `--repo-id` | 仓库 ID | 是 |

### 示例

```bash
yunxiao codeup repos get --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx
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

### 示例

```bash
yunxiao codeup branches list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx
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
```

---

## 列出合并请求

### 基本用法

```bash
yunxiao codeup mr list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 仓库 ID | 是 |

### 示例

```bash
yunxiao codeup mr list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx
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
yunxiao codeup files list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx

# 查看特定目录
yunxiao codeup files list --repo-id repo-xxxxxxxx --path src --org-id org-xxxxxxxx

# 查看特定分支
yunxiao codeup files list --repo-id repo-xxxxxxxx --ref develop --org-id org-xxxxxxxx
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
yunxiao codeup files get --repo-id repo-xxxxxxxx --path src/main.rs --org-id org-xxxxxxxx

# 查看特定分支的文件
yunxiao codeup files get --repo-id repo-xxxxxxxx --path README.md --ref develop --org-id org-xxxxxxxx
```

---

## 常见用法

### 创建分支并发起 MR

```bash
# 1. 列出仓库
yunxiao codeup repos list --org-id org-xxx

# 2. 创建分支
yunxiao codeup branches create --repo-id repo-xxx --branch feature/new --ref main --org-id org-xxx

# 3. 创建合并请求
yunxiao codeup mr create --repo-id repo-xxx --source feature/new --target main --title "新功能" --org-id org-xxx
```

### 查看项目代码结构

```bash
# 查看仓库列表
yunxiao codeup repos list --org-id org-xxx

# 选择仓库
REPO_ID=$(yunxiao codeup repos list --org-id org-xxx --output json | jq -r '.[0].id')

# 查看分支
yunxiao codeup branches list --repo-id $REPO_ID --org-id org-xxx

# 查看文件结构
yunxiao codeup files list --repo-id $REPO_ID --org-id org-xxx

# 查看文件内容
yunxiao codeup files get --repo-id $REPO_ID --path README.md --org-id org-xxx
```

### 查看代码变更

```bash
# 列出合并请求
yunxiao codeup mr list --repo-id repo-xxx --org-id org-xxx

# 查看特定分支的文件
yunxiao codeup files get --repo-id repo-xxx --path src/main.rs --ref feature/new --org-id org-xxx
```

---

## 故障排查

### "Repository not found"

**原因**: 仓库 ID 错误或无权限

**解决方案**:
```bash
# 搜索仓库
yunxiao codeup repos list --org-id org-xxx
```

### "Branch not found"

**原因**: 分支名称错误

**解决方案**:
```bash
# 列出所有分支
yunxiao codeup branches list --repo-id repo-xxx --org-id org-xxx
```

### "File not found"

**原因**: 文件路径错误或分支引用错误

**解决方案**:
```bash
# 查看文件列表
yunxiao codeup files list --repo-id repo-xxx --org-id org-xxx

# 确认分支引用
yunxiao codeup files list --repo-id repo-xxx --ref main --org-id org-xxx
```
