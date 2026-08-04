---
name: yunxiao-create-requirement
description: 在云效 Projex 上创建需求。当用户提到"创建需求"、"新建需求"、"提一个需求"、"录入需求"时触发。专注于需求创建流程，通用命令参考 yunxiao-cli 技能。
---

# 创建云效需求

指导 AI 在云效 Projex 创建需求（Requirement）。

**前置知识**：通用命令、YUNXIAO.md 配置、ID 查询流程参考 [yunxiao-cli](../yunxiao-cli/SKILL.md) 技能。

---

## 创建前准备

### 1. 检查 YUNXIAO.md

读取项目根目录的 `YUNXIAO.md`，获取预定义的项目 ID 和配置。如不存在，按 yunxiao-cli 技能指引查询。

### 2. 获取需求类型 ID

```bash
yunxiao projex workitems types --space-id <SPACE_ID> --category Req --org-id <ORG_ID> --output json
```

从结果提取需求类型的 `id` 字段。

---

## 创建流程

### 第一步：收集信息

向用户询问（使用 AskUserQuestion）：

**必填：**
- 需求标题 (subject)
- 需求描述 (description，支持 Markdown)
- 指派人 userId（**必需**，从 `org members list` 获取 `userId` 字段，不是 `id`）

**描述处理：**
- 如果用户未提供描述，**必须询问**或使用最小默认值："待补充"
- 避免使用空描述，某些项目可能强制要求

**可选：**
- 优先级、迭代、版本、标签、父工作项

**ID 验证：**
- 查询到的 type_id、user_id 等必须仔细核对，避免转录错误
- 建议在创建前再次确认关键 ID 的正确性

### 第二步：查询可选字段 ID

根据用户需求查询对应 ID（参考 yunxiao-cli 技能的 ID 查询流程）。

### 第三步：创建需求

```bash
yunxiao projex workitems create \
  --space-id <SPACE_ID> \
  --type-id <TYPE_ID> \
  --subject "需求标题" \
  --description "需求描述内容" \
  --assignee <USER_ID> \
  --org-id <ORG_ID> \
  --output json
```

可选参数：`--priority-id`, `--sprint-id`, `--version-id`, `--label-ids`, `--parent-id`

### 第四步：确认结果

创建成功返回工作项 ID，查询详情确认：

```bash
yunxiao projex workitems get --space-id <SPACE_ID> --workitem-id <WORKITEM_ID> --org-id <ORG_ID> --output json
```

向用户展示：工作项 ID、标题、状态、指派人。

---

## 常见错误

| 错误 | 解决方案 |
|------|---------|
| "工作项类型id不能为空" | 先查询 `workitems types --category Req` |
| "工作项类型不存在" | 检查 type_id 是否正确，重新查询并核对 |
| "指派人不能为空" / "accountId is invalid" | 必须提供 `--assignee`，使用 `userId` 而非 `memberId` |
| "项目不存在" | 检查 space_id 是否正确 |

---

## 示例

**用户**："帮我创建一个需求，优化登录流程，指派给张三"

**AI 流程**：
1. 读取 YUNXIAO.md（如存在）获取项目/类型 ID
2. 查询张三的 userId（`org members list`）
3. 询问详细描述（必填，不能为空）
4. 执行创建命令
5. 展示结果：ID、标题、状态、指派人
