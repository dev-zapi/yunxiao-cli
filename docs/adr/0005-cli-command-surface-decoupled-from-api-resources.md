# CLI 命令面与 API 资源名解耦

## 背景

云效 Codeup 的代码组底层以 `namespaces` 资源暴露：查询代码组列表与详情走
`GET .../namespaces`、`GET .../namespaces/{id}`，而组内代码库列表走
`GET .../groups/{groupId}/repositories`。同一个 `namespaces` 资源既承载代码组，
也承载组织根空间——组织根是空间而非代码组。API 的两套资源名（namespace / group）
与用户心智中的"代码组"并不一致，直接镜像到命令面会让用户被迫理解空间与组的区别。

## 决策

CLI 命令面与 API 资源名解耦：以领域概念而非 API 资源名组织命令。

- 新增 `groups` 命令组承载代码组与根空间两种资源。`groups list`、`groups get`
  底层调用 namespaces 接口，但对外统一呈现为"代码组"。
- `groups get` / `groups repos list` 的标识参数命名为 `--group-id`（而非
  `--namespace-id`），并接受代码组 ID、组织根空间 ID 或全路径；含 `/` 的全路径
  在拼入 URL 路径段时百分号编码。
- 帮助文本只出现"代码组 / 根空间"，不暴露 namespace 术语。

## 替代方案

- **镜像 API：直接暴露 `namespaces` 命令组**：被否。会把实现资源名泄漏给用户，
  并迫使调用方理解"每个组都是空间、根是空间不是组"的区分。
- **为 group 与 namespace 分别建命令组**：被否。两套命令面指向同一批资源，
  重复且割裂。

## 后果

- 命令面术语（代码组）与 API 资源名（namespaces）解耦，未来同类差异沿用此先例
  （如 API 的 changeRequest 与 CLI 的 `mr`）。
- 文档与词汇表需显式记录"每个代码组都是空间，组织根是空间而非代码组"的关系，
  避免后续实现者重新引入 namespace 术语。
- 全路径参数需要路径段编码，避免 `/` 被解释为多级路径。
