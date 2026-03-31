---
feature: product-docs-and-roadmap
role: tester
status: draft
date: 2026-03-29
---

# SwitchHostsR -- 测试计划文档

## 1. 测试范围

### 1.1 项目概述

SwitchHostsR 是一款跨平台桌面应用（macOS/Windows/Linux），基于 Tauri 2.x + Vue 3 + Rust + Diesel(SQLite) 构建，用于管理系统 hosts 文件。本测试计划覆盖已实现功能的全量测试，以及待实现功能的测试预案。

### 1.2 测试目标

- 验证所有已实现的后端 Service 层和 API 层功能的正确性
- 验证前端页面交互逻辑和用户体验
- 验证前后端通信（Tauri Command RPC）的完整性
- 验证认证授权系统（JWT + RBAC + Session）的安全性
- 验证数据库操作的完整性和一致性
- 验证跨平台兼容性
- 评估系统性能指标（启动时间、内存占用、响应速度）

### 1.3 功能范围

#### 已实现功能（In Scope - 完整测试）

| 模块 | 后端状态 | 前端状态 | 测试优先级 |
|------|---------|---------|-----------|
| 用户认证（注册/登录/登出/Token刷新） | 完整 | 基本对接 | P0 |
| Hosts CRUD（创建/读取/更新/删除） | 完整 | 部分对接 | P0 |
| Hosts 状态切换（启用/禁用） | 完整 | 已对接 | P0 |
| Hosts 搜索/分页/统计 | 完整 | 已对接 | P1 |
| Hosts 导入/导出 | 完整 | 未对接 | P1 |
| 主机组 CRUD | 完整 | 未完全对接 | P1 |
| 主机组成员管理 | 完整 | 未对接 | P1 |
| 操作日志系统 | 完整 | 基本对接 | P1 |
| 用户管理（管理后台） | 完整 | 部分桩实现 | P1 |
| RBAC 权限控制 | 完整 | 后端完整 | P0 |
| JWT Token 管理 | 完整 | 后端完整 | P0 |
| Session 会话管理 | 完整 | 后端完整 | P0 |
| 系统管理（配置/健康检查） | 完整 | 后端完整 | P2 |
| 文件操作命令 | 完整 | 后端完整 | P2 |
| 系统托盘 | 完整 | 已实现 | P1 |
| 国际化（i18n） | N/A | 已实现 | P2 |
| 应用设置 | N/A | 已实现 | P2 |
| 单实例控制 | 完整 | N/A | P1 |

#### 待实现功能（Out of Scope - 仅预留测试框架）

| 功能 | 编号 | 备注 |
|------|------|------|
| Hosts 写入系统文件 | WIP-01 | 核心功能缺失，后续需要重点测试 |
| 远程 Hosts 同步 | WIP-02 | UI 已设计，后端未实现 |
| 备份与恢复 | WIP-04 | 当前为桩实现 |
| 全局快捷键 | WIP-09 | 依赖已引入，逻辑未实现 |
| uTools 模式 | WIP-05 | Bridge 架构已搭建 |

## 2. 现有测试分析

### 2.1 测试代码总览

| 文件 | 类型 | 测试数量 | 覆盖模块 |
|------|------|---------|---------|
| `src-tauri/tests/api_tests.rs` | Service 层单元测试 | 4 个 | 日志、用户注册登录、Hosts CRUD、主机组 CRUD |
| `src-tauri/tests/integration_tests.rs` | 集成测试 | 6 个 | 完整用户流程、Hosts 完整流程、主机组完整流程、权限系统、日志系统、数据一致性、并发操作 |
| `src-tauri/src/auth/token.rs` | 单元测试（内联） | 3 个 | Token 生成、验证、撤销 |
| `src-tauri/src/auth/session.rs` | 单元测试（内联） | 4 个 | Session 创建、过期、管理器操作、AuthContext 转换 |
| `src-tauri/src/auth/permissions.rs` | 单元测试（内联） | 5 个 | 各角色权限分配、权限检查函数 |
| `src-tauri/src/api/mod.rs` | 单元测试（内联） | 4 个 | ApiResult 成功/错误/认证错误、分页参数 |
| `src-tauri/src/db/services/auth_service.rs` | 单元测试（内联） | 4 个 | 注册、登录、无效登录、用户名可用性 |
| `src-tauri/src/db/services/host_service.rs` | 单元测试（内联） | 3 个 | 创建主机、按ID获取、切换状态 |
| `src-tauri/src/db/services/user_service.rs` | 单元测试（内联） | 3 个 | 按ID获取用户、更新用户、管理员操作 |
| `src-tauri/src/api/hosts.rs` | 占位测试（内联） | 1 个 | 被注释，未实际运行 |
| `src-tauri/src/api/users.rs` | 占位测试（内联） | 1 个 | 被注释，未实际运行 |
| `src-tauri/src/api/host_groups.rs` | 占位测试（内联） | 1 个 | 被注释，未实际运行 |
| `src-tauri/src/api/logs.rs` | 占位测试（内联） | 1 个 | 被注释，未实际运行 |
| `src-tauri/src/api/system.rs` | 占位测试（内联） | 1 个 | 被注释，未实际运行 |
| `src-tauri/src/api/commands.rs` | 单元测试（内联） | 1 个 | get_system_info（唯一实际运行的 API 测试） |

### 2.2 测试基础设施分析

**已有设施：**

- 内存 SQLite 数据库（`:memory:`）用于测试隔离
- `create_test_db()` 辅助函数：创建内存数据库并手动建表
- `create_test_user_and_auth()` 辅助函数：创建测试用户和认证上下文
- `create_test_service_factory()` / `create_test_app_state()` 辅助函数
- `tempfile::TempDir` 用于测试目录隔离
- `tokio::test` 支持异步测试
- Tauri `mock_app` 用于模拟 Tauri 环境（但实际未使用）

**存在问题：**

