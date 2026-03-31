---
feature: product-docs-and-roadmap
role: reviewer
status: draft
depends_on:
  - docs/prd/feature-product-docs-and-roadmap.md
date: 2026-03-29
---

# product-docs-and-roadmap — 代码审查报告

## 1. 合规检查清单

| 检查项 | 状态 | 备注 |
|--------|------|------|
| 所有文件使用中文注释 | OK | Rust 和 Vue 文件均使用中文 |
| Vue 组件使用 `<script setup lang="ts">` | OK | 新增/修改的页面组件均已遵循（about、hosts-list、edit），`third-login.vue` 保留旧 Options API 未在本次变更范围 |
| Props/Emits 使用 TypeScript 类型声明 | OK | `add-hosts.vue` 的 emits 已声明类型 |
| 无硬编码敏感信息 | OK | `.env.tauri` 中 Token 已注释为占位符 |
| CSP 策略已配置 | OK | `tauri.conf.json` 已设置 |
| CI/CD 流水线存在 | OK | `.github/workflows/build.yml` 已配置多平台构建 |

## 2. 代码质量

### 2.1 Rust 后端

**优点：**
- JWT 实现从自制 base64 签名迁移到标准 `jsonwebtoken` 库，显著提升安全性
- `TokenManager` 使用 `OnceLock` 实现安全的全局单例
- `AuthState` 使用 `RwLock` 保证读写并发安全
- `system_hosts.rs` 使用标记区域（Marker）方案保护用户原有 hosts 内容
- 宏 `require_auth!` 和 `safe_execute!` 减少了样板代码
- 单元测试覆盖 token 生成/验证/撤销/跨密钥拒绝

**问题：**
- `commands.rs` 中的文件操作 API（`read_text_file`、`write_text_file`、`delete_file`、`create_directory`、`list_directory`）接受任意路径，存在路径遍历风险（见安全评估 #1）
- `ui/app_state.rs` 中有 `unsafe impl Send for AppState {}` 和 `unsafe impl Sync for AppState {}`（见安全评估 #4）
- 部分测试函数体被注释掉（`hosts.rs:331-338`、`users.rs:307-314`、`system.rs:139-141`），测试覆盖不完整

### 2.2 前端

**优点：**
- API 层类型定义清晰，`hosts.ts` 的接口与后端 Rust 模型一一对应
- `hosts-list/index.vue` 实现了搜索过滤、状态统计、创建/删除/切换操作的完整功能
- `edit/index.vue` 正确处理了只读/系统 hosts 的场景
- `add-hosts.vue` 表单验证完整，包含远程 URL 和刷新类型的条件校验
- 使用 `globalEventEmitter` 实现组件间通信（创建成功后刷新列表）

**问题：**
- `admin/user.vue` 中 `User` 接口重新定义而非复用后端类型
- `about/index.vue` 引用了 `__BUILD_DATE__` 全局变量但未在 `vite-env.d.ts` 中有对应的运行时注入（已在 `vite-config/base.config.ts:18` 通过 `define` 注入，但 `about/index.vue:10` 使用 `||` 而非 `??`，当 `__BUILD_DATE__` 为空字符串时行为不同）
- 多处 `any` 使用（`third-login.vue:64,76`、`login.vue:51`、`register.vue:49`），未在本次变更中修复

## 3. 安全评估

### 3.1 JWT 签名实现 — `src-tauri/src/auth/token.rs`

| 项 | 评估 |
|----|------|
| 签名算法 | HMAC-SHA256（jsonwebtoken 默认 Header），安全 |
| 密钥管理 | 从环境变量 `JWT_SECRET` 读取，有硬编码 fallback（第 217 行） |
| 过期验证 | `validation.validate_exp = true`，由 jsonwebtoken 自动校验 |
| 撤销机制 | 内存 HashMap + RwLock，进程重启后撤销列表丢失 |
| 刷新令牌 | 未区分 access/refresh token 类型（`refresh_access_token` 仅验证签名，不检查 token_type） |

