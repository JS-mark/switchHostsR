# 认证授权

Switch Hosts R API 使用多种认证方式来确保 API 的安全访问。本文档详细介绍了各种认证方法的使用方式和最佳实践。

## 认证方式概览

### 支持的认证方式

1. **API Key 认证**：适用于服务端应用和自动化脚本
2. **JWT Token 认证**：适用于 Web 应用和移动应用
3. **OAuth 2.0**：适用于第三方应用集成
4. **Basic 认证**：适用于简单的测试场景

### 认证流程图

```
sequenceDiagram
    participant Client
    participant API
    participant Auth
    
    Client->>Auth: 请求认证
    Auth->>Auth: 验证凭据
    Auth->>Client: 返回 Token
    Client->>API: 携带 Token 请求
    API->>Auth: 验证 Token
    Auth->>API: 返回验证结果
    API->>Client: 返回响应
```

## API Key 认证

### 生成 API Key

#### 通过 Web 界面

1. 登录 Switch Hosts R
2. 进入「设置」→「API 密钥」
3. 点击「生成新密钥」
4. 设置密钥名称和权限
5. 保存并复制密钥

#### 通过 CLI

```bash
# 生成新的 API Key
switchhosts-cli auth generate-key \
  --name "Production API" \
  --permissions "hosts:read,hosts:write" \
  --expires-in "30d"

# 列出所有 API Key
switchhosts-cli auth list-keys

# 撤销 API Key
switchhosts-cli auth revoke-key --key-id "key_123456"
```

### 使用 API Key

#### HTTP Header 方式（推荐）

```bash
curl -H "Authorization: Bearer sk_live_1234567890abcdef" \
     https://api.switchhosts.com/v1/hosts
```

#### Query Parameter 方式

```bash
curl "https://api.switchhosts.com/v1/hosts?api_key=sk_live_1234567890abcdef"
```

#### 代码示例

```javascript
// JavaScript/Node.js
const response = await fetch('https://api.switchhosts.com/v1/hosts', {
  headers: {
    'Authorization': 'Bearer sk_live_1234567890abcdef',
    'Content-Type': 'application/json'
  }
})
```

```python
# Python
import requests

headers = {
    'Authorization': 'Bearer sk_live_1234567890abcdef',
    'Content-Type': 'application/json'
}

response = requests.get('https://api.switchhosts.com/v1/hosts', headers=headers)
```

```go
// Go
req, _ := http.NewRequest("GET", "https://api.switchhosts.com/v1/hosts", nil)
req.Header.Set("Authorization", "Bearer sk_live_1234567890abcdef")
req.Header.Set("Content-Type", "application/json")

client := &http.Client{}
resp, err := client.Do(req)
```

### API Key 管理

#### 密钥类型

```typescript
interface ApiKey {
  id: string
  name: string
  key: string // 只在创建时返回
  prefix: string // sk_live_, sk_test_
  permissions: string[]
  created_at: string
  expires_at?: string
  last_used_at?: string
  is_active: boolean
}
```

#### 权限范围

```json
{
  "permissions": [
    "hosts:read",      // 读取 Hosts 配置
    "hosts:write",     // 创建和更新 Hosts 配置
    "hosts:delete",    // 删除 Hosts 配置
    "users:read",      // 读取用户信息
    "users:write",     // 管理用户
    "logs:read",       // 查看日志
    "admin:*"          // 管理员权限
  ]
}
```

## JWT Token 认证

### 获取 JWT Token

#### 用户名密码登录

```bash
curl -X POST https://api.switchhosts.com/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password123"
  }'
```

#### 响应示例

```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "id": "user_123",
      "username": "admin",
      "email": "admin@example.com",
      "role": "admin"
    }
  }
}
```

### 使用 JWT Token

```bash
curl -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
     https://api.switchhosts.com/v1/hosts
```

### Token 刷新

```bash
curl -X POST https://api.switchhosts.com/v1/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }'
```

### JWT 实现示例

