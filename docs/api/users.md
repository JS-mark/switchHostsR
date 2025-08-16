# Users API

Users API 提供了完整的用户管理功能，包括用户账户管理、权限控制、团队协作等。通过这些 API，管理员可以管理用户账户，配置权限，以及实现团队协作功能。

## API 端点概览

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/users` | 获取用户列表 |
| GET | `/users/{id}` | 获取用户详情 |
| POST | `/users` | 创建新用户 |
| PUT | `/users/{id}` | 更新用户信息 |
| PATCH | `/users/{id}` | 部分更新用户 |
| DELETE | `/users/{id}` | 删除用户 |
| POST | `/users/{id}/activate` | 激活用户 |
| POST | `/users/{id}/deactivate` | 停用用户 |
| POST | `/users/{id}/reset-password` | 重置密码 |
| GET | `/users/{id}/permissions` | 获取用户权限 |
| PUT | `/users/{id}/permissions` | 更新用户权限 |
| GET | `/users/{id}/activity` | 获取用户活动日志 |
| POST | `/users/batch` | 批量用户操作 |
| GET | `/users/me` | 获取当前用户信息 |
| PUT | `/users/me` | 更新当前用户信息 |
| POST | `/users/me/change-password` | 修改当前用户密码 |

## 数据模型

### 用户对象

```typescript
interface User {
  id: string                    // 用户唯一标识
  username: string              // 用户名
  email: string                 // 邮箱地址
  display_name: string          // 显示名称
  avatar_url?: string           // 头像 URL
  role: string                  // 用户角色
  permissions: string[]         // 权限列表
  is_active: boolean           // 是否激活
  is_verified: boolean         // 是否已验证邮箱
  last_login_at?: string       // 最后登录时间
  created_at: string           // 创建时间
  updated_at: string           // 更新时间
  created_by?: string          // 创建者 ID
  metadata: Record<string, any> // 用户元数据
  preferences: UserPreferences  // 用户偏好设置
  profile: UserProfile         // 用户资料
}

interface UserPreferences {
  language: string             // 界面语言
  timezone: string             // 时区
  theme: 'light' | 'dark' | 'auto' // 主题
  notifications: {
    email: boolean             // 邮件通知
    desktop: boolean           // 桌面通知
    mobile: boolean            // 移动端通知
  }
}

interface UserProfile {
  first_name?: string          // 名
  last_name?: string           // 姓
  phone?: string               // 电话号码
  company?: string             // 公司
  department?: string          // 部门
  title?: string               // 职位
  bio?: string                 // 个人简介
}
```

### 角色和权限

```typescript
interface Role {
  id: string
  name: string
  description: string
  permissions: string[]
  is_system: boolean           // 是否为系统角色
  created_at: string
  updated_at: string
}

interface Permission {
  id: string
  name: string
  resource: string             // 资源类型
  action: string               // 操作类型
  description: string
  scope?: string               // 权限范围
}
```

## 获取用户列表

### 请求

```bash
GET /api/v1/users
```

### 查询参数

| 参数 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `page` | number | 页码 | 1 |
| `per_page` | number | 每页数量 | 20 |
| `sort` | string | 排序字段 | created_at |
| `order` | string | 排序方向 | desc |
| `search` | string | 搜索关键词 | - |
| `role` | string | 过滤角色 | - |
| `is_active` | boolean | 过滤激活状态 | - |
| `is_verified` | boolean | 过滤验证状态 | - |
| `created_after` | string | 创建时间起始 | - |
| `created_before` | string | 创建时间结束 | - |

### 请求示例

```bash
# 获取所有激活用户
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/users?is_active=true"

# 搜索用户
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/users?search=admin"

# 按角色过滤
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/users?role=admin"
```

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "id": "user_001",
      "username": "admin",
      "email": "admin@example.com",
      "display_name": "系统管理员",
      "avatar_url": "https://example.com/avatars/admin.jpg",
      "role": "admin",
      "permissions": ["admin:*"],
      "is_active": true,
      "is_verified": true,
      "last_login_at": "2024-01-01T12:00:00Z",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z",
      "preferences": {
        "language": "zh-CN",
        "timezone": "Asia/Shanghai",
        "theme": "auto",
        "notifications": {
          "email": true,
          "desktop": true,
          "mobile": false
        }
      }
    }
  ],
  "meta": {
    "total": 15,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

## 获取用户详情

### 请求

```bash
GET /api/v1/users/{id}
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `include_permissions` | boolean | 是否包含详细权限信息 |
| `include_activity` | boolean | 是否包含最近活动 |
| `include_stats` | boolean | 是否包含统计信息 |