### 3.2 全局 AuthState — `src-tauri/src/auth/middleware.rs`

- `AuthState` 内部使用 `std::sync::RwLock<AuthMiddleware>` 包裹，线程安全
- `OnceLock` 保证全局实例只初始化一次
- **注意：** 桌面应用为单用户场景，全局 AuthState 设计合理

### 3.3 系统 Hosts 文件写入 — `src-tauri/src/utils/system_hosts.rs`

- 写入路径硬编码为 `/etc/hosts` 或 `C:\Windows\System32\drivers\etc\hosts`，不受用户输入影响，无路径注入风险
- 使用临时文件 + `sudo cp` 方案，避免直接以 root 写入
- Marker 区域设计保护了用户手动编辑的 hosts 内容
- DNS 刷新覆盖 macOS/Windows/Linux 三平台，多策略回退

### 3.4 CSP 策略 — `src-tauri/tauri.conf.json`

```
default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https://api.github.com https://*.githubusercontent.com ipc: http://ipc.localhost
```

- `'unsafe-inline'` 和 `'unsafe-eval'` 存在于 `script-src`，这在 Tauri 环境中通常是必需的（Vue 运行时/Monaco Editor 需要），可接受
- `connect-src` 正确限制为 self + GitHub API
- `img-src` 允许 `https:` 宽泛加载，考虑到用户头像等场景，可接受

### 3.5 GitHub Token — `src/apis/user.ts`

- Token 从 `import.meta.env.VITE_GITHUB_TOKEN` 读取，不再硬编码
- `.env.tauri` 中该变量已注释为占位符，`.gitignore` 中有 `*.local` 规则

## 4. 性能评估

| 项 | 评估 |
|----|------|
| Token 撤销列表 | 内存 HashMap，查找 O(1)，进程内高效 |
| hosts 列表加载 | `getAllHosts()` 一次性加载全部，列表量不大时无问题；长远看应加分页 |
| 编辑器内容保存 | 使用 `debounce(300ms)`，合理防抖 |
| 系统 hosts 写入 | 同步写入+sudo cp，阻塞当前请求，但操作不频繁，可接受 |
| ServiceFactory | 使用 `Arc` 共享，连接池 r2d2 管理，合理 |

## 5. 测试覆盖率

| 模块 | 测试状态 | 备注 |
|------|----------|------|
| `auth/token.rs` | 4 个测试，全部实质性 | 覆盖生成、验证、撤销、跨密钥拒绝 |
| `auth/middleware.rs` | 4 个测试 | 覆盖中间件、权限、资源访问、状态管理 |
| `utils/system_hosts.rs` | 4 个测试 | 覆盖合并、替换、空列表、路径 |
| `api/hosts.rs` | 1 个测试（注释掉） | 缺乏实际断言 |
| `api/users.rs` | 1 个测试（注释掉） | 缺乏实际断言 |
| `api/mod.rs` | 4 个测试 | 覆盖成功/错误/认证/分页 |
| 前端 | 无单元测试 | 未配置 Vitest 等测试框架 |

## 6. 发现的问题

