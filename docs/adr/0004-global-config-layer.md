# 新增全局配置层 global.md

常用成员映射、用户身份、org 别名是 org 级或机器级的个人事实：不随任何仓库提交，却在每个仓库的 `.yunxiao/local.md` 里重复一份——[ADR-0003](./0003-yunxiao-config-split.md) 的两层切不出这一层。决定业务层配置扩为三层，新增全局配置 `~/.config/yunxiao-cli/global.md`（本机、跨仓库、不入库），与同目录的技术层 `config.toml`（token/endpoint/默认 org）互补不重叠：config.toml 管"CLI 怎么连"，global.md 管"agent 认得什么"。查找链延长为 共享 > 本地 > 全局 > 实时查询；常用成员映射归属由本地上移到全局。

## Considered Options

- **扩展 config.toml 字段承载**：被否。config.toml 是 CLI 二进制的机器格式，塞入成员映射、别名等 agent 语义信息会模糊两层职责，且 schema 变更需随 CLI 发版。
- **扁平单 org 结构**：被否。org 别名这一需求本身即暗示多 org；按别名分节的结构对单 org 只是一个条目，无额外成本。
- **保留成员映射在本地、全局仅兜底**：被否。"成员映射写哪"会变成永久模糊地带，每个新仓库 setup 都要重问一遍。

## Consequences

- 共享仍赢一切（漂移检测）；本地与全局冲突时本地胜出（更具体）但需提示。
- 存量 local.md 中的成员条目不迁移，自然淘汰（沿 0003 先例）。
- 写全局配置与写其他配置文件一样，必须征得用户同意；由 yunxiao-setup 的全局模式创建，或在项目 setup 的上提询问中顺手生成。