1. **API 层测试大量被注释**：`hosts.rs`、`users.rs`、`host_groups.rs`、`logs.rs`、`system.rs` 中的测试均为占位，注释说"这里需要模拟认证上下文"，未实际实现
2. **认证上下文模拟缺失**：API 层使用 `require_auth!()` 宏获取认证，但测试中无法注入 mock 认证上下文，导致所有 API 层测试无法运行
3. **前端无测试配置**：`package.json` 中没有任何前端测试框架（无 Vitest、Jest、Playwright 等）
4. **缺少测试覆盖率工具**：Rust 端未配置 `cargo-tarpaulin` 或 `cargo-llvm-cov`
5. **`create_test_db` 代码重复**：`api_tests.rs` 和 `integration_tests.rs` 中的建表代码完全重复，缺少共享测试工具模块
6. **Service 层和集成测试存在重叠**：部分测试逻辑在 `api_tests.rs`（Service 级）和 `integration_tests.rs`（集成级）中重复

### 2.3 覆盖缺口分析

| 模块 | 已覆盖 | 未覆盖 |
|------|--------|--------|
| **AuthService** | 注册、登录、无效登录、用户名可用性 | 登出、Token 刷新、改密码、邮箱可用性、重复注册、Token 验证--->用户不存在 |
| **HostService** | 创建、按ID获取、切换状态 | 更新、删除、搜索、分页查询、统计、导入导出、名称重复检查、权限隔离（普通用户 vs 管理员） |
| **HostGroupService** | 创建、按ID获取、列表查询 | 更新、删除、添加/移除主机、切换状态、统计、搜索 |
| **UserService** | 按ID获取、更新、管理员操作 | 删除用户、重置密码、搜索用户、自删保护、管理员状态切换 |
| **LogService** | 创建、查询 | 删除、批量删除、清理旧日志、导出、搜索、统计 |
| **SystemService** | 无 | 配置获取/更新、健康检查、日志清理、备份恢复 |
| **API 层** | get_system_info | 全部 Tauri Command 接口（需解决认证上下文注入问题） |
| **前端** | 无 | 全部组件、页面、Store、Router |

## 3. 测试策略

### 3.1 单元测试

#### 3.1.1 Rust 后端单元测试

**测试框架**：`#[test]` + `#[tokio::test]`（已有）

**重构建议**：
1. 将 `create_test_db()` 提取到 `src-tauri/src/db/mod.rs` 的 `#[cfg(test)] pub mod tests` 中，消除代码重复
2. 创建通用的测试 fixture 工厂（UserFixture、HostFixture 等）
3. 引入 `cargo-tarpaulin` 或 `cargo-llvm-cov` 进行覆盖率统计

**测试范围**：

| 目标模块 | 测试场景 | 优先级 |
|---------|---------|--------|
| `auth/token.rs` | Token 生成、验证、过期检测、撤销、清理过期撤销记录、刷新 Access Token | P0 |
| `auth/session.rs` | Session 创建、过期检测、活动更新、延长时间、移除会话、移除用户所有会话、清理过期会话、会话计数 | P0 |
| `auth/permissions.rs` | 四级角色权限分配正确性、`requires_user_resource_check`、`is_admin_only_permission` | P0 |
| `auth/mod.rs` | `AuthContext.has_permission`（各角色）、`can_access_user_resource`、`is_expired` | P0 |
| `db/services/auth_service.rs` | 注册（正常/重复邮箱/重复用户名/无用户名）、登录（正常/用户不存在/密码错误/记住我）、登出、Token 刷新、密码修改（正确旧密码/错误旧密码）、Token 验证、用户名可用性、邮箱可用性 | P0 |
| `db/services/host_service.rs` | 创建（正常/名称重复）、获取（ID存在/不存在/权限隔离）、更新（正常/名称冲突/不存在）、删除（正常/不存在/权限隔离）、切换状态、获取列表（分页/搜索/活跃过滤）、统计信息、导出、导入（正常/覆盖/不覆盖）、搜索 | P0 |
| `db/services/host_group_service.rs` | 创建、获取、更新、删除、添加主机到组（正常/重复添加）、从组移除主机、切换组状态、列表查询、统计、搜索 | P1 |
| `db/services/user_service.rs` | 获取用户（自己/他人/管理员访问）、列表（普通用户禁止/管理员允许）、更新（自己/他人/管理员修改角色）、删除（普通/自删保护/管理员）、重置密码、切换管理员状态（正常/自身保护）、统计、搜索 | P1 |
| `db/services/log_service.rs` | 创建日志、按ID获取、列表查询（分页/过滤）、删除、批量删除、清理旧日志、导出、搜索、统计、获取操作类型/目标类型 | P1 |
| `db/services/system_service.rs` | 获取系统配置、更新配置、健康检查、日志清理 | P2 |
| `api/public_type.rs` | `ApiResult` 各状态码构造、`PageData` 结构、`PageParams` 分页计算 | P2 |

#### 3.1.2 前端组件单元测试

**推荐框架**：Vitest + @vue/test-utils + happy-dom

**测试范围**：

| 目标组件/模块 | 测试场景 | 优先级 |
|-------------|---------|--------|
| `store/useUser.ts` | 初始状态、setMode、setLogin、setUserInfo（正常/空数据）、clearUserInfo | P0 |
| `store/useHosts.ts` | 初始状态、updateHosts（新增/已存在）、addGolblHosts（local/remote/已存在）、setContent、show/hide、持久化配置 | P0 |
| `store/useSettings.ts` | 初始状态、各设置项修改、持久化配置 | P1 |
| `store/useLocal.ts` | 初始状态、设置操作 | P2 |
| `router/index.ts` | 路由守卫逻辑（已登录跳转/未登录重定向/7天过期检测） | P0 |
| `apis/user.ts` | loginPlatform、logoutPlatform、refreshToken、changePassword、verifyToken、getCurrentUserInfo 的调用参数验证 | P1 |
| `apis/hosts.ts` | getAllHosts、updateHostsData 的调用参数验证 | P1 |
| `utils/bridge/` | Bridge 类实例化、useBridgeFunc 调用分发（Tauri vs uTools） | P1 |

### 3.2 集成测试

#### 3.2.1 后端集成测试（Service 层跨模块）

**已有基础**：`integration_tests.rs` 中已包含完整工作流测试，需扩展。

