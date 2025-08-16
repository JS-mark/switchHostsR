# Logs API

Logs API 提供了完整的日志查询和管理功能，支持系统日志、操作日志、网络日志等多种类型的日志查询。通过这些 API，您可以监控系统运行状态、追踪用户操作、分析网络请求等。

## API 端点概览

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/logs` | 获取日志列表 |
| GET | `/logs/{id}` | 获取日志详情 |
| POST | `/logs/search` | 高级日志搜索 |
| GET | `/logs/stats` | 获取日志统计信息 |
| POST | `/logs/export` | 导出日志 |
| DELETE | `/logs` | 清理日志 |
| GET | `/logs/types` | 获取日志类型列表 |
| GET | `/logs/levels` | 获取日志级别列表 |
| POST | `/logs/alerts` | 创建日志告警 |
| GET | `/logs/alerts` | 获取告警规则 |
| PUT | `/logs/alerts/{id}` | 更新告警规则 |
| DELETE | `/logs/alerts/{id}` | 删除告警规则 |

## 数据模型

### 日志对象

```typescript
interface LogEntry {
  id: string                    // 日志唯一标识
  timestamp: string             // 时间戳
  level: LogLevel              // 日志级别
  type: LogType                // 日志类型
  category: string             // 日志分类
  message: string              // 日志消息
  details?: Record<string, any> // 详细信息
  user_id?: string             // 用户 ID（操作日志）
  session_id?: string          // 会话 ID
  ip_address?: string          // IP 地址
  user_agent?: string          // 用户代理
  request_id?: string          // 请求 ID
  duration?: number            // 执行时长（毫秒）
  status?: string              // 状态
  error?: LogError             // 错误信息
  metadata: Record<string, any> // 元数据
  tags: string[]               // 标签
  source: string               // 日志来源
  correlation_id?: string      // 关联 ID
}

type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'fatal'

type LogType = 'system' | 'operation' | 'network' | 'security' | 'performance' | 'audit'

interface LogError {
  code: string                 // 错误代码
  message: string              // 错误消息
  stack?: string               // 错误堆栈
  context?: Record<string, any> // 错误上下文
}
```

### 日志统计

```typescript
interface LogStats {
  total_count: number          // 总日志数
  by_level: Record<LogLevel, number> // 按级别统计
  by_type: Record<LogType, number>   // 按类型统计
  by_hour: Array<{             // 按小时统计
    hour: string
    count: number
  }>
  by_day: Array<{              // 按天统计
    date: string
    count: number
  }>
  error_rate: number           // 错误率
  avg_response_time: number    // 平均响应时间
  top_errors: Array<{          // 高频错误
    error: string
    count: number
  }>
  top_users: Array<{           // 活跃用户
    user_id: string
    username: string
    count: number
  }>
}
```

### 告警规则

```typescript
interface AlertRule {
  id: string
  name: string
  description: string
  is_active: boolean
  conditions: AlertCondition[]
  actions: AlertAction[]
  created_at: string
  updated_at: string
  last_triggered?: string
  trigger_count: number
}

interface AlertCondition {
  field: string                // 字段名
  operator: 'eq' | 'ne' | 'gt' | 'lt' | 'gte' | 'lte' | 'contains' | 'regex'
  value: any                   // 比较值
  time_window?: number         // 时间窗口（秒）
  threshold?: number           // 阈值
}

interface AlertAction {
  type: 'email' | 'webhook' | 'slack' | 'sms'
  config: Record<string, any>  // 配置参数
}
```

## 获取日志列表

### 请求

```bash
GET /api/v1/logs
```

### 查询参数

| 参数 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `page` | number | 页码 | 1 |
| `per_page` | number | 每页数量 | 50 |
| `sort` | string | 排序字段 | timestamp |
| `order` | string | 排序方向 | desc |
| `level` | string | 日志级别过滤 | - |
| `type` | string | 日志类型过滤 | - |
| `category` | string | 日志分类过滤 | - |
| `user_id` | string | 用户 ID 过滤 | - |
| `search` | string | 搜索关键词 | - |
| `start_time` | string | 开始时间 | - |
| `end_time` | string | 结束时间 | - |
| `tags` | string | 标签过滤（逗号分隔） | - |
| `source` | string | 日志来源过滤 | - |
| `correlation_id` | string | 关联 ID 过滤 | - |
| `include_details` | boolean | 是否包含详细信息 | false |

### 请求示例

```bash
# 获取错误日志
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs?level=error&type=system"

