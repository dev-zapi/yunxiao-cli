# 项目协作命令手册

**文档版本**: v2.1  
**对应 CLI 版本**: v0.5.0+  
**更新日期**: 2025-05-10  
**适用范围**: Agent 编程与自动化脚本

`yunxiao projex` 命令用于管理项目、项目集、工作项、迭代、版本、工时和标签。

---

## Agent 快速参考

编写 Agent 调用 `yunxiao projex` 时的关键规则：

### 关键注意事项

**参数风格差异**：不同命令的 ID 参数风格不同，这是云效 API 的设计差异：
- `projects get`: 使用**位置参数** `yunxiao projex projects get <PROJECT_ID>`
- `workitems get`: 使用**flag 参数** `yunxiao projex workitems get --workitem-id <ID>`
- `sprints get`: 使用**flag 参数** `yunxiao projex sprints get --sprint-id <ID>`
- `versions get`: 使用**flag 参数** `--version-id <ID>`

Agent 必须仔细区分这些差异，避免参数传递错误。

### ID 链路查询流程

Agent 查询数据时需要按以下链路逐步获取 ID:

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

**获取顺序示例**:
1. 搜索项目 → 获取 `space_id`
2. 查询工作项类型 → 获取 `type_id`
3. 查询工作流状态 → 获取 `status_id`(用于更新状态)
4. 查询字段配置 → 获取 `fieldIdentifier`(用于动态字段)

### 编写 Agent 调用规则

1. **Agent 场景**：始终使用 `--output json`，这是 Agent 解析结果的唯一可靠方式。
   - JSON 输出格式稳定、结构化，便于 jq 或代码解析
   - table/plain 输出格式可能变化，不适合程序解析
   
2. **人类使用**：可省略 `--output json`，默认 table 格式更易读。

3. **`--org-id`** 是全局标志，几乎每条命令都需要。
3. **列表命令使用 `--output json` 获取数组**，然后用 `jq` 或代码提取 ID。
4. **ID 链规律**：`projects search` → `space_id` → `workitems search` → `workitem_id`。
5. **创建/更新前先查字段配置**：`workitems fields` 获取可用字段 ID，`workitems flow` 获取可用状态。

### 核心 ID 获取速查

| 需要 ID | 使用命令 |
|--------|---------|
| `project_id` (space_id) | `projects search --keyword <kw>` |
| `workitem_id` | `workitems search --space-id <SPACE_ID>` |
| `type_id` | `workitems types --space-id <SPACE_ID>` |
| `sprint_id` | `sprints list --space-id <SPACE_ID>` |
| `version_id` | `versions list --space-id <SPACE_ID>` |
| `label_id` | `labels list --space-id <SPACE_ID>` |
| `priority_id` | `workitems fields --project-id <ID> --type-id <TYPE_ID>` |
| `status_id` | `workitems flow --space-id <SPACE_ID> --type-id <TYPE_ID>` |
| `user_id` (assignee/owner) | `yunxiao org members list --org-id <ORG_ID>` |

---

## 命令列表

### 项目管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex projects search` | 搜索项目 |
| `yunxiao projex projects list` | 列出所有项目（支持过滤/排序） |
| `yunxiao projex projects get` | 查看项目详情 |

### 项目集管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex programs search` | 搜索项目集 |

### 工作项管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex workitems search` | 搜索工作项 |
| `yunxiao projex workitems get` | 查看工作项详情 |
| `yunxiao projex workitems create` | 创建工作项 |
| `yunxiao projex workitems update` | 更新工作项 |
| `yunxiao projex workitems types` | 列出工作项类型 |
| `yunxiao projex workitems fields` | 查看字段配置 |
| `yunxiao projex workitems flow` | 查看工作流信息 |
| `yunxiao projex workitems comments list` | 列出工作项评论 |
| `yunxiao projex workitems comments create` | 添加工作项评论 |
| `yunxiao projex workitems attachments list` | 列出工作项附件 |
| `yunxiao projex workitems relations list` | 列出工作项关联项 |
| `yunxiao projex workitems relations create` | 创建工作项关联 |
| `yunxiao projex workitems relations delete` | 删除工作项关联 |

### 迭代管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex sprints list` | 列出迭代 |
| `yunxiao projex sprints get` | 查看迭代详情 |
| `yunxiao projex sprints create` | 创建迭代 |
| `yunxiao projex sprints update` | 更新迭代 |

### 版本管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex versions list` | 列出版本 |
| `yunxiao projex versions create` | 创建版本 |
| `yunxiao projex versions update` | 更新版本 |
| `yunxiao projex versions delete` | 删除版本 |

### 工时管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex efforts list` | 列出工时记录 |
| `yunxiao projex efforts create` | 创建工时记录 |

### 标签管理

| 命令 | 说明 |
|------|------|
| `yunxiao projex labels list` | 列出标签 |
| `yunxiao projex labels create` | 创建标签 |
| `yunxiao projex labels update` | 更新标签 |

