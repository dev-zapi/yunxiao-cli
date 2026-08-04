---
name: yunxiao-create-requirement
description: 在云效 Projex 上创建需求（Requirement）。当用户提到"创建需求"、"新建需求"、"提一个需求"、"录入需求"、"添加需求到项目"等表达时触发此技能。指导 AI 完成完整的需求创建流程，包括获取必要 ID、收集需求信息、创建需求并关联相关项。
---

# 创建云效需求（Requirement）

指导 AI 在云效 Projex 项目管理工作项中创建需求。

---

## 执行前检查

### 1. 检查 YUNXIAO.md

**首先检查项目根目录的 `YUNXIAO.md` 文件**，获取预定义的项目信息：

```bash
# 如果文件存在，读取以下信息：
- 组织 ID (org_id)
- 项目 ID (space_id / project_id)
- 需求类型 ID (type_id for Req category)
```

如果 YUNXIAO.md 存在且包含所需 ID，直接使用，跳过查询步骤。

### 2. 查询缺失的 ID

如果 YUNXIAO.md 不存在或缺少必要信息，按以下顺序查询：

```bash
# 步骤 1: 获取组织 ID（如果未配置）
yunxiao config list  # 查看是否已配置 organization_id

# 步骤 2: 搜索项目，获取 space_id
yunxiao projex projects search --keyword "项目名称" --org-id <ORG_ID> --output json
# 从结果中提取 id 字段

# 步骤 3: 获取需求类型 ID
yunxiao projex workitems types --space-id <SPACE_ID> --category Req --org-id <ORG_ID> --output json
# 从结果中提取 id 字段（通常是 "Req" 或类似值）
```

---

## 需求创建流程

### 第一步：收集需求信息

向用户询问以下信息（使用 AskUserQuestion 工具）：

**必填信息：**
- **需求标题** (subject): 简洁描述需求内容
- **需求描述** (description): 详细说明，支持 Markdown 格式

**可选信息（根据项目配置询问）：**
- **优先级**: P0/P1/P2/P3（如果项目配置了优先级字段）
- **指派给**: 用户名或用户 ID（如果需要指派）
- **迭代**: 迭代名称或 ID（如果需要关联到迭代）
- **版本**: 版本名称或 ID（如果需要关联到版本）
- **标签**: 标签名称列表（如果需要打标签）
- **父工作项**: 父需求或史诗的 ID（如果需要建立父子关系）

### 第二步：获取可选字段的 ID

根据用户提供的信息，查询对应的 ID：

```bash
# 获取可用状态（通常创建时使用初始状态，可跳过）
yunxiao projex workitems flow --space-id <SPACE_ID> --type-id <TYPE_ID> --org-id <ORG_ID> --output json

# 获取优先级 ID（如果用户指定了优先级）
yunxiao projex workitems fields --project-id <SPACE_ID> --type-id <TYPE_ID> --org-id <ORG_ID> --output json
# 从结果中找到 priority 字段的可选值

# 获取迭代 ID（如果用户指定了迭代）
yunxiao projex sprints list --space-id <SPACE_ID> --org-id <ORG_ID> --output json

# 获取版本 ID（如果用户指定了版本）
yunxiao projex versions list --space-id <SPACE_ID> --org-id <ORG_ID> --output json

# 获取标签 ID（如果用户指定了标签）
yunxiao projex labels list --space-id <SPACE_ID> --org-id <ORG_ID> --output json

# 获取用户 ID（如果需要指派给特定用户）
yunxiao org members list --org-id <ORG_ID> --output json
```

### 第三步：创建需求

使用收集到的信息创建需求：

```bash
yunxiao projex workitems create \
  --space-id <SPACE_ID> \
  --type-id <TYPE_ID> \
  --subject "需求标题" \
  --description "## 背景
## 需求描述
- 功能点 1
- 功能点 2

## 验收标准
- [ ] 标准 1
- [ ] 标准 2" \
  --org-id <ORG_ID>
```

**带可选参数的完整示例：**

```bash
yunxiao projex workitems create \
  --space-id proj-xxx \
  --type-id type-xxx \
  --subject "用户登录功能优化" \
  --description "## 背景
当前登录流程复杂，用户反馈较多。

## 需求描述
- 简化登录步骤
- 支持第三方登录
- 增加记住密码功能

## 验收标准
- [ ] 登录步骤从 3 步减少到 2 步
- [ ] 支持微信、支付宝登录
- [ ] 记住密码功能正常工作" \
  --priority-id priority-xxx \
  --assigned-to user-xxx \
  --sprint-id sprint-xxx \
  --org-id org-xxx \
  --output json
```

### 第四步：确认创建结果

创建成功后，命令会返回新创建的工作项 ID。可以查询详情确认：

```bash
yunxiao projex workitems get --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

向用户展示创建结果，包括：
- 工作项 ID
- 标题
- 状态
- 链接（如果有）

---

## 常见场景示例

### 场景 1: 快速创建简单需求

用户说："帮我在项目中创建一个需求，标题是'优化页面加载速度'"

AI 操作：
1. 检查 YUNXIAO.md 获取项目 ID
2. 询问描述（可选）
3. 直接创建，使用默认值

### 场景 2: 创建完整需求

用户说："创建一个需求，要在下个迭代完成，指派给张三，优先级 P1"

AI 操作：
1. 检查 YUNXIAO.md 获取基础 ID
2. 查询迭代列表，找到"下个迭代"的 ID
3. 查询用户列表，找到"张三"的 ID
4. 查询优先级字段，找到 P1 的 ID
5. 询问需求详细描述
6. 创建需求并关联所有信息

### 场景 3: 批量创建需求

用户说："帮我创建 3 个需求：1) 登录优化 2) 注册流程改进 3) 密码找回功能"

AI 操作：
1. 检查 YUNXIAO.md 获取项目 ID 和类型 ID
2. 为每个需求分别创建（可以并行执行）
3. 返回所有创建的工作项 ID

---

## 错误处理

### "工作项类型id不能为空"
确保查询了需求类型 ID：
```bash
yunxiao projex workitems types --space-id <SPACE_ID> --category Req --org-id <ORG_ID> --output json
```

### "项目不存在"或"无权访问"
检查 space_id 是否正确，用户是否有权限访问该项目。

### "优先级字段不存在"
不是所有项目都配置了优先级字段，跳过该参数或查询项目配置。

### "迭代不存在"
检查迭代 ID 是否正确，或者项目是否启用了迭代管理。

---

## 最佳实践

1. **描述使用 Markdown**: 支持标题、列表、复选框等格式，使需求更清晰
2. **先查询后创建**: 确保所有 ID 都有效后再创建
3. **提供反馈**: 创建后向用户确认结果，展示工作项 ID 和链接
4. **建议更新 YUNXIAO.md**: 如果用户经常创建需求，建议将常用 ID 保存到 YUNXIAO.md
5. **使用 JSON 输出**: Agent 场景始终使用 `--output json` 便于解析

---

## 相关命令速查

| 操作 | 命令 |
|------|------|
| 搜索项目 | `yunxiao projex projects search --keyword <关键词>` |
| 获取需求类型 | `yunxiao projex workitems types --category Req` |
| 创建工作项 | `yunxiao projex workitems create --space-id <ID> --type-id <ID>` |
| 查询工作项 | `yunxiao projex workitems search --space-id <ID>` |
| 获取工作项详情 | `yunxiao projex workitems get --workitem-id <ID>` |
| 列出迭代 | `yunxiao projex sprints list --space-id <ID>` |
| 列出成员 | `yunxiao org members list --org-id <ID>` |
