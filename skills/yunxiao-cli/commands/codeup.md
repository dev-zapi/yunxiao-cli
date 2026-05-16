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