```javascript
class AuthManager {
  constructor() {
    this.accessToken = localStorage.getItem('access_token')
    this.refreshToken = localStorage.getItem('refresh_token')
  }

  async login(username, password) {
    const response = await fetch('/api/v1/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    })

    const data = await response.json()
    if (data.success) {
      this.accessToken = data.data.access_token
      this.refreshToken = data.data.refresh_token
      localStorage.setItem('access_token', this.accessToken)
      localStorage.setItem('refresh_token', this.refreshToken)
    }
    return data
  }

  async refreshAccessToken() {
    const response = await fetch('/api/v1/auth/refresh', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refresh_token: this.refreshToken })
    })

    const data = await response.json()
    if (data.success) {
      this.accessToken = data.data.access_token
      localStorage.setItem('access_token', this.accessToken)
    }
    return data
  }

  async apiRequest(url, options = {}) {
    const headers = {
      'Authorization': `Bearer ${this.accessToken}`,
      'Content-Type': 'application/json',
      ...options.headers
    }

    let response = await fetch(url, { ...options, headers })

    // Token 过期，尝试刷新
    if (response.status === 401) {
      await this.refreshAccessToken()
      headers['Authorization'] = `Bearer ${this.accessToken}`
      response = await fetch(url, { ...options, headers })
    }

    return response
  }
}
```

## OAuth 2.0 认证

### 授权码流程

#### 1. 重定向到授权页面

```bash
https://api.switchhosts.com/v1/oauth/authorize?
  response_type=code&
  client_id=your_client_id&
  redirect_uri=https://yourapp.com/callback&
  scope=hosts:read hosts:write&
  state=random_state_string
```

#### 2. 获取授权码

用户授权后，会重定向到您的回调 URL：

```bash
https://yourapp.com/callback?
  code=authorization_code&
  state=random_state_string
```

#### 3. 交换访问令牌

```bash
curl -X POST https://api.switchhosts.com/v1/oauth/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=authorization_code" \
  -d "code=authorization_code" \
  -d "client_id=your_client_id" \
  -d "client_secret=your_client_secret" \
  -d "redirect_uri=https://yourapp.com/callback"
```

### 客户端凭据流程

适用于服务端应用：

```bash
curl -X POST https://api.switchhosts.com/v1/oauth/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=client_credentials" \
  -d "client_id=your_client_id" \
  -d "client_secret=your_client_secret" \
  -d "scope=hosts:read hosts:write"
```

### OAuth 应用管理

#### 创建 OAuth 应用

```json
POST /api/v1/oauth/applications
{
  "name": "My Application",
  "description": "Application description",
  "redirect_uris": [
    "https://myapp.com/callback",
    "https://myapp.com/auth/callback"
  ],
  "scopes": ["hosts:read", "hosts:write"]
}
```

#### 应用信息

```typescript
interface OAuthApplication {
  id: string
  name: string
  description: string
  client_id: string
  client_secret: string // 只在创建时返回
  redirect_uris: string[]
  scopes: string[]
  created_at: string
  is_active: boolean
}
```

## 权限系统

### 权限模型

```typescript
interface Permission {
  resource: string // hosts, users, logs, admin
  action: string   // read, write, delete, *
  scope?: string   // 可选的范围限制
}

// 权限示例
const permissions = [
  'hosts:read',           // 读取所有 Hosts
  'hosts:write:own',      // 只能修改自己的 Hosts
  'users:read',           // 读取用户信息
  'logs:read:own',        // 只能查看自己的日志
  'admin:*'               // 管理员权限
]
```

### 角色定义

```json
{
  "roles": {
    "admin": {
      "name": "管理员",
      "permissions": ["admin:*"]
    },
    "user": {
      "name": "普通用户",
      "permissions": [
        "hosts:read:own",
        "hosts:write:own",
        "logs:read:own"
      ]
    },
    "readonly": {
      "name": "只读用户",
      "permissions": [
        "hosts:read",
        "logs:read"
      ]
    }
  }
}
```

### 权限检查

```javascript
// 中间件示例
const checkPermission = (requiredPermission) => {
  return (req, res, next) => {
    const userPermissions = req.user.permissions
    
    if (hasPermission(userPermissions, requiredPermission)) {
      next()
    } else {
      res.status(403).json({
        success: false,
        error: {
          code: 'PERMISSION_DENIED',
          message: '权限不足'
        }
      })
    }
  }
}

// 使用示例
app.get('/api/v1/hosts', 
  authenticate,
  checkPermission('hosts:read'),
  getHosts
)
```

## 安全最佳实践

### API Key 安全

1. **环境变量存储**：
```bash
# .env 文件
SWITCHHOSTS_API_KEY=sk_live_1234567890abcdef
```

