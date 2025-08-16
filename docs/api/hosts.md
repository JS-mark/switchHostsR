# Hosts API

Hosts API 是 Switch Hosts R 的核心 API，提供了完整的 Hosts 文件管理功能。通过这些 API，您可以创建、读取、更新和删除 Hosts 配置，以及执行批量操作和高级管理功能。

## API 端点概览

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/hosts` | 获取 Hosts 配置列表 |
| GET | `/hosts/{id}` | 获取指定 Hosts 配置 |
| POST | `/hosts` | 创建新的 Hosts 配置 |
| PUT | `/hosts/{id}` | 更新 Hosts 配置 |
| PATCH | `/hosts/{id}` | 部分更新 Hosts 配置 |
| DELETE | `/hosts/{id}` | 删除 Hosts 配置 |
| POST | `/hosts/{id}/enable` | 启用 Hosts 配置 |
| POST | `/hosts/{id}/disable` | 禁用 Hosts 配置 |
| POST | `/hosts/{id}/duplicate` | 复制 Hosts 配置 |
| POST | `/hosts/batch` | 批量操作 |
| GET | `/hosts/{id}/history` | 获取变更历史 |
| POST | `/hosts/import` | 导入 Hosts 配置 |
| POST | `/hosts/export` | 导出 Hosts 配置 |

## 数据模型

### Hosts 配置对象

```typescript
interface HostsConfig {
  id: string                    // 唯一标识符
  name: string                  // 配置名称
  content: string               // Hosts 文件内容
  enabled: boolean              // 是否启用
  description?: string          // 描述信息
  tags: string[]               // 标签列表
  type: 'local' | 'remote'     // 配置类型
  remote_url?: string          // 远程 URL（仅远程类型）
  auto_refresh?: boolean       // 自动刷新（仅远程类型）
  refresh_interval?: number    // 刷新间隔（秒）
  created_at: string           // 创建时间
  updated_at: string           // 更新时间
  created_by: string           // 创建者 ID
  updated_by: string           // 更新者 ID
  version: number              // 版本号
  checksum: string             // 内容校验和
  metadata: Record<string, any> // 元数据
}
```

### 响应格式

```typescript
interface ApiResponse<T> {
  success: boolean
  data: T
  message?: string
  meta?: {
    total?: number
    page?: number
    per_page?: number
    total_pages?: number
  }
  links?: {
    first?: string
    prev?: string
    next?: string
    last?: string
  }
}
```

## 获取 Hosts 配置列表

### 基本请求

```bash
GET /api/v1/hosts
```

### 查询参数

| 参数 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `page` | number | 页码 | 1 |
| `per_page` | number | 每页数量 | 20 |
| `sort` | string | 排序字段 | created_at |
| `order` | string | 排序方向 (asc/desc) | desc |
| `search` | string | 搜索关键词 | - |
| `enabled` | boolean | 过滤启用状态 | - |
| `type` | string | 过滤配置类型 | - |
| `tags` | string | 过滤标签（逗号分隔） | - |
| `created_by` | string | 过滤创建者 | - |
| `created_after` | string | 创建时间起始 | - |
| `created_before` | string | 创建时间结束 | - |

### 请求示例

```bash
# 获取所有启用的 Hosts 配置
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/hosts?enabled=true"

# 搜索包含 "dev" 的配置
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/hosts?search=dev"

# 按标签过滤
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/hosts?tags=production,api"
```

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "id": "host_001",
      "name": "开发环境",
      "content": "127.0.0.1 api.dev.example.com\n127.0.0.1 web.dev.example.com",
      "enabled": true,
      "description": "开发环境的 API 和 Web 服务",
      "tags": ["development", "api"],
      "type": "local",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z",
      "created_by": "user_123",
      "updated_by": "user_123",
      "version": 2,
      "checksum": "sha256:abc123..."
    }
  ],
  "meta": {
    "total": 25,
    "page": 1,
    "per_page": 20,
    "total_pages": 2
  },
  "links": {
    "next": "/api/v1/hosts?page=2&per_page=20"
  }
}
```

## 获取单个 Hosts 配置

### 请求

```bash
GET /api/v1/hosts/{id}
```

