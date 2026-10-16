# TransferRepository - 转移代码库

> 来源: https://help.aliyun.com/zh/yunxiao/developer-reference/transfer-repository-transfer-repository
> 抓取时间: 2026-10-16（agent-browser 渲染提取）
> 说明: 官方导航列了两个 TransferRepository 页面（另一为 transfer-repository-transfer-repository-1），内容重复，仅 organizationId 必填说明略有差异，已合并到本文件。

通过 OpenAPI 将代码库转移到目标代码组下。repositoryId、groupId 支持纯数字（资源主键）或 path_with_namespace 全路径字符串。

| 适用版本 | 中心版、Region版 |
| --- | --- |

## 服务接入点与授权信息

- 获取服务接入点：替换 API 请求语法中的 {domain} 。
- 获取个人访问令牌。
- 获取organizationId：仅中心版需要。请前往组织管理后台的基本信息页面获取组织 ID 。

| 产品 | 资源 | 所需权限 |
| --- | --- | --- |
| 代码管理 | 代码仓库 | 读写 |

## 请求语法

### 中心版

```
POST https://{domain}/oapi/v1/codeup/organizations/{organizationId}/repositories/transfer
```

### Region版

```
POST https://{domain}/oapi/v1/codeup/repositories/transfer
```

## 请求头

| 参数 | 类型 | 是否必填 | 描述 | 示例值 |
| --- | --- | --- | --- | --- |
| x-yunxiao-token | string | 是 | 个人访问令牌。 | pt-0fh3****0fbG_35af****0484 |

## 请求参数

| 参数 | 类型 | 位置 | 是否必填 | 描述 | 示例值 |
| --- | --- | --- | --- | --- | --- |
| organizationId | string | path | 是：中心版 否：Region版 | 组织 ID。 | 60d54f3daccf2bbd6659f3ad |
| - | object | body | 否 |  |  |
| groupId | string | body | 是 | 目标代码组 ID（数字字符串）或 path_with_namespace 全路径。 | 60de7a6852743a5162b5f957/target-group |
| repositoryId | string | body | 是 | 代码库 ID（数字字符串）或 path_with_namespace 全路径。 | 60de7a6852743a5162b5f957/DemoRepo |

## 请求示例

### 中心版

```
curl -X 'POST' \
  'https://{domain}/oapi/v1/codeup/organizations/60d54f3daccf2bbd6659f3ad/repositories/transfer' \
  -H 'Content-Type: application/json' \
  -H 'x-yunxiao-token: pt-0fh3****0fbG_35af****0484' \
  --data '
    {
        "groupId": "60de7a6852743a5162b5f957/target-group",
        "repositoryId": "60de7a6852743a5162b5f957/DemoRepo"
    }'
```

### Region版

```
curl -X 'POST' \
  'https://{domain}/oapi/v1/codeup/repositories/transfer' \
  -H 'Content-Type: application/json' \
  -H 'x-yunxiao-token: pt-0fh3****0fbG_35af****0484' \
  --data '
    {
        "groupId": "60de7a6852743a5162b5f957/target-group",
        "repositoryId": "60de7a6852743a5162b5f957/DemoRepo"
    }'
```

## 返回参数

| 参数 | 类型 | 描述 | 示例值 |
| --- | --- | --- | --- |
| - | object |  |  |
| accessLevel | integer | 当前用户在该代码库上的权限类型，可能的值：[20 30 40]。 | 40 |
| archived | boolean | 代码库是否归档。 | false |
| avatarUrl | string | 头像地址。 | https://example/example/w/100/h/100 |
| createdAt | string | 创建时间。 | 2024-10-05T15:30:45Z |
| creatorId | integer | 代码库创建者。 | 1 |
| creatorUid | string | 代码库创建者（云效用户 ID）。 | 5f8b3b3b3b3b3b3b3b3b3b3b |
| demoProject | boolean | 是否是 demo 库。 | false |
| description | string | 代码库描述。 | demo repo |
| encrypted | boolean | 是否加密。 | false |
| id | integer | 代码库 ID。 | 2813489 |
| lastActivityAt | string | 最后活跃时间。 | 2024-10-05T15:30:45Z |
| name | string | 代码库名称。 | demo-repo |
| nameWithNamespace | string | 代码库完整名称（含完整组名称）。 | 60de7a6852743a5162b5f957 / DemoRepo |
| namespaceId | integer | 上级路径的 ID。 | 2813489 |
| path | string | 代码库路径。 | demo-repo |
| pathWithNamespace | string | 代码库完整路径（含完整组路径）。 | 60de7a6852743a5162b5f957/DemoRepo |
| repositorySize | string | 代码库大小(MB)。 | 1 |
| repositoryTags | array | 代码库标签列表。 |  |
| - | object | 代码库标签。 |  |
| tagColor | string | 标签颜色。 | #C4C6CF |
| tagName | string | 标签名称。 | 标签1 |
| starCount | integer | 被收藏的数量。 | 1 |
| starred | boolean | 是否被当前用户收藏。 | false |
| updatedAt | string | 最近更新时间。 | 2024-10-05T15:30:45Z |
| visibility | string | 可见性：private（私有）、internal（组织内公开），可能的值：[private internal]。 | private |
| webUrl | string | 页面访问时的 URL。 | http://example.com/org-demo/example-repo |

## 返回示例

```
{
    "accessLevel": 40,
    "archived": false,
    "avatarUrl": "https://example/example/w/100/h/100",
    "createdAt": "2024-10-05T15:30:45Z",
    "creatorId": 1,
    "creatorUid": "5f8b3b3b3b3b3b3b3b3b3b3b",
    "demoProject": false,
    "description": "demo repo",
    "encrypted": false,
    "id": 2813489,
    "lastActivityAt": "2024-10-05T15:30:45Z",
    "name": "demo-repo",
    "nameWithNamespace": "60de7a6852743a5162b5f957 / DemoRepo",
    "namespaceId": 2813489,
    "path": "demo-repo",
    "pathWithNamespace": "60de7a6852743a5162b5f957/DemoRepo",
    "repositorySize": "1",
    "repositoryTags": [
        {
            "tagColor": "#C4C6CF",
            "tagName": "标签1"
        }
    ],
    "starCount": 1,
    "starred": false,
    "updatedAt": "2024-10-05T15:30:45Z",
    "visibility": "private",
    "webUrl": "http://example.com/org-demo/example-repo"
}
```

## 错误码

访问错误码中心查看 API 相关错误码。