2. **最小权限原则**：
```javascript
// 只授予必要的权限
const apiKey = await createApiKey({
  name: 'CI/CD Pipeline',
  permissions: ['hosts:read', 'hosts:write'],
  expires_in: '30d'
})
```

3. **定期轮换**：
```javascript
// 定期更新 API Key
const rotateApiKey = async (oldKeyId) => {
  const newKey = await createApiKey({
    name: 'Rotated Key',
    permissions: oldKey.permissions
  })
  
  // 更新应用配置
  await updateAppConfig({ apiKey: newKey.key })
  
  // 撤销旧密钥
  await revokeApiKey(oldKeyId)
}
```

### JWT 安全

1. **短期有效期**：
```javascript
const tokenConfig = {
  accessTokenExpiry: '15m',  // 访问令牌 15 分钟
  refreshTokenExpiry: '7d'   // 刷新令牌 7 天
}
```

2. **安全存储**：
```javascript
// 使用 HttpOnly Cookie 存储刷新令牌
res.cookie('refresh_token', refreshToken, {
  httpOnly: true,
  secure: true,
  sameSite: 'strict',
  maxAge: 7 * 24 * 60 * 60 * 1000 // 7 天
})
```

3. **Token 撤销**：
```javascript
// 实现 Token 黑名单
const revokeToken = async (tokenId) => {
  await redis.set(`revoked_token:${tokenId}`, '1', 'EX', tokenExpiry)
}
```

### 网络安全

1. **HTTPS 强制**：
```javascript
// 强制使用 HTTPS
app.use((req, res, next) => {
  if (!req.secure && process.env.NODE_ENV === 'production') {
    return res.redirect(`https://${req.headers.host}${req.url}`)
  }
  next()
})
```

2. **速率限制**：
```javascript
const rateLimit = require('express-rate-limit')

const apiLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 分钟
  max: 100, // 最多 100 个请求
  message: '请求过于频繁，请稍后再试'
})

app.use('/api/', apiLimiter)
```

3. **请求验证**：
```javascript
// 验证请求来源
const validateOrigin = (req, res, next) => {
  const allowedOrigins = process.env.ALLOWED_ORIGINS?.split(',')
  const origin = req.headers.origin
  
  if (allowedOrigins && !allowedOrigins.includes(origin)) {
    return res.status(403).json({ error: 'Origin not allowed' })
  }
  
  next()
}
```

## 故障排查

### 常见认证错误

#### 401 Unauthorized

```json
{
  "success": false,
  "error": {
    "code": "AUTHENTICATION_FAILED",
    "message": "认证失败",
    "details": "Invalid or expired token"
  }
}
```

**解决方案**：
- 检查 API Key 或 Token 是否正确
- 确认 Token 是否已过期
- 验证请求头格式是否正确

#### 403 Forbidden

```json
{
  "success": false,
  "error": {
    "code": "PERMISSION_DENIED",
    "message": "权限不足",
    "details": "Missing required permission: hosts:write"
  }
}
```

**解决方案**：
- 检查用户或 API Key 的权限设置
- 确认请求的资源是否在权限范围内
- 联系管理员分配必要权限

### 调试工具

#### Token 解析工具

```javascript
// JWT Token 解析
const jwt = require('jsonwebtoken')

const decodeToken = (token) => {
  try {
    const decoded = jwt.decode(token, { complete: true })
    console.log('Token Header:', decoded.header)
    console.log('Token Payload:', decoded.payload)
    return decoded
  } catch (error) {
    console.error('Token decode error:', error.message)
  }
}
```

#### API 测试脚本

```bash
#!/bin/bash
# test-auth.sh

API_KEY="sk_live_1234567890abcdef"
BASE_URL="https://api.switchhosts.com/v1"

# 测试 API Key 认证
echo "Testing API Key authentication..."
curl -s -H "Authorization: Bearer $API_KEY" \
     "$BASE_URL/auth/verify" | jq

# 测试权限
echo "Testing permissions..."
curl -s -H "Authorization: Bearer $API_KEY" \
     "$BASE_URL/hosts" | jq
```

通过合理的认证授权机制，Switch Hosts R API 确保了数据的安全性和访问的可控性，为不同场景的应用集成提供了灵活的解决方案。