---

## 未实现的命令

云效 API 支持但 CLI 暂未实现的功能：

### 项目管理未实现

- `projects create`: 创建项目
- `projects update`: 更新项目
- `projects delete`: 删除项目
- `projects members list`: 项目成员管理
- `projects members update`: 更新项目成员

### 工作项管理未实现

- `workitems delete`: 删除工作项
- `workitems attachments create`: 上传附件
- `workitems comments delete`: 删除评论
- `workitems comments update`: 更新评论

### 迭代管理未实现

- `sprints delete`: 删除迭代

### 标签管理未实现

- `labels delete`: 删除标签

### 工时管理未实现

- `efforts update`: 更新工时记录
- `efforts delete`: 删除工时记录

**替代方案**: 这些功能可通过云效 Web 界面操作。未来版本将逐步补充这些命令。

---

## 响应字段参考

Agent 解析 JSON 输出时常用的顶层字段：

### projects search / list
```
id, name, identifier, description, logicalStatus, gmtCreate, creator (object), scope
```

### workitems search
```
id, identifier (编号如 PROJ-123), subject, status (object), assignedTo (object), 
sprint (list), spaceId, categoryId, gmtCreate
```

### workitems get
```
完整工作项对象。customFieldValues 数组包含优先级等自定义字段。
推送字段可能为 null，仅在有值时返回。
```

### sprints list
```
id, name, startDate, endDate, status, capacity (hours), owner (list)
```

### versions list
```
id, name, status, description, gmtCreate, gmtModified
```

### labels list
```
id, name, color (hex), spaceId, gmtCreate
```

### workitems types
```
id, name, identifier, categoryId, gmtCreate
```

### workitems fields
```
数组。每个元素包含:
- id: 字段唯一标识
- fieldIdentifier: 字段标识符(用于条件组查询和 --field 参数)
- name: 字段名称
- format: 字段格式(input/list/multiList/date/number等)
- className: 字段类名(string/user/date等)
- required: 是否必需(bool)
- type: 字段类型

注意: fieldIdentifier 是 Agent 构造查询条件的关键字段。
```

### workitems flow
```
id, name, defaultStatusId, statuses (数组: id, name, nameEn, displayName)
```

### efforts list
```
id, workitemId, duration (hours), description, createdAt
```

### programs search
```
id, name, identifier, description, gmtCreate
```

### 分页响应 headers (search/list 命令)

列表类命令(如 projects search/list、workitems search)的响应包含分页信息 headers:

```
x-total: 总数量
x-page: 当前页码
x-per-page: 每页数量
x-total-pages: 总页数
```

**注意事项**:
- 部分 API 返回完整数组而非分页对象，需根据 headers 判断分页情况
- CLI 的 `list` 命令会自动聚合多页数据
- Agent 解析时应先检查 headers 判断是否需要继续分页请求

### API 错误响应

当 API 请求失败时,返回错误响应:

```json
{
  "errorCode": "InvalidParameter",
  "errorMessage": "工作项类型id不能为空",
  "success": false
}
```

**常见错误码**:
- `InvalidParameter`: 参数错误
- `Unauthorized`: 认证失败(Token 无效或过期)
- `Forbidden`: 权限不足
- `NotFound`: 资源不存在
- `InternalError`: 服务端错误

---

## 搜索项目

### 基本用法

```bash
yunxiao projex projects search --keyword <keyword> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--keyword` | 搜索关键词 | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |
| `--order-by` | 排序字段：`gmtCreate`（默认）、`name` | 否 |
| `--sort` | 排序方向：`desc`（默认）、`asc` | 否 |
| `--status` | 按逻辑状态过滤（NORMAL, ARCHIVED, DELETED） | 否 |
| `--creator` | 按创建者用户 ID 过滤 | 否 |
| `--admin` | 按管理员用户 ID 过滤 | 否 |

### 示例

```bash
# 搜索所有项目
yunxiao projex projects search --org-id org-xxxxxxxx --output json

# 搜索特定项目
yunxiao projex projects search --keyword "demo" --org-id org-xxxxxxxx --output json

# 按状态过滤
yunxiao projex projects search --keyword "demo" --status NORMAL --org-id org-xxxxxxxx --output json

# 按创建时间升序
yunxiao projex projects search --org-id org-xxxxxxxx --order-by gmtCreate --sort asc --output json
```

---

## 列出所有项目

`projects list` 是 `projects search` 的替代方案，无需关键词即可列出项目。

### 基本用法

