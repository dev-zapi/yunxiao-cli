# 认证命令手册

`yunxiao auth` 命令用于管理认证状态。

---

## 命令列表

| 命令 | 说明 |
|------|------|
| `yunxiao auth whoami` | 查看当前登录用户 |
| `yunxiao auth status` | 查看认证状态 |
| `yunxiao auth logout` | 登出（清除本地 token） |

---

## 查看当前用户

### 基本用法

```bash
yunxiao auth whoami
```

### 输出示例

```
当前登录用户:
  用户ID: user-xxxxxxxx
  用户名: alice
  组织: org-xxxxxxxx
```

### 参数说明

无额外参数，使用全局配置的 token。

---

## 查看认证状态

### 基本用法

```bash
yunxiao auth status
```

### 输出示例

```
认证状态: 已认证
令牌来源: 配置文件 (~/.config/yunxiao/config.toml)
令牌有效期: 2026-12-31
```

### 参数说明

无额外参数。

---

## 登出

### 基本用法

```bash
yunxiao auth logout
```

### 说明

- 清除本地配置文件中的 token
- 不影响云效平台上的令牌状态
- 需要重新配置 token 才能继续使用

### 输出示例

```
已登出，本地令牌已清除。
如需继续使用，请重新配置令牌:
  yunxiao config set token <your_token>
```

---

## 常见用法

### 检查认证是否有效

```bash
yunxiao auth whoami
```

如果返回用户信息，表示认证有效。

### 切换用户

```bash
# 登出当前用户
yunxiao auth logout

# 配置新用户的令牌
yunxiao config set token <new_token>

# 验证新用户
yunxiao auth whoami
```

### 查看令牌来源

```bash
yunxiao auth status
```

查看令牌是从配置文件、环境变量还是命令行参数加载的。

---

## 故障排查

### "Authentication error"

**原因**: 令牌无效或过期

**解决方案**:
```bash
# 检查当前令牌
yunxiao config get token

# 更新令牌
yunxiao config set token <new_token>

# 验证
yunxiao auth whoami
```

### "Token not found"

**原因**: 未配置令牌

**解决方案**:
```bash
# 配置令牌
yunxiao config set token pt_xxxxxxxxxxxxxxxx

# 或使用环境变量
export YUNXIAO_CLI_TOKEN="pt_xxxxxxxxxxxxxxxx"
```