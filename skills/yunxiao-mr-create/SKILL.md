---
name: yunxiao-mr-create
description: 创建云效 MR（合并请求）。当用户提到创建 MR、合并请求、发起合并、提交代码审查，或类似表达（如"创建MR"、"把某分支合并到某分支"、"使用本分支创建MR"）时触发此技能。支持自动生成描述、推断工作项关联、智能询问缺失信息。
---

# YunXiao MR 创建技能

## 工作流程

创建云效 MR 时，遵循以下流程：

### 1. 解析用户意图

从用户输入中提取：
- **源分支**：用户指定的源分支，或"本分支"（需查询当前分支）
- **目标分支**：用户指定的目标分支，或默认主分支
- **MR 标题**：用户提供的标题，或从分支名/commit 信息推断
- **描述信息**：用户提供的完整描述、概述或额外信息
- **工作项 ID**：用户明确提供的 ID 列表

### 2. 收集必要信息

#### 组织、项目、仓库信息

优先级（从高到低）：
1. **上下文中已存在**：从当前对话、文件内容、git remote 信息推断
2. **配置文件/环境变量**：检查 yunxiao-cli 配置
3. **询问用户**：无法自动获取时，向用户提问

#### 分支信息

- **源分支缺失**：查询 git 当前分支（`git branch --show-current`）
- **目标分支缺失**：默认使用 `main` 或 `master`，或询问用户
- **分支验证**：通过 yunxiao-cli 查询分支是否存在

### 3. 查询分支差异和提交信息

使用 git 命令和 yunxiao-cli 查询：

```bash
# 查询 commit 历史
git log <TARGET_BRANCH>..<SOURCE_BRANCH> --oneline

# 查询代码变更统计
git diff <TARGET_BRANCH>..<SOURCE_BRANCH> --stat

# 查询云效 commit 历史
yunxiao-cli codeup commits list --repo-id <REPO_ID> --ref-name <SOURCE_BRANCH> --org-id <ORG_ID>
```

### 4. 使用 pr-documentation 技能生成 MR 描述

**调用 pr-documentation 技能来生成专业的 MR 描述**。

使用 Skill tool 调用 pr-documentation 技能：

```markdown
使用 pr-documentation 技能为本次 MR 生成描述。

**输入上下文**：
- 源分支：feat/PROJ-9999
- 目标分支：master
- Commit 历史：[已查询的 commit 列表]
- 文件变更统计：[已查询的 diff stat]
- 工作项信息：PROJ-9999 - 【示例项目】按照框架重构详情页

请根据 pr-documentation 技能的标准结构生成 MR 描述。
```

**pr-documentation 技能会执行**：
1. 确定 base branch（master/main）
2. 收集 git log 和 diff 信息
3. 识别变更类型（功能、修复、重构等）
4. 按标准结构生成描述，包含：
   - 背景（关联工作项）
   - 变更内容（按模块/功能分组）
   - 影响范围（接口、数据、页面等）
   - 测试说明（实际执行的验证）
   - 兼容性与风险
   - Review 重点
   - Checklist

**生成的描述示例**：

```markdown
## 背景

关联工作项 PROJ-9999：按照框架重构详情页面。

原有详情页面使用旧版组件库，样式和交互不符合规范，影响用户体验和后续维护效率。本次重构统一使用新组件，优化布局和交互细节。

## 变更内容

- 重构详情页面布局，使用 Card、Descriptions、Form 等组件
- 嵌入相关板块（基本信息、配置信息）
- 新增 ExampleConfigCard 组件，支持默认配置
- 完善详情页面展示细节，包括锚点导航、概览卡片、数据加载逻辑
- 添加重构设计文档和规格说明

## 影响范围

- 示例模块 (MOD)
- 影响页面：详情页
- 新增组件：ExampleAnchorNav、ExampleOverviewCard、ExampleConfigCard
- 影响文件：15 个文件变更，新增 1289 行

## 测试说明

- 本地开发环境启动成功，页面正常加载
- 详情页面各板块数据正确展示
- 功能正常
- 单元测试覆盖新增组件核心逻辑

## 兼容性与风险

- **兼容性**：纯前端重构，不影响后端接口和数据结构
- **风险**：旧版页面样式可能存在差异，需要视觉回归测试

## Review 重点

- src/views/example/index.vue - 详情页面主入口
- src/components/Example/ExampleAnchorNav.vue - 锚点导航逻辑
- src/components/Example/ExampleConfigCard.vue - 新增配置

## Checklist

- [ ] 已完成自测
- [ ] 已补充单元测试
- [ ] 已确认无敏感信息
- [ ] 已说明兼容性和风险
- [ ] 无破坏性变更
```

### MR 标题格式

**固定格式**：`<工作项ID>：<MR标题>`

**示例**：
- `PROJ-9999：示例详情页面重构`
- `wi-123：添加用户认证功能`

**规则**：
- 工作项ID和标题之间使用中文冒号分隔
- 工作项ID必须是从分支名推断或用户提供的真实编号
- 标题部分简洁描述变更内容（15-30字）

### 4. 生成 MR 描述

#### 策略选择

