# Yunxiao CLI Skills

云效（阿里云 DevOps）命令行工具的 Agent 技能集合。

## 可用技能

### yunxiao-cli
云效命令行工具的完整使用指南，支持项目查询、代码仓库、流水线、工作项管理等操作。

**触发词**: 云效、阿里云DevOps、项目管理、代码仓库、流水线、工作项、迭代、版本等

### yunxiao-setup
交互式创建项目级云效配置：随仓库提交的 YUNXIAO.md（共享）与被版本库忽略的 .yunxiao/local.md（本地），并提供旧单文件迁移。

**触发词**: 创建 YUNXIAO.md、初始化云效项目配置、yunxiao init、设置云效项目、配置云效

### yunxiao-create-requirement
在云效 Projex 上创建需求的完整工作流指导。

**触发词**: 创建需求、新建需求、提一个需求、录入需求等

### create-yunxiao-flow
安全创建云效 Flow 流水线：生成本地 YAML、明确选择 Codeup 服务连接，并在创建后只读验证。

**触发词**: 创建流水线、新建流水线、创建 Flow 流水线

### yunxiao-flow-step-query
查询云效 Flow YAML 内置 Step 类型、分类、官方说明和 `with` 参数文档。

**触发词**: 查询步骤类型、Step 类型、流水线步骤、Flow steps、选择构建步骤

### yunxiao-feedback
收集当前会话中 yunxiao CLI 及其技能的使用问题，梳理成结构化反馈报告并保存（默认存系统临时目录，可指定路径）。

**触发词**: 记录云效问题、收集使用反馈、把这个问题记下来、反馈给 yunxiao-cli（仅人为触发，不自动收集）

## 安装

```bash
# 安装所有技能
npx skills add your-username/yunxiao-cli --all

# 安装特定技能
npx skills add your-username/yunxiao-cli --skill yunxiao-cli
npx skills add your-username/yunxiao-cli --skill yunxiao-create-requirement
npx skills add your-username/yunxiao-cli --skill create-yunxiao-flow
npx skills add your-username/yunxiao-cli --skill yunxiao-flow-step-query

# 全局安装
npx skills add your-username/yunxiao-cli -g
```

## 使用

安装后，在 Claude Code 中直接使用相关功能，技能会自动触发。

例如：
- "帮我查询云效项目列表"
- "创建一个新需求"
- "查看流水线状态"

## 项目配置

项目上下文配置分两层：随仓库提交的 `YUNXIAO.md`（共享配置，团队共识的稳定信息）和被版本库忽略的 `.yunxiao/local.md`（本地配置，个人易变缓存）。忽略规则由随仓库提交的 `.yunxiao/.gitignore`（内容为 `*` 和 `!.gitignore`）分发。

```markdown
# YUNXIAO.md（共享，随仓库提交）
## 基本信息
- 组织 ID: org-xxxxxxxx
- 项目 ID: proj-xxxxxxxx

## 工作项类型
- 需求类型 ID: type-xxx
```

```markdown
# .yunxiao/local.md（本地，不入库）
## 迭代与版本
- 当前迭代 ID: sprint-xxx
```

这样技能会优先使用这些预定义的值，减少重复查询。字段归属与冲突规则详见 yunxiao-cli 技能；初始化与旧文件迁移使用 yunxiao-setup 技能。
