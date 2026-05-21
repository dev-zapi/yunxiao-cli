# 流水线命令手册

`yunxiao flow` 命令用于管理流水线、触发运行、查看任务日志和服务连接。

---

## 命令列表

### 流水线管理

| 命令 | 说明 |
|------|------|
| `yunxiao flow pipelines list` | 列出流水线 |
| `yunxiao flow pipelines get` | 查看流水线详情 |
| `yunxiao flow pipelines update` | 更新流水线 YAML 定义 |

### 运行管理

| 命令 | 说明 |
|------|------|
| `yunxiao flow runs create` | 触发流水线运行 |
| `yunxiao flow runs list` | 列出运行记录 |
| `yunxiao flow runs get` | 查看运行详情 |
| `yunxiao flow runs latest` | 查看最新运行 |

### 任务管理

| 命令 | 说明 |
|------|------|
| `yunxiao flow jobs list` | 按类别列出任务 |
| `yunxiao flow jobs history` | 查看任务运行历史 |
| `yunxiao flow jobs run` | 触发特定任务执行 |
| `yunxiao flow jobs log` | 查看任务日志 |

### 服务连接

| 命令 | 说明 |
|------|------|
| `yunxiao flow connections list` | 列出服务连接 |

---

## 列出流水线

### 基本用法

```bash
yunxiao flow pipelines list --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--keyword` | 搜索关键词 | 否 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
# 列出所有流水线
yunxiao flow pipelines list --org-id org-xxxxxxxx --output json

# 搜索流水线
yunxiao flow pipelines list --keyword "构建" --org-id org-xxxxxxxx --output json
```

---

## 查看流水线详情

### 基本用法

```bash
yunxiao flow pipelines get --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow pipelines get --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 更新流水线 YAML 定义

### 基本用法

```bash
yunxiao flow pipelines update --pipeline-id <PIPELINE_ID> --yaml <YAML_CONTENT> --org-id <ORG_ID>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--yaml` | YAML 定义内容 | 是 |

### 示例

```bash
# 直接传入 YAML 内容
yunxiao flow pipelines update --pipeline-id pipeline-xxx \
    --yaml "version: 1.0\nstages:\n  - name: build\n    jobs:\n      - name: compile" \
    --org-id org-xxxxxxxx

# 从文件读取 YAML（使用 shell）
YAML_CONTENT=$(cat pipeline.yaml)
yunxiao flow pipelines update --pipeline-id pipeline-xxx --yaml "$YAML_CONTENT" --org-id org-xxxxxxxx
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
| `--params` | 运行参数（JSON 格式） | 否 |

### 示例

```bash
# 基本触发
yunxiao flow runs create --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx --output json

# 带参数触发
yunxiao flow runs create --pipeline-id pipeline-xxxxxxxx \
    --params '{"branch": "feature/new", "env": "staging"}' \
    --org-id org-xxxxxxxx --output json
```

---

## 列出运行记录

### 基本用法

```bash
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--page` | 页码 | 否（默认 1） |
| `--per-page` | 每页数量 | 否（默认 20） |

### 示例

```bash
yunxiao flow runs list --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 查看运行详情

### 基本用法

```bash
yunxiao flow runs get --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--run-id` | 运行 ID | 是 |

### 示例

```bash
yunxiao flow runs get --pipeline-id pipeline-xxxxxxxx --run-id run-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 查看最新运行

### 基本用法

```bash
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |

### 示例

```bash
yunxiao flow runs latest --pipeline-id pipeline-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 按类别列出任务

### 基本用法

```bash
yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --category <CATEGORY> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--category` | 任务类别（如 BUILD、DEPLOY、TEST） | 是 |

### 示例

```bash
# 列出构建任务
yunxiao flow jobs list --pipeline-id pipeline-xxxxxxxx --category BUILD --org-id org-xxxxxxxx --output json

