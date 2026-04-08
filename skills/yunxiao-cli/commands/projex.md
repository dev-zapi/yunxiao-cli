# 项目协作命令手册

`yunxiao projex` 命令用于管理项目、工作项、迭代、版本和标签。

---

## 命令列表

### 项目管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex projects search` | 搜索项目 |
| `yunxiao projex projects get` | 查看项目详情 |

### 工作项管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex workitems search` | 搜索工作项 |
| `yunxiao projex workitems get` | 查看工作项详情 |
| `yunxiao projex workitems create` | 创建工作项 |
| `yunxiao projex workitems update` | 更新工作项 |
| `yunxiao projex workitems types` | 列出工作项类型 |
| `yunxiao projex workitems fields` | 查看字段配置 |

### 迭代管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex sprints list` | 列出迭代 |
| `yunxiao projex sprints create` | 创建迭代 |

### 版本管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex versions list` | 列出版本 |

### 标签管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex labels list` | 列出标签 |
| `yunxiao projex labels create` | 创建标签 |
| `yunxiao projex labels update` | 更新标签 |

---

## 搜索项目

### 基本用法

```bash
yunxiao projex projects search --keyword <keyword> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--keyword` | 搜索关键词 | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
# 搜索所有项目
yunxiao projex projects search --org-id org-xxxxxxxx

# 搜索特定项目
yunxiao projex projects search --keyword "demo" --org-id org-xxxxxxxx
```

---

## 查看项目详情

### 基本用法

```bash
yunxiao projex projects get --project-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--project-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao projex projects get --project-id proj-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 搜索工作项

### 基本用法

```bash
yunxiao projex workitems search --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID（spaceId） | 是 |
| `--category` | 工作项类别 | 否 |

### 工作项类别

| 类别 | 说明 |
|------|------|
| `Req` | 需求 |
| `Task` | 任务 |
| `Bug` | 缺陷 |

### 示例

```bash
# 查看所有工作项
yunxiao projex workitems search --space-id proj-xxxxxxxx --org-id org-xxxxxxxx

# 查看需求
yunxiao projex workitems search --space-id proj-xxxxxxxx --category Req --org-id org-xxxxxxxx

# 查看任务
yunxiao projex workitems search --space-id proj-xxxxxxxx --category Task --org-id org-xxxxxxxx

# 查看缺陷
yunxiao projex workitems search --space-id proj-xxxxxxxx --category Bug --org-id org-xxxxxxxx
```

---

## 查看工作项详情

### 基本用法

```bash
yunxiao projex workitems get --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |

### 示例

```bash
yunxiao projex workitems get --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 创建工作项

### 基本用法

```bash
yunxiao projex workitems create --space-id <PROJECT_ID> --category <category> --subject <subject> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--category` | 工作项类别（Req/Task/Bug） | 是 |
| `--subject` | 标题 | 是 |
| `--assignee` | 负责人用户 ID | 否 |
| `--sprint-id` | 迭代 ID | 否 |
| `--priority` | 优先级 ID | 否 |
| `--description` | 描述内容（直接输入） | 否 |
| `--description-file` | 描述文件路径（从文件读取） | 否 |
| `--description-format` | 描述格式：text（富文本）或 markdown（默认 markdown） | 否 |
| `--field` | 动态字段，格式 `fieldId=value`，可多次使用 | 否 |

### 描述参数说明

- `--description`：直接输入描述文本，优先级高于 `--description-file`
- `--description-file`：从指定文件读取描述内容
- `--description-format`：
  - `markdown`（默认）：Markdown 格式，对应 API 的 `MARKDOWN`
  - `text`：富文本格式，对应 API 的 `RICHTEXT`

### 示例

```bash
# 创建需求（基本）
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Req --subject "用户登录功能" --org-id org-xxxxxxxx

# 创建带描述的需求（Markdown 格式）
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Req --subject "用户登录功能" \
  --description "## 功能说明\n\n- 支持用户名密码登录\n- 支持手机号登录" \
  --org-id org-xxxxxxxx

# 从文件读取描述
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Req --subject "API 设计文档" \
  --description-file ./api-design.md \
  --org-id org-xxxxxxxx

# 使用富文本格式描述
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Task --subject "修复样式问题" \
  --description "<p>这是一个富文本描述</p>" \
  --description-format text \
  --org-id org-xxxxxxxx

# 创建任务
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Task --subject "编写单元测试" --org-id org-xxxxxxxx

# 创建缺陷
yunxiao projex workitems create --space-id proj-xxxxxxxx --category Bug --subject "登录页面加载缓慢" --org-id org-xxxxxxxx
```

---

## 更新工作项

### 基本用法

```bash
yunxiao projex workitems update --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |
| `--subject` | 新标题 | 否 |
| `--assignee` | 新负责人用户 ID | 否 |
| `--status` | 新状态 ID | 否 |
| `--priority` | 新优先级 ID | 否 |
| `--description` | 新描述内容（直接输入） | 否 |
| `--description-file` | 新描述文件路径 | 否 |
| `--description-format` | 新描述格式：text 或 markdown | 否 |
| `--field` | 动态字段，格式 `fieldId=value`，可多次使用 | 否 |

### 示例

```bash
# 更新标题
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --subject "更新后的标题" --org-id org-xxxxxxxx

# 更新描述（Markdown 格式）
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --description "## 更新内容\n\n- 新增功能 A\n- 修复问题 B" \
  --description-format markdown \
  --org-id org-xxxxxxxx

# 从文件更新描述
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --description-file ./new-description.md \
  --org-id org-xxxxxxxx

# 更新状态和负责人
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --status "status-xxx" --assignee "user-xxx" --org-id org-xxxxxxxx
```

