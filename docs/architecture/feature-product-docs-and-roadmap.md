---
feature: product-docs-and-roadmap
role: architect
status: draft
date: 2026-03-29
---

# SwitchHostsR -- 技术架构设计文档

## 1. 系统架构总览

### 1.1 架构定位

SwitchHostsR 采用 **Tauri 2.x 混合桌面架构**，将应用分为 Rust 原生后端和 Web 前端两个独立层：

```
+---------------------------------------------------------------+
|                     SwitchHostsR Desktop App                   |
+---------------------------------------------------------------+
|  +--------------------------+  +----------------------------+  |
|  |      Frontend (Vue 3)    |  |    Backend (Rust/Tauri)    |  |
|  |                          |  |                            |  |
|  |  Pages / Components      |  |  API Layer (Commands)      |  |
|  |  Pinia State Management  |  |  Service Layer (Business)  |  |
|  |  Vue Router (Hash Mode)  |  |  Handler Layer (Data)      |  |
|  |  Bridge Abstraction      |  |  Auth Module (JWT/RBAC)    |  |
|  |  Monaco Editor           |  |  DB Layer (Diesel/SQLite)  |  |
|  |  Naive UI + UnoCSS       |  |  System UI (Tray/Window)   |  |
|  |  i18n (zh/en/ja)         |  |  Utils (Security/Time)     |  |
|  +-----------+--------------+  +-------------+--------------+  |
|              |        Tauri IPC (invoke)      |                 |
|              +-------------------------------+                 |
+---------------------------------------------------------------+
|                   OS Layer (macOS/Windows/Linux)                |
|    WebView (WKWebView/WebView2/WebKitGTK)  +  Native Process  |
+---------------------------------------------------------------+
```

### 1.2 核心技术栈

| 层级 | 技术 | 版本 | 用途 |
|------|------|------|------|
| **桌面框架** | Tauri | 2.0.1 | 原生桌面容器，提供 IPC、窗口管理、系统托盘 |
| **前端框架** | Vue 3 | 3.5.x | 响应式 UI 层，`<script setup>` 组合式 API |
| **UI 库** | Naive UI | 2.43.x | 企业级 Vue 3 组件库 |
| **原子 CSS** | UnoCSS | 66.x (Wind v3) | 原子化 CSS 工具，替代 Tailwind |
| **预处理器** | SCSS / LESS / Stylus | -- | 组件级样式（混用，存在统一性问题） |
| **状态管理** | Pinia | 3.0.x | Vue 3 官方状态管理 + persistedstate 持久化 |
| **路由** | Vue Router | 4.6.x | Hash 模式路由，含登录守卫 |
| **编辑器** | Monaco Editor | 0.55.x | Hosts 文件在线编辑器（VS Code 同款） |
| **国际化** | Vue i18n | 11.x | 中/英/日三语言 |
| **HTTP 客户端** | Alova | 3.4.x | 轻量 HTTP 请求库 |
| **后端语言** | Rust | 2021 Edition | 系统级后端语言 |
| **ORM** | Diesel | 2.1.6 | Rust ORM，支持 SQLite |
| **数据库** | SQLite | -- | 嵌入式关系数据库 |
| **连接池** | r2d2 | 0.8.x | 数据库连接池 |
| **密码哈希** | bcrypt | 0.17.x | 密码安全存储 |
| **构建工具** | Vite | 7.x | 前端构建 + HMR |
| **包管理** | pnpm | -- | 前端依赖管理 |
| **文档** | Rspress | 1.46.x | 静态文档站 |

### 1.3 架构模式

项目整体遵循 **分层架构（Layered Architecture）** 模式，同时在后端采用 **工厂模式（Factory Pattern）** 统一管理服务实例：

- **API 层**：Tauri Command handlers，负责请求参数解析和响应封装
- **Service 层**：业务逻辑层，实现认证校验和业务规则
- **Handler 层**：数据访问层，封装 Diesel ORM 操作
- **Model 层**：Diesel 数据模型定义

前端采用 **组件化架构** + **平台抽象层（Bridge Pattern）** 模式，通过 Bridge 类屏蔽 Tauri 和 uTools 两种运行环境的差异。

---

## 2. 前端架构

### 2.1 组件层次

前端采用典型的 Vue 3 组件层次结构：

```
App.vue
  |-- Provider.vue (Naive UI 主题/国际化配置)
  |     |-- NConfigProvider (主题注入)
  |     |-- NLoadingBarProvider / NNotificationProvider / NMessageProvider
  |
  |-- LHeader (全局头部，自定义标题栏)
  |-- LSider (侧边导航菜单)
  |-- RouterView (页面路由出口，带 fade 过渡动画)
  |-- AddHosts (抽屉式 hosts 添加面板)
  |-- Settings (设置面板)
  |-- GlobalModel (全屏弹窗)
```

