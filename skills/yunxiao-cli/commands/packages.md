# 制品库命令手册

`yunxiao packages` 命令用于管理制品仓库和构建产物。

---

## 命令列表

### 仓库管理

| 命令 | 说明 |
|------|------|
| `yunxiao packages repos list` | 列出制品仓库 |

### 制品管理

| 命令 | 说明 |
|------|------|
| `yunxiao packages artifacts list` | 列出制品版本 |

---

## 列出制品仓库

### 基本用法

```bash
yunxiao packages repos list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao packages repos list --org-id org-xxxxxxxx
```

---

## 列出制品版本

### 埍本用法

```bash
yunxiao packages artifacts list --repo-id <REPO_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--repo-id` | 制品仓库 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao packages artifacts list --repo-id repo-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看组织制品仓库

```bash
yunxiao packages repos list --org-id org-xxx
```

### 查看仓库中的制品版本

```bash
# 列出仓库
yunxiao packages repos list --org-id org-xxx

# 选择仓库
REPO_ID=$(yunxiao packages repos list --org-id org-xxx --output json | jq -r '.[0].id')

# 列出制品版本
yunxiao packages artifacts list --repo-id $REPO_ID --org-id org-xxx
```

### 查看特定制品详情

```bash
yunxiao packages artifacts list --repo-id repo-xxx --org-id org-xxx --output json | jq '.[] | select(.version == "1.0.0")'
```

---

## 故障排查

### "Repository not found"

**原因**: 仓库 ID 错误或无权限

**解决方案**:
```bash
# 搜索仓库
yunxiao packages repos list --org-id org-xxx
```

### "Artifacts not found"

**原因**: 仓库中无制品或参数错误

**解决方案**:
```bash
# 确认仓库 ID
yunxiao packages repos list --org-id org-xxx

# 列出制品
yunxiao packages artifacts list --repo-id repo-xxx --org-id org-xxx
```