# 配置命令手册

`yunxiao config` 命令用于管理 CLI 配置。

---

## 命令列表

| 命令 | 说明 |
|------|------|
| `yunxiao config list` | 查看所有配置 |
| `yunxiao config get <key>` | 获取单个配置项 |
| `yunxiao config set <key> <value>` | 设置配置项 |
| `yunxiao config delete <key>` | 删除配置项 |
| `yunxiao config path` | 查看配置文件路径 |

---

## 查看所有配置

### 基本用法

```bash
yunxiao config list
```

### 输出示例

```
配置文件: ~/.config/yunxiao/config.toml

配置项:
  token = pt_xxxxxxxxxxxxxxxx
  organization_id = org-xxxxxxxx
  default_output = table
  timeout = 30
  log_level = warn
  domain = openapi-rdc.aliyuncs.com
```

---

## 获取单个配置项

### 基本用法

```bash
yunxiao config get <key>
```

### 可用配置项

| 配置项 | 说明 |
|--------|------|
| `token` | 个人访问令牌 |
| `organization_id` | 默认组织 ID |
| `default_output` | 默认输出格式 |
| `timeout` | 超时时间（秒） |
| `log_level` | 日志级别 |
| `domain` | API 端点域名 |

### 示例

```bash
# 查看令牌
yunxiao config get token

# 查看默认组织
yunxiao config get organization_id

# 查看输出格式
yunxiao config get default_output

# 查看超时时间
yunxiao config get timeout
```

---

## 设置配置项

### 基本用法

```bash
yunxiao config set <key> <value>
```

### 示例

```bash
# 设置令牌
yunxiao config set token pt_xxxxxxxxxxxxxxxx

# 设置组织 ID
yunxiao config set organization_id org-xxxxxxxx

# 设置输出格式为表格
yunxiao config set default_output table

# 设置超时时间为 60 秒
yunxiao config set timeout 60

# 设置日志级别
yunxiao config set log_level debug
```

---

## 删除配置项

### 基本用法

```bash
yunxiao config delete <key>
```

### 说明

删除配置项后，该配置项恢复为：
- 默认值（如果存在）
- 或依赖环境变量
- 或依赖命令行参数

### 示例

```bash
# 删除组织 ID
yunxiao config delete organization_id

# 删除令牌
yunxiao config delete token
```

---

## 查看配置文件路径

### 基本用法

```bash
yunxiao config path
```

### 输出示例

```
配置文件路径: ~/.config/yunxiao/config.toml
```

### 说明

- Linux/macOS: `~/.config/yunxiao/config.toml`
- Windows: `%APPDATA%\yunxiao\config.toml`

---

## 常见用法

### 初始化配置

```bash
# 查看配置文件路径
yunxiao config path

# 设置令牌
yunxiao config set token pt_xxx

# 设置默认组织
yunxiao config set organization_id org-xxx

# 设置默认输出格式
yunxiao config set default_output table

# 验证配置
yunxiao config list
```

### 临时切换组织

```bash
# 保存当前组织
OLD_ORG=$(yunxiao config get organization_id)

# 切换到其他组织
yunxiao config set organization_id org-xxx

# 执行操作...

# 恢复原组织
yunxiao config set organization_id $OLD_ORG
```

### 查看配置来源

```bash
yunxiao config list
```

配置项显示 "环境变量" 或 "配置文件" 表示来源。

---

## 故障排查

### "Configuration file not found"

**原因**: 配置文件不存在

**解决方案**:
```bash
# 创建配置目录
mkdir -p ~/.config/yunxiao

# 设置任意配置项会自动创建文件
yunxiao config set default_output table
```

### "Invalid config key"

**原因**: 配置项名称错误

**解决方案**:
检查可用配置项列表：
```bash
yunxiao config list
```

### 配置项未生效

**原因**: 环境变量优先级更高

**解决方案**:
```bash
# 检查环境变量
env | grep YUNXIAO_CLI

# 清除环境变量或更新配置文件
unset YUNXIAO_CLI_OUTPUT
```