# 审查修复日志

> 基于审查报告 `feature-product-docs-and-roadmap.md` 的修复记录
> 修复日期：2026-03-29

## Blocker 修复

### B-01: JWT 密钥硬编码 fallback

- **问题**：`get_token_manager()` 在未设置 `JWT_SECRET` 环境变量时，fallback 到硬编码字符串 `"default_secret_key_change_in_production"`，攻击者可利用已知密钥伪造任意 JWT。
- **修复方式**：移除硬编码 fallback，改为使用 `rand` crate 生成 32 字节随机密钥（base64 编码）。桌面应用场景下，每次启动生成新密钥，重启后旧 token 自动失效（用户需重新登录），这是可接受的行为。
- **涉及文件**：
  - `src-tauri/src/auth/token.rs`：新增 `generate_random_secret()` 函数，修改 `get_token_manager()` 逻辑，新增 `use base64::Engine` 和 `use rand::Rng` 导入

### B-02: 文件操作 API 路径遍历漏洞

- **问题**：`read_text_file`、`write_text_file`、`delete_file`、`create_directory`、`list_directory` 等命令接受任意路径参数，认证用户可读写/删除系统任意文件。
- **修复方式**：
  1. 新增路径验证函数 `validate_safe_path()`：拒绝包含 `..` 的路径、拒绝绝对路径，将相对路径解析到应用数据目录 (`~/.switchhostsr/`) 下，使用 `canonicalize()` 验证路径不会逃逸。
  2. 新增 `validate_read_path()`：在标准验证基础上，额外允许系统 hosts 文件（`/etc/hosts` 或 Windows 等效路径）的只读访问。
  3. 所有文件操作命令均经过路径验证后才执行。
  4. 新增 5 个单元测试验证路径安全性。
- **涉及文件**：
  - `src-tauri/src/api/commands.rs`：全面重写，新增路径验证逻辑
  - `src-tauri/Cargo.toml`：新增 `dirs = "5.0"` 依赖（获取用户主目录）

## Warning 修复

### W-01: `create_user` 缺少认证检查

- **问题**：`create_user` 命令没有 `require_auth!()` 认证检查，任何未登录用户都可以调用。
- **修复方式**：添加 `require_auth!()` 认证检查。此命令定位为管理员后台创建用户操作，开放注册应使用独立的注册接口。
- **涉及文件**：`src-tauri/src/api/users.rs`

### W-02: `refresh_access_token` 不区分 token 类型

- **问题**：`refresh_access_token` 方法没有校验令牌类型，使用有效的 access token 也可以刷新获取新 token，绕过过期机制。
- **修复方式**：
  1. 在 `Claims` 结构体中新增 `token_type` 字段（`"access"` 或 `"refresh"`）。
  2. `generate_token()` 时根据 `TokenType` 枚举设置 `token_type` 字段。
  3. `refresh_access_token()` 中校验 `claims.token_type == "refresh"`，拒绝非 refresh 类型的令牌。
  4. `AuthService::refresh_token()` 中同步添加类型校验。
  5. 新增 2 个单元测试验证 token 类型校验逻辑。
- **涉及文件**：
  - `src-tauri/src/auth/token.rs`：修改 Claims 结构体、generate_token、refresh_access_token
  - `src-tauri/src/db/services/auth_service.rs`：修改 refresh_token 方法

### W-03: `unsafe impl Send/Sync` 不必要

- **问题**：`ui/app_state.rs` 中 `AppState` 使用 `unsafe impl Send` 和 `unsafe impl Sync`，但所有字段均为 `Mutex<T>` 类型，内部类型（`i32`、`DbPool` = `Arc<Pool<...>>`）均为 Send+Sync，不需要 unsafe 声明。
- **修复方式**：移除 `unsafe impl Send for AppState {}` 和 `unsafe impl Sync for AppState {}`，替换为安全性说明注释，让编译器自动推导 Send+Sync。编译通过证明类型确实满足约束。
- **涉及文件**：`src-tauri/src/ui/app_state.rs`

### W-04: 系统 hosts 写入 sudo 问题

- **问题**：使用 `sudo cp` 写入系统 hosts 文件，在 GUI 环境中 `sudo` 无法获得交互式密码输入，导致静默失败。
- **修复方式**：
  - macOS：改用 `osascript -e 'do shell script ... with administrator privileges'` 弹出系统级权限对话框。
  - Linux：优先使用 `pkexec`（GUI 友好的 PolicyKit 提权），失败时回退到 `sudo`。
  - 改进错误提示信息，明确告知用户需要授予管理员权限。
- **涉及文件**：`src-tauri/src/utils/system_hosts.rs`

### W-05: 临时文件名固定

- **问题**：临时文件路径为固定名称 `switchhostsr_hosts_temp`，并发场景下可能产生竞态条件。
- **修复方式**：使用 `uuid::Uuid::new_v4()` 为每次写入生成唯一临时文件名（格式：`switchhostsr_hosts_{uuid}`）。
- **涉及文件**：`src-tauri/src/utils/system_hosts.rs`

### W-06: 前端 User 接口重复定义

- **问题**：`admin/user.vue` 中重新定义了 `User` 接口，未复用统一类型，可能导致前后端字段不一致。
- **修复方式**：在 `src/apis/public.ts` 中新增统一的 `User` 接口定义并导出，`admin/user.vue` 改为从 `@/apis/public` 导入复用。
- **涉及文件**：
  - `src/apis/public.ts`：新增 `User` 接口
  - `src/pages/admin/user.vue`：移除本地 `User` 接口定义，改为导入

## 验证结果

- `cargo test`：80 个测试全部通过（含新增的 7 个安全相关测试）
- `pnpm lint:eslint`：0 个错误（仅有预存的 warning，非本次修复引入）
- `cargo build`：编译通过，无新增 warning
