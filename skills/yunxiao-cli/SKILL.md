---
name: yunxiao-cli
description: 云效命令行工具。用于查询云效数据、管理项目、操作代码仓库、触发流水线或查看测试结果。
cli_version: ">=0.1.0"
---

# YunXiao CLI Skill

## 用途

云效命令行工具。

## 详细手册

完整命令手册请参阅 [commands/](./commands/) 目录下的各模块文档：

- [auth](./commands/auth.md) - 认证管理
- [org](./commands/org.md) - 组织管理
- [projex](./commands/projex.md) - 项目协作
- [codeup](./commands/codeup.md) - 代码管理
- [flow](./commands/flow.md) - 流水线
- [appstack](./commands/appstack.md) - 应用交付
- [packages](./commands/packages.md) - 制品库
- [testhub](./commands/testhub.md) - 测试管理
- [insight](./commands/insight.md) - 效能洞察
- [thoughts](./commands/thoughts.md) - 知识库
- [config](./commands/config.md) - 配置管理
- [completion](./commands/completion.md) - Shell 补全

## Help 命令

如果本手册未找到所需信息，可使用 `--help` 获取命令帮助。命令可能有多级子命令，可以逐级尝试：

```bash
yunxiao-cli --help                    # 查看全局帮助
yunxiao-cli <command> --help          # 查看子命令帮助
yunxiao-cli <command> <sub> --help    # 查看更深层级的帮助
```

## 配置说明

详见 [configuration.md](./configuration.md)。