**页面组件（`pages/`）**：

| 页面 | 路径 | 功能 |
|------|------|------|
| `home/welcome.vue` | `/home` | 首页欢迎页 |
| `login/index.vue` | `/login` | 登录页（含三方登录） |
| `hosts-list/index.vue` | `/hosts-list` | Hosts 列表管理主页 |
| `edit/index.vue` | `/edit/:id` | Monaco 编辑器页面 |
| `admin/index.vue` | `/admin` | 管理后台（用户管理、日志） |
| `about/index.vue` | `/about` | 关于页面 |
| `404/404.vue` | `/*` | 404 兜底页 |

**可复用组件（`components/`）**：

| 组件目录 | 功能 |
|---------|------|
| `editor/` | Monaco Editor 封装，支持 hosts 语法高亮 |
| `settings/` | 设置面板（通用/代理/命令/高级子页面） |
| `add-hosts/` | 添加 hosts 配置抽屉（本地/远程类型选择） |
| `layout/` | 全局布局组件（Header, Sider） |
| `svg-icon/` | SVG 图标封装 |
| `global-model/` | 全屏弹窗 |

### 2.2 状态管理

采用 **Pinia 3.x** + **pinia-plugin-persistedstate** 进行状态管理和持久化。

#### Store 设计

```
store/
  |-- index.ts           # 统一导出
  |-- useUser.ts         # 用户状态（登录态、用户信息、登录模式）
  |-- useHosts.ts        # Hosts 数据状态（编辑器显隐、hosts 数据、分组列表）
  |-- useSettings.ts     # 应用设置（主题、语言、代理、高级配置）
  |-- useLocal.ts        # 本地路由状态（动态路由管理）
  |-- featureStore.ts    # 功能开关（Feature Flag 管理）
```

**关键设计**：

1. **`useUser`**：管理登录模式（支持 GitHub、Email 等多种登录方式）、用户信息和登录状态。通过 `setLoginUser` 工具函数将用户信息序列化到 localStorage，路由守卫中读取并恢复。

2. **`useHosts`**：管理 hosts 数据的核心状态，使用 `Map<string, LocalHosts | RemoteHosts>` 存储所有 hosts 数据（按 ID 索引），同时维护 localHosts 和 remoteHosts 两个列表索引。启用 Pinia 持久化。

3. **`useSettings`**：分为 system、user、general、proxy、cmd、advanced 六个命名空间，覆盖所有应用配置项。通过 `setSettingsByData` 实现深度合并更新。

4. **`useLocal`**：管理动态路由注册，通过事件系统（`globalEventEmitter`）与路由模块通信。实现了 hosts 编辑页面的动态路由创建和恢复。

5. **`featureStore`**：Feature Flag 模式，通过 Map 管理功能开关，支持运行时动态启用/禁用功能。

### 2.3 路由设计

**路由模式**：Hash Mode（`createWebHashHistory`），适配 Tauri WebView 环境。

**路由结构**：

```
/home          -> 首页（默认路由）
/login         -> 登录页（hidden）
/hosts-list    -> Hosts 列表
/about         -> 关于页面
/admin         -> 管理后台
/edit/:id      -> 编辑器页面（hidden，动态参数）
/error/404     -> 404 页面（重定向到首页）
/*             -> NotFound 兜底
```

**路由守卫逻辑**（`router/index.ts` -> `beforeEach`）：

1. 从 localStorage 读取已保存的登录用户信息（`getLoginUser`）
2. 检查 7 天有效期：`now - data.time >= 7 * 24 * 60 * 3600 * 1000`
3. 有效期内：恢复 Pinia 登录状态
4. 未登录：强制重定向到 `/login`

**存在的问题**：路由守卫中的条件逻辑 `to.name !== 'Login' || (to.name !== 'Login' && from.name !== 'Login')` 存在冗余，第二个条件始终为第一个条件的子集。

### 2.4 平台抽象层

项目通过 **Bridge 模式** 实现对 Tauri 和 uTools 两种运行环境的统一抽象：

```
Bridge Class (EventEmitter)
  |
  +-- Tauri 模式: initTauri() -> @tauri-apps/api
  |
  +-- uTools 模式: initUTool() -> window.utools
```

**工作流程**：

1. `useJSBridge()` 检测 `window.utools` 判断运行环境
2. 创建 `Bridge` 实例，异步初始化对应平台 API
3. 通过 `useBridge<SYSTEM_ENV.TAURI>()` 获取类型安全的平台 API
4. `useBridgeFunc` 工具函数统一封装双平台调用（第一参数为 uTools 实现，第二参数为 Tauri 实现）

**当前状态**：uTools 端的回调函数大多为空实现（标注 `// NOTE: 待实现`），仅 Tauri 端功能完整。