### 路径参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | string | Hosts 配置 ID |

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `include_history` | boolean | 是否包含变更历史 |
| `include_metadata` | boolean | 是否包含元数据 |

### 请求示例

```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/hosts/host_001?include_history=true"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "host_001",
    "name": "开发环境",
    "content": "127.0.0.1 api.dev.example.com\n127.0.0.1 web.dev.example.com",
    "enabled": true,
    "description": "开发环境的 API 和 Web 服务",
    "tags": ["development", "api"],
    "type": "local",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z",
    "created_by": "user_123",
    "updated_by": "user_123",
    "version": 2,
    "checksum": "sha256:abc123...",
    "metadata": {
      "source": "manual",
      "environment": "development"
    },
    "history": [
      {
        "version": 2,
        "action": "update",
        "changes": {
          "content": {
            "old": "127.0.0.1 api.dev.example.com",
            "new": "127.0.0.1 api.dev.example.com\n127.0.0.1 web.dev.example.com"
          }
        },
        "updated_by": "user_123",
        "updated_at": "2024-01-01T12:00:00Z"
      }
    ]
  }
}
```

## 创建 Hosts 配置

### 请求

```bash
POST /api/v1/hosts
```

### 请求体

```typescript
interface CreateHostsRequest {
  name: string                  // 必需：配置名称
  content: string               // 必需：Hosts 内容
  description?: string          // 可选：描述
  tags?: string[]              // 可选：标签
  enabled?: boolean            // 可选：是否启用（默认 false）
  type?: 'local' | 'remote'    // 可选：类型（默认 local）
  remote_url?: string          // 可选：远程 URL
  auto_refresh?: boolean       // 可选：自动刷新
  refresh_interval?: number    // 可选：刷新间隔
  metadata?: Record<string, any> // 可选：元数据
}
```

### 请求示例

```bash
curl -X POST "https://api.switchhosts.com/v1/hosts" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "生产环境",
    "content": "192.168.1.100 api.example.com\n192.168.1.101 web.example.com",
    "description": "生产环境的服务器配置",
    "tags": ["production", "api"],
    "enabled": false,
    "metadata": {
      "environment": "production",
      "team": "backend"
    }
  }'
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "host_002",
    "name": "生产环境",
    "content": "192.168.1.100 api.example.com\n192.168.1.101 web.example.com",
    "enabled": false,
    "description": "生产环境的服务器配置",
    "tags": ["production", "api"],
    "type": "local",
    "created_at": "2024-01-02T00:00:00Z",
    "updated_at": "2024-01-02T00:00:00Z",
    "created_by": "user_123",
    "updated_by": "user_123",
    "version": 1,
    "checksum": "sha256:def456..."
  },
  "message": "Hosts 配置创建成功"
}
```

## 更新 Hosts 配置

### 完整更新 (PUT)

```bash
PUT /api/v1/hosts/{id}
```

### 部分更新 (PATCH)

```bash
PATCH /api/v1/hosts/{id}
```

### 请求体

```typescript
// PUT 请求需要完整的配置对象
interface UpdateHostsRequest {
  name: string
  content: string
  description?: string
  tags?: string[]
  enabled?: boolean
  type?: 'local' | 'remote'
  remote_url?: string
  auto_refresh?: boolean
  refresh_interval?: number
  metadata?: Record<string, any>
}

// PATCH 请求只需要要更新的字段
interface PatchHostsRequest {
  name?: string
  content?: string
  description?: string
  tags?: string[]
  enabled?: boolean
  // ... 其他可选字段
}
```

### 请求示例

```bash
# 完整更新
curl -X PUT "https://api.switchhosts.com/v1/hosts/host_002" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "生产环境 - 更新",
    "content": "192.168.1.100 api.example.com\n192.168.1.101 web.example.com\n192.168.1.102 cdn.example.com",
    "description": "生产环境的服务器配置（已更新）",
    "tags": ["production", "api", "cdn"],
    "enabled": true
  }'

# 部分更新
curl -X PATCH "https://api.switchhosts.com/v1/hosts/host_002" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": true,
    "description": "已启用的生产环境配置"
  }'
```

## 删除 Hosts 配置

### 请求

