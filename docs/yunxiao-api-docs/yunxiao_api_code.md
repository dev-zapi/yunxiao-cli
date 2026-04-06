# 云效 API - 代码管理分类

收集时间：2026-04-06
收集状态：**已完成**

## 代码管理子分类结构

从文档侧边栏收集的完整子分类列表（共18个子分类）：

1. ✅ **代码仓库** - 5个API
2. ✅ **代码组** - 4个API
3. ✅ **分支** - 4个API
4. ✅ **提交** - 3个API
5. ✅ **文件** - 7个API
6. ✅ **标签** - 3个API
7. ✅ **保护分支** - 5个API
8. ✅ **合并请求** - 19个API
9. ✅ **代码比较** - 1个API
10. ✅ **推送规则** - 5个API
11. ✅ **SSH 密钥** - 5个API
12. ✅ **WebHook** - 5个API
13. ✅ **运行检查** - 4个API
14. ✅ **部署密钥** - 3个API
15. ✅ **项目类标** - 4个API
16. ✅ **提交状态** - 2个API
17. ✅ **成员** - 9个API
18. ✅ **用户资源** - 1个API

---

## 已收集的 API 列表

### ✅ 代码仓库（5个API）

- CreateRepository - 创建代码库: https://help.aliyun.com/zh/yunxiao/developer-reference/createreposition-creates-a-code-base
- DeleteRepository - 删除代码库: https://help.aliyun.com/zh/yunxiao/developer-reference/deletereposition-delete-code-base
- GetRepository - 查询代码库: https://help.aliyun.com/zh/yunxiao/developer-reference/getrepository-query-the-code-base
- ListRepositories - 查询代码库列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listrepositories-query-code-base-list
- ListTemplateRepositories - 查询模板代码库列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listtemplaterepositories-query-the-list-of-template-code-libraries

### ✅ 代码组（4个API）

- CreateGroup - 创建代码组: https://help.aliyun.com/zh/yunxiao/developer-reference/creategroup
- DeleteGroup - 删除代码组: https://help.aliyun.com/zh/yunxiao/developer-reference/deletegroup
- GetGroup - 查询代码组: https://help.aliyun.com/zh/yunxiao/developer-reference/getgroup
- ListGroups - 查询代码组列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listgroups

### ✅ 分支（4个API）

- CreateBranch - 创建分支: https://help.aliyun.com/zh/yunxiao/developer-reference/createbranch
- DeleteBranch - 删除分支: https://help.aliyun.com/zh/yunxiao/developer-reference/deletebranch
- GetBranch - 查询分支: https://help.aliyun.com/zh/yunxiao/developer-reference/getbranch
- ListBranches - 查询分支列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listbranches

### ✅ 提交（3个API）

- GetCommit - 查询提交记录: https://help.aliyun.com/zh/yunxiao/developer-reference/getcommit
- ListCommits - 查询提交记录列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listcommits
- ListRepositoryCommits - 查询代码库提交列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listrepositorycommits

### ✅ 文件（7个API）

- CreateFile - 创建文件: https://help.aliyun.com/zh/yunxiao/developer-reference/createfile
- DeleteFile - 删除文件: https://help.aliyun.com/zh/yunxiao/developer-reference/deletefile
- GetFileBlobs - 查询文件内容: https://help.aliyun.com/zh/yunxiao/developer-reference/getfileblobs
- ListFiles - 查询文件树: https://help.aliyun.com/zh/yunxiao/developer-reference/listfiles
- UpdateFile - 更新文件内容: https://help.aliyun.com/zh/yunxiao/developer-reference/updatefile
- GetFileBlame - 获取文件 blame 信息: https://help.aliyun.com/zh/yunxiao/developer-reference/getfileblame-get-file-blame-information
- CommitMultipleFiles - 多文件变更提交: https://help.aliyun.com/zh/yunxiao/developer-reference/commitmultiplefiles-multi-file-change-commit

### ✅ 标签（3个API）

- CreateTag - 创建标签: https://help.aliyun.com/zh/yunxiao/developer-reference/createtag
- DeleteTag - 删除标签: https://help.aliyun.com/zh/yunxiao/developer-reference/deletetag
- ListTags - 查询标签列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listtags

### ✅ 保护分支（5个API）

- CreateProtectedBranch - 创建保护分支: https://help.aliyun.com/zh/yunxiao/developer-reference/createprotectedbranch
- DeleteProtectedBranch - 移除保护分支: https://help.aliyun.com/zh/yunxiao/developer-reference/deleteprotectedbranch
- GetProtectedBranch - 查询保护分支: https://help.aliyun.com/zh/yunxiao/developer-reference/getprotectedbranch
- ListProtectedBranches - 查询保护分支列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listprotectedbranches
- UpdateProtectedBranch - 更新保护分支: https://help.aliyun.com/zh/yunxiao/developer-reference/updateprotectedbranch