---

## 3. 后端架构

### 3.1 分层设计

Rust 后端严格遵循四层架构：

```
+---------------------------------------------------------+
|                    API Layer (api/)                       |
|  Tauri #[command] handlers                               |
|  请求参数解析 | 响应封装 | 宏级别认证校验                      |
+---------------------------------------------------------+
          |  调用 ServiceFactory 获取服务实例
+---------------------------------------------------------+
|                 Service Layer (db/services/)              |
|  业务逻辑 | 认证/权限校验 | 跨表事务 | 数据组装               |
|  UserService | HostService | HostGroupService             |
|  LogService | AuthService | SystemService                  |
+---------------------------------------------------------+
          |  调用 Handler 执行数据库操作
+---------------------------------------------------------+
|                 Handler Layer (db/handlers/)              |
|  底层数据库操作 | Diesel DSL | 单表 CRUD                     |
|  UserHandler | HostHandler | HostGroupHandler | LogHandler |
+---------------------------------------------------------+
          |  Diesel ORM 映射
+---------------------------------------------------------+
|                 Model Layer (db/models/)                  |
|  数据模型 (Queryable/Insertable) | Schema 映射              |
|  User | Host | HostGroup | HostGroupRelation | Log         |
+---------------------------------------------------------+
          |
+---------------------------------------------------------+
|            Database (SQLite + r2d2 ConnectionPool)        |
+---------------------------------------------------------+
```

**API 层关键设计**：

- 所有命令通过 `#[tauri::command]` 宏标注
- 使用 `require_auth!()` 宏在函数入口进行认证校验
- 使用 `safe_execute!()` 宏统一错误处理和响应转换
- 统一响应格式 `ApiResult<T>` 包含 `code`、`msg`、`data` 三个字段
- 标准 HTTP 状态码：200（成功）、400（参数错误）、401（认证错误）、403（权限错误）、404（不存在）、500（内部错误）

**Service 层关键设计**：

- 所有服务实现 `BaseService` trait，提供统一的 pool 访问和权限校验能力
- 服务间通过 `ServiceFactory` 解耦，不直接依赖
- 每个服务方法接收 `AuthContext` 参数进行权限校验

### 3.2 服务工厂模式

```rust
ServiceFactory {
    pool: DbPool (Arc<Pool<ConnectionManager<SqliteConnection>>>)
}
  |-- user_service()       -> UserService
  |-- host_service()       -> HostService
  |-- host_group_service() -> HostGroupService
  |-- log_service()        -> LogService
  |-- auth_service()       -> AuthService
  |-- system_service()     -> SystemService
```

**全局初始化流程**：

1. `lib.rs::run()` 启动时调用 `db::create_pool()` 创建数据库连接池
2. `init_service_factory(pool)` 将连接池注入全局 `OnceLock<ServiceFactory>` 单例
3. `AppState::new(get_service_factory().clone())` 将工厂实例注入 Tauri 状态管理
4. API handler 通过 `state.service_factory.xxx_service()` 获取服务实例

**优势**：
- 服务实例按需创建，无长期持有
- 连接池在 `Arc` 下安全共享
- 新增服务只需扩展 `ServiceFactory`

**注意**：项目中存在 **两套 AppState**：
- `api/mod.rs::AppState`：当前使用，基于 `ServiceFactory`
- `ui/app_state.rs::AppState`：旧版设计，基于 `Mutex<Handler>`，已不再使用但代码未删除

### 3.3 数据库设计

**数据库**：SQLite（嵌入式，无需外部服务）
**ORM**：Diesel 2.1.6（编译时类型检查）
**连接池**：r2d2，生产默认连接数，测试环境 max_size=5
**迁移管理**：diesel_migrations（嵌入式迁移，`embed_migrations!`）

#### 表结构

**users 表**（用户表）：
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER PK | 自增主键 |
| username | TEXT NOT NULL UNIQUE | 用户名（唯一） |
| password | TEXT NOT NULL | bcrypt 哈希密码 |
| email | TEXT | 可选邮箱 |
| avatar | TEXT | 头像 URL |
| is_admin | INTEGER | 管理员标志（布尔） |
| created_at | INTEGER | 创建时间戳（毫秒） |
| updated_at | INTEGER | 更新时间戳（毫秒） |

**hosts 表**（Hosts 配置表）：
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER PK | 自增主键 |
| user_id | INTEGER NOT NULL FK | 所属用户 |
| name | TEXT NOT NULL | 配置名称 |
| description | TEXT | 描述 |
| content | TEXT NOT NULL | Hosts 内容文本 |
| is_active | INTEGER NOT NULL | 是否激活（0/1） |
| is_system | INTEGER NOT NULL | 是否系统级（0/1） |
| created_at | INTEGER | 创建时间戳 |
| updated_at | INTEGER | 更新时间戳 |