# 搜索特定用户的操作日志
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs?type=operation&user_id=user_001"

# 按时间范围查询
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs?start_time=2024-01-01T00:00:00Z&end_time=2024-01-02T00:00:00Z"

# 搜索包含关键词的日志
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs?search=hosts+create"
```

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "id": "log_001",
      "timestamp": "2024-01-01T12:00:00Z",
      "level": "info",
      "type": "operation",
      "category": "hosts",
      "message": "用户创建了新的 Hosts 配置",
      "user_id": "user_001",
      "session_id": "sess_abc123",
      "ip_address": "192.168.1.100",
      "request_id": "req_xyz789",
      "duration": 150,
      "status": "success",
      "metadata": {
        "hosts_id": "host_001",
        "hosts_name": "开发环境",
        "action": "create"
      },
      "tags": ["hosts", "create", "user-action"],
      "source": "web-app"
    },
    {
      "id": "log_002",
      "timestamp": "2024-01-01T11:58:30Z",
      "level": "error",
      "type": "system",
      "category": "database",
      "message": "数据库连接超时",
      "duration": 5000,
      "status": "failed",
      "error": {
        "code": "DB_TIMEOUT",
        "message": "Connection timeout after 5000ms",
        "context": {
          "host": "db.example.com",
          "port": 5432,
          "database": "switchhosts"
        }
      },
      "tags": ["database", "timeout", "error"],
      "source": "api-server"
    }
  ],
  "meta": {
    "total": 1250,
    "page": 1,
    "per_page": 50,
    "total_pages": 25
  }
}
```

## 获取日志详情

### 请求

```bash
GET /api/v1/logs/{id}
```

### 请求示例

```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs/log_001"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "id": "log_001",
    "timestamp": "2024-01-01T12:00:00Z",
    "level": "info",
    "type": "operation",
    "category": "hosts",
    "message": "用户创建了新的 Hosts 配置",
    "details": {
      "request": {
        "method": "POST",
        "url": "/api/v1/hosts",
        "headers": {
          "content-type": "application/json",
          "user-agent": "SwitchHostsR/1.0.0"
        },
        "body": {
          "name": "开发环境",
          "content": "127.0.0.1 dev.example.com",
          "is_active": true
        }
      },
      "response": {
        "status": 201,
        "body": {
          "id": "host_001",
          "name": "开发环境",
          "created_at": "2024-01-01T12:00:00Z"
        }
      },
      "user": {
        "id": "user_001",
        "username": "developer",
        "role": "user"
      }
    },
    "user_id": "user_001",
    "session_id": "sess_abc123",
    "ip_address": "192.168.1.100",
    "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
    "request_id": "req_xyz789",
    "duration": 150,
    "status": "success",
    "metadata": {
      "hosts_id": "host_001",
      "hosts_name": "开发环境",
      "action": "create",
      "client_version": "1.0.0",
      "platform": "macos"
    },
    "tags": ["hosts", "create", "user-action"],
    "source": "web-app",
    "correlation_id": "corr_123456"
  }
}
```

## 高级日志搜索

### 请求

```bash
POST /api/v1/logs/search
```

### 请求体

```typescript
interface LogSearchRequest {
  query?: string               // 搜索查询
  filters?: {
    level?: LogLevel[]
    type?: LogType[]
    category?: string[]
    user_id?: string[]
    tags?: string[]
    source?: string[]
    status?: string[]
  }
  time_range?: {
    start: string
    end: string
  }
  aggregations?: {
    group_by?: string[]          // 分组字段
    metrics?: string[]           // 聚合指标
    interval?: string            // 时间间隔
  }
  sort?: Array<{
    field: string
    order: 'asc' | 'desc'
  }>
  page?: number
  per_page?: number
  include_details?: boolean
}
```

### 请求示例