根据用户提供的信息选择策略：

| 用户输入 | 描述生成策略 |
|---------|-------------|
| 提供完整描述 | 直接使用用户描述 |
| 提供概述/方向 | 结合自动提取内容 + 用户信息 |
| 无描述信息 | 自动从 commit 信息生成 |

#### 自动生成描述模板

```
## 变更概述
[从 commit messages 提取主要变更点]

## 主要修改
- [文件1]: [变更内容]
- [文件2]: [变更内容]

## 影响范围
[根据变更文件推断影响范围]

## 测试建议
[根据变更类型提供测试建议]
```

### 5. 关联工作项

#### 关联策略

| 用户输入 | 关联策略 |
|---------|---------|
| 明确提供工作项 ID | 使用用户提供的 ID |
| 分支名包含编号 | 从分支名提取（如 `feature/PROJ-9998-login` → `PROJ-9998`） |
| 无任何线索 | 中止进程，询问用户 |

#### 完整的工作项关联流程

1. **从分支名推断编号**
   - `feat/PROJ-9999` → `PROJ-9999`
   - `feature/wi-123` → `wi-123`

2. **查询项目空间 ID**
   ```bash
   # 通过项目列表获取 space-id
   yunxiao-cli projex projects search --org-id <ORG_ID> --output json
   
   # 根据分支名中的编号前缀匹配项目 customCode
   # PROJ-9999 → 项目 customCode 为 "PROJ" → space-id: "000000000000000000000000"
   ```

3. **查询工作项详情**
   ```bash
   # 通过序列号查询工作项，获取真实的 workitem-id
   yunxiao-cli projex workitems search \
     --space-id <SPACE_ID> \
     --serial-number <WORKITEM_NUMBER> \
     --org-id <ORG_ID> \
     --output json
   
   # 从返回结果中提取 workitem-id
   # PROJ-9999 → workitem-id: "000000000000000000000001"
   ```

4. **创建 MR 时关联工作项**
   ```bash
   yunxiao-cli codeup mr create \
     --repo-id <REPO_ID> \
     --source <SOURCE_BRANCH> \
     --target <TARGET_BRANCH> \
     --title "<MR_TITLE>" \
     --description "<DESCRIPTION>" \
     --workitem-id <WORKITEM_ID> \
     --org-id <ORG_ID>
   ```

#### 分支名推断规则

常见分支命名模式：
- `feat/PROJ-xxxx-description` → 工作项编号 `PROJ-xxxx` → 项目 `示例云平台`
- `feature/wi-yyy-description` → 工作项编号 `wi-yyy`
- `bugfix/PROJ-zzzz-issue` → 工作项编号 `PROJ-zzzz`

**重要**：推断后必须通过 API 查询获取真实的 workitem-id：

```bash
# 步骤1: 匹配项目 customCode
yunxiao-cli projex projects search --org-id <ORG_ID>

# 步骤2: 查询工作项
yunxiao-cli projex workitems search \
  --space-id <SPACE_ID> \
  --serial-number <SERIAL_NUMBER> \
  --org-id <ORG_ID>
```

#### 无法推断时的询问策略

**中止进程并询问**：
```
未能自动识别关联的工作项。请提供工作项 ID 或编号：
1. 输入工作项 ID（如：wi-xxxxxxxx）
2. 输入"无"跳过工作项关联
3. 输入多个 ID 用逗号分隔（如：wi-xxx,wi-yyy）

等待用户回复后继续...
```

### 6. 组合 MR 标题

根据工作项 ID 和变更内容组合标题：

**固定格式**：`<工作项ID>：<从commit提取的简要描述>`

**示例**：
- `PROJ-9999：示例详情页面重构`
- `wi-123：添加用户认证功能`

**标题提取规则**：
1. 如果用户提供标题 → 直接使用
2. 否则从 commit messages 提取核心变更 → 生成简要描述（15-30字）
3. 组合格式：`<工作项ID>：<简要描述>`

### 7. 创建 MR

使用 yunxiao-cli 创建 MR：

```bash
yunxiao-cli codeup mr create \
  --repo-id <REPO_ID> \
  --source <SOURCE_BRANCH> \
  --target <TARGET_BRANCH> \
  --title "<WORKITEM_ID>：<MR_TITLE>" \
  --description "<DESCRIPTION_FROM_PR_DOCUMENTATION>" \
  --workitem-id <WORKITEM_ID> \
  --org-id <ORG_ID>
```

### 8. 输出结果

创建成功后，输出：
```
✅ MR 创建成功

MR 编号: <MR_ID>
MR 链接: https://codeup.aliyun.com/<ORG_ID>/<REPO_PATH>/change/<MR_ID>
关联工作项: <WORKITEM_NUMBER> (<WORKITEM_ID>)
```

---

## 技能调用参考

此技能依赖 `yunxiao-cli` skill，创建 MR 时使用以下命令：

### 核心命令

