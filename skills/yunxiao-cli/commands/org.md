# 组织命令手册

`yunxiao org` 命令用于管理组织和成员。

---

## 命令列表

| 命令 | 说明 |
|------|------|
| `yunxiao org list` | 列出所有组织 |
| `yunxiao org info` | 查看组织详情 |
| `yunxiao org members list` | 列出成员 |
| `yunxiao org members search` | 搜索成员 |
| `yunxiao org members get` | 查看成员详情 |
| `yunxiao org departments list` | 列出部门 |
| `yunxiao org roles list` | 列出角色 |

---

## 列出组织

### 基本用法

```bash
yunxiao org list
```

### 输出示例

```
组织列表:
  1. org-xxxxxxxx - 云效演示组织
  2. org-yyyyyyyy - 研发团队
```

### 参数说明

无额外参数。

---

## 查看组织详情

### 基本用法

```bash
yunxiao org info --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao org info --org-id org-xxxxxxxx
```

### 输出示例

```
组织详情:
  ID: org-xxxxxxxx
  名称: 云效演示组织
  创建时间: 2024-01-01
  成员数量: 50
```

---

## 列出成员

### 基本用法

```bash
yunxiao org members list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao org members list --org-id org-xxxxxxxx
yunxiao org members list --org-id org-xxxxxxxx --page 2 --per-page 50
```

---

## 搜索成员

### 基本用法

```bash
yunxiao org members search --query <query> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--query` | 搜索关键词 | 是 |

### 示例

```bash
# 按用户名搜索
yunxiao org members search --query "alice" --org-id org-xxxxxxxx

# 按邮箱搜索
yunxiao org members search --query "alice@example.com" --org-id org-xxxxxxxx
```

---

## 查看成员详情

### 基本用法

```bash
yunxiao org members get --user-id <USER_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--user-id` | 用户 ID | 是 |

### 示例

```bash
yunxiao org members get --user-id user-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 列出部门

### 基本用法

```bash
yunxiao org departments list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao org departments list --org-id org-xxxxxxxx
```

---

## 列出角色

### 基本用法

```bash
yunxiao org roles list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao org roles list --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看所有组织并设置默认组织

```bash
# 列出组织
yunxiao org list

# 选择组织并设置为默认
yunxiao config set organization_id org-xxxxxxxx

# 验证
yunxiao config get organization_id
```

### 搜索团队成员

```bash
# 搜索成员
yunxiao org members search --query "张三" --org-id org-xxxxxxxx

# 查看成员详情
yunxiao org members get --user-id user-xxx --org-id org-xxxxxxxx
```

### 查看组织结构

```bash
# 查看部门
yunxiao org departments list --org-id org-xxxxxxxx

# 查看角色
yunxiao org roles list --org-id org-xxxxxxxx

# 查看成员
yunxiao org members list --org-id org-xxxxxxxx
```

---

## 故障排查

### "Organization ID required"

**原因**: 未指定组织 ID

**解决方案**:
```bash
# 设置默认组织
yunxiao config set organization_id <ORG_ID>

# 或在命令中指定
yunxiao org members list --org-id <ORG_ID>
```

### "Member not found"

**原因**: 搜索关键词不匹配

**解决方案**:
```bash
# 尝试不同的关键词
yunxiao org members search --query "alice" --org-id org-xxx

# 或列出所有成员
yunxiao org members list --org-id org-xxx
```