| 测试场景 | 描述 | 优先级 |
|---------|------|--------|
| 完整用户工作流 | 注册 -> 登录 -> Token 验证 -> Token 刷新 -> 修改密码 -> 重新登录 -> 登出 | P0 |
| 完整 Hosts 工作流 | 登录 -> 创建 Host -> 获取 -> 更新 -> 切换状态 -> 搜索 -> 列表 -> 导出 -> 删除 | P0 |
| 完整主机组工作流 | 登录 -> 创建 Host -> 创建 Group -> 添加 Host 到 Group -> 获取 Group 详情 -> 更新 Group -> 移除 Host -> 删除 Group | P0 |
| 权限隔离验证 | 普通用户 A 不能访问用户 B 的 Hosts/Groups；管理员可以访问所有资源 | P0 |
| 数据一致性 | 删除 Host 后关联的 HostGroupRelation 自动清理 | P0 |
| 并发操作安全 | 10 个并发创建 Host 请求全部成功且数据一致 | P1 |
| 日志审计链 | 所有 CRUD 操作后验证日志表中有正确的审计记录 | P1 |
| 级联删除 | 删除用户后，其所有 Hosts、HostGroups、Logs 是否正确处理 | P1 |
| Session 过期处理 | 使用已过期的 AuthContext 调用 Service，验证返回 SessionExpired 错误 | P0 |
| Token 吊销后拒绝 | 吊销 Token 后使用该 Token 验证身份，验证返回 InvalidToken 错误 | P0 |

#### 3.2.2 前后端通信集成测试

**推荐框架**：Tauri Test Utils（`tauri::test::mock_app`）

**当前痛点**：API 层的 `require_auth!()` 宏依赖全局认证状态，测试时无法注入认证上下文。

**解决方案建议**：
1. 重构 `require_auth!()` 宏，支持从 Tauri State 中获取认证上下文
2. 或引入中间件层，将认证逻辑从命令处理器中解耦
3. 短期方案：为每个 API command 创建无认证版本的内部方法（`_internal`），测试直接调用内部方法

| 测试场景 | 描述 | 优先级 |
|---------|------|--------|
| Tauri Command 注册验证 | 验证 `lib.rs` 中所有 55 个注册的命令可以正确加载 | P1 |
| AppState 初始化 | 验证 ServiceFactory 通过 Tauri State 正确注入 | P1 |
| 错误响应格式 | 验证各种错误（认证失败/权限不足/数据不存在/内部错误）返回正确的 ApiResult 格式和错误码 | P1 |

### 3.3 E2E 测试

**推荐框架**：WebdriverIO + Tauri Driver（官方推荐）或 Playwright + Tauri

**关键用户流程：**

| 流程编号 | 流程描述 | 步骤 | 优先级 |
|---------|---------|------|--------|
| E2E-01 | **首次使用流程** | 启动应用 -> 跳转登录页 -> 注册新用户 -> 登录成功 -> 进入主页 | P0 |
| E2E-02 | **登录登出流程** | 打开应用 -> 输入用户名密码 -> 登录 -> 验证主页显示 -> 登出 -> 验证跳转登录页 | P0 |
| E2E-03 | **GitHub OAuth 登录** | 点击 GitHub 登录按钮 -> 输入 GitHub 用户名 -> 验证信息获取 -> 登录成功 | P1 |
| E2E-04 | **Hosts 管理流程** | 登录 -> 查看 Hosts 列表 -> 选择一个 Host -> 在 Monaco 编辑器中编辑内容 -> 自动保存 -> 验证内容更新 | P0 |
| E2E-05 | **Hosts 启用/禁用** | 登录 -> 查看 Hosts 列表 -> 点击切换开关 -> 验证状态变化 -> 再次切换 -> 验证恢复 | P0 |
| E2E-06 | **管理后台用户管理** | 管理员登录 -> 进入管理后台 -> 查看用户列表 -> 搜索用户 -> 查看用户详情 | P1 |
| E2E-07 | **管理后台日志查看** | 管理员登录 -> 进入管理后台 -> 查看日志列表 -> 分页浏览 -> 查看日志详情 | P1 |
| E2E-08 | **设置修改** | 登录 -> 打开设置面板 -> 修改主题 -> 修改语言 -> 验证界面响应 | P2 |
| E2E-09 | **系统托盘交互** | 点击窗口关闭 -> 验证隐藏到托盘 -> 右键托盘 -> 点击"打开主页" -> 验证窗口恢复 | P1 |
| E2E-10 | **国际化切换** | 登录 -> 切换语言为英文 -> 验证 UI 文本变更 -> 切换为日文 -> 验证 UI 文本变更 -> 恢复中文 | P2 |
| E2E-11 | **Monaco 编辑器功能** | 登录 -> 打开编辑器 -> 输入 hosts 内容 -> 验证语法高亮 -> 防抖保存触发 | P1 |

### 3.4 安全测试

| 测试编号 | 测试场景 | 描述 | 优先级 |
|---------|---------|------|--------|
| SEC-01 | **密码存储安全** | 验证数据库中密码使用 bcrypt 哈希存储，不是明文 | P0 |
| SEC-02 | **SQL 注入防护** | 在搜索接口中注入 SQL 语句（`'; DROP TABLE users;--`），验证 Diesel ORM 参数化查询阻止注入 | P0 |
| SEC-03 | **Token 过期拒绝** | 使用过期 Token 访问受保护接口，验证返回 401 | P0 |
| SEC-04 | **Token 签名伪造** | 篡改 Token 中的 payload 但保留原签名，验证签名验证失败 | P0 |
| SEC-05 | **Token 吊销生效** | 调用 logout 后使用原 Token 访问接口，验证返回 401 | P0 |
| SEC-06 | **RBAC 权限越权** | 普通用户尝试访问管理员接口（如 `get_users`、`delete_user`），验证返回 403 | P0 |
| SEC-07 | **资源隔离** | 用户 A 尝试通过 ID 访问用户 B 的 Host/HostGroup，验证返回 403 | P0 |
| SEC-08 | **自删保护** | 管理员尝试删除自己的账户，验证返回错误 | P1 |
| SEC-09 | **密码强度** | 尝试使用空密码/极短密码注册，观察系统行为（当前无密码强度校验） | P1 |
| SEC-10 | **GitHub Token 硬编码** | 检查 `src/apis/user.ts` 第 10 行的硬编码 `ghp_Zxm3...` Token 是否已移除 | P0 |
| SEC-11 | **JWT Secret 安全** | 验证生产环境不使用默认 Secret（`default_secret_key_change_in_production`） | P0 |
| SEC-12 | **Token 签名算法** | 当前使用 `DefaultHasher` 签名，非标准 HMAC-SHA256，存在安全风险 | P0 |
| SEC-13 | **CSP 策略** | 检查 `tauri.conf.json` 中 `security.csp` 是否正确配置（当前为 null） | P1 |
| SEC-14 | **文件操作路径遍历** | 通过 `read_text_file` / `write_text_file` 传入 `../../etc/passwd` 等路径，验证是否有路径限制 | P0 |
| SEC-15 | **XSS 防护** | 在 Hosts name/description 中注入 `<script>alert(1)</script>`，验证前端渲染时转义 | P1 |
| SEC-16 | **并发登录控制** | 同一账号多处同时登录，验证 Session 管理策略 | P2 |