### ✅ 合并请求（19个API）

- CreateChangeRequest - 创建合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/createchangerequest-create-merge-request
- CloseChangeRequest - 关闭合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/closechangerequest-close-merge-request
- GetChangeRequest - 查询合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/getchangerequest-query-merge-request
- AttachLabelsToChangeRequest - 关联类标到合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/attachlabelstochangerquest
- GetChangeRequestLabels - 获取合并请求的类标: https://help.aliyun.com/zh/yunxiao/developer-reference/getchangerequestlabels
- ListChangeRequests - 查询合并请求列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listchangerequests-query-the-list-of-merge-requests
- GetChangeRequestTree - 查询合并请求的变更文件树: https://help.aliyun.com/zh/yunxiao/developer-reference/getchangerequesttree-queries-the-change-file-tree-for-merge-requests
- ListChangeRequestPatchSets - 查询合并请求版本列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listchangerequestpatchsets-query-the-list-of-merge-request-versions
- ReviewChangeRequest - 评审合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/reviewchangerequest-review-merge-request
- UpdateChangeRequest - 更新合并请求基本信息: https://help.aliyun.com/zh/yunxiao/developer-reference/updatechangerequest-update-merge-request-basic-information
- UpdateChangeRequestRelatedPerson - 更新合并请求干系人: https://help.aliyun.com/zh/yunxiao/developer-reference/update-merge-request-stakeholders
- MergeChangeRequest - 合并合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/mergechangerequest-merge-merge-request
- ReopenChangeRequest - 重新打开合并请求: https://help.aliyun.com/zh/yunxiao/developer-reference/reopenchangerequest-reopen-merge-request
- GetMergeRequest - 查询合并请求(旧): https://help.aliyun.com/zh/yunxiao/developer-reference/getmergerequest-query-merge-request-old
- ListMergeRequests - 查询合并请求列表(旧): https://help.aliyun.com/zh/yunxiao/developer-reference/listmergerequests-query-the-list-of-merge-requests-old
- CreateChangeRequestComment - 创建合并请求评论: https://help.aliyun.com/zh/yunxiao/developer-reference/createchangerequestcomment
- UpdateChangeRequestComment - 更新合并请求评论: https://help.aliyun.com/zh/yunxiao/developer-reference/updatechangerequestcomment
- DeleteChangeRequestComment - 删除合并请求评论: https://help.aliyun.com/zh/yunxiao/developer-reference/deletechangerequestcomment
- ListMergeRequestComments - 查询评论列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listmergerequestcomments

### ✅ 代码比较（1个API）

- GetCompare - 查询代码比较内容: https://help.aliyun.com/zh/yunxiao/developer-reference/getcompare

### ✅ 推送规则（5个API）

- CreatePushRule - 创建推送规则: https://help.aliyun.com/zh/yunxiao/developer-reference/createpushrule
- DeletePushRule - 删除推送规则: https://help.aliyun.com/zh/yunxiao/developer-reference/deletepushrule
- GetPushRule - 查询推送规则: https://help.aliyun.com/zh/yunxiao/developer-reference/getpushrule
- ListPushRules - 查询推送规则列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listpushrules
- UpdatePushRule - 更新推送规则: https://help.aliyun.com/zh/yunxiao/developer-reference/updatepushrule

### ✅ SSH 密钥（5个API）

- CreateSSHKey - 创建 SSH Key: https://help.aliyun.com/zh/yunxiao/developer-reference/createsshkey-create-an-ssh-key
- DeleteSSHKey - 删除 SSH Key: https://help.aliyun.com/zh/yunxiao/developer-reference/deletesshkey-deletes-the-ssh-key
- GetSSHKey - 查询 SSH Key: https://help.aliyun.com/zh/yunxiao/developer-reference/getsshkey-query-the-ssh-key
- ListSSHKeys - 查询 SSH Key 列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listsshkeys-query-the-ssh-key-list
- ListUserSSHKeys - 查询指定用户 SSH Key 列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listusersshkeys

### ✅ WebHook（5个API）

- CreateWebHook - 创建 Webhook: https://help.aliyun.com/zh/yunxiao/developer-reference/createwebhook-create-a-webhook
- DeleteWebHook - 删除 WebHook: https://help.aliyun.com/zh/yunxiao/developer-reference/deletewebhook-delete-a-webhook
- GetWebHook - 查询 WebHook: https://help.aliyun.com/zh/yunxiao/developer-reference/getwebhook
- ListWebHooks - 查询 Webhook 列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listwebhooks
- UpdateWebHook - 更新 WebHook: https://help.aliyun.com/zh/yunxiao/developer-reference/updatewebhook

### ✅ 运行检查（4个API）

