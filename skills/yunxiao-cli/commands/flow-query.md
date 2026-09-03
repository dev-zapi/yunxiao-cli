# 流水线查询命令手册

`flow-query` 是 `yunxiao flow` 查询类接口的按需参考。这里新增的查询命令都只发送一次请求并输出 API 原始响应；不会自动翻页、下载文件、跟随 URL 或改写敏感字段。已有命令的输出约定（例如 `jobs log` 的日志文本输出）保持不变。Agent 场景使用 `--output json`。

## Pipeline 查询

```bash
yunxiao flow pipelines artifact-url --file-path <FILE_PATH> --file-name <FILE_NAME> --org-id <ORG_ID> --output json
yunxiao flow pipelines emas-artifact-url --emas-job-instance-id <ID> --md5 <MD5> --pipeline-id <ID> --pipeline-run-id <ID> --service-connection-id <ID> --org-id <ORG_ID> --output json
yunxiao flow pipelines scan-report-url --report-path <REPORT_PATH> --org-id <ORG_ID> --output json
yunxiao flow pipelines relations list --pipeline-id <PIPELINE_ID> --rel-object-type VARIABLE_GROUP --org-id <ORG_ID> --output json
```

`rel-object-type` 的完整枚举未由官方文档公开，`VARIABLE_GROUP` 只是示例，应按组织配置和最新文档验证。

## Pipeline Run 与 Job 查询

已有命令：

```bash
yunxiao flow runs list --pipeline-id <PIPELINE_ID> --page 1 --per-page 20 --org-id <ORG_ID> --output json
yunxiao flow runs get --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --org-id <ORG_ID> --output json
yunxiao flow runs latest --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --category DEPLOY --org-id <ORG_ID> --output json
yunxiao flow jobs history --pipeline-id <PIPELINE_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
```

Job 步骤和步骤日志：

```bash
yunxiao flow jobs steps --pipeline-id <PIPELINE_ID> --pipeline-run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs step-log --pipeline-id <PIPELINE_ID> --pipeline-run-id <RUN_ID> --job-id <JOB_ID> --step-index 0 --offset 0 --limit 1000 --build-id <BUILD_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs step-log-url --pipeline-id <PIPELINE_ID> --pipeline-run-id <RUN_ID> --job-id <JOB_ID> --step-index 0 --build-id <BUILD_ID> --org-id <ORG_ID> --output json
```

`step-log` 的 `offset` 和 `limit` 是 API 日志分页参数，CLI 不自动请求后续页。`pipeline-run-id` 也接受可见别名 `--run-id`。

Job ID 应通过 `flow runs get` 或 `flow runs latest` 返回的 `.stages[].stageInfo.jobs[].id` 获取；`jobs list` 不接受 `--run-id`，当前官方仅记录 `DEPLOY` 分类（传入值由服务端校验）。日志查询链路是 pipeline → run → run detail → job → `jobs log`/`jobs steps`/`jobs step-log`。`jobs log` 默认输出 `content`（兼容旧响应的 `log`）；`--output json` 输出完整原始对象，不会仅提取正文。`step-log` 每次只请求一页，需根据响应的 `more` 自行调整 `offset`。

## 部署与主机组

```bash
yunxiao flow deploy order get --pipeline-id <PIPELINE_ID> --deploy-order-id <ORDER_ID> --org-id <ORG_ID> --output json
yunxiao flow deploy machine-log --pipeline-id <PIPELINE_ID> --deploy-order-id <ORDER_ID> --machine-sn <MACHINE_SN> --org-id <ORG_ID> --output json
yunxiao flow host-groups list --name build --page 1 --per-page 10 --org-id <ORG_ID> --output json
yunxiao flow host-groups get --id <HOST_GROUP_ID> --org-id <ORG_ID> --output json
```

`host-groups list` 还支持 `--ids`、`--creator-account-ids`、`--create-start-time`、`--create-end-time`、`--page-sort` 和 `--page-order`。

## Pipeline 分组

```bash
yunxiao flow pipeline-groups list --page 1 --per-page 10 --org-id <ORG_ID> --output json
yunxiao flow pipeline-groups get --group-id <GROUP_ID> --org-id <ORG_ID> --output json
yunxiao flow pipeline-groups pipelines --group-id <GROUP_ID_OR_0> --org-id <ORG_ID> --output json
```

`pipeline-groups pipelines` 的 `--group-id 0` 表示未分组流水线；还支持创建/执行时间、名称、状态和分页过滤参数。

## Resource Members、Tags、Variable Groups

```bash
yunxiao flow resource-members list --resource-type pipeline --resource-id <RESOURCE_ID> --org-id <ORG_ID> --output json
yunxiao flow tags list --org-id <ORG_ID> --output json
yunxiao flow tags get --id <TAG_GROUP_ID> --org-id <ORG_ID> --output json
yunxiao flow variable-groups list --page 1 --per-page 10 --org-id <ORG_ID> --output json
yunxiao flow variable-groups get --id <VARIABLE_GROUP_ID> --org-id <ORG_ID> --output json
```

`resource-type` 的官方示例包括 `pipeline` 和 `hostGroup`，但不是完整枚举。变量组列表还支持 `--page-sort` 和 `--page-order`。

## Service Auth 与 Credential

```bash
yunxiao flow connections auths list --service-auth-type RAM --org-id <ORG_ID> --output json
yunxiao flow connections credentials list --service-credential-type username_password --org-id <ORG_ID> --output json
```

`service-auth-type` 和 `service-credential-type` 的完整枚举未在官方文档公开；`RAM` 和 `username_password` 只是示例，使用前应结合组织配置和最新文档验证。返回值按 API 原样输出，不自动脱敏。

资源 ID 即使官方文档标为整数，也按字符串传入，以兼容数字字符串和 UUID。列表命令只请求指定页，不自动翻页。