---

## 列出工作项类型

### 基本用法

```bash
yunxiao projex workitems types --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--category` | 过滤类别 | 否 |
| `--keyword` | 名称关键词过滤 | 否 |

### 示例

```bash
# 列出所有工作项类型（使用组织级 API）
yunxiao projex workitems types --space-id proj-xxxxxxxx --org-id org-xxxxxxxx

# 按类别过滤
yunxiao projex workitems types --space-id proj-xxxxxxxx --category Req --org-id org-xxxxxxxx

# 按名称关键词过滤（本地过滤，不区分大小写）
yunxiao projex workitems types --space-id proj-xxxxxxxx --keyword "需求" --org-id org-xxxxxxxx
```

---

## 查看工作项字段配置

### 基本用法

```bash
yunxiao projex workitems fields --project-id <PROJECT_ID> --type-id <TYPE_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--project-id` | 项目 ID | 是 |
| `--type-id` | 工作项类型 ID | 是 |

### 示例

```bash
yunxiao projex workitems fields --project-id proj-xxxxxxxx --type-id type-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 列出迭代

### 基本用法

```bash
yunxiao projex sprints list --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao projex sprints list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 创建迭代

### 基本用法

```bash
yunxiao projex sprints create --space-id <PROJECT_ID> --name <name> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--name` | 迭代名称 | 是 |

### 示例

```bash
yunxiao projex sprints create --space-id proj-xxxxxxxx --name "迭代1" --org-id org-xxxxxxxx
```

---

## 列出版本

### 基本用法

```bash
yunxiao projex versions list --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao projex versions list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 列出标签

### 基本用法

```bash
yunxiao projex labels list --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--keyword` | 名称关键词过滤 | 否 |

### 示例

```bash
# 列出所有标签
yunxiao projex labels list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx

# 按关键词过滤（不区分大小写）
yunxiao projex labels list --space-id proj-xxxxxxxx --keyword "bug" --org-id org-xxxxxxxx
```

---

## 创建标签

### 基本用法

```bash
yunxiao projex labels create --space-id <PROJECT_ID> --name <name> --color <color> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--name` | 标签名称 | 是 |
| `--color` | 颜色（格式 #RRGGBB） | 是 |

### 示例

```bash
yunxiao projex labels create --space-id proj-xxxxxxxx --name "bug" --color "#FF0000" --org-id org-xxxxxxxx
yunxiao projex labels create --space-id proj-xxxxxxxx --name "feature" --color "#00FF00" --org-id org-xxxxxxxx
yunxiao projex labels create --space-id proj-xxxxxxxx --name "urgent" --color "#FFA500" --org-id org-xxxxxxxx
```

---

## 更新标签

### 基本用法

```bash
yunxiao projex labels update --space-id <PROJECT_ID> --label-id <LABEL_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--label-id` | 标签 ID | 是 |
| `--name` | 新名称 | 否 |
| `--color` | 新颜色 | 否 |

### 示例

```bash
# 更新名称
yunxiao projex labels update --space-id proj-xxxxxxxx --label-id label-xxxxxxxx --name "缺陷" --org-id org-xxxxxxxx

# 更新颜色
yunxiao projex labels update --space-id proj-xxxxxxxx --label-id label-xxxxxxxx --color "#00FF00" --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看项目工作项概览

```bash
# 搜索项目
yunxiao projex projects search --keyword "demo" --org-id org-xxx

# 获取项目 ID
PROJECT_ID=$(yunxiao projex projects search --keyword "demo" --org-id org-xxx --output json | jq -r '.[0].id')

# 查看需求
yunxiao projex workitems search --space-id $PROJECT_ID --category Req --org-id org-xxx

# 查看缺陷
yunxiao projex workitems search --space-id $PROJECT_ID --category Bug --org-id org-xxx
```

### 创建完整工作项流程

```bash
# 创建需求（带描述）
yunxiao projex workitems create --space-id proj-xxx --category Req --subject "新功能需求" \
  --description "## 需求说明\n\n- 功能点 1\n- 功能点 2" \
  --org-id org-xxx

# 创建迭代
yunxiao projex sprints create --space-id proj-xxx --name "迭代1" --org-id org-xxx

# 创建标签
yunxiao projex labels create --space-id proj-xxx --name "高优先级" --color "#FF0000" --org-id org-xxx

# 更新工作项描述（从文件读取）
yunxiao projex workitems update --space-id proj-xxx --workitem-id wi-xxx \
  --description-file ./detailed-spec.md \
  --org-id org-xxx
```

### 查看迭代进度

```bash
# 列出迭代
yunxiao projex sprints list --space-id proj-xxx --org-id org-xxx

# 查看迭代中的工作项（需手动过滤）
yunxiao projex workitems search --space-id proj-xxx --org-id org-xxx
```

---

## 故障排查

### "Project not found"

**原因**: 项目 ID 错误或无权限

**解决方案**:
```bash
# 搜索项目确认 ID
yunxiao projex projects search --keyword "项目名" --org-id org-xxx
```

### "Space ID required"

**原因**: 工作项命令需要 `--space-id`（项目 ID）

**解决方案**:
```bash
# 获取项目 ID
yunxiao projex projects search --org-id org-xxx

# 使用项目 ID
yunxiao projex workitems search --space-id proj-xxx --org-id org-xxx
```

### "Invalid category"

**原因**: 类别参数错误

**解决方案**:
使用正确的类别值：`Req`, `Task`, `Bug`