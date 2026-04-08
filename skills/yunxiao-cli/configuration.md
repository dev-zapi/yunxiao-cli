# 配置说明

详细说明 YunXiao CLI 的配置系统、优先级和参数。

---

## 配置优先级

配置按以下优先级从高到低生效：

| 优先级 | 配置来源 | 示例 |
|--------|----------|------|
| 1（最高） | 命令行参数 | `--output json`, `--timeout 60` |
| 2 | 环境变量 | `YUNXIAO_CLI_TOKEN`, `YUNXIAO_CLI_OUTPUT` |
| 3 | 配置文件 | `~/.config/yunxiao/config.toml` |
| 4（最低） | 默认值 | `json`, `30秒`, `warn` |

---

## 全局参数

所有命令都支持以下全局参数：

| 参数 | 环境变量 | 说明 | 默认值 |
|------|---------|------|--------|
| `-o, --output` | `YUNXIAO_CLI_OUTPUT` | 输出格式：json, text, table, markdown | json |
| `--timeout` | `YUNXIAO_CLI_TIMEOUT` | API 请求超时时间（秒） | 30 |
| `--log-level` | `YUNXIAO_CLI_LOG_LEVEL` | 日志级别：debug, info, warn, error | warn |
| `--token` | `YUNXIAO_CLI_TOKEN` | 个人访问令牌 | 无 |
| `--endpoint` | `YUNXIAO_CLI_ENDPOINT` | API 端点地址 | openapi-rdc.aliyuncs.com |
| `--org-id` | `YUNXIAO_CLI_ORG_ID` | 组织 ID | 无 |

---

## 配置文件

### 配置文件路径

- **Linux/macOS**: `~/.config/yunxiao/config.toml`
- **Windows**: `%APPDATA%\yunxiao\config.toml`

查看配置文件路径：
```bash
yunxiao config path
```

### 配置文件示例

```toml
# 个人访问令牌
token = "pt_xxxxxxxxxxxxxxxxxxxx"

# API 端点域名（默认：openapi-rdc.aliyuncs.com）
domain = "openapi-rdc.aliyuncs.com"

# 默认组织 ID
organization_id = "org-xxxxxxxx"

# 默认输出格式：json | text | table | markdown
default_output = "table"

# 日志级别：debug | info | warn | error
log_level = "warn"

# 请求超时时间（秒，默认 30）
timeout = 30
```

---

## 配置管理命令

### 查看配置

```bash
# 查看所有配置
yunxiao config list

# 查看配置文件路径
yunxiao config path

# 获取单个配置项
yunxiao config get token
yunxiao config get default_output
yunxiao config get organization_id
yunxiao config get timeout
yunxiao config get log_level
yunxiao config get domain
```

### 设置配置

```bash
# 设置输出格式
yunxiao config set default_output table

# 设置超时时间（秒）
yunxiao config set timeout 60

# 设置组织 ID
yunxiao config set organization_id <ORG_ID>

# 设置日志级别
yunxiao config set log_level debug

# 设置令牌
yunxiao config set token pt_xxxxxxxxxxxxxxxx
```

### 删除配置

```bash
# 删除配置项（恢复为默认值或环境变量）
yunxiao config delete organization_id
yunxiao config delete token
```

---

## 配置项详解

### token（认证令牌）

**用途**: 个人访问令牌，用于 API 认证

**获取方式**: [快速开始](./getting-started.md#获取个人访问令牌)

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_TOKEN="pt_xxx"

# 配置文件
yunxiao config set token pt_xxx

# 命令行参数
yunxiao --token pt_xxx
```

### organization_id（组织 ID）

**用途**: 默认组织 ID，避免每个命令重复指定

**查看组织 ID**:
```bash
yunxiao org list
```

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_ORG_ID="org-xxx"

# 配置文件
yunxiao config set organization_id org-xxx

# 命令行参数
yunxiao --org-id org-xxx
```

### default_output（输出格式）

**用途**: 默认输出格式

**可选值**:
- `json` - JSON 格式，完整数据，适合脚本处理
- `table` - 表格格式，人类可读，适合终端查看
- `text` - 文本格式，简洁输出
- `markdown` - Markdown 格式，适合文档

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_OUTPUT="table"

# 配置文件
yunxiao config set default_output table

# 命令行参数
yunxiao --output table
```

### timeout（超时时间）

**用途**: API 请求超时时间（秒）

**默认值**: 30

**适用场景**: 网络较慢或处理大量数据时

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_TIMEOUT="60"

# 配置文件
yunxiao config set timeout 60

# 命令行参数
yunxiao --timeout 60
```

### log_level（日志级别）

**用途**: 控制日志输出详细程度

**可选值**:
- `debug` - 详细调试信息
- `info` - 一般信息
- `warn` - 警告信息（默认）
- `error` - 仅错误信息

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_LOG_LEVEL="debug"

# 配置文件
yunxiao config set log_level debug

# 命令行参数
yunxiao --log-level debug
```

### domain（API 端点）

**用途**: 云效 API 服务域名

**默认值**: `openapi-rdc.aliyuncs.com`

**适用场景**: 使用私有部署或特殊端点

**配置方式**:
```bash
# 环境变量
export YUNXIAO_CLI_ENDPOINT="custom.example.com"

# 配置文件
yunxiao config set domain custom.example.com

# 命令行参数
yunxiao --endpoint custom.example.com
```

---

## 多环境配置

### 使用环境变量切换环境

```bash
# 开发环境
export YUNXIAO_CLI_ORG_ID="org-dev"
export YUNXIAO_CLI_TOKEN="pt-dev-token"

# 生产环境
export YUNXIAO_CLI_ORG_ID="org-prod"
export YUNXIAO_CLI_TOKEN="pt-prod-token"
```

### 使用配置文件切换

创建多个配置文件：
```bash
# 开发配置
~/.config/yunxiao/config.dev.toml

# 生产配置
~/.config/yunxiao/config.prod.toml
```

切换配置：
```bash
# 使用符号链接
ln -sf ~/.config/yunxiao/config.dev.toml ~/.config/yunxiao/config.toml
```

---

## 安全建议

### 不要提交令牌到代码仓库

```bash
# 添加到 .gitignore
echo "config.toml" >> .gitignore
```

### 使用环境变量管理敏感信息

```bash
# 在 ~/.bashrc 或 ~/.zshrc 中
export YUNXIAO_CLI_TOKEN="pt_xxx"

# 或使用 .env 文件（不提交到仓库）
echo "YUNXIAO_CLI_TOKEN=pt_xxx" >> .env
```

### 定期更新令牌

在云效控制台定期更新或撤销旧令牌。

---

## 故障排查

### 配置文件不存在

```bash
yunxiao config path
# 手动创建
mkdir -p ~/.config/yunxiao
```

### 配置项未生效

检查优先级：
```bash
# 查看当前配置
yunxiao config list

# 检查环境变量
env | grep YUNXIAO_CLI
```

### 令牌失效

重新获取令牌并更新配置：
```bash
yunxiao config set token pt_new_token
```