```bash
yunxiao projex projects list --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |
| `--order-by` | 排序字段：`gmtCreate`（默认）、`name` | 否 |
| `--sort` | 排序方向：`desc`（默认）、`asc` | 否 |
| `--status` | 按逻辑状态过滤（NORMAL, ARCHIVED, DELETED） | 否 |
| `--creator` | 按创建者用户 ID 过滤 | 否 |
| `--admin` | 按管理员用户 ID 过滤 | 否 |
| `--scope` | 附加条件：`managed`（管理的）、`joined`（参与的）、`starred`（收藏的） | 否 |

### 示例

```bash
# 列出所有项目
yunxiao projex projects list --org-id org-xxxxxxxx --output json

# 列出我参与的项目
yunxiao projex projects list --org-id org-xxxxxxxx --scope joined --output json
```

---

## 查看项目详情

### 基本用法

```bash
yunxiao projex projects get <PROJECT_ID> --org-id <ORG_ID> --output json
```

> **注意**：`<PROJECT_ID>` 是位置参数（不需要 `--project-id`），直接跟在 `get` 后面。

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `id` | 项目 ID（位置参数） | 是 |

### 示例

```bash
yunxiao projex projects get proj-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 搜索项目集

### 基本用法

```bash
yunxiao projex programs search --keyword <keyword> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--keyword` | 搜索关键词 | 否 |

### 示例

```bash
yunxiao projex programs search --org-id org-xxxxxxxx --output json
yunxiao projex programs search --keyword "核心" --org-id org-xxxxxxxx --output json
```

---

## 搜索工作项

### 基本用法

```bash
yunxiao projex workitems search --space-id <PROJECT_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID（spaceId） | 是 |
| `-c`, `--category` | 工作项类别，可多次传入；省略时默认搜索 `Req`、`Task`、`Bug` | 否 |
| `-k`, `--keyword` | 标题关键词过滤 | 否 |
| `-n`, `--serial-number` | 按编号过滤（如 PROJ-123） | 否 |
| `-v`, `--version-id` | 按版本 ID 过滤 | 否 |
| `-S`, `--sprint-id` | 按迭代 ID 过滤 | 否 |
| `-p`, `--page` | 页码 | 否（默认 1） |
| `-P`, `--page-size` | 每页数量 | 否（默认 20） |

### 工作项类别

| 类别 | 说明 |
|------|------|
| `Req` | 需求 |
| `Task` | 任务 |
| `Bug` | 缺陷 |

支持多次传入，例如：`-c Req -c Task`。

**CLI 默认行为**：当省略 `--category` 参数时，CLI 会自动传递 `Req`、`Task`、`Bug` 三个类别给搜索接口。这是 CLI 层面的默认值，并非云效 API 的默认行为。API 本身需要明确的类别参数。

### 示例

```bash
# 查看所有工作项
yunxiao projex workitems search --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json

# 查看需求
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req --org-id org-xxxxxxxx --output json

# 查看任务
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Task --org-id org-xxxxxxxx --output json

# 查看缺陷
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Bug --org-id org-xxxxxxxx --output json

# 同时查看需求和任务
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req -c Task --org-id org-xxxxxxxx --output json

# 按关键词搜索
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req -k "登录" --org-id org-xxxxxxxx --output json

# 按编号查找
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req -n PROJ-123 --org-id org-xxxxxxxx --output json

# 按版本过滤（version-id 通过 versions list 获取）
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req -v 6e7f811c... --org-id org-xxxxxxxx --output json

# 按迭代过滤（sprint-id 通过 sprints list 获取）
yunxiao projex workitems search --space-id proj-xxxxxxxx -c Req -S sprint-xxx --org-id org-xxxxxxxx --output json
```

---

## 查看工作项详情

### 基本用法

```bash
yunxiao projex workitems get --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |

### 示例