### 请求示例

```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/users/user_001?include_permissions=true"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "user_001",
    "username": "admin",
    "email": "admin@example.com",
    "display_name": "系统管理员",
    "avatar_url": "https://example.com/avatars/admin.jpg",
    "role": "admin",
    "permissions": [
      {
        "id": "perm_001",
        "name": "admin:*",
        "resource": "admin",
        "action": "*",
        "description": "管理员权限"
      }
    ],
    "is_active": true,
    "is_verified": true,
    "last_login_at": "2024-01-01T12:00:00Z",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z",
    "profile": {
      "first_name": "管理",
      "last_name": "员",
      "company": "Switch Hosts R",
      "department": "技术部",
      "title": "系统管理员"
    },
    "stats": {
      "hosts_created": 25,
      "hosts_updated": 150,
      "login_count": 342,
      "last_activity": "2024-01-01T12:00:00Z"
    }
  }
}
```

## 创建用户

### 请求

```bash
POST /api/v1/users
```

### 请求体

```typescript
interface CreateUserRequest {
  username: string              // 必需：用户名
  email: string                 // 必需：邮箱
  password: string              // 必需：密码
  display_name?: string         // 可选：显示名称
  role?: string                 // 可选：角色（默认 user）
  permissions?: string[]        // 可选：额外权限
  is_active?: boolean          // 可选：是否激活（默认 true）
  send_welcome_email?: boolean  // 可选：是否发送欢迎邮件
  profile?: Partial<UserProfile> // 可选：用户资料
  preferences?: Partial<UserPreferences> // 可选：偏好设置
  metadata?: Record<string, any> // 可选：元数据
}
```

### 请求示例

```bash
curl -X POST "https://api.switchhosts.com/v1/users" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "developer",
    "email": "dev@example.com",
    "password": "SecurePassword123!",
    "display_name": "开发者",
    "role": "user",
    "profile": {
      "first_name": "开发",
      "last_name": "者",
      "company": "Example Corp",
      "department": "研发部"
    },
    "send_welcome_email": true
  }'
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "user_002",
    "username": "developer",
    "email": "dev@example.com",
    "display_name": "开发者",
    "role": "user",
    "permissions": ["hosts:read:own", "hosts:write:own"],
    "is_active": true,
    "is_verified": false,
    "created_at": "2024-01-02T00:00:00Z",
    "updated_at": "2024-01-02T00:00:00Z"
  },
  "message": "用户创建成功，欢迎邮件已发送"
}
```

## 更新用户

### 完整更新 (PUT)

```bash
PUT /api/v1/users/{id}
```

### 部分更新 (PATCH)

```bash
PATCH /api/v1/users/{id}
```

### 请求体

```typescript
interface UpdateUserRequest {
  username?: string
  email?: string
  display_name?: string
  role?: string
  permissions?: string[]
  is_active?: boolean
  profile?: Partial<UserProfile>
  preferences?: Partial<UserPreferences>
  metadata?: Record<string, any>
}
```

### 请求示例

```bash
# 更新用户角色和权限
curl -X PATCH "https://api.switchhosts.com/v1/users/user_002" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "moderator",
    "permissions": ["hosts:read", "hosts:write", "users:read"]
  }'

# 更新用户资料
curl -X PATCH "https://api.switchhosts.com/v1/users/user_002" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "profile": {
      "title": "高级开发工程师",
      "phone": "+86 138 0013 8000"
    }
  }'
```

## 删除用户

### 请求

```bash
DELETE /api/v1/users/{id}
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `transfer_to` | string | 将用户数据转移给指定用户 |
| `force` | boolean | 强制删除（忽略依赖检查） |

### 请求示例

```bash
# 删除用户并转移数据
curl -X DELETE "https://api.switchhosts.com/v1/users/user_002?transfer_to=user_001" \
  -H "Authorization: Bearer YOUR_API_KEY"

# 强制删除用户
curl -X DELETE "https://api.switchhosts.com/v1/users/user_002?force=true" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

## 激活/停用用户

### 激活用户

```bash
POST /api/v1/users/{id}/activate
```

### 停用用户

```bash
POST /api/v1/users/{id}/deactivate
```

### 请求示例

