---
name: yunxiao-flow-step-query
description: "查询云效 Flow YAML 内置 Step 类型、分类、说明和官方参数文档；当用户需要选择、解释或配置流水线 steps 时使用。"
---

# Yunxiao Flow Step Query

查询的是云效 Flow YAML 的内置 `step` 类型，不是流水线运行实例。

## Workflow

1. Run the bundled query helper against the official Steps list:

   ```bash
   python3 /path/to/yunxiao-flow-step-query/scripts/query_steps.py \
     --query <IDENTIFIER_OR_KEYWORD> --json
   ```

   Use `--category <CATEGORY>` to narrow results, `--exact` for an exact
   identifier match, `--list-categories` to inspect categories, and `--all` to
   return the complete list. The helper fetches the current official Markdown
   page by default. For a deterministic offline query, pass
   `--input <saved-official-response>` (or pipe it through stdin); otherwise
   consult [the normalized catalog](references/steps-catalog.md). The catalog
   is a dated fallback, not a replacement for the official page.

2. Report each match with its exact YAML identifier, category, short description,
   and official detail URL. Keep the localized display name separate from the
   identifier: the YAML value is the trailing identifier such as `JavaBuild` or
   `Command`.

3. When the user needs `with` parameters, read the matched detail URL before
   proposing a configuration. The Steps list identifies the type but does not
   define every input. Do not invent parameter names from the display name.

4. Show the smallest relevant YAML fragment. A step job uses this shape:

   ```yaml
   stages:
     build:
       jobs:
         build_job:
           name: Build
           runsOn: public/cn-beijing
           steps:
             build_step:
               step: <STEP_IDENTIFIER>
               name: Build step
               with:
                 # parameters copied from the matched detail document
   ```

   A job that calls a `component` is a different YAML form; do not report a
   component as a `step`.

5. State the source URL and retrieval date when reporting the catalog. If the
   official page is unavailable, use the normalized reference only as a fallback
   and label it as a snapshot. Report an unknown identifier as “not found” and
   offer close matches; never silently substitute a different Step.

## API Boundary

There is no public OpenAPI endpoint that enumerates all available Step types and
their `with` schemas. `GetPipelineJobSteps` only lists steps for a specific
pipeline run (`pipelineId`, `pipelineRunId`, and `jobId`) and returns execution
status. Creating or updating jobs and steps submits the complete YAML `content`
through CreatePipeline or UpdatePipeline.

Official references:

- [Steps list](https://help.aliyun.com/zh/yunxiao/user-guide/step-steps-list)
- [Steps syntax](https://help.aliyun.com/zh/yunxiao/user-guide/pipeline-steps)
- [GetPipelineJobSteps](https://help.aliyun.com/zh/yunxiao/developer-reference/getpipelinejobsteps)
- [CreatePipeline](https://help.aliyun.com/zh/yunxiao/developer-reference/createpipeline-create-pipeline)