```bash
yunxiao projex workitems get --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 创建工作项

### 基本用法

```bash
yunxiao projex workitems create --space-id <PROJECT_ID> --type-id <TYPE_ID> --subject <subject> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--type-id` | 工作项类型 ID。通过 `yunxiao projex workitems types --space-id <SPACE_ID>` 获取 | 是 |
| `--subject` | 标题 | 是 |
| `--assignee` | 负责人用户 ID | 否 |
| `--sprint-id` | 迭代 ID | 否 |
| `--priority` | 优先级 ID | 否 |
| `--labels` | 标签 ID 列表（逗号分隔）。通过 `yunxiao projex labels list --space-id <SPACE_ID>` 获取 | 否 |
| `--description` | 描述内容（直接输入） | 否 |
| `--description-file` | 描述文件路径（从文件读取） | 否 |
| `--description-format` | 描述格式：text（富文本）或 markdown（默认 markdown） | 否 |
| `--field` | 动态字段，格式 `fieldId=value`，可多次使用 | 否 |

### 动态字段值格式规则

`--field` 参数支持不同格式类型的字段，值格式如下：

| format 类型 | className   | 值格式          | 示例                          | 说明                         |
|-------------|-------------|-----------------|-------------------------------|------------------------------|
| input       | string      | 直接文本        | `--field "customNote=备注内容"` | 直接输入文本值               |
| list        | user        | 单个用户 ID     | `--field "assignedTo=user-123"` | 传用户 ID(通过 org members 获取) |
| list        | status      | 单个状态 ID     | `--field "status=status-xxx"`  | 传状态 ID(通过 workitems flow 获取) |
| multiList   | user        | 逗号分隔用户 ID | `--field "participants=user1,user2"` | 多个用户 ID 用逗号分隔       |
| multiList   | 其他        | 逗号分隔多个 ID | `--field "labels=label1,label2"` | 多个选项 ID 用逗号分隔       |
| date        | date        | YYYY-MM-DD      | `--field "dueDate=2025-03-15"` | 日期格式必须为 YYYY-MM-DD    |
| number      | number      | 数值            | `--field "estimate=8"`         | 直接传数值                   |

**获取字段 ID 和格式**：
```bash
# 查看字段配置，获取 fieldIdentifier 和 format
yunxiao projex workitems fields --project-id proj-xxx --type-id type-xxx --org-id org-xxx --output json
```

### 描述参数说明

- `--description`：直接输入描述文本，优先级高于 `--description-file`
- `--description-file`：从指定文件读取描述内容
- `--description-format`：
  - `markdown`（默认）：Markdown 格式
  - `text`：富文本格式

### 示例

```bash
# 创建需求（基本）
# 先获取工作项类型 ID
yunxiao projex workitems types --space-id proj-xxxxxxxx --category Req --org-id org-xxxxxxxx --output json
# 使用返回的类型 ID 创建工作项
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx --subject "用户登录功能" --org-id org-xxxxxxxx

# 创建带描述的需求（Markdown 格式）
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx --subject "用户登录功能" \
  --description "## 功能说明\n\n- 支持用户名密码登录\n- 支持手机号登录" \
  --org-id org-xxxxxxxx

# 从文件读取描述
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx --subject "API 设计文档" \
  --description-file ./api-design.md \
  --org-id org-xxxxxxxx

# 使用富文本格式描述
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx --subject "修复样式问题" \
  --description "<p>这是一个富文本描述</p>" \
  --description-format text \
  --org-id org-xxxxxxxx

# 创建带优先级的缺陷
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id bug-type-id \
  --subject "登录页面报错" \
  --priority "1025f7ffdb587024db6a3e845b" \
  --org-id org-xxxxxxxx

# 创建带多个自定义字段的工作项
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx \
  --subject "测试工作项" \
  --priority "priority-id" \
  --field "seriousLevel=serious-id" \
  --field "customNote=这是备注" \
  --org-id org-xxxxxxxx

# 创建带标签的工作项（标签 ID 逗号分隔）
# 先获取标签 ID
yunxiao projex labels list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json
# 创建时关联标签
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx \
  --subject "高优先级任务" \
  --labels "label-id-1,label-id-2" \
  --org-id org-xxxxxxxx

# 通过 --field 参数设置标签（效果与 --labels 相同）
yunxiao projex workitems create --space-id proj-xxxxxxxx --type-id type-xxxxxxxx \
  --subject "重要功能" \
  --field "labels=label-id-1,label-id-2" \
  --org-id org-xxxxxxxx
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
| `--type-id` | 工作项类型 ID（可选，用于字段校验）。通过 `yunxiao projex workitems get` 获取 | 否 |
| `--subject` | 新标题 | 否 |
| `--assignee` | 新负责人用户 ID | 否 |
| `--status` | 新状态 ID | 否 |
| `--priority` | 新优先级 ID | 否 |
| `--labels` | 新标签 ID 列表（逗号分隔）。通过 `yunxiao projex labels list --space-id <SPACE_ID>` 获取 | 否 |
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
  --org-id org-xxxxxxxx

# 从文件更新描述
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --description-file ./new-description.md \
  --org-id org-xxxxxxxx

# 更新状态和负责人
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --status status-xxx --assignee user-xxx --org-id org-xxxxxxxx

# 更新优先级
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --priority new-priority-id --org-id org-xxxxxxxx

# 更新多个自定义字段
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --priority new-priority-id \
  --field "seriousLevel=new-serious-id" \
  --field "customNote=更新后的备注" \
  --org-id org-xxxxxxxx

# 更新工作项标签
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --labels label-id-1,label-id-2,label-id-3 \
  --org-id org-xxxxxxxx

# 通过 --field 更新标签（效果与 --labels 相同）
yunxiao projex workitems update --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --field "labels=label-id-1,label-id-2" \
  --org-id org-xxxxxxxx
```

---

## 列出工作项类型

### 基本用法

```bash
yunxiao projex workitems types --space-id <PROJECT_ID> --org-id <ORG_ID> --output json
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
yunxiao projex workitems types --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json

# 按类别过滤
yunxiao projex workitems types --space-id proj-xxxxxxxx --category Req --org-id org-xxxxxxxx --output json

