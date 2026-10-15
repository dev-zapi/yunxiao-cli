---
name: yunxiao-setup
description: 交互式创建云效配置：随仓库提交的 YUNXIAO.md（共享）+ 只在本地的 .yunxiao/local.md（本地）+ 本机全局不入库的 ~/.config/yunxiao-cli/global.md（全局）。当用户提到"创建 YUNXIAO.md"、"初始化云效项目配置"、"yunxiao init"、"设置云效项目"、"配置云效"、"设置全局云效配置"、"创建 global.md"时触发。用于首次设置项目与云效的关联关系、本机跨仓库共享个人信息，以及旧单文件配置的迁移。
---

# 云效项目配置初始化

指导 AI 交互式创建云效配置：随仓库提交的 `YUNXIAO.md`（共享配置）、只在本地、被版本库忽略的 `.yunxiao/local.md`（本地配置），以及本机全局、不入库的 `~/.config/yunxiao-cli/global.md`（全局配置）。

**定位**：这是业务层配置，存储项目实体信息（项目ID、工作项类型等）与跨仓库个人信息（身份、成员映射）。技术层配置（token、认证）由 `yunxiao config` 管理，两者独立；全局配置与 config.toml 同居 `~/.config/yunxiao-cli/` 但互补不重叠——config.toml 管"CLI 怎么连"，global.md 管"agent 认得什么"。

**字段归属与读取顺序的规范正主是 [yunxiao-cli](../yunxiao-cli/SKILL.md) 技能的"项目上下文配置"章节**，本技能负责生成与迁移，不另行定义归属。

---

## 何时使用

当用户需要：
- 首次将本地项目关联到云效项目
- 创建或更新共享配置 YUNXIAO.md / 本地配置 .yunxiao/local.md
- 创建或更新全局配置 global.md（用户身份、org 别名、常用成员映射，本机跨仓库）
- 将旧式单文件 YUNXIAO.md 迁移到三层结构

---

## 目标文件结构

```
./YUNXIAO.md            # 共享配置，随仓库提交
./.yunxiao/
├── .gitignore          # 内容为 * 和 !.gitignore 两行，随仓库提交
└── local.md            # 本地配置，被忽略、不入库
~/.config/yunxiao-cli/global.md   # 全局配置，本机跨仓库，不入库
```

`.yunxiao/.gitignore` 中的 `*` 忽略目录内全部内容，`!.gitignore` 让规则文件本身可提交——克隆即得忽略规则，不依赖每个协作者跑过本技能。

`global.md` 不属于项目三件套，按需创建（见"全局配置模式"）；它存放 org 级个人信息，任何仓库都不该提交它。

---

## 核心流程

### 1. 准备阶段

**检查前置条件**：
- 确认 yunxiao CLI 已安装并可执行
- 确认已配置认证信息（token，通过 `yunxiao config list` 或环境变量）
- 检测当前目录是否为 Git 仓库（用于提示，非必需）

**检查现有配置**：
- 查看项目根目录是否已存在 `YUNXIAO.md` 和 `.yunxiao/local.md`
- 两者都存在：询问用户要更新哪个（或都更新）
- 只存在 YUNXIAO.md 且含本地/全局类字段（当前迭代、成员映射等）：主动提供自动迁移（见"旧单文件迁移"）
- 都不存在：走全新创建流程
- 顺带读取本机 `~/.config/yunxiao-cli/global.md`（如存在）：身份与常用成员可直接复用，不必重复询问

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

询问用户是否需要补充，并告知各字段去向：
- 工作项类型 ID（需求/任务/缺陷）→ 共享配置；选定类型后对该类型运行 `workitems flow`（状态）与 `workitems fields`（优先级），在共享配置生成 name→ID 字典
- 当前迭代 ID → 本地配置
- 常用成员显示名与 Account User `userId` → 全局配置（org 级，跨仓库复用）
- 常用标签名称与标签 ID → 本地配置

如果用户选择跳过，生成最小版本配置。

**上提询问**：用户提供成员映射或身份信息而全局配置尚无对应条目时，询问"要存到全局配置吗？本机所有仓库共用"。同意则写入 `~/.config/yunxiao-cli/global.md`（见"全局配置模式"），不同意则本次不落盘。

### 3. 生成配置文件

三个文件一起创建，缺一不可（local.md 允许只含节标题的空模板）。

**共享配置 `./YUNXIAO.md`**（提交进仓库）：

```markdown
# 云效项目配置（共享）

## 基本信息
- 组织 ID: <ORG_ID>
- 项目 ID: <SPACE_ID>
- 项目名称: <PROJECT_NAME>

## 仓库信息
- 主仓库 ID: <REPO_ID>（可选）
- 默认分支: main

## 工作项类型
- 需求类型 ID: <TYPE_ID>（可选）
- 任务类型 ID: <TYPE_ID>（可选）
- 缺陷类型 ID: <TYPE_ID>（可选）

## 状态与优先级字典（name → ID）

按已声明的工作项类型生成；由 `workitems flow`（状态）与 `workitems fields`（优先级）查询。

### <类型名>（<TYPE_ID>）
- 状态: <显示名>=<ID>、…
- 优先级: <P0>=<ID>、…
```

**忽略规则 `./.yunxiao/.gitignore`**（提交进仓库）：

```
*
!.gitignore
```

**本地配置 `./.yunxiao/local.md`**（被忽略，不入库；即使暂无内容也总是创建，空模板自带自解释性）：

```markdown
# 云效项目配置（本地，不入库）

## 迭代与版本
- 当前迭代 ID: <SPRINT_ID>（可选，可能过期）

## 常用标签（可选，名称 → label ID）
- <标签名>: <LABEL_ID>
```

