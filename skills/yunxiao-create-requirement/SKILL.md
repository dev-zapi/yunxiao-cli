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

### 2. 解析缺失的项目引用

优先读取 `YUNXIAO.md` 中的组织 ID、项目空间 ID 和需求类型 ID。只查询缺失的值：

```bash
# 仅当 YUNXIAO.md 没有需求类型 ID 时执行
yunxiao projex workitems types --space-id <SPACE_ID> --category Req --org-id <ORG_ID> --output json

# 按姓名或邮箱精确查找负责人；使用结果中的 userId，不是成员 id
yunxiao org members search "<NAME_OR_EMAIL>" --org-id <ORG_ID> --output json
```

标签可以直接传名称或 ID，CLI 会在项目空间内精确解析和验证，无需手工执行 `labels list`。

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

**引用验证：**
- 负责人必须使用成员结果中的 `userId`；同名或无匹配时停止并展示候选
- 类型 ID、负责人 ID 和标签引用只在缺失时查询；标签名称区分大小写

### 第二步：创建需求

```bash
yunxiao projex workitems create \
  --space-id <SPACE_ID> \
  --type-id <TYPE_ID> \
  --subject "需求标题" \
  --description "需求描述内容" \
  --assignee <USER_ID> \
  --labels "ready-for-agent" \
  --org-id <ORG_ID> \
  --output json
```

实际可选参数：`--priority`, `--sprint-id`, `--labels`, `--description-file` 和 `--field fieldId=value`。
`--labels` 支持逗号分隔的标签 ID 或精确名称；不要使用 `--field labels=...`。
创建命令默认等待并返回已稳定读取的完整工作项详情；`--wait-timeout 0` 才返回原始创建响应。

### 第三步：确认结果

CLI 已经根据创建返回的 ID 自动轮询详情并校验标题、负责人和请求标签。向用户展示 CLI 返回的工作项 ID、标题、状态、指派人和标签。
若返回“已创建但未稳定”或“部分成功”，保留返回的工作项 ID，禁止再次创建。

---

## 常见错误

| 错误 | 解决方案 |
|------|---------|
| "工作项类型id不能为空" | 先查询 `workitems types --category Req` |
| "工作项类型不存在" | 检查 type_id 是否正确，重新查询并核对 |
| "指派人不能为空" / "accountId is invalid" | 提供成员搜索结果中的 `userId`，不要使用成员记录的 `id` |
| "项目不存在" | 检查 space_id 是否正确 |
| "标签不存在/未找到" | CLI 会刷新标签目录并对同一工作项重试一次；仍失败时保留工作项 ID，不要重试创建 |

---

## 示例

**用户**："帮我创建一个需求，优化登录流程，指派给张三"

**AI 流程**：
1. 读取 YUNXIAO.md，只有缺失时查询项目/类型 ID
2. 用 `org members search` 精确解析张三的 `userId`
3. 询问详细描述（必填，不能为空）
4. 调用一次 `workitems create`；CLI 内部完成标签解析、字段预检和稳定详情读取
5. 展示 CLI 返回的完整详情；失败时保留已有工作项 ID
