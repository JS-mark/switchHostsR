# API 文档

Switch Hosts R 提供了完整的 RESTful API，支持所有核心功能的编程访问。通过 API，您可以将 Switch Hosts R 集成到自动化工作流、CI/CD 流程或其他应用程序中。

## API 概述

### 基础信息

- **API 版本**：v1
- **基础 URL**：`http://localhost:46754/api/v1`
- **数据格式**：JSON
- **认证方式**：API Key / JWT Token
- **请求方法**：GET, POST, PUT, DELETE

### API 特性

- **RESTful 设计**：遵循 REST 架构原则
- **统一响应格式**：标准化的响应结构
- **错误处理**：详细的错误码和错误信息
- **分页支持**：大数据集的分页查询
- **过滤排序**：灵活的数据过滤和排序
- **版本控制**：向后兼容的版本管理

## 快速开始

### 1. 获取 API Key

在 Switch Hosts R 设置中生成 API Key：

```bash
# 通过 CLI 生成 API Key
switchhosts-cli auth generate-key --name "My API Key"
```

### 2. 发送第一个请求

```bash
# 获取所有 Hosts 配置
curl -H "Authorization: Bearer YOUR_API_KEY" \
     http://localhost:46754/api/v1/hosts
```

### 3. 响应格式

```json
{
  "success": true,
  "data": [
    {
      "id": "host_001",
      "name": "开发环境",
      "content": "127.0.0.1 api.example.com",
      "enabled": true,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "total": 1,
    "page": 1,
    "per_page": 20
  }
}
```

## API 端点概览

### Hosts 管理

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/hosts` | 获取所有 Hosts 配置 |
| GET | `/hosts/{id}` | 获取指定 Hosts 配置 |
| POST | `/hosts` | 创建新的 Hosts 配置 |
| PUT | `/hosts/{id}` | 更新 Hosts 配置 |
| DELETE | `/hosts/{id}` | 删除 Hosts 配置 |
| POST | `/hosts/{id}/enable` | 启用 Hosts 配置 |
| POST | `/hosts/{id}/disable` | 禁用 Hosts 配置 |

### 用户管理

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/users` | 获取用户列表 |
| GET | `/users/{id}` | 获取用户信息 |
| POST | `/users` | 创建新用户 |
| PUT | `/users/{id}` | 更新用户信息 |
| DELETE | `/users/{id}` | 删除用户 |

### 日志查询

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/logs` | 获取日志列表 |
| GET | `/logs/{id}` | 获取日志详情 |
| POST | `/logs/export` | 导出日志 |

## 请求格式

### 请求头

```
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY
User-Agent: YourApp/1.0
```

### 查询参数

```bash
# 分页
GET /api/v1/hosts?page=1&per_page=20

# 过滤
GET /api/v1/hosts?enabled=true&name=dev

# 排序
GET /api/v1/hosts?sort=created_at&order=desc