```bash
# 列出仓库（获取 repo_id）
yunxiao-cli codeup repos list --org-id <ORG_ID>

# 列出分支（验证分支存在）
yunxiao-cli codeup branches list --repo-id <REPO_ID> --org-id <ORG_ID>

# 查询 commit 历史
yunxiao-cli codeup commits list --repo-id <REPO_ID> --ref-name <BRANCH> --org-id <ORG_ID>

# 比较分支差异
yunxiao-cli codeup compare --repo-id <REPO_ID> --from <FROM> --to <TO> --org-id <ORG_ID>

# 查询项目列表（获取 space-id）
yunxiao-cli projex projects search --org-id <ORG_ID>

# 查询工作项（获取 workitem-id）
yunxiao-cli projex workitems search --space-id <SPACE_ID> --serial-number <SERIAL_NUMBER> --org-id <ORG_ID>

# 创建 MR（关联工作项）
yunxiao-cli codeup mr create --repo-id <REPO_ID> --source <SRC> --target <TGT> \
  --title "<TITLE>" --description "<DESC>" --workitem-id <WORKITEM_ID> --org-id <ORG_ID>
```

### Git 命令辅助

```bash
# 获取当前分支
git branch --show-current

# 获取远程仓库 URL（推断 org_id 和 repo_id）
git remote get-url origin

# 查看本地 commit 历史
git log <TARGET_BRANCH>..<SOURCE_BRANCH> --oneline
```

---

## 示例场景

### 场景 1：用户明确指定所有信息

**用户输入**：
```
创建 MR，从 feat/PROJ-9998-login 合并到 main，标题是"添加登录功能"
```

**处理流程**：
1. 提取：源分支=feat/PROJ-9998-login，目标=main，用户标题=添加登录功能
2. 从分支名推断工作项编号：PROJ-9998
3. 查询项目列表，匹配 customCode "PROJ" → space-id: "xxx"
4. 查询工作项 PROJ-9998 → workitem-id: "yyy"
5. 查询 git log 和 diff 获取变更信息
6. **调用 pr-documentation 技能生成描述**（提供 commit 信息和工作项背景）
7. 组合标题：`PROJ-9998：添加登录功能`
8. 创建 MR 并关联工作项
9. 输出编号、链接和关联的工作项

### 场景 2：用户只说"创建 MR"

**用户输入**：
```
创建 MR
```

**处理流程**：
1. 查询当前分支：`git branch --show-current` → feat/PROJ-9997-api
2. 默认目标分支：master（或从 git remote 确定）
3. 从分支名推断工作项编号：PROJ-9997
4. 查询项目列表，匹配 customCode "PROJ" → space-id: "xxx"
5. 查询工作项 PROJ-9997 → workitem-id: "yyy"
6. 查询 git log 和 diff 获取 commit 信息
7. **调用 pr-documentation 技能生成描述和标题**（提供完整 git 变更信息）
8. 组合标题：`PROJ-9997：<从commit提取的标题>`
9. 创建 MR 并关联工作项
10. 输出编号、链接和关联的工作项

### 场景 3：用户提供部分描述

**用户输入**：
```
创建 MR，这个功能是新的支付模块，主要是为了支持微信支付
```

**处理流程**：
1. 查询当前分支
2. 从分支名推断工作项编号
3. 查询工作项详情
4. 查询 git log 和 diff 获取变更细节
5. **调用 pr-documentation 技能生成描述**（提供用户概述 + git 变更信息）
   - pr-documentation 会结合："新的支付模块，支持微信支付"（用户意图）
   - + 技术细节（commit diff、新增文件、接口调用等）
6. 组合标题：`<工作项ID>：新的支付模块，支持微信支付`
7. 创建 MR 并关联工作项
8. 输出编号、链接和关联的工作项

### 场景 4：无法推断工作项

**用户输入**：
```
创建 MR
```

**情况**：
- 当前分支：feature/refactor-code
- 无法从分支名推断工作项编号

**处理**：
```
未能自动识别关联的工作项。请提供工作项 ID 或编号：
1. 输入工作项 ID（如：wi-xxxxxxxx）
2. 输入"无"跳过工作项关联
3. 输入多个 ID 用逗号分隔

等待用户回复...
```

用户回复：`wi-789`

继续流程，创建 MR...

---

## 错误处理

### 分支不存在

```
❌ 源分支 <branch> 不存在

请确认分支名称，或使用以下命令创建分支：
yunxiao-cli codeup branches create --repo-id <REPO_ID> --branch <branch> --ref main
```

### 工作项不存在

```
❌ 工作项 <workitem_id> 不存在

请检查工作项 ID 是否正确，或跳过工作项关联。
```

### 权限不足

```
❌ 无权限访问仓库 <repo_id>

请确认：
1. 组织 ID 是否正确
2. 是否有仓库访问权限
3. 是否已配置认证信息（yunxiao-cli auth）
```

---

## 重要提醒

1. **优先使用上下文信息**：尽量从对话历史、git 信息、配置文件中推断参数
2. **智能询问**：只在真正无法获取必要信息时才询问用户
3. **验证推断结果**：推断的工作项 ID 需通过 API 验证存在性
4. **描述要有价值**：自动生成的描述应包含变更概述、主要修改、影响范围、测试建议
5. **输出简洁**：只返回 MR 编号和链接，不需要返回完整的 MR 详情