**注意事项**：
- 标注可选字段的状态（如"可选，可能过期"）
- 使用 Markdown 格式便于人工编辑
- 提示用户将 `YUNXIAO.md` 与 `.yunxiao/.gitignore` 一起提交

---

## 全局配置模式

用户显式要求"设置全局云效配置"、"创建 global.md"，或项目流程中的上提询问被接受时执行：

1. **读取现有文件**：`~/.config/yunxiao-cli/global.md` 已存在则先读取，增量更新而非覆盖
2. **用户身份**：询问用户显示名，用 `yunxiao org members search <名字> --org-id <ORG_ID>` 确认并提取 Account User `userId`
3. **组织分节**：`yunxiao org list --output json` 选择组织，请用户起一个别名（节标识，如 `main`、`test`）
4. **常用成员**（可选）：逐个 `yunxiao org members search <NAME_OR_EMAIL> --org-id <ORG_ID>` 建立显示名 → userId 映射
5. **写入并确认**：向用户展示最终内容，确认后写入

**global.md 模板**（头部用户身份 + 按 org 别名分节）：

```markdown
# 云效全局配置（本机，不入库）

## 用户身份
- 显示名: <DISPLAY_NAME>
- Account User userId: <USER_ID>

## 组织：<别名>
- 组织 ID: <ORG_ID>

### 常用成员（显示名 → Account User userId）
- <显示名>: <USER_ID>
```

多个组织即多个 `## 组织：` 节，别名不能重复。global.md 属于本机个人文件，绝不提交、不进任何仓库。

---

## 旧单文件迁移

检测到已存在的 YUNXIAO.md 包含本地/全局类字段（当前迭代、成员映射等）时，提供自动迁移：

1. **拆分**：按 yunxiao-cli 技能的字段归属表，当前迭代、标签映射搬到 `.yunxiao/local.md`；成员映射经询问存入全局配置（用户拒绝则跳过，条目自然淘汰）；其余（含旧式状态/优先级条目）留在 `YUNXIAO.md`
2. **建忽略规则**：创建 `.yunxiao/.gitignore`（`*` + `!.gitignore`）
3. **清理旧忽略条目**：根 `.gitignore` 中如有 `YUNXIAO.md` 条目则移除——旧版技能建议写入过它，现在共享文件需要提交，留着会挡路
4. **保持跟踪**：YUNXIAO.md 若已被 git 跟踪则保持跟踪不动
5. 迁移完成后向用户展示两个文件的内容确认

---

## 后续维护

### 自动更新

其他 yunxiao 技能在使用过程中发现配置缺失某些字段时：
- 询问用户是否要补充
- 按字段归属表补到归属文件（共享字段进 YUNXIAO.md，本地字段进 local.md，全局字段进 global.md）

创建需求时，CLI 已经可以直接解析标签名称并校验负责人；技能不得未经用户同意自动写入这些映射。

### 错误处理

当使用配置中的 ID 报错时：
- 提示用户配置可能过期
- 状态/优先级字典失效：征得用户同意后用 `workitems flow` / `workitems fields` 重新生成对应类型的字典，并提示这是一次需要提交的仓库变更
- 其他字段：建议重新运行 `yunxiao-setup` 或手动更新对应文件

---

## 常见场景

### 场景 1：首次设置

**用户**："帮我初始化云效项目配置"

**AI 流程**：
1. 检查认证配置
2. 查询并选择组织、项目
3. 询问是否关联仓库、补充其他配置
4. 同时创建 YUNXIAO.md、.yunxiao/.gitignore、.yunxiao/local.md
5. 提示用户提交 YUNXIAO.md 与 .yunxiao/.gitignore

### 场景 2：快速初始化

**用户**："快速创建 YUNXIAO.md"

**AI 流程**：
1. 快速创建最小版本（共享文件只含组织和项目；本地文件为空模板）
2. 提示用户后续可补充其他字段

### 场景 3：更新迭代

**用户**："更新云效配置里的迭代 ID"

**AI 流程**：
1. 读取 .yunxiao/local.md（迭代 ID 属于本地配置）
2. 查询当前迭代列表
3. 让用户选择新的迭代
4. 更新 local.md 中的迭代 ID 字段

### 场景 4：迁移旧配置

**用户**："我之前建过 YUNXIAO.md，听说现在拆成两个文件了"

**AI 流程**：
1. 读取现有 YUNXIAO.md，识别其中的本地类字段
2. 按"旧单文件迁移"章节拆分并清理根 .gitignore
3. 展示两个文件的内容，提示提交共享文件与忽略规则

### 场景 5：设置全局配置

**用户**："设置全局云效配置" / "帮我建一个 global.md，把常用成员放进去"

**AI 流程**：
1. 读取现有 ~/.config/yunxiao-cli/global.md（如存在）
2. 确认用户身份（org members search 校验 userId）
3. 选择组织并起别名，逐个补充常用成员
4. 展示最终内容，确认后写入

---

## 错误处理

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| "未配置认证信息" | token 未设置 | 提示用户运行 `yunxiao config set token <TOKEN>` 或设置环境变量 |
| "组织列表为空" | 无访问权限或 token 无效 | 检查 token 权限或联系管理员 |
| "项目列表为空" | 该组织下无项目或无权限 | 确认组织 ID 正确，或检查权限 |
| "文件已存在" | 目标文件已存在 | 询问用户是覆盖、更新还是迁移 |

---

## 与其他技能的关系

- **yunxiao-cli**：基础技能，字段归属表与读取顺序的正主；定义通用命令和 ID 查询流程
- **yunxiao-create-requirement**：创建需求时优先读取项目上下文配置

所有 yunxiao 技能都会优先检查项目根目录的 YUNXIAO.md 与 .yunxiao/local.md，以及本机的 global.md，使用预定义配置减少查询。