# 搜索
GET /api/v1/hosts?search=example.com
```

### 请求体示例

```json
{
  "name": "生产环境",
  "content": "192.168.1.100 api.example.com\n192.168.1.101 web.example.com",
  "enabled": false,
  "description": "生产环境的 Hosts 配置",
  "tags": ["production", "api"]
}
```

## 响应格式

### 成功响应

```json
{
  "success": true,
  "data": {
    "id": "host_001",
    "name": "生产环境",
    "content": "192.168.1.100 api.example.com",
    "enabled": false,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "message": "Hosts 配置创建成功"
}
```

### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请求参数验证失败",
    "details": {
      "name": ["名称不能为空"],
      "content": ["内容格式不正确"]
    }
  },
  "request_id": "req_123456789"
}
```

## 状态码

| 状态码 | 描述 |
|--------|------|
| 200 | 请求成功 |
| 201 | 资源创建成功 |
| 400 | 请求参数错误 |
| 401 | 未授权访问 |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 409 | 资源冲突 |
| 422 | 请求参数验证失败 |
| 429 | 请求频率限制 |
| 500 | 服务器内部错误 |

## 错误码

### 通用错误码

| 错误码 | 描述 |
|--------|------|
| `INVALID_REQUEST` | 无效的请求 |
| `VALIDATION_ERROR` | 参数验证失败 |
| `AUTHENTICATION_FAILED` | 认证失败 |
| `AUTHORIZATION_FAILED` | 授权失败 |
| `RESOURCE_NOT_FOUND` | 资源不存在 |
| `RESOURCE_CONFLICT` | 资源冲突 |
| `RATE_LIMIT_EXCEEDED` | 请求频率超限 |
| `INTERNAL_ERROR` | 内部服务器错误 |

### 业务错误码

| 错误码 | 描述 |
|--------|------|
| `HOSTS_INVALID_FORMAT` | Hosts 文件格式无效 |
| `HOSTS_ALREADY_EXISTS` | Hosts 配置已存在 |
| `HOSTS_IN_USE` | Hosts 配置正在使用中 |
| `USER_ALREADY_EXISTS` | 用户已存在 |
| `PERMISSION_DENIED` | 权限被拒绝 |

## 分页

### 分页参数

```bash
GET /api/v1/hosts?page=2&per_page=50
```

### 分页响应

```json
{
  "success": true,
  "data": [...],
  "meta": {
    "total": 150,
    "page": 2,
    "per_page": 50,
    "total_pages": 3,
    "has_next": true,
    "has_prev": true
  },
  "links": {
    "first": "/api/v1/hosts?page=1&per_page=50",
    "prev": "/api/v1/hosts?page=1&per_page=50",
    "next": "/api/v1/hosts?page=3&per_page=50",
    "last": "/api/v1/hosts?page=3&per_page=50"
  }
}
```

## 过滤和搜索

### 过滤参数

```bash
# 按状态过滤
GET /api/v1/hosts?enabled=true

# 按标签过滤
GET /api/v1/hosts?tags=production,api

# 按日期范围过滤
GET /api/v1/hosts?created_after=2024-01-01&created_before=2024-12-31
```

### 搜索功能

```bash
# 全文搜索
GET /api/v1/hosts?search=example.com

# 字段搜索
GET /api/v1/hosts?name_contains=dev&content_contains=localhost
```

## 批量操作

### 批量创建

```json
POST /api/v1/hosts/batch
{
  "items": [
    {
      "name": "开发环境1",
      "content": "127.0.0.1 dev1.example.com"
    },
    {
      "name": "开发环境2",
      "content": "127.0.0.1 dev2.example.com"
    }
  ]
}
```

### 批量更新

```json
PUT /api/v1/hosts/batch
{
  "items": [
    {
      "id": "host_001",
      "enabled": true
    },
    {
      "id": "host_002",
      "enabled": false
    }
  ]
}
```

## 实时通知

### WebSocket 连接

```javascript
const ws = new WebSocket('ws://localhost:46754/api/v1/ws')

ws.onmessage = (event) => {
  const data = JSON.parse(event.data)
  console.log('收到通知:', data)
}
```

### 事件类型

```json
{
  "type": "hosts.updated",
  "data": {
    "id": "host_001",
    "name": "开发环境",
    "enabled": true
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## SDK 和工具

### 官方 SDK

- **JavaScript/TypeScript**：`@switchhosts/sdk-js`
- **Python**：`switchhosts-python`
- **Go**：`github.com/switchhosts/go-sdk`
- **Rust**：`switchhosts-rs`

### 第三方工具

- **Postman Collection**：预配置的 API 测试集合
- **OpenAPI Spec**：标准的 API 规范文件
- **CLI 工具**：命令行接口工具

## 最佳实践

### 性能优化

1. **使用分页**：避免一次性获取大量数据
2. **合理过滤**：使用过滤参数减少数据传输
3. **缓存结果**：对不经常变化的数据进行缓存
4. **批量操作**：使用批量 API 减少请求次数

### 错误处理

1. **检查状态码**：根据 HTTP 状态码判断请求结果
2. **解析错误信息**：使用错误码和详细信息进行错误处理
3. **重试机制**：对临时性错误实现重试逻辑
4. **日志记录**：记录 API 调用日志便于调试

### 安全考虑

1. **保护 API Key**：不要在客户端代码中暴露 API Key
2. **使用 HTTPS**：在生产环境中使用 HTTPS 协议
3. **限制权限**：为不同用途创建不同权限的 API Key
4. **定期轮换**：定期更新 API Key 提高安全性

通过这些 API，您可以轻松地将 Switch Hosts R 集成到您的工作流程中，实现自动化的 Hosts 文件管理。