### 3.5 性能测试

**参考 PRD 非功能性需求指标（NFR-01 ~ NFR-06）：**

| 测试编号 | 测试场景 | 预期指标 | 测试方法 | 优先级 |
|---------|---------|---------|---------|--------|
| PERF-01 | 应用冷启动时间 | < 2 秒 | 使用系统计时器测量从进程启动到窗口可交互的时间 | P1 |
| PERF-02 | 应用热启动时间 | < 0.5 秒 | 从托盘恢复窗口的时间 | P2 |
| PERF-03 | 常驻内存占用 | < 80MB | 启动后空闲 30 秒，通过 Activity Monitor / Task Manager 读取 RSS | P1 |
| PERF-04 | 数据库查询响应 | < 50ms | 对 Service 层方法进行基准测试，包含 1000 条记录的表查询 | P1 |
| PERF-05 | Hosts 切换响应 | < 500ms | 从点击切换按钮到 UI 状态更新完成 | P1 |
| PERF-06 | Monaco Editor 首次加载 | < 1 秒 | 从页面进入编辑视图到编辑器可输入的时间 | P2 |
| PERF-07 | 安装包大小 | macOS < 15MB, Windows < 10MB | 构建后检查产物体积 | P2 |
| PERF-08 | 大量数据分页性能 | 10000 条 Hosts 记录下列表加载 < 200ms | 批量插入测试数据后测试分页查询性能 | P2 |
| PERF-09 | 并发写入性能 | 100 并发写入无数据丢失 | 使用 tokio 并发测试框架 | P2 |
| PERF-10 | 内存泄漏检测 | 连续操作 1 小时内存增长 < 10MB | 长时间运行后对比内存快照 | P3 |

### 3.6 兼容性测试

| 测试编号 | 平台 | 测试项 | 优先级 |
|---------|------|--------|--------|
| COMPAT-01 | macOS (Intel) | 应用安装、启动、全流程功能验证 | P0 |
| COMPAT-02 | macOS (Apple Silicon) | 应用安装、启动、全流程功能验证 | P0 |
| COMPAT-03 | Windows 10 x64 | 应用安装、启动、全流程功能验证 | P0 |
| COMPAT-04 | Windows 11 x64 | 应用安装、启动、全流程功能验证 | P0 |
| COMPAT-05 | Ubuntu 22.04 x64 | 应用安装、启动、全流程功能验证 | P1 |
| COMPAT-06 | 单实例控制 | 各平台下启动第二个实例时验证被阻止 | P1 |
| COMPAT-07 | 系统托盘 | 各平台下托盘图标显示、菜单交互 | P1 |
| COMPAT-08 | 窗口管理 | 各平台下无边框窗口、透明窗口表现 | P2 |
| COMPAT-09 | 数据库路径 | 各平台下 SQLite 数据库文件路径正确性 | P1 |
| COMPAT-10 | 文件操作 | 各平台下文件读写路径分隔符处理 | P2 |

## 4. 测试用例清单

### 4.1 用户认证模块

#### 4.1.1 用户注册

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| AUTH-REG-001 | 正常注册 | 数据库为空 | 提供合法用户名、邮箱、密码注册 | 返回用户对象，is_admin=false | P0 |
| AUTH-REG-002 | 不提供用户名注册 | 数据库为空 | 仅提供邮箱和密码注册 | 用户名默认使用邮箱前缀 | P0 |
| AUTH-REG-003 | 重复邮箱注册 | 已有用户 a@test.com | 使用相同邮箱注册 | 返回"邮箱已存在"错误 | P0 |
| AUTH-REG-004 | 重复用户名注册 | 已有用户名 test_user | 使用相同用户名注册 | 返回"用户名已存在"错误 | P0 |
| AUTH-REG-005 | 密码 bcrypt 存储 | 无 | 注册后检查数据库中密码字段 | 密码为 bcrypt 哈希值，非明文 | P0 |
| AUTH-REG-006 | 注册生成日志 | 无 | 注册后查询日志表 | 存在 action="register" 的日志记录 | P1 |

#### 4.1.2 用户登录

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| AUTH-LOGIN-001 | 正常登录 | 已注册用户 | 使用正确用户名和密码登录 | 返回 LoginResponse，包含 user、session、tokens | P0 |
| AUTH-LOGIN-002 | 错误密码 | 已注册用户 | 使用正确用户名和错误密码 | 返回 InvalidPassword 错误 | P0 |
| AUTH-LOGIN-003 | 用户不存在 | 无该用户 | 使用不存在的用户名登录 | 返回 UserNotFound 错误 | P0 |
| AUTH-LOGIN-004 | 记住我功能 | 已注册用户 | remember_me=true 登录 | Session 有效期为 7 天（168 小时） | P0 |
| AUTH-LOGIN-005 | 不记住我 | 已注册用户 | remember_me=false 登录 | Session 有效期为 24 小时 | P0 |
| AUTH-LOGIN-006 | Token 对生成 | 已注册用户 | 登录后验证 tokens | access_token 和 refresh_token 均非空且有效 | P0 |
| AUTH-LOGIN-007 | 登录日志记录 | 已注册用户 | 登录后查询日志 | 存在 action="login" 的日志记录 | P1 |

