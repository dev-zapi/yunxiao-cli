---
name: yunxiao-cli
description: 阿里云云效（YunXiao）DevOps 平台的命令行工具，用于管理完整的 DevOps 生命周期，包括认证、组织、项目、代码、流水线、应用、制品库、测试、效能分析和知识库等。当用户需要查询云效数据、管理项目、操作代码仓库、触发流水线或查看测试结果时使用此工具。
cli_version: ">=0.1.0"
---

# YunXiao CLI Skill

## 用途

阿里云云效（YunXiao）DevOps 平台的命令行工具，用于通过终端管理完整的 DevOps 生命周期。

## 使用前提

1. 登录云效控制台获取个人访问令牌（PAT）
2. 使用以下方式之一配置认证：
   - 环境变量: `export YUNXIAO_CLI_TOKEN=<your_token>`
   - 配置文件: `~/.config/yunxiao/config.toml`

## 详细手册

完整命令手册请参阅 [commands/](./commands/) 目录下的各模块文档。

## 快速开始

详见 [getting-started.md](./getting-started.md)。

## 配置说明

详见 [configuration.md](./configuration.md)。