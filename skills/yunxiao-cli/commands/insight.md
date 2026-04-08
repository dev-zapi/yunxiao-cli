# 效能洞察命令手册

`yunxiao insight` 命令用于查看效能分析数据。

---

## 命令列表

| 命令 | 说明 |
|------|------|
| `yunxiao insight metrics` | 查看效能指标 |

---

## 查看效能指标

### 基本用法

```bash
yunxiao insight metrics --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |

### 示例

```bash
yunxiao insight metrics --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看组织效能指标

```bash
yunxiao insight metrics --org-id org-xxx
```

---

## 说明

效能洞察模块的具体参数和功能可能根据云效 API 实际情况调整，请参考：

- 云效官方文档
- `yunxiao insight --help` 命令输出