#### 4.1.3 Token 管理

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| AUTH-TOKEN-001 | Access Token 验证 | 已登录用户 | 使用 access_token 调用 verify_token | 返回正确的 AuthContext | P0 |
| AUTH-TOKEN-002 | Refresh Token 刷新 | 已登录用户 | 使用 refresh_token 调用 refresh_token | 返回新的 TokenPair | P0 |
| AUTH-TOKEN-003 | 过期 Token 拒绝 | 已过期 Token | 使用过期 Token 调用 verify_token | 返回 TokenExpired 错误 | P0 |
| AUTH-TOKEN-004 | 篡改 Token 拒绝 | 有效 Token | 修改 Token 的 payload 部分 | 返回 InvalidToken 错误 | P0 |
| AUTH-TOKEN-005 | 撤销 Token 拒绝 | 已撤销 Token | 撤销后使用该 Token | 返回 InvalidToken 错误 | P0 |
| AUTH-TOKEN-006 | 清理过期撤销记录 | 有撤销记录 | 调用 cleanup_revoked_tokens | 过期撤销记录被清理，返回清理数量 | P2 |

#### 4.1.4 登出与密码修改

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| AUTH-LOGOUT-001 | 正常登出 | 已登录用户 | 调用 logout | Session 被移除，日志记录登出 | P0 |
| AUTH-PWD-001 | 正确修改密码 | 已登录用户 | 提供正确旧密码和新密码 | 密码更新成功，所有 Session 被清除 | P0 |
| AUTH-PWD-002 | 错误旧密码 | 已登录用户 | 提供错误旧密码 | 返回 InvalidPassword 错误 | P0 |
| AUTH-CHECK-001 | 用户名可用性检查 | 已有用户 test_user | 检查 "test_user" | 返回 false（不可用） | P1 |
| AUTH-CHECK-002 | 用户名可用性检查 | 无 | 检查 "new_user" | 返回 true（可用） | P1 |
| AUTH-CHECK-003 | 邮箱可用性检查 | 已有邮箱 test@test.com | 检查该邮箱 | 返回 false（不可用） | P1 |

### 4.2 Hosts 管理模块

#### 4.2.1 Hosts CRUD

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| HOST-CREATE-001 | 创建 Host（默认禁用） | 已登录用户 | 创建 Host，不指定 is_active | Host 创建成功，is_active=0 | P0 |
| HOST-CREATE-002 | 创建 Host（指定启用） | 已登录用户 | 创建 Host，is_active=true | Host 创建成功，is_active=1 | P0 |
| HOST-CREATE-003 | 创建同名 Host | 已有同名 Host | 创建同名 Host | 返回"主机名称已存在"错误 | P0 |
| HOST-CREATE-004 | 创建 Host 权限检查 | 已登录用户 | 创建 Host | user_id 等于当前用户 ID | P0 |
| HOST-CREATE-005 | 创建 Host 日志记录 | 已登录用户 | 创建 Host 后查询日志 | 存在 action="create_host" 日志 | P1 |
| HOST-GET-001 | 按 ID 获取（自己的） | 已有 Host | 获取自己的 Host | 返回正确的 Host 对象 | P0 |
| HOST-GET-002 | 按 ID 获取（他人的，普通用户） | 他人有 Host | 普通用户获取他人 Host | 返回 PermissionDenied 错误 | P0 |
| HOST-GET-003 | 按 ID 获取（他人的，管理员） | 他人有 Host | 管理员获取他人 Host | 返回成功 | P0 |
| HOST-GET-004 | 按 ID 获取不存在 | 无 | 获取不存在的 ID | 返回"主机不存在"错误 | P0 |
| HOST-UPDATE-001 | 更新 Host 名称 | 已有 Host | 修改 name 字段 | 更新成功，updated_at 变更 | P0 |
| HOST-UPDATE-002 | 更新 Host 内容 | 已有 Host | 修改 content 字段 | 更新成功 | P0 |
| HOST-UPDATE-003 | 更新为已存在名称 | 已有两个 Host | 将 Host A 名称改为 Host B 的名称 | 返回"主机名已存在"错误 | P0 |
| HOST-UPDATE-004 | 更新他人 Host（普通用户） | 他人有 Host | 普通用户更新他人 Host | 返回 PermissionDenied | P0 |
| HOST-DELETE-001 | 删除自己的 Host | 已有 Host | 删除 Host | 删除成功，查询返回错误 | P0 |
| HOST-DELETE-002 | 删除他人 Host（普通用户） | 他人有 Host | 普通用户删除他人 Host | 返回 PermissionDenied | P0 |
| HOST-DELETE-003 | 删除不存在的 Host | 无 | 删除不存在的 ID | 返回"主机不存在"错误 | P1 |

#### 4.2.2 Hosts 状态与查询

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| HOST-TOGGLE-001 | 切换状态（禁用->启用） | Host is_active=0 | 调用 toggle_host_active | is_active 变为 1 | P0 |
| HOST-TOGGLE-002 | 切换状态（启用->禁用） | Host is_active=1 | 调用 toggle_host_active | is_active 变为 0 | P0 |
| HOST-LIST-001 | 分页查询（第1页） | 20 个 Hosts | 查询 page=1, page_size=10 | 返回 10 条记录，total=20 | P0 |
| HOST-LIST-002 | 分页查询（超出范围） | 5 个 Hosts | 查询 page=2, page_size=10 | 返回 0 条记录，total=5 | P1 |
| HOST-LIST-003 | 按关键词搜索 | 多个 Hosts | 搜索 name/description/content 中的关键词 | 仅返回匹配的 Hosts | P1 |
| HOST-LIST-004 | 按活跃状态过滤 | 混合状态 Hosts | active_only=true | 仅返回 is_active=1 的 Hosts | P1 |
| HOST-LIST-005 | 普通用户仅看到自己的 | 多用户多 Hosts | 普通用户查询列表 | 仅返回当前用户的 Hosts | P0 |
| HOST-ACTIVE-001 | 获取活跃主机列表 | 混合状态 | 调用 get_active_hosts | 仅返回活跃 Hosts | P1 |
| HOST-STATS-001 | 获取统计信息 | 多个 Hosts | 调用 get_host_stats | total/active/inactive/user_hosts 计数正确 | P1 |