**host_groups 表**（主机组表）：
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER PK | 自增主键 |
| user_id | INTEGER NOT NULL FK | 所属用户 |
| name | TEXT NOT NULL | 组名称 |
| description | TEXT | 描述 |
| created_at | INTEGER | 创建时间戳 |
| updated_at | INTEGER | 更新时间戳 |

**host_group_relations 表**（多对多关联表）：
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER PK | 自增主键 |
| group_id | INTEGER NOT NULL FK | 主机组 ID |
| host_id | INTEGER NOT NULL FK | 主机 ID |
| created_at | INTEGER | 创建时间戳 |
| | UNIQUE(group_id, host_id) | 联合唯一约束 |

**logs 表**（操作日志表）：
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER PK | 自增主键 |
| user_id | INTEGER NOT NULL FK | 操作用户 |
| action | TEXT NOT NULL | 操作类型 |
| target_type | TEXT NOT NULL | 操作目标类型 |
| target_id | INTEGER | 操作目标 ID |
| details | TEXT | 操作详情（JSON） |
| created_at | INTEGER | 创建时间戳 |

**索引策略**：
- `idx_users_username`：用户名索引（登录查询加速）
- `idx_hosts_user_id`：按用户查询 hosts
- `idx_hosts_is_active`：查询活跃 hosts
- `idx_logs_user_id`：按用户查询日志
- `idx_logs_created_at`：按时间范围查询日志
- `idx_host_groups_user_id`：按用户查询主机组

### 3.4 认证与授权

认证体系由 `auth/` 模块实现，包含五个子模块：

#### 3.4.1 令牌管理（token.rs）

采用 **双 Token 机制**：

- **Access Token**：有效期 2 小时，用于 API 请求认证
- **Refresh Token**：有效期 7 天，用于刷新 Access Token

Token 格式：`base64(claims_json).hash_signature`

**注意**：当前实现是 **简化版 JWT**，使用 `base64 + DefaultHasher` 签名，而非标准 JWT（jsonwebtoken 库）。注释中也明确标注了"应使用真正的 JWT 库"。这是一个 **技术债务**。

全局实例通过 `OnceLock<TokenManager>` 管理，密钥从环境变量 `JWT_SECRET` 获取（默认值为硬编码的 `default_secret_key_change_in_production`）。

**Token 撤销**：通过内存中的 `HashMap<JTI, RevokeTime>` 记录已撤销的令牌，支持定期清理过期记录。

#### 3.4.2 会话管理（session.rs）

- 会话存储在内存中（`HashMap<SessionId, Session>`），由 `OnceLock<SessionManager>` 全局管理
- 默认会话有效期 24 小时
- 支持：创建会话、获取会话、更新活动时间、延长会话、移除会话、清理过期会话
- Session 可转换为 `AuthContext`，包含用户 ID、角色和权限集合

#### 3.4.3 角色体系（roles.rs）

四级角色层级，支持权限继承：

```
SuperAdmin (Level 3)
    |-- 继承自 Admin
    |-- 拥有 AdminAll 权限（通配）
Admin (Level 2)
    |-- 继承自 User
    |-- 用户管理、系统配置、日志删除等管理权限
User (Level 1)
    |-- 继承自 Guest
    |-- 管理自己的 hosts、主机组；查看自己的日志
Guest (Level 0)
    |-- 仅有 SystemInfo 查看权限
```

#### 3.4.4 权限系统（permissions.rs）

22 种细粒度权限点：

- 用户管理：UserCreate, UserRead, UserUpdate, UserDelete, UserList
- 主机管理：HostCreate, HostRead, HostUpdate, HostDelete, HostList, HostActivate
- 主机组管理：HostGroupCreate, HostGroupRead, HostGroupUpdate, HostGroupDelete, HostGroupList
- 日志管理：LogRead, LogList, LogDelete
- 系统管理：SystemConfig, SystemInfo
- 超级权限：AdminAll

权限检查通过 `AuthContext::has_permission()` 方法，SuperAdmin 直接放行，其他角色查表。

#### 3.4.5 中间件（middleware.rs）

`AuthMiddleware` 管理当前会话状态，`AuthState` 封装线程安全的认证状态访问。提供宏级别的权限检查：

- `require_auth!()`：验证用户已登录
- `require_permission!($auth, $perm)`：验证用户拥有指定权限
- `require_user_access!($auth, $user_id)`：验证用户可访问目标资源

---

## 4. 前后端通信

### 4.1 通信机制

采用 **Tauri IPC（Inter-Process Communication）** 机制，前端通过 `invoke` 调用后端 Rust 命令：

