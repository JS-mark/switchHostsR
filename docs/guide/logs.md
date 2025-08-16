# 日志查看

Switch Hosts R 提供了完善的日志系统，帮助用户监控系统状态、排查问题和审计操作记录。

## 日志类型

### 系统日志

系统日志记录了应用程序的运行状态和系统级事件：

- **启动日志**：应用启动、关闭事件
- **错误日志**：系统错误、异常信息
- **性能日志**：资源使用情况、响应时间
- **安全日志**：权限验证、访问控制事件

### 操作日志

操作日志记录了用户的所有操作行为：

- **Hosts 操作**：创建、编辑、删除、切换配置
- **用户操作**：登录、注销、权限变更
- **配置操作**：设置修改、导入导出
- **系统操作**：备份、恢复、同步

### 网络日志

网络日志记录了网络相关的活动：

- **DNS 解析**：域名解析记录
- **网络请求**：HTTP/HTTPS 请求日志
- **连接状态**：网络连接建立、断开
- **流量统计**：数据传输量统计

## 日志查看界面

### 主日志面板

```vue
<template>
  <div class="logs-panel">
    <!-- 日志过滤器 -->
    <div class="logs-filter">
      <n-space>
        <n-select
          v-model:value="selectedLogType"
          :options="logTypeOptions"
          placeholder="选择日志类型"
        />
        <n-date-picker
          v-model:value="dateRange"
          type="daterange"
          placeholder="选择日期范围"
        />
        <n-input
          v-model:value="searchKeyword"
          placeholder="搜索关键词"
          clearable
        />
      </n-space>
    </div>

    <!-- 日志列表 -->
    <div class="logs-list">
      <n-data-table
        :columns="logColumns"
        :data="filteredLogs"
        :pagination="pagination"
        :loading="loading"
      />
    </div>
  </div>
</template>
```

### 日志详情弹窗

点击日志条目可以查看详细信息：

- **基本信息**：时间戳、日志级别、操作类型
- **详细内容**：完整的日志消息和上下文
- **相关数据**：操作前后的数据对比
- **堆栈信息**：错误日志的详细堆栈跟踪

## 日志过滤和搜索

### 按类型过滤

```typescript
interface LogFilter {
  type?: 'system' | 'operation' | 'network' | 'security'
  level?: 'debug' | 'info' | 'warn' | 'error'
  dateRange?: [Date, Date]
  keyword?: string
  userId?: string
}

const filterLogs = (logs: LogEntry[], filter: LogFilter) => {
  return logs.filter(log => {
    if (filter.type && log.type !== filter.type) return false
    if (filter.level && log.level !== filter.level) return false
    if (filter.keyword && !log.message.includes(filter.keyword)) return false
    if (filter.dateRange) {
      const logDate = new Date(log.timestamp)
      if (logDate < filter.dateRange[0] || logDate > filter.dateRange[1]) {
        return false
      }
    }
    return true
  })
}
```

### 高级搜索

支持多种搜索语法：

- **关键词搜索**：`error`、`hosts`、`user`
- **字段搜索**：`level:error`、`type:operation`
- **时间范围**：`date:2024-01-01..2024-01-31`
- **用户过滤**：`user:admin`、`user:guest`

## 日志导出

### 导出格式

支持多种导出格式：

```typescript
interface ExportOptions {
  format: 'json' | 'csv' | 'txt' | 'xlsx'
  dateRange?: [Date, Date]
  filter?: LogFilter
  fields?: string[]
}

const exportLogs = async (options: ExportOptions) => {
  const logs = await getFilteredLogs(options.filter)
  
  switch (options.format) {
    case 'json':
      return JSON.stringify(logs, null, 2)
    case 'csv':
      return convertToCSV(logs, options.fields)
    case 'txt':
      return convertToText(logs)
    case 'xlsx':
      return convertToExcel(logs, options.fields)
  }
}
```

### 自动导出

配置自动导出规则：

- **定时导出**：每日、每周、每月自动导出
- **条件导出**：达到特定条件时自动导出
- **邮件发送**：导出后自动发送到指定邮箱
- **云存储**：自动上传到云存储服务

## 日志配置

### 日志级别设置

```typescript
interface LogConfig {
  level: 'debug' | 'info' | 'warn' | 'error'
  enableConsole: boolean
  enableFile: boolean
  maxFileSize: number // MB
  maxFiles: number
  rotateDaily: boolean
}

const defaultLogConfig: LogConfig = {
  level: 'info',
  enableConsole: true,
  enableFile: true,
  maxFileSize: 10,
  maxFiles: 5,
  rotateDaily: true
}
```

### 日志存储

- **本地存储**：日志文件存储在本地目录
- **数据库存储**：结构化存储在 SQLite 数据库
- **内存缓存**：最近的日志保存在内存中
- **远程存储**：可选的远程日志服务

## 日志分析

### 统计图表

提供可视化的日志分析：

- **时间趋势**：日志数量随时间的变化
- **类型分布**：不同类型日志的占比
- **错误统计**：错误日志的频率和类型
- **用户活动**：用户操作的活跃度分析

### 异常检测

自动检测异常模式：

- **错误激增**：短时间内错误日志大量增加
- **性能异常**：响应时间异常增长
- **安全威胁**：可疑的访问模式
- **资源耗尽**：系统资源使用异常

## 日志维护

### 日志清理

```typescript
interface CleanupPolicy {
  maxAge: number // 天数
  maxSize: number // MB
  keepCritical: boolean // 保留关键日志
  archiveOld: boolean // 归档旧日志
}

const cleanupLogs = async (policy: CleanupPolicy) => {
  const cutoffDate = new Date(Date.now() - policy.maxAge * 24 * 60 * 60 * 1000)
  
  if (policy.archiveOld) {
    await archiveOldLogs(cutoffDate)
  }
  
  await deleteLogs({
    before: cutoffDate,
    keepCritical: policy.keepCritical
  })
}
```

### 性能优化

- **索引优化**：为常用查询字段建立索引
- **分页加载**：大量日志的分页显示
- **异步处理**：日志写入的异步处理
- **压缩存储**：历史日志的压缩存储

## 故障排查

### 常见问题

**日志不显示**
- 检查日志级别设置
- 确认日志文件权限
- 验证过滤条件

**日志文件过大**
- 启用日志轮转
- 调整保留策略
- 定期清理旧日志

**搜索性能慢**
- 缩小搜索范围
- 使用更精确的关键词
- 考虑建立索引

### 调试技巧

- **启用调试模式**：获取更详细的日志信息
- **实时监控**：使用实时日志查看功能
- **日志关联**：通过请求 ID 关联相关日志
- **性能分析**：使用性能日志分析瓶颈

## 最佳实践

### 日志管理

1. **合理设置日志级别**：避免过多的调试日志影响性能
2. **定期清理日志**：防止日志文件占用过多磁盘空间
3. **监控关键指标**：设置关键错误的告警机制
4. **备份重要日志**：定期备份重要的操作日志

### 安全考虑

1. **敏感信息过滤**：避免在日志中记录密码等敏感信息
2. **访问控制**：限制日志查看权限
3. **日志完整性**：确保日志不被恶意篡改
4. **审计跟踪**：记录日志查看和导出操作

通过完善的日志系统，Switch Hosts R 为用户提供了强大的监控和故障排查能力，确保系统的稳定运行和问题的快速定位。