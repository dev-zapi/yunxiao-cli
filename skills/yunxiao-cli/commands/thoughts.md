# 知识库命令手册

`yunxiao thoughts` 命令用于管理文档和页面。

---

## 命令列表

### 文档管理

| 命令 | 说明 |
|------|------|
| `yunxiao thoughts documents list` | 列出文档 |

### 页面管理

| 命令 | 说明 |
|------|------|
| `yunxiao thoughts pages list` | 查看页面 |

---

## 列出文档

### 基本用法

```bash
yunxiao thoughts documents list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao thoughts documents list --org-id org-xxxxxxxx
```

---

## 查看页面

### 基本用法

```bash
yunxiao thoughts pages list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao thoughts pages list --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看组织知识库

```bash
# 列出文档
yunxiao thoughts documents list --org-id org-xxx

# 查看页面
yunxiao thoughts pages list --org-id org-xxx
```

---

## 说明

知识库模块的具体参数和功能可能根据云效 API 实际情况调整，请参考：

- 云效官方文档
- `yunxiao thoughts --help` 命令输出