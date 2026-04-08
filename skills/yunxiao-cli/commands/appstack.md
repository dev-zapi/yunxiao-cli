# 应用交付命令手册

`yunxiao appstack` 命令用于管理应用、环境变量和部署。

---

## 命令列表

### 应用管理

| 命令 | 说明 |
|------|------|
| `yunxiao appstack apps list` | 列出应用 |
| `yunxiao appstack apps get` | 查看应用详情 |

### 变量管理

| 命令 | 说明 |
|------|------|
| `yunxiao appstack vars list` | 列出环境变量 |

### 部署管理

| 命令 | 说明 |
|------|------|
| `yunxiao appstack deploy create` | 创建部署 |

---

## 列出应用

### 基本用法

```bash
yunxiao appstack apps list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao appstack apps list --org-id org-xxxxxxxx
```

---

## 查看应用详情

### 基本用法

```bash
yunxiao appstack apps get --app-name <APP_NAME> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--app-name` | 应用名称 | 是 |

### 示例

```bash
yunxiao appstack apps get --app-name my-app --org-id org-xxxxxxxx
```

---

## 列出环境变量

### 基本用法

```bash
yunxiao appstack vars list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao appstack vars list --org-id org-xxxxxxxx
```

---

## 创建部署

### 基本用法

```bash
yunxiao appstack deploy create --app-name <APP_NAME> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--app-name` | 应用名称 | 是 |

### 示例

```bash
yunxiao appstack deploy create --app-name my-app --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看应用列表

```bash
yunxiao appstack apps list --org-id org-xxx
```

### 查看应用详情和配置

```bash
# 查看应用详情
yunxiao appstack apps get --app-name my-app --org-id org-xxx

# 查看环境变量
yunxiao appstack vars list --org-id org-xxx
```

### 触发部署

```bash
# 列出应用
yunxiao appstack apps list --org-id org-xxx

# 创建部署
yunxiao appstack deploy create --app-name my-app --org-id org-xxx
```

---

## 故障排查

### "Application not found"

**原因**: 应用名称错误或无权限

**解决方案**:
```bash
# 搜索应用
yunxiao appstack apps list --org-id org-xxx
```

### "Deploy failed"

**原因**: 应用配置错误或环境问题

**解决方案**:
```bash
# 查看应用详情
yunxiao appstack apps get --app-name my-app --org-id org-xxx

# 检查环境变量
yunxiao appstack vars list --org-id org-xxx
```