```
前端 TypeScript                     后端 Rust
      |                                |
  bridge.core.invoke('command_name',   |
    { param1, param2 })                |
      |                                |
      +-----[Tauri IPC Channel]------->|
      |                          #[tauri::command]
      |                          fn command_name(
      |                            state: State<AppState>,
      |                            param1: Type1,
      |                            param2: Type2
      |                          ) -> Result<ApiResult<T>, ()>
      |                                |
      |<------[JSON Response]----------+
      |                                |
  ApiResult { code, msg, data }        |
```

### 4.2 通信协议

**请求**：通过 `invoke` 的参数对象传递，自动序列化为 JSON。
**响应**：统一返回 `ApiResult<T>` 结构：

```typescript
interface ApiResult<T> {
  code: number   // HTTP 语义状态码
  msg: string    // 消息（成功时为 "success"）
  data?: T       // 业务数据（可选）
}
```

**分页响应**：

```typescript
interface PageData<T> {
  list: T[]      // 数据列表
  total: number  // 总记录数
}
```

### 4.3 前端 API 封装

前端在 `apis/` 目录下封装了所有 Tauri Command 调用，通过 `useBridgeFunc` 实现双平台兼容：

```typescript
// apis/user.ts
export function loginPlatform(options) {
  return useBridgeFunc(
    () => { /* uTools 实现（待实现） */ },
    (bridge, resolve, reject) => {
      bridge.core.invoke('user_login', options)
        .then(res => resolve(res))
        .catch(err => reject(err))
    }
  )
}
```

### 4.4 已注册的 Tauri Commands

当前共注册了 **41 个 Tauri Command**，分为以下类别：

| 类别 | 数量 | 示例 |
|------|------|------|
| 系统管理 | 8 | get_system_config, health_check, create_backup |
| 文件操作 | 7 | read_text_file, write_text_file, list_directory |
| 用户认证 | 7 | user_login, logout, refresh_token, verify_token |
| 用户管理 | 6 | get_users, create_user, update_user, search_users |
| Hosts 管理 | 11 | get_hosts, create_host, toggle_host_active, import_hosts |
| 主机组管理 | 8 | get_host_groups, add_host_to_group, toggle_group_active |
| 日志管理 | 11 | get_logs, create_log, export_logs, get_operation_types |

---

## 5. 数据模型

### 5.1 ER 关系图

```
+----------+       1:N       +----------+       N:M       +-------------+
|  users   |<--------------->|  hosts   |<--------------->| host_groups |
|----------|                 |----------|   通过           |-------------|
| id (PK)  |                 | id (PK)  | host_group_     | id (PK)     |
| username |                 | user_id  | relations       | user_id     |
| password |                 | name     |                 | name        |
| email    |                 | content  |                 | description |
| avatar   |                 | is_active|                 | created_at  |
| is_admin |                 | is_system|                 | updated_at  |
+----------+                 +----------+                 +-------------+
     |                                                          |
     | 1:N                                                      |
     v                                                          |
+----------+                 +---------------------+            |
|   logs   |                 | host_group_relations|<-----------+
|----------|                 |---------------------|
| id (PK)  |                 | id (PK)            |
| user_id  |                 | group_id (FK)      |
| action   |                 | host_id (FK)       |
| target_  |                 | created_at         |
|   type   |                 | UNIQUE(group_id,   |
| target_id|                 |   host_id)         |
| details  |                 +---------------------+
+----------+
```

### 5.2 关系说明

- **users -> hosts**：1:N，每个用户拥有多个 hosts 配置
- **users -> host_groups**：1:N，每个用户拥有多个主机组
- **users -> logs**：1:N，每个用户产生多条操作日志
- **hosts <-> host_groups**：N:M，通过 `host_group_relations` 关联表实现多对多
- **所有外键**均设置了 `ON DELETE CASCADE`，删除用户时级联删除其所有数据

### 5.3 Diesel 模型设计

每个数据表对应两个 Rust 结构体：

- **查询模型**（如 `Host`）：`#[derive(Queryable, Selectable)]`，用于数据库读取
- **插入模型**（如 `NewHost`）：`#[derive(Insertable)]`，用于数据库写入

更新操作直接使用 Diesel DSL 的 `update().set()` 链式调用，未定义独立的 Changeset 结构体。

---

## 6. 安全架构

### 6.1 安全层级

```
+---------------------------------------------------+
|  Layer 1: 传输安全                                   |
|  Tauri IPC (进程内通信，无网络传输风险)                  |
+---------------------------------------------------+
|  Layer 2: 认证安全                                   |
|  双 Token (Access + Refresh) + Session 管理           |
|  bcrypt 密码哈希 (DEFAULT_COST)                      |
+---------------------------------------------------+
|  Layer 3: 授权安全                                   |
|  RBAC 四级角色 + 22 种细粒度权限                       |
|  资源级别访问控制 (can_access_user_resource)           |
+---------------------------------------------------+
|  Layer 4: 应用安全                                   |
|  单实例控制 | 窗口关闭隐藏 | 托盘常驻                    |
+---------------------------------------------------+
|  Layer 5: 数据安全                                   |
|  SQLite 本地存储 | 外键级联删除 | 操作审计日志            |
+---------------------------------------------------+
```

