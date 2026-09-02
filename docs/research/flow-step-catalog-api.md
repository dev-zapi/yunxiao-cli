# 云效 Flow 步骤与组件目录 API 调研

调研日期：2026-09-02
范围：阿里云/云效官方帮助文档、公开 OpenAPI 参考，以及该官方文档链接的
`flow-steps/system_steps` 一方步骤说明。

## 结论

截至调研日，**未发现云效公开 OpenAPI 提供“列出可选 step/component/plugin
类型及其 `with` 输入 schema”的接口**。公开创建、更新接口接收完整的 YAML
字符串；可选步骤、组件和插件应从官方的静态清单及每个条目的说明取得，而不是
从运行查询接口取得。

官方提供的静态入口：

- [步骤 steps 清单](https://help.aliyun.com/zh/yunxiao/user-guide/step-steps-list)：列出步骤名（如 `JavaBuild`、`ArtifactUpload`、`Command`）及分类；各条目链接到官方文档所引用的 `flow-steps/system_steps` 说明，作为对应 `with` 参数的来源。
- [组件 component 清单](https://help.aliyun.com/zh/yunxiao/user-guide/component-manifest)。
- [插件 plugins 清单](https://help.aliyun.com/zh/yunxiao/user-guide/plug-in-plugins-list)。

这意味着 CLI 可以把这些清单固化或抓取为辅助资料，但不能把它们包装成一个已由
官方承诺的、按组织或运行时动态返回 schema 的 OpenAPI。

## 不要混淆的运行接口

`GetPipelineJobSteps` 是已创建并执行的某一次 job run 的步骤状态接口：

```text
GET /oapi/v1/flow/organizations/{organizationId}/pipelines/{pipelineId}/pipelineRuns/{pipelineRunId}/jobs/{jobId}/steps
```

其路径必需 `pipelineRunId` 和 `jobId`，返回 `nodeIndex`、`nodeName`、
`status`、`stepApiVersion`、`stepName` 等运行数据；它不返回可创建的 step
名称集合或 `with` schema。来源：[GetPipelineJobSteps](https://help.aliyun.com/zh/yunxiao/developer-reference/getpipelinejobsteps)。

同一边界适用于 [GetPipelineJobStepLog](https://help.aliyun.com/zh/yunxiao/developer-reference/getpipelinejobsteplog)：它以 `pipelineRunId`、`jobId`、`stepIndex` 查询某一已运行步骤的日志。`ListPipelineJobs` 也不是步骤目录；官方说明它按分类返回流水线任务，且当前 `category` 仅支持 `DEPLOY`。来源：[ListPipelineJobs](https://help.aliyun.com/zh/yunxiao/developer-reference/listpipelinejobs)。

## jobs 和 steps 如何创建

创建和更新不是逐个调用 job/step API，而是提交整个流水线 YAML：

- `POST /oapi/v1/flow/organizations/{organizationId}/pipelines`
- `PUT /oapi/v1/flow/organizations/{organizationId}/pipelines/{pipelineId}`

两个接口的请求体 `content` 都是必填的 YAML 字符串。来源：[CreatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/createpipeline-create-pipeline)、[UpdatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/updatepipeline-update-pipeline)。

步骤 job 的最小形状如下；`step` 取自步骤清单，参数放进 `with`：

```yaml
stages:
  build:
    jobs:
      build_job:
        name: Build
        runsOn: public/cn-beijing
        steps:
          build_step:
            step: JavaBuild
            name: Java build
            with:
              run: mvn -B package
```

job 也可直接调用一个 `component`，其参数同样放在 `with`：

```yaml
stages:
  deploy:
    jobs:
      deploy_job:
        name: Deploy
        component: VMDeploy
        with:
          machineGroup: <machine-group-id>
```

官方 YAML 语法明确说明：job 是一个或多个共享工作空间的 `steps` 的组合，或一个
`component` 调用；`steps.<step_id>.step` 选择步骤，`with` 填步骤或组件的
参数。来源：[YAML 流水线](https://help.aliyun.com/zh/yunxiao/user-guide/yaml-preliminary-experience/)、[流水线步骤 steps](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-steps)、[流水线组件 component](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-component)。

## Job 执行环境 (`runsOn`)

官方 [流水线任务 jobs](https://help.aliyun.com/zh/yunxiao/user-guide/pipelined-task-jobs)
文档将 `stages.<stage_id>.jobs.<job_id>.runsOn` 定义为 Job 运行的集群环境。该字段
非必填，省略时默认使用云效北京构建集群；文档同时说明默认环境已处于 Deprecated
状态，2025.4.1 之后创建的组织可能不再支持，应优先显式配置。

官方文档描述了三种运行模式：

1. 默认环境：使用字符串形式选择集群，例如 `runsOn: public/cn-beijing`，私有集群
   可写为 `runsOn: private/<PRIVATE_BUILD_CLUSTER_ID>`。
2. 指定容器环境：使用 `group` 和 `container`，当前文档要求该模式使用公共构建集群：

   ```yaml
   runsOn:
     group: public/cn-beijing
     container: build-steps-public-registry.cn-beijing.cr.aliyuncs.com/build-steps/alinux3:latest
   ```

3. 默认 VM 环境：使用私有集群的 `group`，并设置 `vm: true`；可用 `labels` 选择
   操作系统和架构，例如 `linux,amd64`。省略 `labels` 时由集群选择机器。

`runsOn.instanceType` 可选，用于指定构建规格。官方文档列出的规格为
`SMALL_1C2G`、`MEDIUM_2C4G`、`LARGE_4C8G`、`XLARGE_8C16G` 和 `XXLARGE_16C32G`。
公共集群标识、私有集群 ID、镜像和规格均应按当前组织能力及最新官方文档验证，不能
视为永久枚举。