#### 4.2.3 Hosts 导入导出

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| HOST-EXPORT-001 | 导出所有 Hosts | 已有多个 Hosts | 调用 export_hosts | 返回 HostExportData，包含所有 Hosts | P1 |
| HOST-IMPORT-001 | 导入新 Hosts | 无重复名 | 导入 3 个新 Host | 3 个 Host 全部创建成功 | P1 |
| HOST-IMPORT-002 | 导入覆盖已有 | 有同名 Host | overwrite=true 导入 | 已有 Host 被更新 | P1 |
| HOST-IMPORT-003 | 导入不覆盖跳过 | 有同名 Host | overwrite=false 导入 | 已有 Host 被跳过，仅新增新 Host | P1 |

### 4.3 主机组模块

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| GROUP-CREATE-001 | 创建主机组 | 已登录用户 | 提供 name 和 description 创建 | 创建成功，user_id 正确 | P1 |
| GROUP-GET-001 | 获取主机组详情 | 已有主机组 | 按 ID 获取 | 返回 HostGroupDetailResponse，含 hosts 列表 | P1 |
| GROUP-UPDATE-001 | 更新主机组 | 已有主机组 | 修改 name 和 description | 更新成功 | P1 |
| GROUP-DELETE-001 | 删除主机组 | 已有主机组 | 删除主机组 | 删除成功，查询返回错误 | P1 |
| GROUP-ADD-001 | 添加主机到组 | 已有组和 Host | 调用 add_host_to_group | 成功，获取组详情可见该 Host | P1 |
| GROUP-ADD-002 | 重复添加主机 | Host 已在组中 | 再次添加相同 Host | 返回唯一性约束错误 | P1 |
| GROUP-REMOVE-001 | 从组移除主机 | Host 在组中 | 调用 remove_host_from_group | 成功，组的 hosts 列表为空 | P1 |
| GROUP-TOGGLE-001 | 切换组状态 | 已有主机组 | 调用 toggle_host_group_active | 状态翻转 | P1 |
| GROUP-STATS-001 | 获取组统计 | 多个组 | 调用 get_host_group_stats | 统计数据正确 | P2 |
| GROUP-SEARCH-001 | 搜索主机组 | 多个组 | 按关键词搜索 | 仅返回匹配的组 | P2 |
| GROUP-CASCADE-001 | 删除 Host 后组关系清理 | Host 在组中 | 删除 Host | 组详情中 hosts 列表不再包含该 Host | P1 |

### 4.4 日志模块

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| LOG-CREATE-001 | 创建日志 | 已登录用户 | 手动创建日志记录 | 日志创建成功，字段正确 | P1 |
| LOG-GET-001 | 按 ID 获取日志 | 已有日志 | 按 ID 获取 | 返回 LogWithUser，含用户名 | P1 |
| LOG-LIST-001 | 分页查询日志 | 多条日志 | 分页查询 | 分页数据和总数正确 | P1 |
| LOG-LIST-002 | 按用户过滤日志 | 多用户日志 | 指定 user_id 查询 | 仅返回该用户日志 | P1 |
| LOG-LIST-003 | 按操作类型过滤 | 多类型日志 | 指定 action 查询 | 仅返回该操作类型日志 | P1 |
| LOG-DELETE-001 | 删除单条日志 | 已有日志 | 删除指定日志 | 删除成功 | P1 |
| LOG-BATCH-001 | 批量删除日志 | 多条日志 | 传入多个 ID 批量删除 | 返回删除的数量 | P1 |
| LOG-CLEAN-001 | 清理旧日志 | 有旧日志 | 指定天数清理 | 超过指定天数的日志被删除 | P2 |
| LOG-EXPORT-001 | 导出日志 | 有日志 | 调用 export_logs | 返回 LogExportData | P2 |
| LOG-STATS-001 | 获取日志统计 | 有日志 | 调用 get_log_stats | total_logs 和 today_logs 正确 | P2 |
| LOG-SEARCH-001 | 搜索日志 | 多条日志 | 按关键词搜索 | 匹配操作内容的日志被返回 | P1 |
| LOG-TYPES-001 | 获取操作类型列表 | 有多类日志 | 调用 get_operation_types | 返回不重复的操作类型 | P2 |
| LOG-TYPES-002 | 获取目标类型列表 | 有多类日志 | 调用 get_target_types | 返回不重复的目标类型 | P2 |

### 4.5 系统管理模块

| 用例编号 | 测试场景 | 前置条件 | 测试步骤 | 预期结果 | 优先级 |
|---------|---------|---------|---------|---------|--------|
| SYS-INFO-001 | 获取系统信息 | 无 | 调用 get_system_info | 返回 OS、内核、内存、CPU 信息，字段非空 | P1 |
| SYS-CONFIG-001 | 获取系统配置 | 管理员登录 | 调用 get_system_config | 返回配置 HashMap | P2 |
| SYS-CONFIG-002 | 更新系统配置 | 管理员登录 | 更新一个配置项 | 配置更新成功 | P2 |
| SYS-HEALTH-001 | 健康检查 | 无 | 调用 health_check | 返回 true | P2 |
| SYS-CLEAN-001 | 清理日志 | 管理员登录 | 指定天数清理 | 返回清理数量 | P2 |
| SYS-BACKUP-001 | 创建备份 | 管理员登录 | 调用 create_backup | 返回成功消息（当前桩实现） | P3 |
| SYS-FILE-001 | 读取文本文件 | 已认证 | 读取存在的文本文件 | 返回文件内容 | P2 |
| SYS-FILE-002 | 读取不存在文件 | 已认证 | 读取不存在的路径 | 返回错误消息 | P2 |
| SYS-FILE-003 | 写入文本文件 | 已认证 | 写入内容到文件 | 文件内容正确 | P2 |
| SYS-FILE-004 | 检查文件存在 | 已认证 | 检查存在/不存在的文件 | 返回正确的布尔值 | P2 |
| SYS-FILE-005 | 列出目录内容 | 有目录 | 调用 list_directory | 返回正确的条目列表 | P2 |