# 按名称关键词过滤（本地过滤，不区分大小写）
yunxiao projex workitems types --space-id proj-xxxxxxxx --keyword "需求" --org-id org-xxxxxxxx --output json
```

---

## 查看工作项字段配置

### 基本用法

```bash
yunxiao projex workitems fields --project-id <PROJECT_ID> --type-id <TYPE_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--project-id` | 项目 ID | 是 |
| `--type-id` | 工作项类型 ID | 是 |

### 示例

```bash
yunxiao projex workitems fields --project-id proj-xxxxxxxx --type-id type-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 查看工作流信息

### 基本用法

有两种模式可以查询工作流信息：

**模式 A**：通过工作项 ID 获取该工作项的工作流信息

```bash
yunxiao projex workitems flow --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

**模式 B**：通过项目 ID + 工作项类型 ID 获取该类型的工作流状态列表

```bash
yunxiao projex workitems flow --space-id <PROJECT_ID> --type-id <TYPE_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--workitem-id` | 工作项 ID（模式 A） | 模式 A 必需 |
| `--space-id` | 项目 ID（模式 B） | 模式 B 必需 |
| `--type-id` | 工作项类型 ID（模式 B） | 模式 B 必需 |

### 返回字段说明

| 字段 | 说明 |
|------|------|
| `id` | 工作流 ID |
| `name` | 工作流名称 |
| `defaultStatusId` | 默认状态 ID |
| `statuses` | 状态列表数组 |

### statuses 数组元素

| 字段 | 说明 |
|------|------|
| `id` | 状态 ID |
| `name` | 状态名称（中文） |
| `nameEn` | 状态名称（英文） |
| `displayName` | 状态显示名称 |

### 示例

```bash
# 模式 A：获取某个工作项的工作流信息
yunxiao projex workitems flow --workitem-id wi-xxxxxxxx --org-id org-xxxxxxxx --output json

# 模式 B：获取工作项类型的工作流状态列表
# 1. 先查询工作项类型 ID
yunxiao projex workitems types --space-id proj-xxxxxxxx --category Req --org-id org-xxxxxxxx --output json

# 2. 使用类型 ID 查询工作流
yunxiao projex workitems flow --space-id proj-xxxxxxxx --type-id 9uy29901re573f561d69jn40 --org-id org-xxxxxxxx --output json
```

### 典型使用场景

```bash
# 场景：查询产品类需求的工作流状态
# 1. 获取项目 ID
yunxiao projex projects search --keyword "项目名" --org-id org-xxx --output json

# 2. 获取需求类型 ID（产品类需求）
yunxiao projex workitems types --space-id proj-xxx --category Req --org-id org-xxx --output json

# 3. 查询该类型的工作流
yunxiao projex workitems flow --space-id proj-xxx --type-id <TYPE_ID> --org-id org-xxx --output json
```

---

## 工作项评论管理

### 列出评论

```bash
yunxiao projex workitems comments list --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |

#### 示例

```bash
yunxiao projex workitems comments list --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx --org-id org-xxxxxxxx --output json
```

### 添加评论

```bash
yunxiao projex workitems comments create --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --content <CONTENT> --org-id <ORG_ID>
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |
| `--content` | 评论内容 | 是 |

#### 示例

```bash
yunxiao projex workitems comments create --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --content "已确认问题，正在修复中" \
  --org-id org-xxxxxxxx
```

---

## 工作项附件管理

### 列出附件

```bash
yunxiao projex workitems attachments list --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |

#### 示例

```bash
yunxiao projex workitems attachments list --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 工作项关联管理

管理工作项之间的关联关系（父项、子项、关联项、依赖项、支撑项）。

### 关联类型

| 值          | 含义   | 使用场景                  | 示例说明                   |
|-------------|--------|---------------------------|----------------------------|
| `PARENT`    | 父项   | Epic → Story 层级关系     | Epic 是 Story 的父项       |
| `SUB`       | 子项   | Story → Task 任务拆解     | Task 是 Story 的子任务     |
| `ASSOCIATED`| 关联项 | 相关工作项引用            | 相关需求、参考工作项       |
| `DEPEND_ON` | 依赖项 | 依赖其他工作项完成        | 必须先完成 A 才能做 B      |
| `DEPENDED_BY`| 支撑项| 被其他工作项依赖          | A 完成后才能进行 B         |

### 列出关联项

```bash
yunxiao projex workitems relations list --workitem-id <WORKITEM_ID> --relation-type <TYPE> --org-id <ORG_ID> --output json
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |
| `--relation-type` | 关联类型 | 是 |

#### 示例

```bash
# 列出工作项的子项
yunxiao projex workitems relations list --workitem-id wi-xxxxxxxx --relation-type SUB --org-id org-xxxxxxxx --output json

# 列出工作项的关联项
yunxiao projex workitems relations list --workitem-id wi-xxxxxxxx --relation-type ASSOCIATED --org-id org-xxxxxxxx --output json
```