# 列出部署任务
yunxiao flow jobs list --pipeline-id pipeline-xxxxxxxx --category DEPLOY --org-id org-xxxxxxxx --output json
```

---

## 查看任务运行历史

### 基本用法

```bash
yunxiao flow jobs history --pipeline-id <PIPELINE_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--pipeline-id` | 流水线 ID | 是 |
| `--job-id` | 任务 ID | 是 |

### 示例

```bash
yunxiao flow jobs history --pipeline-id pipeline-xxxxxxxx --job-id job-xxxxxxxx --org-id org-xxxxxxxx --output json
```

---

## 触发特定任务执行

### 基本用法

```bash
yunxiao flow jobs run --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID>
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
yunxiao flow jobs run --pipeline-id pipeline-xxxxxxxx --run-id run-xxxxxxxx --job-id job-xxxxxxxx --org-id org-xxxxxxxx --output json
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

## 列出服务连接

### 基本用法

```bash
yunxiao flow connections list --org-id <ORG_ID> --output json
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--org-id` | 组织 ID | 是 |
| `--type` | 连接类型过滤 | 否 |

### 示例

```bash
# 列出所有服务连接
yunxiao flow connections list --org-id org-xxxxxxxx --output json

# 按类型过滤
yunxiao flow connections list --type sonarqube --org-id org-xxxxxxxx --output json
```

---

## 常见用法

### 触发构建并查看结果

```bash
# 1. 列出流水线
yunxiao flow pipelines list --org-id org-xxx --output json

# 2. 触发运行
yunxiao flow runs create --pipeline-id pipeline-xxx --org-id org-xxx --output json

# 3. 查看最新运行状态
yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json

# 4. 查看运行记录
yunxiao flow runs list --pipeline-id pipeline-xxx --org-id org-xxx --output json
```

### 查看构建失败原因

```bash
# 1. 查看最新运行
yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json

# 2. 获取运行 ID 和失败任务 ID
RUN_ID=$(yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.id')
JOB_ID=$(yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.jobs[0].id')

# 3. 查看任务日志
yunxiao flow jobs log --pipeline-id pipeline-xxx --run-id $RUN_ID --job-id $JOB_ID --org-id org-xxx

# 4. 或查看运行详情获取所有任务状态
yunxiao flow runs get --pipeline-id pipeline-xxx --run-id $RUN_ID --org-id org-xxx --output json
```

### 监控流水线状态

```bash
# 循环查看最新运行状态
while true; do
  yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json | jq -r '.status'
  sleep 10
done
```

### 更新流水线配置

```bash
# 1. 获取当前流水线 YAML
yunxiao flow pipelines get --pipeline-id pipeline-xxx --org-id org-xxx --output json

# 2. 更新 YAML（从文件）
YAML=$(cat new-pipeline.yaml)
yunxiao flow pipelines update --pipeline-id pipeline-xxx --yaml "$YAML" --org-id org-xxx --output json
```

---

## ID 获取速查

| 需要 ID | 使用命令 |
|--------|---------|
| `pipeline_id` | `yunxiao flow pipelines list --org-id <ORG_ID>` |
| `run_id` | `yunxiao flow runs list --pipeline-id <PIPELINE_ID>` 或 `yunxiao flow runs latest --pipeline-id <PIPELINE_ID>` |
| `job_id` | `yunxiao flow runs get --pipeline-id <PIPELINE_ID> --run-id <RUN_ID>` (从 jobs 数组中获取) |

---

## 故障排查

### "Pipeline not found"

**原因**: 流水线 ID 错误或无权限

**解决方案**:
```bash
# 搜索流水线
yunxiao flow pipelines list --org-id org-xxx --output json
```

### "Run not found"

**原因**: 运行 ID 错误

**解决方案**:
```bash
# 列出运行记录
yunxiao flow runs list --pipeline-id pipeline-xxx --org-id org-xxx --output json

# 或查看最新运行
yunxiao flow runs latest --pipeline-id pipeline-xxx --org-id org-xxx --output json
```

### "Job not found"

**原因**: 任务 ID 错误或任务已完成

**解决方案**:
```bash
# 查看运行详情获取任务 ID
yunxiao flow runs get --pipeline-id pipeline-xxx --run-id run-xxx --org-id org-xxx --output json

# 或按类别列出任务
yunxiao flow jobs list --pipeline-id pipeline-xxx --category BUILD --org-id org-xxx --output json
```

### "Invalid params JSON"

**原因**: `--params` 参数格式错误

**解决方案**:
确保 JSON 格式正确，使用单引号包裹：
```bash
yunxiao flow runs create --pipeline-id pipeline-xxx \
    --params '{"branch": "main"}' \
    --org-id org-xxx --output json
```