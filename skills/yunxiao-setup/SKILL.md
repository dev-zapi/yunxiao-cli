---
name: yunxiao-setup
description: 交互式创建项目级云效配置文件 YUNXIAO.md。当用户提到"创建 YUNXIAO.md"、"初始化云效项目配置"、"yunxiao init"、"设置云效项目"、"配置云效"时触发。用于首次设置项目与云效的关联关系。
---

# 云效项目配置初始化

指导 AI 交互式创建 `YUNXIAO.md` 文件，建立本地项目与云效项目的关联关系。

**定位**：这是业务层配置，存储项目实体信息（项目ID、工作项类型等）。技术层配置（token、认证）由 `yunxiao config` 管理，两者独立。

---

## 何时使用

当用户需要：
- 首次将本地项目关联到云效项目
- 创建或更新 YUNXIAO.md 配置文件
- 设置项目级云效业务数据缓存

---

## 核心流程

### 1. 准备阶段

**检查前置条件**：
- 确认 yunxiao CLI 已安装并可执行
- 确认已配置认证信息（token，通过 `yunxiao config list` 或环境变量）
- 检测当前目录是否为 Git 仓库（用于提示，非必需）

**检查现有配置**：
- 查看项目根目录是否已存在 `YUNXIAO.md`
- 如果存在，询问用户是否要更新现有配置

### 2. 交互式创建

**步骤 1：选择组织**

```bash
yunxiao org list --output json
```

展示可访问的组织列表，让用户选择。提取 `orgId` 和 `orgName`。

**步骤 2：选择项目**

```bash
yunxiao projex projects search --org-id <ORG_ID> --output json
```

展示项目列表，让用户选择。提取 `id`（即 space_id）和 `name`。

**步骤 3：关联仓库（可选）**

询问用户是否要关联云效仓库：
- 如果是：查询组织下的仓库列表，让用户选择
- 如果否：跳过此步骤

```bash
yunxiao codeup repos list --org-id <ORG_ID> --output json
```

提取 `id`（repo_id）和 `name`。

**步骤 4：补充配置（可选）**

询问用户是否需要补充：
- 工作项类型 ID（需求/任务/缺陷）
- 当前迭代 ID
- 常用成员显示名与 Account User `userId`
- 常用标签名称与标签 ID
- 其他常用配置

如果用户选择跳过，生成最小版本配置。

### 3. 生成配置文件

**文件位置**：项目根目录 `./YUNXIAO.md`

**文件格式**：

```markdown
# 云效项目配置

## 基本信息
- 组织 ID: <ORG_ID>
- 项目 ID: <SPACE_ID>
- 项目名称: <PROJECT_NAME>

## 仓库信息
- 主仓库 ID: <REPO_ID>（可选）
- 默认分支: main

## 常用 ID（可选，可能过期）
<!-- 这些 ID 可能因云效系统配置变更而失效，使用前建议验证 -->

### 工作项类型
- 需求类型 ID: <TYPE_ID>
- 任务类型 ID: <TYPE_ID>
- 缺陷类型 ID: <TYPE_ID>

### 常用成员（可选，显示名 → Account User userId）
- <显示名>: <USER_ID>

### 常用标签（可选，名称 → label ID）
- <标签名>: <LABEL_ID>

### 迭代与版本
- 当前迭代 ID: <SPRINT_ID>

### 其他配置
- 优先级 P0 ID: <PRIORITY_ID>
- 状态"进行中" ID: <STATUS_ID>
```

**注意事项**：
- 标注可选字段的状态（如"可选，可能过期"）
- 提供清晰的字段说明
- 使用 Markdown 格式便于人工编辑

---

## 后续维护

### 自动更新

其他 yunxiao 技能在使用过程中发现 YUNXIAO.md 缺失某些字段时：
- 询问用户是否要补充
- 补充后更新文件

创建需求时，CLI 已经可以直接解析标签名称并校验负责人；技能不得未经用户同意自动写入这些映射。

### 错误处理

当使用 YUNXIAO.md 中的 ID 报错时：
- 提示用户配置可能过期
- 建议重新运行 `yunxiao-setup` 或手动更新

### 版本控制建议

建议将 `YUNXIAO.md` 加入 `.gitignore`，避免泄露业务信息：

```bash
echo "YUNXIAO.md" >> .gitignore
```

---

## 常见场景

### 场景 1：首次设置

**用户**："帮我初始化云效项目配置"

**AI 流程**：
1. 检查认证配置
2. 查询可访问的组织列表
3. 展示组织让用户选择
4. 查询组织下的项目列表
5. 展示项目让用户选择
6. 询问是否关联仓库
7. 询问是否补充其他配置
8. 生成 YUNXIAO.md 文件
9. 提示用户加入 .gitignore

### 场景 2：快速初始化

**用户**："快速创建 YUNXIAO.md"

**AI 流程**：
1. 快速创建最小版本（只包含组织和项目）
2. 提示用户后续可补充其他字段

### 场景 3：更新配置

**用户**："更新 YUNXIAO.md 中的迭代 ID"

**AI 流程**：
1. 读取现有 YUNXIAO.md
2. 查询当前迭代列表
3. 让用户选择新的迭代
4. 更新文件中的迭代 ID 字段

---

## 错误处理

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| "未配置认证信息" | token 未设置 | 提示用户运行 `yunxiao config set token <TOKEN>` 或设置环境变量 |
| "组织列表为空" | 无访问权限或 token 无效 | 检查 token 权限或联系管理员 |
| "项目列表为空" | 该组织下无项目或无权限 | 确认组织 ID 正确，或检查权限 |
| "文件已存在" | YUNXIAO.md 已存在 | 询问用户是否覆盖或更新 |

---

## 与其他技能的关系

- **yunxiao-cli**：基础技能，定义通用命令和 ID 查询流程
- **yunxiao-create-requirement**：创建需求时优先读取 YUNXIAO.md
- **yunxiao-mr-create**：创建 MR 时优先读取 YUNXIAO.md

所有 yunxiao 技能都会优先检查项目根目录的 YUNXIAO.md，使用预定义配置减少查询。