### 创建关联

```bash
yunxiao projex workitems relations create --workitem-id <WORKITEM_ID> --target-workitem-id <TARGET_ID> --relation-type <TYPE> --org-id <ORG_ID>
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--workitem-id` | 源工作项 ID | 是 |
| `--target-workitem-id` | 目标工作项 ID | 是 |
| `--relation-type` | 关联类型 | 是 |

#### 示例

```bash
# 将工作项 B 设为工作项 A 的子项
yunxiao projex workitems relations create --workitem-id wi-A --target-workitem-id wi-B --relation-type SUB --org-id org-xxxxxxxx

# 创建关联关系
yunxiao projex workitems relations create --workitem-id wi-A --target-workitem-id wi-B --relation-type ASSOCIATED --org-id org-xxxxxxxx
```

### 删除关联

```bash
yunxiao projex workitems relations delete --workitem-id <WORKITEM_ID> --target-workitem-id <TARGET_ID> --relation-type <TYPE> --org-id <ORG_ID>
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--workitem-id` | 源工作项 ID | 是 |
| `--target-workitem-id` | 目标工作项 ID | 是 |
| `--relation-type` | 关联类型 | 是 |

#### 示例

```bash
yunxiao projex workitems relations delete --workitem-id wi-A --target-workitem-id wi-B --relation-type SUB --org-id org-xxxxxxxx
```

### 典型使用场景

```bash
# 场景：创建工作项后立即设置父子关系
# 1. 创建子工作项
yunxiao projex workitems create --space-id proj-xxx --type-id <TYPE_ID> --subject "子任务" --org-id org-xxx

# 2. 将新建的工作项关联为父工作项的子项
yunxiao projex workitems relations create --workitem-id <PARENT_ID> --target-workitem-id <NEW_CHILD_ID> --relation-type SUB --org-id org-xxx

# 3. 查看父工作项的所有子项
yunxiao projex workitems relations list --workitem-id <PARENT_ID> --relation-type SUB --org-id org-xxx --output json
```

---

## 列出迭代

### 基本用法

```bash
yunxiao projex sprints list --space-id <PROJECT_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao projex sprints list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 查看迭代详情

### 基本用法

```bash
yunxiao projex sprints get --space-id <PROJECT_ID> --sprint-id <SPRINT_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--sprint-id` | 迭代 ID | 是 |

### 示例

```bash
yunxiao projex sprints get --space-id proj-xxxxxxxx --sprint-id sprint-xxxxxxxx --org-id org-xxxxxxxx --output json
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
| `--start-date` | 开始日期 (YYYY-MM-DD) | 否 |
| `--end-date` | 结束日期 (YYYY-MM-DD) | 否 |

### 示例

```bash
# 基本创建
yunxiao projex sprints create --space-id proj-xxxxxxxx --name "迭代1" --org-id org-xxxxxxxx

# 创建带日期的迭代
yunxiao projex sprints create --space-id proj-xxxxxxxx --name "迭代1" \
  --start-date "2025-03-01" --end-date "2025-03-31" --org-id org-xxxxxxxx
```

---

## 更新迭代

### 基本用法

```bash
yunxiao projex sprints update --space-id <PROJECT_ID> --sprint-id <SPRINT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--sprint-id` | 迭代 ID | 是 |
| `--name` | 新名称 | 否 |
| `--start-date` | 新开始日期 (YYYY-MM-DD) | 否 |
| `--end-date` | 新结束日期 (YYYY-MM-DD) | 否 |
| `--capacity-hours` | 新工时容量 | 否 |
| `--description` | 新描述 | 否 |
| `--owner` | 负责人用户 ID，可多次指定 | 否 |

### 示例

```bash
# 更新名称
yunxiao projex sprints update --space-id proj-xxxxxxxx --sprint-id sprint-xxxxxxxx \
  --name "新迭代名称" --org-id org-xxxxxxxx

# 更新日期
yunxiao projex sprints update --space-id proj-xxxxxxxx --sprint-id sprint-xxxxxxxx \
  --start-date "2025-03-01" --end-date "2025-03-31" --org-id org-xxxxxxxx

# 更新工时容量和描述
yunxiao projex sprints update --space-id proj-xxxxxxxx --sprint-id sprint-xxxxxxxx \
  --capacity-hours 200 --description "新增功能迭代" --org-id org-xxxxxxxx

# 更新负责人
yunxiao projex sprints update --space-id proj-xxxxxxxx --sprint-id sprint-xxxxxxxx \
  --owner "user-xxx" --org-id org-xxxxxxxx
```

---

## 列出版本

### 基本用法