### 6.2 安全风险清单

| 风险等级 | 问题 | 位置 | 建议 |
|---------|------|------|------|
| **高** | GitHub OAuth Token 硬编码在前端代码中 | `apis/user.ts` L9 | 移至后端环境变量或安全存储 |
| **高** | JWT 签名使用 `DefaultHasher`（非加密安全） | `auth/token.rs` L231-239 | 替换为 `jsonwebtoken` + HMAC-SHA256 |
| **高** | JWT 密钥默认值硬编码 | `auth/token.rs` L249-250 | 强制从安全配置加载 |
| **中** | CSP 安全策略为 null | `tauri.conf.json` | 配置适当的 CSP 规则 |
| **中** | Session 和 Token 撤销列表存储在内存中 | `auth/session.rs`, `auth/token.rs` | 应用重启后丢失，考虑持久化 |
| **低** | 文件操作 API 无路径白名单校验 | `api/commands.rs` | 限制可操作的文件路径范围 |
| **低** | 密码存储字段在 Diesel schema 中名为 `password` | `db/schema.rs` | 属于编码规范问题，不影响安全 |

---

## 7. 当前架构评估

### 7.1 优势

1. **技术选型先进**
   - Tauri 2.x 相比 Electron 在包体积（约 1/10）、内存占用（约 1/3）、启动速度上有显著优势
   - Rust 后端提供内存安全保证和高性能
   - Vue 3 Composition API + TypeScript 提供良好的开发体验

2. **后端分层清晰**
   - API -> Service -> Handler -> Model 四层架构职责明确
   - ServiceFactory 模式实现了良好的依赖注入
   - BaseService trait 统一了认证和权限校验接口

3. **认证授权体系完善**
   - RBAC 四级角色 + 22 种细粒度权限覆盖全面
   - 角色继承机制减少了权限配置的冗余
   - 双 Token 机制兼顾安全性和用户体验
   - 宏级别的权限校验简洁且一致

4. **平台抽象设计良好**
   - Bridge 模式为 Tauri 和 uTools 双平台提供了统一接口
   - 新平台扩展只需实现对应的初始化函数

5. **数据库设计规范**
   - Diesel ORM 提供编译时 SQL 类型检查
   - 合理的索引策略
   - 外键和级联删除保证数据一致性
   - 嵌入式迁移管理

6. **工程化配置完善**
   - ESLint + Stylelint + lint-staged 代码质量保障
   - Commitlint + Commitizen 提交规范
   - i18n 三语言全覆盖
   - TypeScript 严格类型检查

### 7.2 不足与技术债务

#### 架构级问题

1. **核心功能缺失**：最关键的 "将 hosts 内容写入系统 hosts 文件" 功能尚未实现（WIP-01），意味着应用的核心价值还未交付。

2. **两套 AppState 共存**：
   - `api/mod.rs::AppState` 基于 `ServiceFactory`（当前使用）
   - `ui/app_state.rs::AppState` 基于 `Mutex<Handler>`（旧版，未删除）
   - 两套设计理念不同，增加代码理解难度。

3. **认证流程不完整**：`api/mod.rs::get_auth_context()` 始终返回 `Err(NotAuthenticated)`，说明认证状态的传递机制尚未完整实现。`require_auth!()` 宏依赖此函数，导致所有需要认证的 API 实际上都会返回未登录错误。

4. **前后端数据模型不一致**：
   - 前端 `apis/hosts.ts` 定义的 `Hosts` 接口包含 `hosts_type`、`hosts_path`、`status`、`is_del`、`is_readonly`、`hosts_refresh_time` 等字段
   - 后端 `db/models/hosts.rs` 的 `Host` 模型仅包含 `name`、`content`、`is_active`、`is_system` 等字段
   - 二者严重不匹配，说明前端模型可能是早期设计遗留

#### 代码级问题

5. **JWT 实现不安全**：使用 `DefaultHasher`（非加密安全的哈希函数）签名 Token，任何了解内部实现的人都可以伪造 Token。应替换为 `jsonwebtoken` 库 + HMAC-SHA256。

6. **样式预处理器混用**：项目中同时使用了 Stylus（App.vue）、LESS（style.less）和 SCSS（部分组件），缺乏统一的样式规范。