| 严重程度 | 文件 | 行号 | 问题描述 | 修复建议 |
|----------|------|------|----------|----------|
| 🔴 Blocker | `src-tauri/src/auth/token.rs` | 217 | JWT 密钥 fallback 为硬编码字符串 `"default_secret_key_change_in_production"`。如果生产环境未设置 `JWT_SECRET` 环境变量，所有用户的 token 都使用同一个已知密钥签名，攻击者可伪造任意身份的 JWT。 | 启动时检测 `JWT_SECRET` 是否设置，若未设置则 panic 或随机生成密钥并持久化到配置文件。至少应使用 `uuid::Uuid::new_v4().to_string()` 生成随机默认值。 |
| 🔴 Blocker | `src-tauri/src/api/commands.rs` | 67-100 | `read_text_file` 和 `write_text_file` 命令接受任意 `file_path` 参数，认证后的用户可以读取/写入系统上的任意文件（如 `/etc/passwd`、`~/.ssh/id_rsa`）。这是一个严重的路径遍历/任意文件读写漏洞。 | 对 `file_path` 进行严格校验：1) 限制到应用数据目录内；2) 使用 `canonicalize()` 检查路径不会逃逸出允许的目录；3) 或完全移除这些通用文件操作命令，用特定业务命令替代。 |
| 🔴 Blocker | `src-tauri/src/api/commands.rs` | 117-150 | `create_directory` 和 `delete_file` 同样接受任意路径参数。认证用户可以在系统任意位置创建目录或删除文件。 | 同上，限制文件操作到应用沙箱目录内。 |
| 🟡 Warning | `src-tauri/src/api/users.rs` | 212-226 | `create_user` 命令没有 `require_auth!()` 认证检查，任何未登录用户都可以调用注册接口。如果这是有意设计（开放注册），则可接受；但如果仅允许管理员创建用户，则需要加上权限检查。 | 明确注册策略：如需管理员才能创建，加入 `require_auth!()` + admin 权限校验；如开放注册，添加注释说明。 |
| 🟡 Warning | `src-tauri/src/auth/token.rs` | 200-207 | `refresh_access_token` 方法没有区分 access token 和 refresh token 类型。使用一个有效的 access token 也可以调用 refresh 获取新 token，绕过 token 过期机制。 | 在 Claims 中增加 `token_type` 字段，在 `refresh_access_token` 中校验必须是 `TokenType::Refresh`。 |
| 🟡 Warning | `src-tauri/src/ui/app_state.rs` | 32-33 | `unsafe impl Send for AppState {}` 和 `unsafe impl Sync for AppState {}`。`AppState` 所有字段都是 `Mutex<T>`，Mutex 本身已实现 Send+Sync，不应需要 unsafe impl。如果 handler 内部含有非 Send/Sync 类型（如裸指针），则此 unsafe 声明掩盖了潜在的线程安全问题。 | 移除 `unsafe impl`，让编译器自动推导。如果编译失败，排查 handler 内哪些类型不满足 Send/Sync，而非用 unsafe 掩盖。 |
| 🟡 Warning | `src-tauri/src/utils/system_hosts.rs` | 128-134 | 临时文件路径为固定名称 `switchhostsr_hosts_temp`，在多进程或并发调用场景下可能产生竞态条件。 | 使用 `tempfile` crate 或在文件名中加入 PID/时间戳确保唯一性。 |
| 🟡 Warning | `src-tauri/src/utils/system_hosts.rs` | 132 | 使用 `sudo cp` 写入系统 hosts 文件，在 GUI 环境中 `sudo` 可能无法获得交互式密码输入，导致静默失败。 | 考虑使用 `pkexec`（Linux）或 `osascript -e 'do shell script ... with administrator privileges'`（macOS）来弹出系统级权限请求对话框。或使用 `tauri-plugin-shell` 的 sidecar 方案。 |
| 🟡 Warning | `src/pages/admin/user.vue` | 27-35 | `User` 接口在组件内重新定义，字段与后端 `User` 模型不完全一致（如 `is_admin` 为 `boolean | null`，后端实际返回的是 `role` 字段为字符串）。可能导致字段映射不匹配。 | 在 `apis/user.ts` 中定义统一的 `User` 接口并导出，前端组件复用该类型。确保字段名和类型与后端 Rust 模型的序列化输出一致。 |
| 🟡 Warning | `src/pages/hosts-list/index.vue` | 344 | `editor` 组件通过 `v-model` 传入 `hosts.content`，但 `v-model` 在此是直接绑定到数组项的属性。当编辑器内容变化时直接修改了 `reactive` 对象，保存操作使用 `debounce(300ms)` 调用 `updateHost`。如果网络请求失败，本地数据已被修改但未回滚。 | 在 `onEditorChange` 的 catch 中将 `hosts.content` 恢复为请求前的值，或使用独立的 `editingContent` 变量。 |
| 🟡 Warning | `.github/workflows/build.yml` | 59 | `cargo test --lib` 仅运行 lib 测试，不运行集成测试。 | 如果有集成测试，考虑添加 `cargo test --all` 或至少说明仅运行 lib 测试的原因。 |
| 🟢 Info | `src-tauri/src/auth/token.rs` | 53 | 撤销令牌列表存储在内存中，进程重启后所有撤销记录丢失。被撤销的 token 在应用重启后重新生效。 | 对于桌面应用，影响较小。长远可考虑将撤销列表持久化到 SQLite。 |
| 🟢 Info | `src/pages/about/index.vue` | 10 | `buildDate` 使用 `\|\|` 而非 `??` 进行空值判断。当 `__BUILD_DATE__` 为空字符串时，`\|\|` 会取 `'未知'`，而 `??` 只在 `null/undefined` 时才取默认值。 | 建议改为 `const buildDate = ref(__BUILD_DATE__ ?? '未知')` 以更精确地表达意图。 |
| 🟢 Info | `src/pages/about/index.vue` | 9 | `appVersion` 硬编码为 `'0.0.1'`，未来应从 `tauri.conf.json` 或 Tauri API 动态获取。 | 使用 `@tauri-apps/api` 的 `getVersion()` 在 `onMounted` 中获取。 |
| 🟢 Info | `src/vite-env.d.ts` | 11 | `DefineComponent<object, object, any>` 中使用了 `any`。 | 这是 Vue 的通用模块声明惯例，影响有限，可保持。 |
| 🟢 Info | `src-tauri/src/api/hosts.rs` | 221 | `exported_at` 使用 `chrono::Utc::now().timestamp() as i32`，会在 2038 年溢出。 | 改为 `i64` 类型。 |
| 🟢 Info | `src/pages/hosts-list/index.vue` | 463-468 | `editor-header` 使用硬编码颜色值 `#fafafa`、`#e0e0e0`，违反项目规范「优先使用 Naive UI 主题变量，避免硬编码颜色值」。 | 改为使用 Naive UI 主题变量如 `var(--n-color)` 等，或 UnoCSS 工具类。 |