```bash
# 复杂搜索查询
curl -X POST "https://api.switchhosts.com/v1/logs/search" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "error OR timeout",
    "filters": {
      "level": ["error", "warn"],
      "type": ["system", "network"],
      "tags": ["database", "api"]
    },
    "time_range": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-02T00:00:00Z"
    },
    "aggregations": {
      "group_by": ["level", "category"],
      "metrics": ["count", "avg_duration"],
      "interval": "1h"
    },
    "sort": [
      { "field": "timestamp", "order": "desc" }
    ],
    "page": 1,
    "per_page": 100
  }'

# 用户行为分析
curl -X POST "https://api.switchhosts.com/v1/logs/search" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "filters": {
      "type": ["operation"],
      "user_id": ["user_001"]
    },
    "time_range": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-07T23:59:59Z"
    },
    "aggregations": {
      "group_by": ["category", "metadata.action"],
      "metrics": ["count"]
    }
  }'
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "logs": [
      {
        "id": "log_003",
        "timestamp": "2024-01-01T15:30:00Z",
        "level": "error",
        "type": "system",
        "category": "database",
        "message": "查询执行超时",
        "duration": 30000,
        "tags": ["database", "timeout"]
      }
    ],
    "aggregations": {
      "by_level": {
        "error": 15,
        "warn": 8
      },
      "by_category": {
        "database": 12,
        "network": 7,
        "auth": 4
      },
      "timeline": [
        {
          "timestamp": "2024-01-01T14:00:00Z",
          "count": 5,
          "avg_duration": 1250
        },
        {
          "timestamp": "2024-01-01T15:00:00Z",
          "count": 8,
          "avg_duration": 2100
        }
      ]
    }
  },
  "meta": {
    "total": 23,
    "page": 1,
    "per_page": 100,
    "execution_time": 45
  }
}
```

## 获取日志统计

### 请求

```bash
GET /api/v1/logs/stats
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `start_time` | string | 开始时间 |
| `end_time` | string | 结束时间 |
| `granularity` | string | 时间粒度（hour/day/week） |
| `include_details` | boolean | 是否包含详细统计 |

### 请求示例

```bash
# 获取今日统计
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs/stats?start_time=2024-01-01T00:00:00Z&end_time=2024-01-01T23:59:59Z&granularity=hour"

# 获取详细统计
curl -H "Authorization: Bearer YOUR_API_KEY" \
     "https://api.switchhosts.com/v1/logs/stats?include_details=true"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "total_count": 15420,
    "by_level": {
      "trace": 1250,
      "debug": 3200,
      "info": 8950,
      "warn": 1800,
      "error": 200,
      "fatal": 20
    },
    "by_type": {
      "system": 5200,
      "operation": 7800,
      "network": 1900,
      "security": 320,
      "performance": 150,
      "audit": 50
    },
    "by_hour": [
      {
        "hour": "2024-01-01T00:00:00Z",
        "count": 450
      },
      {
        "hour": "2024-01-01T01:00:00Z",
        "count": 380
      }
    ],
    "error_rate": 1.43,
    "avg_response_time": 245,
    "top_errors": [
      {
        "error": "DB_CONNECTION_TIMEOUT",
        "count": 45
      },
      {
        "error": "INVALID_AUTH_TOKEN",
        "count": 32
      }
    ],
    "top_users": [
      {
        "user_id": "user_001",
        "username": "admin",
        "count": 1250
      },
      {
        "user_id": "user_002",
        "username": "developer",
        "count": 890
      }
    ],
    "performance_metrics": {
      "p50_response_time": 120,
      "p95_response_time": 850,
      "p99_response_time": 2100,
      "slowest_operations": [
        {
          "operation": "hosts.export",
          "avg_duration": 3200,
          "count": 15
        }
      ]
    }
  }
}
```

## 导出日志

### 请求

```bash
POST /api/v1/logs/export
```

### 请求体

```typescript
interface LogExportRequest {
  format: 'json' | 'csv' | 'txt'  // 导出格式
  filters?: {
    level?: LogLevel[]
    type?: LogType[]
    start_time?: string
    end_time?: string
    user_id?: string
    tags?: string[]
  }
  fields?: string[]              // 导出字段
  max_records?: number           // 最大记录数
  compression?: 'gzip' | 'zip'   // 压缩格式
  delivery?: {
    method: 'download' | 'email' | 's3'
    config?: Record<string, any>
  }
}
```

### 请求示例

```bash
# 导出 CSV 格式日志
curl -X POST "https://api.switchhosts.com/v1/logs/export" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "csv",
    "filters": {
      "level": ["error", "warn"],
      "start_time": "2024-01-01T00:00:00Z",
      "end_time": "2024-01-07T23:59:59Z"
    },
    "fields": ["timestamp", "level", "message", "user_id", "ip_address"],
    "max_records": 10000,
    "compression": "gzip",
    "delivery": {
      "method": "download"
    }
  }'

