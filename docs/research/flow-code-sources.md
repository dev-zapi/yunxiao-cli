# Flow Code Sources: Official API and YAML Research

Research date: 2026-09-01. This note cites only Alibaba Cloud/Yunxiao
official documentation. All identifiers below are placeholders.

Scope: the conclusions about endpoint paths and parameters use the current
OAPI Flow pages because the investigated request is under `/oapi/v1/flow/`.
Alibaba Cloud also publishes DevOps 2021 API-reference pages with different
`/organization/...` paths; those are a distinct contract and must not be
combined with the OAPI paths below. Source: [DevOps 2021 ListServiceConnections](https://help.aliyun.com/zh/yunxiao/developer-reference/api-devops-2021-06-25-listserviceconnections).

## Service-connection list

### Confirmed facts

1. For a central-edition organization, the documented endpoint is exactly:

   ```text
   GET https://{domain}/oapi/v1/flow/organizations/{organizationId}/serviceConnections
   ```

   `organizationId` is a required path parameter for the central edition.
   Source: [ListServiceConnections](https://help.aliyun.com/zh/yunxiao/developer-reference/listserviceconnections).

2. `serviceConnectionType` is a **required** query parameter. Therefore, a
   request without it is outside the documented request contract and explains
   the observed HTTP 400. The official example sends both the obsolete,
   misspelled `sericeConnectionType=codeup` and the current
   `serviceConnectionType=codeup`; the parameter table marks the former as
   deprecated and says to use the latter. New callers should send only the
   correctly spelled current parameter:

   ```text
   ?serviceConnectionType=codeup
   ```

   Source: [ListServiceConnections](https://help.aliyun.com/zh/yunxiao/developer-reference/listserviceconnections).

3. For Codeup, the documented query value is lower-case `codeup`. The
   central-edition documentation also lists these accepted connection types:
   `ecs`, `Gitee`, `Github`, `docker_register_aliyun`, `ack`, `Codeup`,
   `oss`, `edas`, `sae`, `ros`, `fc`, `emas`, `PACKAGES`, `customGitlab`,
   `git`, `gitlab`, `bitbucket`, `jenkins`, `private_docker_registry`, `ess`,
   `atomGit`, `svn`, `gitlabAPI`, and `vpc`. The page's `codeup` example and
   the mixed-case list are both official; only `codeup` is directly evidenced
   as the query value for Codeup.
   Source: [ListServiceConnections](https://help.aliyun.com/zh/yunxiao/developer-reference/listserviceconnections).

4. This API documents no `page`, `perPage`, page-size, cursor, total, or
   next-page request/response fields. Its response is an array; each listed
   item has `createTime`, `id`, `name`, `ownerAccountId`, `type`, and `uuid`.
   Do not invent pagination parameters or expect a paged envelope without
   separate evidence from a live supported API response.
   Source: [ListServiceConnections](https://help.aliyun.com/zh/yunxiao/developer-reference/listserviceconnections).

### Safe request shape

```text
GET /oapi/v1/flow/organizations/{ORG_ID}/serviceConnections?serviceConnectionType=codeup
x-yunxiao-token: {PAT}
```

`{ORG_ID}` and `{PAT}` are placeholders; neither is an example of a real
organization or credential.

## Pipeline create/update content

### API contract

1. Central-edition CreatePipeline uses
   `POST /oapi/v1/flow/organizations/{organizationId}/pipelines`; both `name`
   and YAML-string `content` are required request-body fields. The same
   requirement applies to UpdatePipeline, whose endpoint is
   `PUT /oapi/v1/flow/organizations/{organizationId}/pipelines/{pipelineId}`.
   Sources: [CreatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/createpipeline-create-pipeline),
   [UpdatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/updatepipeline-update-pipeline).

2. The official YAML guide says pipeline-as-code content uses top-level
   `sources`, `stages`, `jobs`, and `steps`; `sources` can contain one or more
   source definitions. Source: [YAML Flow pipeline](https://help.aliyun.com/zh/yunxiao/user-guide/yaml-preliminary-experience/).

### Codeup source, branch, push trigger, and service connection

The following is the official YAML layout reduced to the fields relevant to a
single Codeup source. Placeholder values intentionally do not identify a real
organization, repository, service connection, or pipeline.

```yaml
name: example-pipeline
sources:
  app_repo:
    type: codeup
    name: application
    endpoint: https://codeup.aliyun.com/{NAMESPACE}/{REPOSITORY}.git
    branch: main
    triggerEvents:
      - push
    certificate:
      type: serviceConnection
      serviceConnection: {SERVICE_CONNECTION_ID}
stages:
  build:
    name: Build
    jobs:
      command:
        name: Command
        runsOn: public/cn-beijing
        steps:
          run:
            step: Command
            name: Run command
            with:
              run: echo hello
```

The exact source syntax and its semantics are documented as follows:

1. `sources.<source_id>.type: codeup` selects Yunxiao Codeup. `source_id` is
   required, must be unique, begin with a letter, contain only letters, digits,
   and `_`, and be no longer than 30 characters. The `endpoint` is required
   for code sources and may be SSH or HTTPS; `name` is optional.
   Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

2. `branch` is optional for code sources and defaults to `master`; it is the
   default branch when execution is scheduled or externally triggered.
   Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

3. `triggerEvents` is optional and defaults to no source-event trigger. For a
   Codeup source, `push` enables code-push triggering; Codeup additionally
   supports `tagPush`, `mergeRequestMerged`, and
   `mergeRequestOpenedOrUpdate`. The official examples permit either a scalar
   `push` or a YAML list.
   Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

4. Code sources other than the documented exceptions require `certificate`.
   To use a service connection, set `certificate.type: serviceConnection` and
   set `certificate.serviceConnection` to the service-connection ID.
   Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

5. `branchesFilter` is an optional regular-expression execution filter.
   `branchFilter` is deprecated in favor of it. `pathFilter` is optional and
   applies a regular-expression path filter to code-push triggers.
   Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

6. With more than one code source, top-level `defaultWorkspace` is required
   and must name one source ID. Source: [Pipeline sources](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-sources).

## GetPipeline response versus create/update content

They are not documented as interchangeable.

`GetPipeline` returns a pipeline object with metadata and a `pipelineConfig`
object. The latter exposes `flow` (a separate YAML-looking UI flow schema
beginning `schema: tb`), `settings`, and a normalized `sources` array. Its
source fields include `data.branch`, `data.events`, `data.isTrigger`,
`data.serviceConnectionId`, `data.triggerFilter`, `data.repo`, and `type`.
The response documents this independently from CreatePipeline/UpdatePipeline,
which accept the single `content` YAML string in the pipeline-as-code syntax.
Sources: [GetPipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/getpipeline-get-pipeline-details),
[CreatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/createpipeline-create-pipeline),
[UpdatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/updatepipeline-update-pipeline),
[YAML Flow pipeline](https://help.aliyun.com/zh/yunxiao/user-guide/yaml-preliminary-experience/).

The response's `pipelineConfig.flow` uses the UI-oriented `schema: tb` /
`pipeline:` representation, while the YAML guide's create/update content uses
the pipeline-as-code top-level `sources:` and `stages:` representation.
Consequently, this research does **not** establish any supported conversion or
round trip between `pipelineConfig.flow`, `pipelineConfig.sources`, and request
`content`. A client must not submit either response field as `content` merely
because both are serialized text/data.

## Remaining unknowns

1. The official ListServiceConnections page proves that
   `serviceConnectionType` is required but does not publish a formal machine
   enum, case-sensitivity rules, or pagination behavior. The `codeup` query
   example is the strongest documented value for Codeup.
2. The official pages do not define a supported API to convert GetPipeline's
   UI-schema `pipelineConfig.flow` or normalized `pipelineConfig.sources` into
   pipeline-as-code request `content`.
3. Whether a given Codeup service connection can access a given repository is
   organization-specific and cannot be established without a live authorized
   request.

## Recommended minimum implementation change

Require a connection type for `flow connections list` and send it as
`serviceConnectionType`. For the Codeup workflow, document/offer `codeup` as
the value. Do not add pagination controls to this command. Keep
CreatePipeline/UpdatePipeline accepting an explicitly supplied pipeline-as-code
YAML `content` string; do not implement a GetPipeline response replay or a
conversion based on the undocumented equivalence.
