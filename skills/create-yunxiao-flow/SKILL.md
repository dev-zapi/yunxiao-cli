---
name: create-yunxiao-flow
description: 创建或新建云效流水线，使用本地 YAML 模板并安全配置 Codeup 代码源。
---

# Create Yunxiao Flow Pipeline

Read `YUNXIAO.md` in the target repository before collecting IDs or running commands.
Treat it as project context, but ask for missing or conflicting organization details.

1. Collect a precise pipeline name, build template type, Codeup clone URL, branch,
   and desired trigger events. The supported template types are `simple`, `maven`,
   `docker`, `maven-docker`, `node`, and `golang`.
2. For a Codeup source, inspect connections before creating anything:

   ```bash
   yunxiao flow connections list --type codeup --org-id <ORG_ID> --output json
   ```

   Select a service connection only by an explicit returned ID. Do not guess from
   its name or choose arbitrarily.
3. Generate and inspect local YAML. With Codeup, use the chosen numeric ID:

   ```bash
   yunxiao flow pipelines template --template-type <TYPE> --file pipeline.yaml \
     --codeup-repo <CLONE_URL> --service-connection-id <INTEGER> \
     --branch <BRANCH> --trigger-events push
   ```

4. Creating a pipeline is non-idempotent. Before `create`, query with
   `yunxiao flow pipelines list --pipeline-name <NAME> --org-id <ORG_ID> --output json`.
   This list may be a fuzzy search: inspect the response and continue only when
   no returned `name` exactly equals the target. If creation reports an ambiguous
   error, do not retry; repeat this exact-name check first.
5. Create exactly once:

   ```bash
   yunxiao flow pipelines create --name <NAME> --content-file pipeline.yaml \
     --org-id <ORG_ID> --output json
   ```

6. Capture the returned pipeline ID, then verify it with a read-only request:

   ```bash
   yunxiao flow pipelines get --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
   ```

7. Trigger a run only after the user asks for one:

   ```bash
   yunxiao flow runs create --pipeline-id <PIPELINE_ID> --org-id <ORG_ID> --output json
   ```