- CreateCheckRun - 创建运行检查: https://help.aliyun.com/zh/yunxiao/developer-reference/createcheckrun
- GetCheckRun - 查询运行检查: https://help.aliyun.com/zh/yunxiao/developer-reference/getcheckrun
- ListCheckRuns - 查询运行检查列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listcheckruns
- UpdateCheckRun - 更新运行检查: https://help.aliyun.com/zh/yunxiao/developer-reference/updatecheckrun

### ✅ 部署密钥（3个API）

- CreateDeployKey - 创建部署密钥: https://help.aliyun.com/zh/yunxiao/developer-reference/createdeploykey
- DisableDeployKey - 禁用部署密钥: https://help.aliyun.com/zh/yunxiao/developer-reference/disabledeploykey
- EnableDeployKey - 启动部署密钥: https://help.aliyun.com/zh/yunxiao/developer-reference/enabledeploykey

### ✅ 项目类标（4个API）

- CreateProjectLabel - 创建项目类标: https://help.aliyun.com/zh/yunxiao/developer-reference/createprojectlabel
- GetProjectLabels - 获取项目类标列表: https://help.aliyun.com/zh/yunxiao/developer-reference/getprojectlabels
- UpdateProjectLabel - 更新项目类标: https://help.aliyun.com/zh/yunxiao/developer-reference/updateprojectlabel
- DeleteProjectLabel - 删除项目类标: https://help.aliyun.com/zh/yunxiao/developer-reference/deleteprojectlabel

### ✅ 提交状态（2个API）

- CreateCommitStatus - 创建提交状态: https://help.aliyun.com/zh/yunxiao/developer-reference/createcommitstatus-create-commit-status
- ListCommitStatuses - 查询提交状态列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listcommitstatuses-query-the-submission-status-list

### ✅ 成员（9个API）

- CreateGroupMember - 增加代码组成员: https://help.aliyun.com/zh/yunxiao/developer-reference/creategroup-member-add-code-group-member
- CreateRepositoryMember - 增加代码库成员: https://help.aliyun.com/zh/yunxiao/developer-reference/createrepositorymember-add-code-base-member
- DeleteGroupMember - 移除代码组成员: https://help.aliyun.com/zh/yunxiao/developer-reference/deletegroupmember-remove-code-group-members
- DeleteRepositoryMember - 移除代码库成员: https://help.aliyun.com/zh/yunxiao/developer-reference/deleterepositorymember-remove-code-base-member
- ListGroupMembers - 查询代码组成员列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listgroupmembers-query-code-group-member-list
- ListRepositoryMembers - 查询代码库成员列表: https://help.aliyun.com/zh/yunxiao/developer-reference/listrepositorymembers-query-code-base-member-list
- UpdateGroupMember - 更改代码组成员的权限: https://help.aliyun.com/zh/yunxiao/developer-reference/updategroup-member-change-permissions-for-code-group-members
- UpdateRepositoryMember - 更改代码库成员的权限: https://help.aliyun.com/zh/yunxiao/developer-reference/updaterepositorymember-change-permissions-for-code-base-members
- GetMemberHttpsCloneUsername - 查询用户克隆账号: https://help.aliyun.com/zh/yunxiao/developer-reference/getmemberhttpscloneusername

### ✅ 用户资源（1个API）

- ListUserResources - 查询用户有权限的资源: https://help.aliyun.com/zh/yunxiao/developer-reference/listuserresources-query-the-resources-to-which-the-user-has-permissions

---

## 相关资源

### API 文档入口
- **文档首页**: https://help.aliyun.com/zh/yunxiao/developer-reference/codeup/
- **OpenAPI 门户**: https://api.aliyun.com/product/codeup
- **服务接入点**: https://help.aliyun.com/zh/yunxiao/developer-reference/service-access-point-domain
- **获取个人访问令牌**: https://help.aliyun.com/zh/yunxiao/developer-reference/obtain-personal-access-token
- **错误码中心**: https://help.aliyun.com/zh/yunxiao/developer-reference/error-code-center
- **云效 MCP 工具使用说明**: https://help.aliyun.com/zh/yunxiao/developer-reference/cloud-effect-mcp-tool-instructions

---

## 收集统计

| 子分类 | API数量 |
|--------|---------|
| 代码仓库 | 5 |
| 代码组 | 4 |
| 分支 | 4 |
| 提交 | 3 |
| 文件 | 7 |
| 标签 | 3 |
| 保护分支 | 5 |
| 合并请求 | 19 |
| 代码比较 | 1 |
| 推送规则 | 5 |
| SSH 密钥 | 5 |
| WebHook | 5 |
| 运行检查 | 4 |
| 部署密钥 | 3 |
| 项目类标 | 4 |
| 提交状态 | 2 |
| 成员 | 9 |
| 用户资源 | 1 |
| **总计** | **89** |

---

*文件更新时间：2026-04-06*