7. **测试覆盖不足**：
   - 后端 API 层的测试用例大多被注释（`// 这里需要模拟认证上下文`）
   - 认证模块和角色权限模块有完整的单元测试
   - 缺乏集成测试和 E2E 测试

8. **硬编码问题**：
   - GitHub OAuth Token 硬编码在前端代码中（`apis/user.ts`）
   - JWT 密钥有硬编码默认值
   - 编辑器页面使用硬编码的示例内容

9. **内存中状态管理**：Session 和 Token 撤销列表均存储在内存 HashMap 中，应用重启后全部丢失。

10. **过度设计问题**：作为本地桌面应用，完整的 RBAC + JWT + Session 三重认证体系可能过于复杂。大多数场景下只有一个用户使用。

---

## 8. 架构演进建议

### 8.1 短期（v0.1.0）-- 功能闭环

#### 8.1.1 实现核心 Hosts 写入功能

**关键设计决策**：

```rust
// 建议在 db/services/ 新增 system_hosts_service.rs
pub struct SystemHostsService {
    pool: DbPool,
}

impl SystemHostsService {
    /// 将所有激活的 hosts 配置合并写入系统 hosts 文件
    pub fn apply_hosts(&self, auth: &AuthContext) -> Result<()> {
        // 1. 查询所有 is_active = 1 的 hosts 记录
        // 2. 按优先级/顺序合并 content
        // 3. 根据 settings.writeMode 决定追加还是覆盖
        // 4. 通过 tauri-plugin-shell 执行系统命令写入 /etc/hosts
        //    - macOS/Linux: sudo tee /etc/hosts
        //    - Windows: 直接写入 C:\Windows\System32\drivers\etc\hosts
        // 5. 可选：执行 DNS 缓存刷新命令
    }
}
```

**权限提升方案**：使用 `tauri-plugin-shell` 执行 `pkexec`（Linux）或 `osascript`（macOS）请求管理员权限。Windows 通过 UAC 提权。

#### 8.1.2 修复认证状态传递

当前 `require_auth!()` 宏依赖的 `get_auth_context()` 始终返回错误。建议：

1. 将 `AuthState` 注入 Tauri 全局状态（`app.manage(AuthState::new())`）
2. 在 API handler 中通过 `State<AuthState>` 获取认证状态
3. 修改 `require_auth!()` 宏接收 `State<AuthState>` 参数

#### 8.1.3 简化认证流程

针对本地桌面应用场景，建议引入 **自动登录模式**：

- 首次启动：引导用户创建本地管理员账户
- 后续启动：自动以本地用户身份登录（跳过登录页）
- 设置中提供"启用密码保护"选项，开启后才要求输入密码

#### 8.1.4 清理旧版代码

删除 `ui/app_state.rs` 中的旧版 `AppState`，统一使用 `api/mod.rs` 中的新版。

### 8.2 中期（v0.2.0）-- 体验提升

#### 8.2.1 远程 Hosts 同步架构

```
+------------------+     HTTP/HTTPS      +------------------+
| HostSyncService  |<------------------->| Remote URL       |
|                  |                     | (GitHub Gist,    |
|  scheduler:      |                     |  HTTP Server,    |
|    - cron 定时    |                     |  WebDAV, etc.)   |
|    - 手动触发     |                     +------------------+
|  conflict:       |
|    - 合并策略     |
|    - 冲突检测     |
+------------------+
```

建议使用 `tauri-plugin-http` 进行远程拉取，在 Rust 侧实现定时任务（tokio interval 或 cron 表达式解析）。

#### 8.2.2 版本历史设计

建议新增 `host_versions` 表：

```sql
CREATE TABLE host_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    version_number INTEGER NOT NULL,
    change_summary TEXT,
    created_by INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id)
);
```

每次 `update_host` 操作前保存当前版本，配合 Monaco DiffEditor 实现可视化差异对比。

#### 8.2.3 替换 JWT 实现

引入 `jsonwebtoken` crate 替换当前的简化实现：

```toml
# Cargo.toml
jsonwebtoken = "9.x"
```

使用 HMAC-SHA256 签名算法，密钥从系统密钥链或加密配置文件加载。

### 8.3 长期（v1.0.0）-- 成熟产品

#### 8.3.1 团队协作架构

如果后续需要团队配置共享，建议以下架构：

```
本地模式（默认）                    团队模式
+------------------+              +------------------+
| SQLite (本地)     |              | SQLite (本地缓存) |
| 完整功能          |              | 离线可用          |
+------------------+              +--------+---------+
                                           |
                                  +--------v---------+
                                  | Sync Engine       |
                                  | (增量同步/冲突解决)|
                                  +--------+---------+
                                           |
                                  +--------v---------+
                                  | Remote Backend    |
                                  | (GitHub Gist /    |
                                  |  WebDAV /         |
                                  |  Self-hosted API) |
                                  +------------------+
```