# 通过邮件发送导出文件
curl -X POST "https://api.switchhosts.com/v1/logs/export" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "json",
    "filters": {
      "type": ["audit"]
    },
    "delivery": {
      "method": "email",
      "config": {
        "to": "admin@example.com",
        "subject": "审计日志导出"
      }
    }
  }'
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "export_id": "export_001",
    "status": "processing",
    "estimated_completion": "2024-01-01T12:05:00Z",
    "download_url": null,
    "file_size": null,
    "record_count": null,
    "created_at": "2024-01-01T12:00:00Z"
  },
  "message": "导出任务已创建，处理完成后将通知您"
}
```

## 清理日志

### 请求

```bash
DELETE /api/v1/logs
```

### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `before` | string | 删除指定时间之前的日志 |
| `level` | string | 删除指定级别的日志 |
| `type` | string | 删除指定类型的日志 |
| `dry_run` | boolean | 预览删除（不实际删除） |

### 请求示例

```bash
# 删除 30 天前的日志
curl -X DELETE "https://api.switchhosts.com/v1/logs?before=2023-12-01T00:00:00Z" \
  -H "Authorization: Bearer YOUR_API_KEY"

# 预览删除操作
curl -X DELETE "https://api.switchhosts.com/v1/logs?before=2023-12-01T00:00:00Z&dry_run=true" \
  -H "Authorization: Bearer YOUR_API_KEY"

# 删除调试日志
curl -X DELETE "https://api.switchhosts.com/v1/logs?level=debug&before=2024-01-01T00:00:00Z" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### 响应示例

```json
{
  "success": true,
  "data": {
    "deleted_count": 15420,
    "freed_space": "2.3 GB",
    "execution_time": "00:02:15",
    "breakdown": {
      "by_level": {
        "debug": 8500,
        "info": 6200,
        "warn": 650,
        "error": 70
      },
      "by_type": {
        "system": 7200,
        "operation": 6800,
        "network": 1420
      }
    }
  },
  "message": "成功删除 15,420 条日志记录"
}
```

## 告警管理

### 创建告警规则

```bash
POST /api/v1/logs/alerts
```

### 请求体

```typescript
interface CreateAlertRequest {
  name: string
  description?: string
  conditions: AlertCondition[]
  actions: AlertAction[]
  is_active?: boolean
  cooldown?: number            // 冷却时间（秒）
}
```

### 请求示例

```bash
# 创建错误率告警
curl -X POST "https://api.switchhosts.com/v1/logs/alerts" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "高错误率告警",
    "description": "当错误率超过 5% 时触发告警",
    "conditions": [
      {
        "field": "level",
        "operator": "eq",
        "value": "error",
        "time_window": 300,
        "threshold": 10
      }
    ],
    "actions": [
      {
        "type": "email",
        "config": {
          "to": ["admin@example.com", "ops@example.com"],
          "subject": "Switch Hosts R 错误率告警"
        }
      },
      {
        "type": "webhook",
        "config": {
          "url": "https://hooks.slack.com/services/...",
          "method": "POST",
          "headers": {
            "Content-Type": "application/json"
          }
        }
      }
    ],
    "cooldown": 1800
  }'

# 创建性能告警
curl -X POST "https://api.switchhosts.com/v1/logs/alerts" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "响应时间告警",
    "description": "当平均响应时间超过 2 秒时告警",
    "conditions": [
      {
        "field": "duration",
        "operator": "gt",
        "value": 2000,
        "time_window": 600
      }
    ],
    "actions": [
      {
        "type": "slack",
        "config": {
          "webhook_url": "https://hooks.slack.com/services/...",
          "channel": "#alerts",
          "username": "SwitchHostsR Bot"
        }
      }
    ]
  }'
```

### 获取告警规则列表