```bash
DELETE /api/v1/hosts/{id}
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `force` | boolean | 强制删除（忽略依赖检查） |

### 请求示例

```bash
curl -X DELETE "https://api.switchhosts.com/v1/hosts/host_002" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### 响应示例

```json
{
  "success": true,
  "message": "Hosts 配置删除成功"
}
```

## 启用/禁用 Hosts 配置

### 启用配置

```bash
POST /api/v1/hosts/{id}/enable
```

### 禁用配置

```bash
POST /api/v1/hosts/{id}/disable
```

### 请求示例

```bash
# 启用配置
curl -X POST "https://api.switchhosts.com/v1/hosts/host_001/enable" \
  -H "Authorization: Bearer YOUR_API_KEY"

# 禁用配置
curl -X POST "https://api.switchhosts.com/v1/hosts/host_001/disable" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "host_001",
    "enabled": true
  },
  "message": "Hosts 配置已启用"
}
```

## 复制 Hosts 配置

### 请求

```bash
POST /api/v1/hosts/{id}/duplicate
```

### 请求体

```typescript
interface DuplicateHostsRequest {
  name?: string     // 新配置名称（默认为 "原名称 - 副本"）
  enabled?: boolean // 是否启用新配置（默认 false）
}
```

### 请求示例

```bash
curl -X POST "https://api.switchhosts.com/v1/hosts/host_001/duplicate" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "开发环境 - 备份",
    "enabled": false
  }'
```

## 批量操作

### 请求

```bash
POST /api/v1/hosts/batch
```

### 请求体

```typescript
interface BatchOperation {
  action: 'create' | 'update' | 'delete' | 'enable' | 'disable'
  items: Array<{
    id?: string        // 对于 update/delete/enable/disable 操作必需
    data?: any         // 对于 create/update 操作的数据
  }>
}
```

### 请求示例

```bash
# 批量创建
curl -X POST "https://api.switchhosts.com/v1/hosts/batch" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "action": "create",
    "items": [
      {
        "data": {
          "name": "测试环境1",
          "content": "127.0.0.1 test1.example.com"
        }
      },
      {
        "data": {
          "name": "测试环境2",
          "content": "127.0.0.1 test2.example.com"
        }
      }
    ]
  }'

# 批量启用
curl -X POST "https://api.switchhosts.com/v1/hosts/batch" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "action": "enable",
    "items": [
      { "id": "host_001" },
      { "id": "host_002" }
    ]
  }'
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "successful": [
      {
        "id": "host_003",
        "name": "测试环境1",
        "status": "created"
      },
      {
        "id": "host_004",
        "name": "测试环境2",
        "status": "created"
      }
    ],
    "failed": []
  },
  "message": "批量操作完成：2 个成功，0 个失败"
}
```

## 变更历史

### 请求

```bash
GET /api/v1/hosts/{id}/history
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `page` | number | 页码 |
| `per_page` | number | 每页数量 |
| `action` | string | 过滤操作类型 |
| `user` | string | 过滤操作用户 |

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "version": 3,
      "action": "update",
      "changes": {
        "content": {
          "old": "127.0.0.1 api.dev.example.com",
          "new": "127.0.0.1 api.dev.example.com\n127.0.0.1 web.dev.example.com"
        },
        "enabled": {
          "old": false,
          "new": true
        }
      },
      "updated_by": "user_123",
      "updated_at": "2024-01-01T12:00:00Z",
      "comment": "添加 Web 服务域名"
    }
  ]
}
```

## 导入导出

### 导入 Hosts 配置

```bash
POST /api/v1/hosts/import
```

#### 请求体

```typescript
interface ImportRequest {
  format: 'json' | 'hosts' | 'csv'
  data: string | object
  options?: {
    merge_duplicates?: boolean  // 合并重复项
    overwrite_existing?: boolean // 覆盖已存在的配置
    default_enabled?: boolean   // 默认启用状态
  }
}
```

#### 请求示例

```bash
# 导入 JSON 格式
curl -X POST "https://api.switchhosts.com/v1/hosts/import" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "json",
    "data": [
      {
        "name": "导入配置1",
        "content": "127.0.0.1 import1.example.com"
      }
    ],
    "options": {
      "default_enabled": false
    }
  }'

# 导入 Hosts 文件格式
curl -X POST "https://api.switchhosts.com/v1/hosts/import" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "hosts",
    "data": "127.0.0.1 localhost\n192.168.1.100 api.example.com"
  }'
```

