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

其他流水线查询（制品 URL、扫描报告、关系）见[流水线查询命令手册](./flow-query.md)。

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
yunxiao flow jobs list --pipeline-id <PIPELINE_ID> --category DEPLOY --org-id <ORG_ID> --output json
yunxiao flow jobs history --pipeline-id <PIPELINE_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs run --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
yunxiao flow jobs log --pipeline-id <PIPELINE_ID> --run-id <RUN_ID> --job-id <JOB_ID> --org-id <ORG_ID> --output json
```

获取执行日志的 ID 链路为 pipeline → run（`runs list`/`latest`）→ run detail（`runs get`，从 `.stages[].stageInfo.jobs[].id` 取得 Job ID）→ job。`jobs list` 不接受 `--run-id`，且当前官方分类为 `DEPLOY`（分类值会原样传给服务端）。`jobs log` 默认输出日志正文；显式 `--output json` 保留 API 原始响应（包括 `content`、`last`、`more`）。

Job 步骤、部署、主机组、流水线分组、资源成员、标签组和变量组查询见[流水线查询命令手册](./flow-query.md)。

## Job 执行环境 (`runsOn`)

`runsOn` 是 Job 级别的执行环境配置，决定该 Job 的 steps 在哪个构建集群、容器或私有主机上运行。一个 Job 内的多个 steps 共享工作空间；`runsOn` 不属于某个 step，也不用于代码源认证。

官方 YAML 文档将 `runsOn` 定义为非必填，未填写时默认使用云效北京构建集群。默认环境对较新组织可能已弃用，生成或修改流水线时应优先显式指定执行环境。集群名称、区域和能力可能随云效版本或组织配置变化，以下仅是官方文档中的常见示例，不是永久枚举。

### 常用写法

公共集群的默认环境：

```yaml
runsOn: public/cn-beijing
```

指定容器环境（当前官方文档要求使用公共构建集群）：

```yaml
runsOn:
  group: public/cn-beijing
  container: maven:3.8-openjdk-17
  instanceType: LARGE_4C8G
```

公共集群标识还包括官方文档列出的 `public/cn-hangzhou` 和 `public/cn-hongkong`；
指定容器模式当前文档示例使用北京或中国香港集群，实际可用区域以组织和当前官方文档为准。
`container` 是 Job 使用的镜像，镜像地址必须满足云效构建环境的访问要求。

私有构建集群的默认环境：

```yaml
runsOn: private/<PRIVATE_BUILD_CLUSTER_ID>
```

私有集群的 VM/宿主机环境：

```yaml
runsOn:
  group: private/<PRIVATE_BUILD_CLUSTER_ID>
  labels: linux,amd64
  vm: true
```

`labels` 用于匹配私有集群中的操作系统和架构；省略时由集群随机选择机器。
`vm: true` 仅用于私有构建集群，表示直接在宿主机或虚拟机上执行步骤，而不是启动容器。

`instanceType` 是可选的构建规格，可用值以当前官方文档为准，常见值包括
`SMALL_1C2G`、`MEDIUM_2C4G`、`LARGE_4C8G`、`XLARGE_8C16G` 和 `XXLARGE_16C32G`。

Agent 编写 Job YAML 时：

1. 普通 steps Job 优先显式配置 `runsOn`；未提供私有集群时，可使用公共集群加可访问的构建镜像。
2. 用户要求私有集群、VM、操作系统或架构时，先取得实际的私有构建集群 ID，再配置 `group`、`labels` 和 `vm` 的组合。
3. Job 直接调用 `component` 时，按该组件文档判断是否需要 `runsOn`；部署目标等参数通常位于 Job 的 `with` 中。
4. 创建或更新流水线时，`runsOn` 必须写入完整 YAML，通过 `pipelines create/update --content` 或 `--content-file` 提交；CLI 没有单独的 `runsOn` 参数。

## 服务连接

连接类型必需传入。使用 `--type`，也兼容可见别名 `--conn-type`。CLI 不对类型做本地枚举校验，会将值原样作为 `serviceConnectionType` 传给 API。

Codeup 的查询值使用小写 `codeup`：

```bash
yunxiao flow connections list --type codeup --org-id <ORG_ID> --output json
```

服务认证、服务凭据及其他查询接口见[流水线查询命令手册](./flow-query.md)。

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