```bash
yunxiao projex versions list --space-id <PROJECT_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao projex versions list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 创建版本

### 基本用法

```bash
yunxiao projex versions create --space-id <PROJECT_ID> --name <name> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--name` | 版本名称 | 是 |
| `--description` | 版本描述 | 否 |

### 示例

```bash
yunxiao projex versions create --space-id proj-xxxxxxxx --name "v1.0.0" --org-id org-xxxxxxxx
yunxiao projex versions create --space-id proj-xxxxxxxx --name "v2.0.0" --description "重大更新版本" --org-id org-xxxxxxxx
```

---

## 更新版本

### 基本用法

```bash
yunxiao projex versions update --space-id <PROJECT_ID> --version-id <VERSION_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--version-id` | 版本 ID | 是 |
| `--name` | 新名称 | 否 |
| `--description` | 新描述 | 否 |
| `--status` | 新状态 | 否 |

**版本状态可选值**：

| 状态值      | 说明     |
|-------------|----------|
| PLANNED     | 规划中   |
| DEVELOPING  | 开发中   |
| RELEASED    | 已发布   |
| ARCHIVED    | 已归档   |

### 示例

```bash
yunxiao projex versions update --space-id proj-xxxxxxxx --version-id version-xxxxxxxx \
  --name "v1.1.0" --org-id org-xxxxxxxx

yunxiao projex versions update --space-id proj-xxxxxxxx --version-id version-xxxxxxxx \
  --description "更新了版本描述" --status "RELEASED" --org-id org-xxxxxxxx
```

---

## 删除版本

### 基本用法

```bash
yunxiao projex versions delete --space-id <PROJECT_ID> --version-id <VERSION_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--version-id` | 版本 ID | 是 |

### 示例

```bash
yunxiao projex versions delete --space-id proj-xxxxxxxx --version-id version-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 工时管理

### 列出工时记录

```bash
yunxiao projex efforts list --org-id <ORG_ID> --output json
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--start-date` | 开始日期过滤 (YYYY-MM-DD) | 否 |
| `--end-date` | 结束日期过滤 (YYYY-MM-DD) | 否 |

#### 示例

```bash
# 列出所有工时记录
yunxiao projex efforts list --org-id org-xxxxxxxx --output json

# 按日期范围过滤
yunxiao projex efforts list --org-id org-xxxxxxxx --start-date "2025-03-01" --end-date "2025-03-31" --output json
```

### 创建工时记录

```bash
yunxiao projex efforts create --space-id <PROJECT_ID> --workitem-id <WORKITEM_ID> --duration <HOURS> --org-id <ORG_ID>
```

#### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--workitem-id` | 工作项 ID | 是 |
| `--duration` | 工时（小时），支持小数 | 是 |
| `--description` | 工时描述 | 否 |

#### 示例

```bash
yunxiao projex efforts create --space-id proj-xxxxxxxx --workitem-id wi-xxxxxxxx \
  --duration 2.5 \
  --description "修复登录页面问题" \
  --org-id org-xxxxxxxx
```

---

## 列出标签

### 基本用法

```bash
yunxiao projex labels list --space-id <PROJECT_ID> --org-id <ORG_ID> --output json
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
yunxiao projex labels list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx --output json

# 按关键词过滤（不区分大小写）
yunxiao projex labels list --space-id proj-xxxxxxxx --keyword "bug" --org-id org-xxxxxxxx --output json
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

### Agent 典型工作流：查找 ID

```bash
# 1. 获取项目 ID
PROJECT_ID=$(yunxiao projex projects search --keyword "demo" --org-id org-xxx --output json | jq -r '.[0].id')

# 2. 获取工作项类型 ID
TYPE_ID=$(yunxiao projex workitems types --space-id $PROJECT_ID --category Req --org-id org-xxx --output json | jq -r '.[0].id')

# 3. 获取迭代 ID
SPRINT_ID=$(yunxiao projex sprints list --space-id $PROJECT_ID --org-id org-xxx --output json | jq -r '.[0].id')

# 4. 获取标签 ID
LABEL_ID=$(yunxiao projex labels list --space-id $PROJECT_ID --org-id org-xxx --output json | jq -r '.[0].id')

# 5. 获取状态 ID（用于 update）
STATUS_ID=$(yunxiao projex workitems flow --space-id $PROJECT_ID --type-id $TYPE_ID --org-id org-xxx --output json | jq -r '.statuses[0].id')
```

### 查看项目工作项概览

```bash
# 搜索项目
yunxiao projex projects search --keyword "demo" --org-id org-xxx --output json

# 获取项目 ID
PROJECT_ID=$(yunxiao projex projects search --keyword "demo" --org-id org-xxx --output json | jq -r '.[0].id')

# 查看需求
yunxiao projex workitems search --space-id $PROJECT_ID -c Req --org-id org-xxx --output json

# 查看缺陷
yunxiao projex workitems search --space-id $PROJECT_ID -c Bug --org-id org-xxx --output json
```

### 创建完整工作项流程

```bash
# 1. 先获取工作项类型 ID
yunxiao projex workitems types --space-id proj-xxx --category Req --org-id org-xxx --output json