```bash
# 激活用户
curl -X POST "https://api.switchhosts.com/v1/users/user_002/activate" \
  -H "Authorization: Bearer YOUR_API_KEY"

# 停用用户
curl -X POST "https://api.switchhosts.com/v1/users/user_002/deactivate" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "违反使用条款",
    "notify_user": true
  }'
```

## 重置密码

### 请求

```bash
POST /api/v1/users/{id}/reset-password
```

### 请求体

```typescript
interface ResetPasswordRequest {
  new_password?: string         // 新密码（可选，不提供则生成随机密码）
  send_email?: boolean         // 是否发送邮件通知（默认 true）
  force_change?: boolean       // 是否强制下次登录时修改密码
}
```

### 请求示例

```bash
# 生成随机密码并发送邮件
curl -X POST "https://api.switchhosts.com/v1/users/user_002/reset-password" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "send_email": true,
    "force_change": true
  }'

# 设置指定密码
curl -X POST "https://api.switchhosts.com/v1/users/user_002/reset-password" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "new_password": "NewSecurePassword123!",
    "send_email": true
  }'
```

## 权限管理

### 获取用户权限

```bash
GET /api/v1/users/{id}/permissions
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "role": {
      "id": "role_user",
      "name": "user",
      "description": "普通用户",
      "permissions": ["hosts:read:own", "hosts:write:own"]
    },
    "additional_permissions": ["logs:read:own"],
    "effective_permissions": [
      "hosts:read:own",
      "hosts:write:own",
      "logs:read:own"
    ],
    "denied_permissions": [],
    "inherited_from": {
      "role": ["hosts:read:own", "hosts:write:own"],
      "direct": ["logs:read:own"]
    }
  }
}
```

### 更新用户权限

```bash
PUT /api/v1/users/{id}/permissions
```

### 请求体

```typescript
interface UpdatePermissionsRequest {
  role?: string                 // 更新角色
  additional_permissions?: string[] // 额外权限
  denied_permissions?: string[] // 拒绝的权限
}
```

### 请求示例

```bash
curl -X PUT "https://api.switchhosts.com/v1/users/user_002/permissions" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "role": "moderator",
    "additional_permissions": ["users:read", "logs:read"],
    "denied_permissions": ["users:delete"]
  }'
```

## 用户活动日志

### 请求

```bash
GET /api/v1/users/{id}/activity
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `page` | number | 页码 |
| `per_page` | number | 每页数量 |
| `action` | string | 过滤操作类型 |
| `resource` | string | 过滤资源类型 |
| `date_from` | string | 开始日期 |
| `date_to` | string | 结束日期 |

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "id": "activity_001",
      "action": "hosts.create",
      "resource": "hosts",
      "resource_id": "host_001",
      "details": {
        "name": "开发环境",
        "ip_address": "192.168.1.100"
      },
      "timestamp": "2024-01-01T12:00:00Z",
      "user_agent": "Mozilla/5.0...",
      "ip_address": "192.168.1.100"
    }
  ],
  "meta": {
    "total": 150,
    "page": 1,
    "per_page": 20
  }
}
```

## 批量操作

### 请求

```bash
POST /api/v1/users/batch
```

### 请求体

```typescript
interface BatchUserOperation {
  action: 'create' | 'update' | 'delete' | 'activate' | 'deactivate' | 'reset_password'
  users: Array<{
    id?: string        // 对于 update/delete 等操作必需
    data?: any         // 对于 create/update 操作的数据
  }>
  options?: {
    send_notifications?: boolean
    force?: boolean
  }
}
```

### 请求示例

```bash
# 批量创建用户
curl -X POST "https://api.switchhosts.com/v1/users/batch" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "action": "create",
    "users": [
      {
        "data": {
          "username": "user1",
          "email": "user1@example.com",
          "password": "Password123!",
          "role": "user"
        }
      },
      {
        "data": {
          "username": "user2",
          "email": "user2@example.com",
          "password": "Password123!",
          "role": "user"
        }
      }
    ],
    "options": {
      "send_notifications": true
    }
  }'

# 批量激活用户
curl -X POST "https://api.switchhosts.com/v1/users/batch" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "action": "activate",
    "users": [
      { "id": "user_003" },
      { "id": "user_004" }
    ]
  }'
```

## 当前用户操作

### 获取当前用户信息

```bash
GET /api/v1/users/me
```

### 更新当前用户信息