## 7. Blocker 汇总

| # | 文件 | 问题 |
|---|------|------|
| 1 | `src-tauri/src/auth/token.rs:217` | JWT 密钥 fallback 为已知硬编码字符串，生产环境如未设置环境变量将导致 token 可被伪造 |
| 2 | `src-tauri/src/api/commands.rs:67-150` | `read_text_file`/`write_text_file`/`delete_file`/`create_directory` 接受任意文件路径，存在任意文件读写/删除漏洞 |

## 8. 总结与建议

### 整体评价

本次变更质量较高，涵盖了从认证体系到功能实现的全栈改造。JWT 从自制签名迁移到标准库是关键的安全改进；系统 hosts 文件写入的标记区域设计考虑周全；前端组件结构清晰，API 类型定义规范。

### 必须修复（发布前）

1. **JWT 密钥 fallback**：这是最高优先级问题。建议在应用启动时检测环境变量，未设置则自动生成随机密钥并写入本地配置文件（如 `~/.switchhostsr/jwt_secret`）。
2. **通用文件操作命令的路径校验**：`commands.rs` 中的文件操作命令是严重的安全风险。建议限制到应用数据目录（`tauri::api::path::app_data_dir()`），或在当前阶段直接移除这些通用命令。

### 建议改进

3. 区分 access/refresh token 类型，防止 token 类型混用
4. 明确 `create_user` 的认证策略
5. 改进系统 hosts 写入的权限请求方式（GUI 友好的提权）
6. 统一前后端 User 类型定义
7. 补充被注释掉的 API 测试

### 架构建议

- 考虑在 `commands.rs` 的文件操作中引入白名单机制或路径沙箱
- `sudo` 调用在桌面 GUI 环境下的交互体验需要进一步评估和测试
- 长期来看，建议引入前端测试框架（Vitest）提升测试覆盖率
