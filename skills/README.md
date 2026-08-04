# Yunxiao CLI Skills

云效（阿里云 DevOps）命令行工具的 Agent 技能集合。

## 可用技能

### yunxiao-cli
云效命令行工具的完整使用指南，支持项目查询、代码仓库、流水线、工作项管理等操作。

**触发词**: 云效、阿里云DevOps、项目管理、代码仓库、流水线、工作项、迭代、版本等

### yunxiao-create-requirement
在云效 Projex 上创建需求的完整工作流指导。

**触发词**: 创建需求、新建需求、提一个需求、录入需求等

## 安装

```bash
# 安装所有技能
npx skills add your-username/yunxiao-cli --all

# 安装特定技能
npx skills add your-username/yunxiao-cli --skill yunxiao-cli
npx skills add your-username/yunxiao-cli --skill yunxiao-create-requirement

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

建议在项目根目录创建 `YUNXIAO.md` 文件，预定义常用的项目 ID 和配置：

```markdown
# 云效项目配置

## 基本信息
- 组织 ID: org-xxxxxxxx
- 项目 ID: proj-xxxxxxxx

## 常用 ID
- 需求类型 ID: type-xxx
- 当前迭代 ID: sprint-xxx
```

这样技能会优先使用这些预定义的值，减少重复查询。
