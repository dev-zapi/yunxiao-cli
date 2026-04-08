# 流水线命令手册

`yunxiao flow` 命令用于管理流水线、触发运行和查看日志。

---

## 命令列表

### 流水线管理

| 命令 | 说明 |
|------|------|
| `yunxiao flow pipelines list` | 列出流水线 |
| `yunxiao flow pipelines get` | 查看流水线详情 |

### 运行管理

| 命令 | 说明 |
|------|------|
| `yunxiao flow runs create` | 触发流水线运行 |
| `yunxiao flow runs list` | 列出运行记录 |
| `yunxiao flow runs latest` | 查看最新运行 |

### 任务日志

| 命令 | 说明 |
|------|------|
| `yunxiao flow jobs log` | 查看任务日志 |

---

## 列出流水线

### 基本用法

```bash
yunxiao flow pipelines list --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao flow pipelines list --org-id org-xxxxxxxx
```

---

## 查看流水线详情

### 基本用法

```bash
yunxiao flow pipelines get --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow pipelines get --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 触发流水线运行

### 基本用法

```bash
yunxiao flow runs create --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow runs create --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 列出运行记录

### 基本用法

```bash
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow runs list --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 查看最新运行

### 基本用法

```bash
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow runs latest --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 查看任务日志

### 基本用法

```bash
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--run-id` | 运行 ID | 是 |
| `--job-id` | 任务 ID | 是 |

### 示例

```bash
yunxiao flow jobs log --pipeline-id pipeline-xxxxxxxx --run-id run-xxxxxxxx --job-id job-xxxxxxxx --org-id org-xxxxxxxx
```

---

## 常见用法

### 触发构建并查看结果

```bash
# 1. 列出流水线
yunxiao flow pipelines list --org-id org-xxx

# 2. 触发运行
yunxiao flow runs create --pipeline-id pipeline-xxx --org-id org-xxx

# 3. 查看最新运行状态
yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx

# 4. 查看运行记录
yunxiao flow runs list --pipeline-id pipeline-xxx --org-id org-xxx
```

### 查看构建失败原因

```bash
# 1. 查看最新运行
yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx

# 2. 获取运行 ID 和失败任务 ID
RUN_ID=$(yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.id')
JOB_ID=$(yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.jobs[0].id')

# 3. 查看任务日志
yunxiao flow jobs log --pipeline-id pipeline-xxx --run-id $RUN_ID --job-id $JOB_ID --org-id org-xxx
```

### 监控流水线状态

```bash
# 循环查看最新运行状态
while true; do
  yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.status'
  sleep 10
done
```

---

## 故障排查

### "Pipeline not found"

**原因**: 流水线 ID 错误或无权限

**解决方案**:
```bash
# 搜索流水线
yunxiao flow pipelines list --org-id org-xxx
```

### "Run not found"

**原因**: 运行 ID 错误

**解决方案**:
```bash
# 列出运行记录
yunxiao flow runs list --pipeline-id pipeline-xxx --org-id org-xxx
```

### "Job not found"

**原因**: 任务 ID 错误或任务已完成

**解决方案**:
```bash
# 查看运行详情获取任务 ID
yunxiao flow runs get --pipeline-id pipeline-xxx --run-id run-xxx --org-id org-xxx
```