#### 8.3.2 插件系统架构

为支持 Hosts 模板市场和社区扩展，建议引入轻量插件系统：

- 插件格式：JSON 定义 + Hosts 内容文件
- 插件仓库：GitHub Release 或自建 CDN
- 安装机制：下载 -> 校验 -> 导入到数据库

---

## 9. 新功能技术方案

### 9.1 Hosts 写入系统文件（WIP-01, FUT-01）

**技术方案**：

| 平台 | 方案 | 所需权限 |
|------|------|---------|
| macOS | `osascript -e 'do shell script "tee /etc/hosts" with administrator privileges'` | 管理员密码弹窗 |
| Linux | `pkexec tee /etc/hosts` 或 `sudo tee /etc/hosts` | polkit 授权或 sudo |
| Windows | 直接 `fs::write` 到 `C:\Windows\System32\drivers\etc\hosts` | UAC 提权（应用以管理员身份运行） |

**DNS 缓存刷新**：

| 平台 | 命令 |
|------|------|
| macOS | `dscacheutil -flushcache && sudo killall -HUP mDNSResponder` |
| Linux | `systemd-resolve --flush-caches` 或 `nscd -i hosts` |
| Windows | `ipconfig /flushdns` |

使用 `tauri-plugin-shell` 的 `Command::new()` API 执行系统命令。

### 9.2 Hosts 语法检查（FUT-02）

**技术方案**：

- 在后端实现 `HostsValidator` 模块，逐行解析 hosts 内容
- 检查规则：IP 格式校验（IPv4/IPv6）、域名格式校验、重复项检测、注释格式
- 通过独立的 Tauri Command（如 `validate_hosts`）返回校验结果
- 前端在 Monaco Editor 中通过 `editor.setModelMarkers()` 显示错误标记

### 9.3 配置差异对比（FUT-04）

**技术方案**：

Monaco Editor 自带 DiffEditor 组件：

```typescript
import * as monaco from 'monaco-editor'

const diffEditor = monaco.editor.createDiffEditor(container, {
  readOnly: true,
  renderSideBySide: true,
})

diffEditor.setModel({
  original: monaco.editor.createModel(oldContent, 'plaintext'),
  modified: monaco.editor.createModel(newContent, 'plaintext'),
})
```

### 9.4 全局快捷键（FUT-08）

**技术方案**：

已引入 `tauri-plugin-global-shortcut`，需在 `lib.rs` 的 setup 中注册：

```rust
use tauri_plugin_global_shortcut::ShortcutManager;

app.plugin(tauri_plugin_global_shortcut::init())?;

// 注册全局快捷键
let manager = app.global_shortcut();
manager.register("CmdOrCtrl+Shift+H", |app, shortcut| {
    // 显示/隐藏主窗口
})?;
```

### 9.5 自动更新（FUT-06）

**技术方案**：

使用 Tauri 内置的 Updater 插件：

```toml
# Cargo.toml
tauri-plugin-updater = "2.x"
```

配合 GitHub Releases 作为更新源，在应用启动时检查最新版本。

---

## 10. 部署与发布架构

### 10.1 构建流程

```
源代码
  |
  +--[前端构建]--> pnpm build:tauri --> Vite 构建 --> dist/
  |
  +--[后端构建]--> cargo build --release --> 原生二进制
  |
  +--[打包]-----> tauri build --> 平台安装包
                    |-- macOS: .dmg / .app
                    |-- Windows: .msi / .nsis
                    |-- Linux: .deb / .AppImage
```

### 10.2 建议的 CI/CD 流程

```
GitHub Push/PR
     |
     v
GitHub Actions
     |
     +-- Lint (ESLint + Clippy)
     +-- Test (cargo test + vitest)
     +-- Build (multi-platform matrix)
     |     |-- macOS (arm64 + x86_64)
     |     |-- Windows (x86_64)
     |     |-- Linux (x86_64)
     |
     +-- Release (on tag push)
           |-- GitHub Releases (附带安装包)
           |-- 更新服务器 (Tauri Updater endpoint)
```

### 10.3 环境配置

| 环境变量 | 用途 | 默认值 |
|---------|------|--------|
| `DATABASE_URL` | SQLite 数据库文件路径 | 需配置 |
| `JWT_SECRET` | JWT 签名密钥 | 硬编码默认值（需修改） |
| `RUST_LOG` | 日志级别 | 未设置 |

**建议**：将 `DATABASE_URL` 默认设置为用户数据目录（如 `~/.config/switchhostsr/data.db`），无需用户手动配置。

---

*本文档基于 SwitchHostsR 仓库（截至 2026-03-29，dev 分支，commit `2b85152`）的完整代码分析编写。所有架构描述和评估均来源于实际代码审查。*