### 4.6 前端 UI 模块

#### 4.6.1 登录页面（`pages/login/`）

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-LOGIN-001 | 登录页面渲染 | 页面包含用户名输入框、密码输入框、登录按钮 | P0 |
| UI-LOGIN-002 | 空字段提交 | 显示表单验证错误提示 | P0 |
| UI-LOGIN-003 | 登录成功跳转 | 跳转到主页（home） | P0 |
| UI-LOGIN-004 | 登录失败提示 | 显示错误消息 | P0 |
| UI-LOGIN-005 | GitHub 登录入口 | 点击 GitHub 图标触发 OAuth 流程 | P1 |
| UI-LOGIN-006 | 注册页面切换 | 点击注册链接切换到注册表单 | P1 |

#### 4.6.2 注册页面（`pages/login/register.vue`）

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-REG-001 | 注册页面渲染 | 包含邮箱、用户名（可选）、密码输入框 | P0 |
| UI-REG-002 | 注册成功 | 跳转到登录页面 | P0 |
| UI-REG-003 | 重复邮箱提示 | 显示"邮箱已存在"错误 | P1 |

#### 4.6.3 Hosts 列表页面（`pages/hosts-list/index.vue`）

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-HOSTS-001 | 列表渲染 | 显示 Hosts 列表，包含名称和状态开关 | P0 |
| UI-HOSTS-002 | 状态切换交互 | 点击开关后状态翻转 | P0 |
| UI-HOSTS-003 | 选中编辑 | 点击 Host 项后编辑器显示其内容 | P0 |
| UI-HOSTS-004 | Monaco 编辑器加载 | 编辑器正确加载并显示 hosts 内容 | P0 |
| UI-HOSTS-005 | 内容编辑防抖保存 | 编辑内容后等待防抖时间，触发自动保存 | P1 |
| UI-HOSTS-006 | 添加新 Hosts 按钮 | 点击后显示"功能开发中..."提示（当前状态） | P2 |

#### 4.6.4 管理后台（`pages/admin/`）

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-ADMIN-001 | 用户列表渲染 | 显示用户数据表格，含用户名、邮箱、角色 | P1 |
| UI-ADMIN-002 | 用户搜索 | 输入关键词后列表过滤 | P1 |
| UI-ADMIN-003 | 日志列表渲染 | 显示日志列表，含操作者、操作类型、时间 | P1 |
| UI-ADMIN-004 | 日志分页 | 点击分页导航后数据更新 | P1 |

#### 4.6.5 设置面板（`components/settings/`）

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-SET-001 | 主题切换 | 切换深色/浅色/跟随系统，界面响应变化 | P2 |
| UI-SET-002 | 语言切换 | 切换中/英/日，所有 UI 文本更新 | P2 |
| UI-SET-003 | 设置持久化 | 修改设置后刷新页面，设置保持不变 | P2 |

#### 4.6.6 路由守卫

| 用例编号 | 测试场景 | 预期结果 | 优先级 |
|---------|---------|---------|--------|
| UI-ROUTE-001 | 未登录访问主页 | 重定向到登录页 | P0 |
| UI-ROUTE-002 | 已登录直接访问 | 正常进入目标页面 | P0 |
| UI-ROUTE-003 | 登录过期（7天） | 本地存储的登录时间超过 7 天后，重定向到登录页 | P0 |
| UI-ROUTE-004 | 访问 404 路径 | 显示 404 页面 | P2 |

## 5. 测试工具与框架

### 5.1 后端测试工具

| 工具 | 用途 | 当前状态 |
|------|------|---------|
| `#[test]` / `#[tokio::test]` | Rust 原生测试框架 | 已使用 |
| `tempfile` | 测试临时目录 | 已引入 |
| `diesel` + `:memory:` | 内存 SQLite 测试数据库 | 已使用 |
| `cargo-tarpaulin` | 测试覆盖率统计 | **建议引入** |
| `criterion` | 性能基准测试 | **建议引入**（用于 PERF 测试） |
| `proptest` | 属性测试/模糊测试 | **可选引入**（用于边界值测试） |
| `mockall` | Mock 框架 | `Cargo.toml` 已声明 dev-dependency，未使用 |

### 5.2 前端测试工具

| 工具 | 用途 | 当前状态 |
|------|------|---------|
| `vitest` | 单元/组件测试运行器 | **需引入** |
| `@vue/test-utils` | Vue 组件测试工具 | **需引入** |
| `happy-dom` / `jsdom` | DOM 模拟环境 | **需引入** |
| `@pinia/testing` | Pinia Store 测试工具 | **需引入** |
| `msw`（Mock Service Worker） | API 请求模拟 | **可选引入** |

### 5.3 E2E 测试工具

| 工具 | 用途 | 当前状态 |
|------|------|---------|
| `WebdriverIO` + `tauri-driver` | Tauri 官方推荐 E2E 方案 | **需引入** |
| 或 `Playwright` | 替代 E2E 方案 | **备选** |

### 5.4 CI/CD 工具

| 工具 | 用途 | 当前状态 |
|------|------|---------|
| GitHub Actions | CI 流水线 | **需配置** |
| `cargo test` | Rust 测试执行 | 已可用 |
| `pnpm lint:eslint` | 前端 lint 检查 | 已可用 |

### 5.5 建议的 package.json 脚本补充

```json
{
  "scripts": {
    "test": "vitest",
    "test:coverage": "vitest --coverage",
    "test:ui": "vitest --ui",
    "test:rust": "cd src-tauri && cargo test",
    "test:rust:coverage": "cd src-tauri && cargo tarpaulin --out html",
    "test:e2e": "wdio run wdio.conf.ts",
    "test:all": "pnpm test && pnpm test:rust"
  }
}
```

## 6. 优先级与排期

