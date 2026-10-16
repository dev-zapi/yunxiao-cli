# DeleteRepository - 删除代码库

> 来源: https://help.aliyun.com/zh/yunxiao/developer-reference/deletereposition-delete-code-base
> 抓取时间: 2026-10-16（agent-browser 渲染提取）

通过 OpenAPI 删除指定代码库。

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
DELETE https://{domain}/oapi/v1/codeup/organizations/{organizationId}/repositories/{repositoryId}
```

### Region版

```
DELETE https://{domain}/oapi/v1/codeup/repositories/{repositoryId}
```

## 请求头

| 参数 | 类型 | 是否必填 | 描述 | 示例值 |
| --- | --- | --- | --- | --- |
| x-yunxiao-token | string | 是 | 个人访问令牌。 | pt-0fh3****0fbG_35af****0484 |

## 请求参数

| 参数 | 类型 | 位置 | 是否必填 | 描述 | 示例值 |
| --- | --- | --- | --- | --- | --- |
| organizationId | string | path | 是：中心版 否：Region版 | 组织 ID。 | 60d54f3daccf2bbd6659f3ad |
| repositoryId | string | path | 是 | 代码库 ID 或者 URL-Encoder 编码的全路径。 | 2813489或者60de7a6852743a5162b5f957%2FDemoRepo |

## 请求示例

### 中心版

```
curl -X 'DELETE' \
  'https://{domain}/oapi/v1/codeup/organizations/60d54f3daccf2bbd6659f3ad/repositories/2813489或者60de7a6852743a5162b5f957%2FDemoRepo' \
  -H 'Content-Type: application/json' \
  -H 'x-yunxiao-token: pt-0fh3****0fbG_35af****0484'
```

### Region版

```
curl -X 'DELETE' \
  'https://{domain}/oapi/v1/codeup/repositories/2813489或者60de7a6852743a5162b5f957%2FDemoRepo' \
  -H 'Content-Type: application/json' \
  -H 'x-yunxiao-token: pt-0fh3****0fbG_35af****0484'
```

## 返回参数

| 参数 | 类型 | 描述 | 示例值 |
| --- | --- | --- | --- |
| - | object |  |  |
| result | boolean | 执行结果。 | true |

## 返回示例

```
{
    "result": true
}
```

## 错误码

访问错误码中心查看 API 相关错误码。