```bash
PUT /api/v1/users/me
```

### 修改当前用户密码

```bash
POST /api/v1/users/me/change-password
```

### 请求示例

```bash
# 获取当前用户信息
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     "https://api.switchhosts.com/v1/users/me"

# 更新个人资料
curl -X PUT "https://api.switchhosts.com/v1/users/me" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "新的显示名称",
    "profile": {
      "phone": "+86 138 0013 8000",
      "bio": "更新的个人简介"
    },
    "preferences": {
      "theme": "dark",
      "language": "en-US"
    }
  }'

# 修改密码
curl -X POST "https://api.switchhosts.com/v1/users/me/change-password" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "current_password": "OldPassword123!",
    "new_password": "NewPassword123!"
  }'
```

## 角色管理

### 获取所有角色

```bash
GET /api/v1/roles
```

### 创建角色

```bash
POST /api/v1/roles
```

### 请求示例

```bash
# 获取角色列表
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/roles"

# 创建自定义角色
curl -X POST "https://api.switchhosts.com/v1/roles" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "team_lead",
    "description": "团队负责人",
    "permissions": [
      "hosts:read",
      "hosts:write",
      "users:read",
      "logs:read"
    ]
  }'
```

## 错误处理

### 常见错误

#### 用户不存在

```json
{
  "success": false,
  "error": {
    "code": "USER_NOT_FOUND",
    "message": "用户不存在",
    "details": "ID 为 'user_999' 的用户不存在"
  }
}
```

#### 用户名已存在

```json
{
  "success": false,
  "error": {
    "code": "USERNAME_EXISTS",
    "message": "用户名已存在",
    "details": "用户名 'admin' 已被使用"
  }
}
```

#### 邮箱已存在

```json
{
  "success": false,
  "error": {
    "code": "EMAIL_EXISTS",
    "message": "邮箱地址已存在",
    "details": "邮箱 'admin@example.com' 已被注册"
  }
}
```

#### 权限不足

```json
{
  "success": false,
  "error": {
    "code": "INSUFFICIENT_PERMISSIONS",
    "message": "权限不足",
    "details": "需要 'users:write' 权限才能执行此操作"
  }
}
```

## 使用示例

### JavaScript/Node.js

```javascript
const { SwitchHostsAPI } = require('@switchhosts/sdk')

const api = new SwitchHostsAPI({
  apiKey: 'sk_live_1234567890abcdef'
})

// 获取用户列表
const users = await api.users.list({
  is_active: true,
  role: 'user'
})

// 创建用户
const newUser = await api.users.create({
  username: 'newuser',
  email: 'newuser@example.com',
  password: 'SecurePassword123!',
  role: 'user',
  send_welcome_email: true
})

// 更新用户权限
await api.users.updatePermissions(newUser.id, {
  additional_permissions: ['logs:read']
})

// 获取用户活动
const activity = await api.users.getActivity(newUser.id, {
  page: 1,
  per_page: 50
})
```

### Python

```python
from switchhosts import SwitchHostsAPI

api = SwitchHostsAPI(api_key='sk_live_1234567890abcdef')

# 获取用户列表
users = api.users.list(is_active=True, role='user')

# 创建用户
new_user = api.users.create(
    username='newuser',
    email='newuser@example.com',
    password='SecurePassword123!',
    role='user',
    send_welcome_email=True
)

# 批量操作
result = api.users.batch(
    action='activate',
    users=[{'id': 'user_001'}, {'id': 'user_002'}]
)
```

### Go

```go
package main

import (
    "github.com/switchhosts/go-sdk"
)

func main() {
    client := switchhosts.NewClient("sk_live_1234567890abcdef")
    
    // 获取用户列表
    users, err := client.Users.List(&switchhosts.UsersListOptions{
        IsActive: true,
        Role:     "user",
    })
    
    // 创建用户
    newUser, err := client.Users.Create(&switchhosts.CreateUserRequest{
        Username:         "newuser",
        Email:           "newuser@example.com",
        Password:        "SecurePassword123!",
        Role:            "user",
        SendWelcomeEmail: true,
    })
    
    // 重置密码
    err = client.Users.ResetPassword(newUser.ID, &switchhosts.ResetPasswordRequest{
        SendEmail:   true,
        ForceChange: true,
    })
}
```

通过这些用户管理 API，您可以完整地管理 Switch Hosts R 的用户系统，实现用户账户管理、权限控制和团队协作功能。