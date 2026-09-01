# 流水线命令手册

`yunxiao flow` 用于管理流水线定义、运行记录、任务和服务连接。脚本及 Agent 工作流应使用 `--output json`。

## 流水线

| 命令 | 必需参数 | 可选参数 |
|---|---|---|
| `pipelines list` | `--org-id` | `--pipeline-name`、`--create-start-time`、`--create-end-time`、`--execute-start-time`、`--execute-end-time`、`--status-list`、`--page`（默认 `1`）、`--per-page`（默认 `10`） |
| `pipelines get` | `--org-id`、`--pipeline-id` | |
| `pipelines create` | `--org-id`、`--name`、`--content` 或 `--content-file` 二选一 | |
| `pipelines update` | `--org-id`、`--pipeline-id`、`--name`、`--content` 或 `--content-file` 二选一 | |
| `pipelines delete` | `--org-id`、`--pipeline-id` | |
| `pipelines template` | | `--template-type`、`--file`，以及下文的 Codeup 代码源参数 |

`template` 为纯本地命令，不需要 token 或组织 ID。支持的模板类型是 `simple`、`maven`、`docker`、`maven-docker`、`node` 和 `golang`。

```bash
yunxiao flow pipelines list --pipeline-name build --org-id <ORG_ID> --output json
yunxiao flow pipelines create --name build --content-file pipeline.yaml --org-id <ORG_ID> --output json
yunxiao flow pipelines update --pipeline-id <PIPELINE_ID> --name build --content-file pipeline.yaml --org-id <ORG_ID> --output json
yunxiao flow pipelines delete --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
yunxiao flow pipelines template --template-type maven --file pipeline.yaml
```

要在模板中加入一个 Codeup 代码源，必须同时传入 `--codeup-repo <CLONE_URL>` 和
`--service-connection-id <INTEGER>`。未显式设置时，代码源 ID、分支和触发事件分别默认为
`repo`、`master` 和 `push`；可用 `--source-id`、`--branch`、`--trigger-events` 覆盖，
`--source-name` 可选。这些代码源专属参数都必须和 `--codeup-repo` 一起使用。代码源 ID 必须以字母开头，仅包含字母、数字和下划线，且最长 30 个字符。

```bash
yunxiao flow pipelines template --template-type maven --file pipeline.yaml \
  --codeup-repo <CLONE_URL> --service-connection-id <INTEGER> \
  --source-id app_repo --branch main --trigger-events push,tagPush
```

## 运行和任务

```bash
yunxiao flow runs create --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
yunxiao flow runs create --pipeline-id <PIPELINE_ID> --params '{"branch":"main"}' --org-id <ORG_ID> --output json
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --page 1 --per-page 20 --org-id <ORG_ID> --output json
yunxiao flow runs get --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --org-id <ORG_ID> --output json
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --category BUILD --org-id <ORG_ID> --output json
yunxiao flow jobs history --pipeline-id <PIPELINE_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs run --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
```

## 服务连接

连接类型必需传入。使用 `--type`，也兼容可见别名 `--conn-type`。Codeup 的类型值为 `codeup`。

```bash
yunxiao flow connections list --type codeup --org-id <ORG_ID> --output json
```
