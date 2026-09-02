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
`--service-connection-uuid <UUID>`。未显式设置时，代码源 ID、分支和触发事件分别默认为
`repo`、`master` 和 `push`；可用 `--source-id`、`--branch`、`--trigger-events` 覆盖，
`--source-name` 可选。这些代码源专属参数都必须和 `--codeup-repo` 一起使用。代码源 ID 必须以字母开头，仅包含字母、数字和下划线，且最长 30 个字符。

**重要**：`--service-connection-uuid` 必须使用 `flow connections list` 返回的 `uuid` 字段（字符串），不能使用数字 `id`。API 只接受 UUID 字符串，数字 ID 会被拒绝。

```bash
# 先查询服务连接，获取 uuid
yunxiao flow connections list --type codeup --org-id <ORG_ID> --output json

# 使用返回的 uuid 生成模板
yunxiao flow pipelines template --template-type maven --file pipeline.yaml \
  --codeup-repo <CLONE_URL> --service-connection-uuid <UUID> \
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

连接类型必需传入。使用 `--type`，也兼容可见别名 `--conn-type`。CLI 不对类型做本地枚举校验，会将值原样作为 `serviceConnectionType` 传给 API。

Codeup 的查询值使用小写 `codeup`：

```bash
yunxiao flow connections list --type codeup --org-id <ORG_ID> --output json
```

### 类型参考与兜底查询

云效 `ListServiceConnections` 官方文档曾列出以下类型。该清单仅作候选值参考，可能随云效版本、地域或文档更新而过期，也不保证完整；它不是 CLI 或 API 公布的正式枚举：

```text
ecs
Gitee
Github
docker_register_aliyun
ack
Codeup
oss
edas
sae
ros
fc
emas
PACKAGES
customGitlab
git
gitlab
bitbucket
jenkins
private_docker_registry
ess
atomGit
svn
gitlabAPI
vpc
```

Agent 需要其他类型时，按以下顺序处理：

1. 优先使用用户、项目配置或最新官方文档明确给出的类型；Codeup 统一先试 `codeup`。
2. 类型未知时，从上面的候选值开始逐个查询，并始终使用 `--output json`：

   ```bash
   yunxiao flow connections list --type <TYPE> --org-id <ORG_ID> --output json
   ```

3. 检查命令是否成功以及返回记录中的 `type`、`name` 和 `uuid`。空数组只表示当前组织没有该类型的连接，不能据此断定类型无效；API 错误也不能当作空结果处理。
4. 候选值都不适用时，当前 API 没有不带类型的“列出全部类型/连接”接口。应根据用户或云效控制台确认实际类型，或查阅最新官方文档后再用 CLI 验证，不要凭名称猜测类型。

服务连接用于流水线代码源时，只能使用查询结果中的 `uuid` 字段，不能使用数字 `id`。