```bash
GET /api/v1/logs/alerts
```

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "id": "alert_001",
      "name": "高错误率告警",
      "description": "当错误率超过 5% 时触发告警",
      "is_active": true,
      "conditions": [
        {
          "field": "level",
          "operator": "eq",
          "value": "error",
          "time_window": 300,
          "threshold": 10
        }
      ],
      "actions": [
        {
          "type": "email",
          "config": {
            "to": ["admin@example.com"]
          }
        }
      ],
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z",
      "last_triggered": "2024-01-01T10:30:00Z",
      "trigger_count": 5
    }
  ]
}
```

## 错误处理

### 常见错误

#### 日志不存在

```json
{
  "success": false,
  "error": {
    "code": "LOG_NOT_FOUND",
    "message": "日志不存在",
    "details": "ID 为 'log_999' 的日志不存在"
  }
}
```

#### 查询超时

```json
{
  "success": false,
  "error": {
    "code": "QUERY_TIMEOUT",
    "message": "查询超时",
    "details": "查询执行时间超过 30 秒限制"
  }
}
```

#### 导出任务失败

```json
{
  "success": false,
  "error": {
    "code": "EXPORT_FAILED",
    "message": "导出任务失败",
    "details": "数据量过大，请缩小查询范围"
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

// 获取错误日志
const errorLogs = await api.logs.list({
  level: 'error',
  start_time: '2024-01-01T00:00:00Z',
  end_time: '2024-01-02T00:00:00Z'
})

// 高级搜索
const searchResults = await api.logs.search({
  query: 'timeout OR connection',
  filters: {
    type: ['system', 'network'],
    level: ['error', 'warn']
  },
  aggregations: {
    group_by: ['category'],
    metrics: ['count', 'avg_duration']
  }
})

// 创建告警规则
const alert = await api.logs.createAlert({
  name: '数据库连接告警',
  conditions: [{
    field: 'category',
    operator: 'eq',
    value: 'database',
    time_window: 300,
    threshold: 5
  }],
  actions: [{
    type: 'email',
    config: {
      to: ['dba@example.com'],
      subject: '数据库连接异常'
    }
  }]
})

// 导出日志
const exportTask = await api.logs.export({
  format: 'csv',
  filters: {
    type: ['audit'],
    start_time: '2024-01-01T00:00:00Z'
  },
  delivery: {
    method: 'email',
    config: {
      to: 'audit@example.com'
    }
  }
})
```

### Python

```python
from switchhosts import SwitchHostsAPI
from datetime import datetime, timedelta

api = SwitchHostsAPI(api_key='sk_live_1234567890abcdef')

# 获取最近 24 小时的统计
stats = api.logs.get_stats(
    start_time=(datetime.now() - timedelta(days=1)).isoformat(),
    end_time=datetime.now().isoformat(),
    granularity='hour'
)

# 搜索用户操作日志
user_logs = api.logs.search(
    filters={
        'type': ['operation'],
        'user_id': ['user_001']
    },
    aggregations={
        'group_by': ['metadata.action'],
        'metrics': ['count']
    }
)

# 清理旧日志
cleanup_result = api.logs.cleanup(
    before=(datetime.now() - timedelta(days=90)).isoformat(),
    dry_run=False
)
```

### Go

```go
package main

import (
    "time"
    "github.com/switchhosts/go-sdk"
)

func main() {
    client := switchhosts.NewClient("sk_live_1234567890abcdef")
    
    // 获取日志列表
    logs, err := client.Logs.List(&switchhosts.LogsListOptions{
        Level:     "error",
        Type:      "system",
        StartTime: time.Now().Add(-24 * time.Hour),
        EndTime:   time.Now(),
    })
    
    // 创建告警规则
    alert, err := client.Logs.CreateAlert(&switchhosts.CreateAlertRequest{
        Name: "系统错误告警",
        Conditions: []switchhosts.AlertCondition{
            {
                Field:      "level",
                Operator:   "eq",
                Value:      "error",
                TimeWindow: 300,
                Threshold:  5,
            },
        },
        Actions: []switchhosts.AlertAction{
            {
                Type: "webhook",
                Config: map[string]interface{}{
                    "url": "https://hooks.slack.com/services/...",
                },
            },
        },
    })
}
```

通过这些日志 API，您可以全面监控 Switch Hosts R 的运行状态，快速定位问题，分析用户行为，并建立完善的告警机制。