### 6.1 Phase 1 -- 基础设施搭建（预计 3 天）

| 任务 | 描述 | 预计工时 |
|------|------|---------|
| 重构测试工具模块 | 将 `create_test_db()` 提取为共享模块，消除重复代码 | 0.5d |
| 创建通用 Fixture 工厂 | UserFixture、HostFixture、GroupFixture、AuthFixture | 0.5d |
| 前端测试环境搭建 | 引入 Vitest + @vue/test-utils + happy-dom + @pinia/testing | 0.5d |
| Rust 覆盖率工具 | 配置 cargo-tarpaulin，设定基准覆盖率目标 | 0.5d |
| CI 配置 | 创建 GitHub Actions workflow（cargo test + pnpm test） | 1d |

### 6.2 Phase 2 -- P0 核心测试（预计 5 天）

| 任务 | 描述 | 预计工时 |
|------|------|---------|
| 认证服务完整测试 | AuthService 所有方法的正常/异常路径测试 | 1d |
| Host 服务完整测试 | HostService CRUD + 权限隔离 + 状态切换 | 1d |
| 安全测试（高优先级） | SEC-01~SEC-07, SEC-10~SEC-12, SEC-14 | 1d |
| 集成测试扩展 | 完善已有集成测试，增加权限隔离和错误路径 | 1d |
| 前端 Store 测试 | useUser、useHosts、useSettings 的单元测试 | 1d |

### 6.3 Phase 3 -- P1 重要测试（预计 5 天）

| 任务 | 描述 | 预计工时 |
|------|------|---------|
| HostGroup 服务完整测试 | HostGroupService 全方法测试 + 级联删除 | 1d |
| User 服务完整测试 | UserService 全方法测试 + 管理员权限 | 0.5d |
| Log 服务完整测试 | LogService 全方法测试 | 0.5d |
| 前端路由守卫测试 | 登录检测、过期跳转、页面权限 | 0.5d |
| 性能基准测试 | PERF-01~PERF-05 初始基准线建立 | 1d |
| 跨平台兼容性验证 | macOS + Windows 基本功能验证 | 1.5d |

### 6.4 Phase 4 -- P2/P3 补充测试（预计 4 天）

| 任务 | 描述 | 预计工时 |
|------|------|---------|
| System 服务测试 | SystemService 全方法测试 | 0.5d |
| 前端 UI 组件测试 | 登录/注册页面、设置面板、管理后台 | 1.5d |
| E2E 测试框架搭建 | WebdriverIO 或 Playwright 配置 + 3 个核心流程 | 1.5d |
| 安全测试（低优先级） | SEC-08, SEC-09, SEC-13, SEC-15, SEC-16 | 0.5d |

### 6.5 覆盖率目标

| 模块 | Phase 2 后 | Phase 3 后 | Phase 4 后 |
|------|-----------|-----------|-----------|
| auth/ | 80% | 90% | 95% |
| db/services/ | 60% | 80% | 90% |
| api/ | 20% | 50% | 70% |
| 前端 store/ | 0% | 60% | 80% |
| 前端 pages/ | 0% | 20% | 40% |
| **总体** | **40%** | **65%** | **80%** |

## 7. 风险评估

### 7.1 高风险项

| 风险 | 影响 | 当前状态 | 缓解措施 |
|------|------|---------|---------|
| **API 层测试无法注入认证上下文** | 所有 Tauri Command 无法进行自动化测试，测试覆盖存在死角 | `require_auth!()` 宏依赖全局状态 | 重构为可注入的认证中间件，或为测试创建 bypass 机制 |
| **JWT 签名使用 DefaultHasher** | 非标准签名算法，可能被碰撞攻击。`token.rs` 第 230-239 行使用 Rust 默认哈希而非 HMAC-SHA256 | 代码注释也标注"简化实现" | 迁移到 `jsonwebtoken` crate，使用 HS256/RS256 |
| **GitHub Token 硬编码** | `src/apis/user.ts` 第 10 行硬编码 GitHub Personal Access Token，存在泄露风险 | 已在 PRD 中标记为 WIP-14 | 移除硬编码，改为后端代理 OAuth 流程 |
| **文件操作无路径限制** | `commands.rs` 中的 `read_text_file`/`write_text_file` 等接口未限制可访问路径，可能导致任意文件读写 | 仅通过 `require_auth!()` 保护 | 添加路径白名单或沙盒限制 |
| **CSP 安全策略为空** | `tauri.conf.json` 中 `security.csp: null`，无 Content Security Policy 保护 | 未配置 | 配置适当的 CSP 策略 |

### 7.2 中风险项

| 风险 | 影响 | 当前状态 | 缓解措施 |
|------|------|---------|---------|
| **前端无任何测试** | 前端代码变更无回归保障 | package.json 无测试框架 | Phase 1 搭建前端测试环境 |
| **Hosts 未写入系统文件** | 核心功能缺失（WIP-01），无法验证端到端流程 | 后端 CRUD 仅操作数据库 | 后续实现后需重点测试权限提升和系统文件操作 |
| **密码无强度校验** | 用户可使用极弱密码 | 仅进行 bcrypt 哈希 | 添加密码强度验证规则 |
| **Session 存储在内存中** | 应用重启后所有 Session 丢失，用户需重新登录 | `SessionManager` 使用 `HashMap` | 评估是否需要持久化 Session |
| **备份恢复为桩实现** | 无法测试真实备份恢复流程 | `system.rs` 返回固定字符串 | 标记为 Known Limitation |

### 7.3 低风险项

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| uTools 模式大量未实现 | 影响 uTools 端使用体验 | Bridge 架构已就位，实现后逐步测试 |
| 编辑页面使用硬编码数据 | 编辑功能不可用 | 待对接后端 API |
| 用户管理编辑/删除为桩实现 | 管理操作不可用 | 待对接后端 API |
| 深色模式样式未完善 | 部分组件显示异常 | 视觉回归测试 |

---

*本测试计划基于 SwitchHostsR 仓库（截至 2026-03-29，commit `2b85152`）的完整代码分析编写。所有测试用例基于实际代码路径设计，覆盖已实现功能的正常路径和异常路径。*
