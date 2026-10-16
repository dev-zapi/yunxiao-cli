# 云效代码管理（Codeup）API 文档 - 代码仓库与代码组

**抓取时间**: 2026-10-16

**来源**: 阿里云帮助中心（agent-browser 渲染提取，含完整参数表格、请求/返回示例）

**说明**: 每个文件对应一个 OpenAPI 接口，内容涵盖适用版本（中心版/Region版）、请求语法、请求头、请求参数、请求示例、返回参数、返回示例。

## 代码仓库（8 个 API）

| API | 说明 | 文件 |
|-----|------|------|
| CreateRepository | 创建代码库 | [repositories/CreateRepository.md](./repositories/CreateRepository.md) |
| DeleteRepository | 删除代码库 | [repositories/DeleteRepository.md](./repositories/DeleteRepository.md) |
| GetRepository | 查询代码库 | [repositories/GetRepository.md](./repositories/GetRepository.md) |
| ListRepositories | 查询代码库列表 | [repositories/ListRepositories.md](./repositories/ListRepositories.md) |
| ListTemplateRepositories | 查询模板代码库列表 | [repositories/ListTemplateRepositories.md](./repositories/ListTemplateRepositories.md) |
| UpdateRepository | 更新代码库 | [repositories/UpdateRepository.md](./repositories/UpdateRepository.md) |
| ArchiveRepository | 归档代码库 | [repositories/ArchiveRepository.md](./repositories/ArchiveRepository.md) |
| TransferRepository | 转移代码库 | [repositories/TransferRepository.md](./repositories/TransferRepository.md) |

## 代码组（5 个 API）

| API | 说明 | 文件 |
|-----|------|------|
| CreateGroup | 创建代码组 | [groups/CreateGroup.md](./groups/CreateGroup.md) |
| GetNamespace | 查询代码组空间信息 | [groups/GetNamespace.md](./groups/GetNamespace.md) |
| ListNamespaces | 查询代码组空间列表 | [groups/ListNamespaces.md](./groups/ListNamespaces.md) |
| ListGroupRepositories | 查询代码组下的代码库列表 | [groups/ListGroupRepositories.md](./groups/ListGroupRepositories.md) |
| UpdateGroup | 更新代码组 | [groups/UpdateGroup.md](./groups/UpdateGroup.md) |

## 与旧索引（yunxiao_api_code.md）的差异

官方文档 API 面已变化（2026-10 导航实况）：

- **代码仓库新增**: UpdateRepository、ArchiveRepository、TransferRepository
- **代码组变更**: 旧索引中的 DeleteGroup、GetGroup、ListGroups 已下线，现为
  CreateGroup、GetNamespace、ListNamespaces、ListGroupRepositories、UpdateGroup
- **TransferRepository**: 官方导航重复列出两页，内容基本重复，已合并为一份文件