### 导出 Hosts 配置

```bash
POST /api/v1/hosts/export
```

#### 请求体

```typescript
interface ExportRequest {
  format: 'json' | 'hosts' | 'csv' | 'yaml'
  ids?: string[]              // 指定要导出的配置 ID
  filters?: {
    enabled?: boolean
    tags?: string[]
    type?: 'local' | 'remote'
  }
  options?: {
    include_metadata?: boolean
    include_history?: boolean
    compress?: boolean
  }
}
```

#### 请求示例

```bash
# 导出所有启用的配置为 JSON
curl -X POST "https://api.switchhosts.com/v1/hosts/export" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "json",
    "filters": {
      "enabled": true
    },
    "options": {
      "include_metadata": true
    }
  }'

# 导出指定配置为 Hosts 文件格式
curl -X POST "https://api.switchhosts.com/v1/hosts/export" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "hosts",
    "ids": ["host_001", "host_002"]
  }'
```

## 错误处理

### 常见错误

#### 配置不存在

```json
{
  "success": false,
  "error": {
    "code": "HOSTS_NOT_FOUND",
    "message": "Hosts 配置不存在",
    "details": "ID 为 'host_999' 的配置不存在"
  }
}
```

#### 配置名称重复

```json
{
  "success": false,
  "error": {
    "code": "HOSTS_NAME_EXISTS",
    "message": "配置名称已存在",
    "details": "名称为 '开发环境' 的配置已存在"
  }
}
```

#### 内容格式错误

```json
{
  "success": false,
  "error": {
    "code": "HOSTS_INVALID_FORMAT",
    "message": "Hosts 内容格式无效",
    "details": {
      "line": 3,
      "content": "invalid line content",
      "reason": "IP 地址格式不正确"
    }
  }
}
```

## 使用示例

### JavaScript/Node.js SDK

```javascript
const { SwitchHostsAPI } = require('@switchhosts/sdk')

const api = new SwitchHostsAPI({
  apiKey: 'sk_live_1234567890abcdef',
  baseURL: 'https://api.switchhosts.com/v1'
})

// 获取所有配置
const hosts = await api.hosts.list({
  enabled: true,
  tags: ['production']
})

// 创建新配置
const newHost = await api.hosts.create({
  name: '新配置',
  content: '127.0.0.1 new.example.com',
  tags: ['development']
})

// 启用配置
await api.hosts.enable(newHost.id)

// 批量操作
await api.hosts.batch({
  action: 'enable',
  items: [{ id: 'host_001' }, { id: 'host_002' }]
})
```

### Python SDK

```python
from switchhosts import SwitchHostsAPI

api = SwitchHostsAPI(
    api_key='sk_live_1234567890abcdef',
    base_url='https://api.switchhosts.com/v1'
)

# 获取配置列表
hosts = api.hosts.list(enabled=True, tags=['production'])

# 创建配置
new_host = api.hosts.create(
    name='新配置',
    content='127.0.0.1 new.example.com',
    tags=['development']
)

# 更新配置
api.hosts.update(new_host['id'], {
    'description': '更新的描述',
    'enabled': True
})
```

### Go SDK

```go
package main

import (
    "github.com/switchhosts/go-sdk"
)

func main() {
    client := switchhosts.NewClient("sk_live_1234567890abcdef")
    
    // 获取配置列表
    hosts, err := client.Hosts.List(&switchhosts.HostsListOptions{
        Enabled: true,
        Tags:    []string{"production"},
    })
    
    // 创建配置
    newHost, err := client.Hosts.Create(&switchhosts.HostsCreateRequest{
        Name:    "新配置",
        Content: "127.0.0.1 new.example.com",
        Tags:    []string{"development"},
    })
    
    // 启用配置
    err = client.Hosts.Enable(newHost.ID)
}
```

通过这些丰富的 API 端点，您可以完全通过编程方式管理 Switch Hosts R 的所有 Hosts 配置，实现自动化的域名解析管理。