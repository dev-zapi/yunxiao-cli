# 测试管理命令手册

`yunxiao testhub` 命令用于管理测试用例、测试计划和测试结果。

---

## 命令列表

### 测试用例

| 命令 | 说明 |
|------|------|
| `yunxiao testhub cases search` | 搜索测试用例 |

### 测试计划

| 命令 | 说明 |
|------|------|
| `yunxiao testhub plans list` | 列出测试计划 |
| `yunxiao testhub plans results list` | 查看测试结果 |

---

## 搜索测试用例

### 基本用法

```bash
yunxiao testhub cases search --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--keyword` | 搜索关键词 | 否 |

### 示例

```bash
# 搜索所有测试用例
yunxiao testhub cases search --space-id proj-xxxxxxxx --org-id org-xxxxxxxx

# 搜索特定关键词
yunxiao testhub cases search --space-id proj-xxxxxxxx --keyword "登录" --org-id org-xxxxxxxx
```

---

## 列出测试计划

### 基本用法

```bash
yunxiao testhub plans list --space-id <PROJECT_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |

### 示例

```bash
yunxiao testhub plans list --space-id proj-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 查看测试结果

### 基本用法

```bash
yunxiao testhub plans results list --space-id <PROJECT_ID> --plan-id <PLAN_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--space-id` | 项目 ID | 是 |
| `--plan-id` | 测试计划 ID | 是 |

### 示例

```bash
yunxiao testhub plans results list --space-id proj-xxxxxxxx --plan-id plan-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 常见用法

### 查看项目测试概览

```bash
# 搜索测试用例
yunxiao testhub cases search --space-id proj-xxx --org-id org-xxx

# 列出测试计划
yunxiao testhub plans list --space-id proj-xxx --org-id org-xxx
```

### 查看测试计划执行结果

```bash
# 列出测试计划
yunxiao testhub plans list --space-id proj-xxx --org-id org-xxx

# 选择计划
PLAN_ID=$(yunxiao testhub plans list --space-id proj-xxx --org-id org-xxx --output json | jq -r '.[0].id')

# 查看测试结果
yunxiao testhub plans results list --space-id proj-xxx --plan-id $PLAN_ID --org-id org-xxx
```

### 分析测试结果

```bash
# 查看测试结果
yunxiao testhub plans results list --space-id proj-xxx --plan-id plan-xxx --org-id org-xxx --output json | jq '.[] | select(.status == "FAILED")'
```

---

## 故障排查

### "Space ID required"

**原因**: 未指定项目 ID

**解决方案**:
```bash
# 获取项目 ID
yunxiao projex projects search --org-id org-xxx

# 使用项目 ID
yunxiao testhub cases search --space-id proj-xxx --org-id org-xxx
```

### "Plan not found"

**原因**: 测试计划 ID 错误

**解决方案**:
```bash
# 列出测试计划
yunxiao testhub plans list --space-id proj-xxx --org-id org-xxx
```