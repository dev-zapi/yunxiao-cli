# YunXiao CLI

阿里云云效（DevOps）平台的命令行工具及其 Agent 技能集。本词汇表收录项目上下文配置相关的领域术语。

## Language

**共享配置（YUNXIAO.md）**:
项目根目录下随仓库提交的团队共识配置，收录稳定的项目关联信息（组织 ID、项目 ID、仓库、工作项类型 ID）与配置型 ID 字典（状态/优先级 name→ID 表）。
_Avoid_: 项目配置（易与本地配置混淆）、全局配置

**本地配置（.yunxiao/local.md）**:
项目 `.yunxiao/` 目录下不入库的个人配置，收录项目级易变或个人化的 ID 缓存（当前迭代、标签映射）。
_Avoid_: 个人配置、YUNXIAO.local.md

**全局配置（~/.config/yunxiao-cli/global.md）**:
本机配置目录下不入库的个人配置，跨仓库生效，收录用户身份（本人 name/userId）、org 别名映射与 org 级常用成员映射。
_Avoid_: 机器配置、user config、与共享配置混称

**配置型 ID**:
由 space+type 维度的平台配置（workflow、fields）派生的 ID（状态、优先级），全员相同、非个人化，归属共享配置。
_Avoid_: 个人快捷 ID、易变缓存

**共享优先**:
同一字段在多层配置中冲突时的默认裁决规则：共享配置胜出并提示冲突（视为配置漂移）；本地与全局冲突时本地胜出（更具体）。用户可在具体场景中对话式要求反转。
_Avoid_: 本地覆盖、local-wins