# 2. 创建需求（带描述）
yunxiao projex workitems create --space-id proj-xxx --type-id type-xxx --subject "新功能需求" \
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
yunxiao projex sprints list --space-id proj-xxx --org-id org-xxx --output json

# 查看某个迭代中的工作项
SPRINT_ID=$(yunxiao projex sprints list --space-id proj-xxx --org-id org-xxx --output json | jq -r '.[0].id')
yunxiao projex workitems search --space-id proj-xxx -c Req -S $SPRINT_ID --org-id org-xxx --output json
```

### 查看版本内工作项

```bash
# 列出版本
yunxiao projex versions list --space-id proj-xxx --org-id org-xxx --output json

# 查看某个版本中的工作项
VERSION_ID=$(yunxiao projex versions list --space-id proj-xxx --org-id org-xxx --output json | jq -r '.[0].id')
yunxiao projex workitems search --space-id proj-xxx -c Req -v $VERSION_ID --org-id org-xxx --output json
```

### 工时管理流程

```bash
# 查看我的工时
yunxiao projex efforts list --org-id org-xxx --start-date "2025-03-01" --end-date "2025-03-31" --output json

# 为工作项记录工时
yunxiao projex efforts create --space-id proj-xxx --workitem-id wi-xxx \
  --duration 3.5 --description "实现了登录接口" \
  --org-id org-xxx
```

### 批量操作技巧

#### 批量创建工作项(从 CSV 导入)

```bash
# 从 CSV 文件批量创建任务
# CSV 格式: subject,assignee,priority
cat tasks.csv | while IFS=, read -r subject assignee priority; do
  yunxiao projex workitems create --space-id proj-xxx \
    --type-id type-xxx \
    --subject "$subject" \
    --assignee "$assignee" \
    --priority "$priority" \
    --org-id org-xxx
done
```

#### 批量更新工作项状态

```bash
# 查询特定状态的工作项并批量更新
IDS=$(yunxiao projex workitems search --space-id proj-xxx \
  -c Task --org-id org-xxx --output json \
  | jq -r '.[] | select(.status.name == "待处理") | .id')

for id in $IDS; do
  yunxiao projex workitems update --space-id proj-xxx \
    --workitem-id "$id" \
    --status status-done \
    --org-id org-xxx
done
```

#### 批量添加标签

```bash
# 为所有缺陷添加"紧急"标签
LABEL_ID=$(yunxiao projex labels list --space-id proj-xxx \
  --keyword "紧急" --org-id org-xxx --output json \
  | jq -r '.[0].id')

IDS=$(yunxiao projex workitems search --space-id proj-xxx \
  -c Bug --org-id org-xxx --output json \
  | jq -r '.[].id')

for id in $IDS; do
  yunxiao projex workitems update --space-id proj-xxx \
    --workitem-id "$id" \
    --labels "$LABEL_ID" \
    --org-id org-xxx
done
```

---

## 故障排查

### "项目未找到" (Project not found)

**原因**: 项目 ID 错误或无权限

**解决方案**:
```bash
# 搜索项目确认 ID
yunxiao projex projects search --keyword "项目名" --org-id org-xxx --output json
```

### "Space ID required"

**原因**: 工作项命令需要 `--space-id`（项目 ID）

**解决方案**:
```bash
# 获取项目 ID
yunxiao projex projects search --org-id org-xxx --output json

# 使用项目 ID
yunxiao projex workitems search --space-id proj-xxx --org-id org-xxx --output json
```

### "Invalid category"

**原因**: 搜索时类别参数错误

**解决方案**:
使用正确的类别值：`Req`, `Task`, `Bug`

### "工作项类型id不能为空"

**原因**: 创建工作项时未提供 `--type-id` 参数

**解决方案**:
```bash
# 先获取工作项类型 ID
yunxiao projex workitems types --space-id proj-xxx --category Req --org-id org-xxx --output json

# 使用返回的类型 ID 创建工作项
yunxiao projex workitems create --space-id proj-xxx --type-id <TYPE_ID> --subject "标题" --org-id org-xxx
```

### "401 Unauthorized"

**原因**: Token 无效或已过期

**解决方案**:
```bash
# 检查当前 token
yunxiao config show

# 重新设置 token
yunxiao auth set --token pt_xxxxxxxx
```

### 字段配置缓存

CLI 会缓存工作项类型的字段配置，避免重复请求。

**缓存清理**:
```bash
# 清理所有缓存
yunxiao config clear-cache

# 或手动删除缓存文件
rm ~/.cache/yunxiao-cli/field_config_*.json
```

### "Organization not found"

**原因**: `--org-id` 无效或不匹配当前 token 所属组织

**解决方案**:
```bash
# 确认组织 ID
yunxiao config show

# 设置正确的组织 ID
yunxiao config set-org-id --org-id org-xxxxxxxx
```
