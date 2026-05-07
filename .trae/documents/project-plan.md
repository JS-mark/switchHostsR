# Switch Hosts R 开发规划

> 最后更新：2026-05-07
> 当前版本：v0.0.1

---

## 一、项目概述

Switch Hosts R 是一个基于 Tauri 2.x + Vue 3 的跨平台 hosts 文件管理工具。

### 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.x |
| 前端框架 | Vue 3 + TypeScript |
| UI 组件库 | Naive UI |
| 后端语言 | Rust |
| 构建工具 | Vite 6.x |
| 数据库 | SQLite (Diesel ORM) |
| 文档工具 | Rspress |

---

## 二、功能实现状态

### 2.1 已完成功能 ✅

#### 前端页面

| 页面 | 文件路径 | 状态 | 说明 |
|------|----------|------|------|
| 登录/注册 | [src/pages/login](file:///workspace/src/pages/login) | ✅ 完成 | 支持邮箱注册、登录 |
| 第三方登录 | [src/pages/login/third-login.vue](file:///workspace/src/pages/login/third-login.vue) | ✅ 完成 | GitHub OAuth |
| 首页欢迎页 | [src/pages/home/welcome.vue](file:///workspace/src/pages/home/welcome.vue) | ✅ 完成 | 应用概览 |
| Hosts 列表页 | [src/pages/hosts-list/index.vue](file:///workspace/src/pages/hosts-list/index.vue) | ✅ 完成 | 列表展示、编辑器集成 |
| 编辑页 | [src/pages/edit/index.vue](file:///workspace/src/pages/edit/index.vue) | ✅ 完成 | 独立编辑器 |
| 用户管理 | [src/pages/admin/user.vue](file:///workspace/src/pages/admin/user.vue) | ✅ 完成 | 用户 CRUD |
| 日志管理 | [src/pages/admin/logs.vue](file:///workspace/src/pages/admin/logs.vue) | ✅ 完成 | 操作日志 |
| 关于页 | [src/pages/about/index.vue](file:///workspace/src/pages/about/index.vue) | ✅ 完成 | 版本信息 |
| 设置页 | [src/components/settings](file:///workspace/src/components/settings) | ✅ 完成 | 基础设置 |

#### 前端组件

| 组件 | 路径 | 状态 |
|------|------|------|
| Monaco Editor 集成 | [src/components/editor](file:///workspace/src/components/editor) | ✅ 完成 |
| 添加 Hosts 弹窗 | [src/components/add-hosts](file:///workspace/src/components/add-hosts) | ✅ 完成 |
| 全局模态框 | [src/components/global-model](file:///workspace/src/components/global-model) | ✅ 完成 |
| 布局组件 | [src/components/layout](file:///workspace/src/components/layout) | ✅ 完成 |
| SVG 图标组件 | [src/components/svg-icon](file:///workspace/src/components/svg-icon) | ✅ 完成 |

#### 后端模块

| 模块 | 路径 | 状态 | 说明 |
|------|------|------|------|
| 用户认证 | [src-tauri/src/auth](file:///workspace/src-tauri/src/auth) | ✅ 完成 | JWT + Session |
| 用户管理 API | [src-tauri/src/api/users.rs](file:///workspace/src-tauri/src/api/users.rs) | ✅ 完成 | CRUD + 权限 |
| Hosts 管理 API | [src-tauri/src/api/hosts.rs](file:///workspace/src-tauri/src/api/hosts.rs) | ✅ 完成 | CRUD + 导入导出 |
| 主机组 API | [src-tauri/src/api/host_groups.rs](file:///workspace/src-tauri/src/api/host_groups.rs) | ✅ 完成 | 分组管理 |
| 日志管理 API | [src-tauri/src/api/logs.rs](file:///workspace/src-tauri/src/api/logs.rs) | ✅ 完成 | 操作审计 |
| 系统配置 API | [src-tauri/src/api/system.rs](file:///workspace/src-tauri/src/api/system.rs) | ✅ 完成 | 系统设置 |
| 数据库服务层 | [src-tauri/src/db/services](file:///workspace/src-tauri/src/db/services) | ✅ 完成 | 业务逻辑层 |
| UI/托盘 | [src-tauri/src/ui](file:///workspace/src-tauri/src/ui) | ✅ 完成 | 窗口+系统托盘 |

#### 基础设施

| 模块 | 状态 | 说明 |
|------|------|------|
| 国际化 | ✅ 完成 | 中文、英文、日文 |
| Pinia 状态管理 | ✅ 完成 | 用户、Hosts、设置状态 |
| 路由系统 | ✅ 完成 | 权限控制 |
| API 请求封装 | ✅ 完成 | Alova |
| 文档站 | ✅ 完成 | Rspress 构建 |
| CI/CD | ✅ 完成 | GitHub Actions |

---

### 2.2 待完善功能 🔨

#### P0 - 高优先级

| 功能 | 页面/模块 | 现状 | 说明 |
|------|----------|------|------|
| 新建 Hosts | [hosts-list](file:///workspace/src/pages/hosts-list/index.vue#L101-L103) | ⚠️ 占位 | 点击提示"功能开发中" |
| 设置面板 | [hosts-list](file:///workspace/src/pages/hosts-list/index.vue#L105-L107) | ⚠️ 占位 | 点击提示"功能开发中" |
| Hosts 写入系统 | [src-tauri/src/db/services/system_service.rs](file:///workspace/src-tauri/src/db/services) | ⚠️ 待实现 | 将启用状态的 hosts 写入系统 |
| 远程 Hosts 同步 | [src/components/add-hosts](file:///workspace/src/components/add-hosts) | ⚠️ 部分 | UI 已完成，后端待集成 |

#### P1 - 中优先级

| 功能 | 说明 |
|------|------|
| 导入/导出功能 | 从文件导入 hosts，导出为文件 |
| 自动刷新远程 hosts | 根据配置的刷新周期自动更新 |
| 备份与恢复 | 数据库备份，系统 hosts 备份 |
| 快捷键支持 | 全局快捷键快速切换 |
| 深色模式 | 主题切换支持 |

#### P2 - 低优先级

| 功能 | 说明 |
|------|------|
| 用户个人资料管理 | 头像、密码修改 |
| 使用统计 | 活跃用户、配置使用频率 |
| 全局搜索 | 跨配置搜索 |
| 插件系统 | 企业版功能预留 |
| 浏览器扩展 | Chrome/Firefox 扩展 |

---

### 2.3 技术债务与优化项

| 类型 | 问题 | 优先级 |
|------|------|--------|
| 性能 | Monaco Editor 按需加载优化 | 中 |
| 安全 | 文件操作权限校验增强 | 高 |
| 代码 | 部分 Vue 组件使用 Options API，应迁移到 Composition API | 低 |
| 测试 | 后端单元测试覆盖不足 | 中 |
| 文档 | API 文档不完整 | 中 |
| 构建 | Windows/macOS/Linux 多平台构建验证 | 高 |

---

## 三、里程碑计划

### v0.1.0 - 基础可用版 (当前目标)

**目标**：实现核心 hosts 管理功能，能够正常使用

- [x] 用户登录注册
- [x] Hosts 列表展示
- [x] Monaco Editor 集成
- [ ] **新建/编辑 Hosts** (🔨 进行中)
- [ ] **系统 hosts 文件写入** (🔨 进行中)
- [ ] Hosts 启用/禁用
- [ ] 基础日志记录

### v0.2.0 - 功能完善版

**目标**：完善周边功能，提升用户体验

- [ ] 远程 hosts 支持
- [ ] 导入/导出功能
- [ ] 备份与恢复
- [ ] 设置页面完善
- [ ] 深色模式

### v0.3.0 - 稳定发布版

**目标**：修复问题，提升稳定性，准备正式发布

- [ ] 全平台构建测试
- [ ] 性能优化
- [ ] 安全性审计
- [ ] 文档完善

### v1.0.0 - 正式发布版

**目标**：首个正式版本发布

- [ ] 完整的 changelog
- [ ] 发布说明
- [ ] 多平台安装包

---

## 四、下一步行动计划

### 立即执行 (本周)

1. **实现新建 Hosts 功能**
   - 完善 [add-hosts.vue](file:///workspace/src/components/add-hosts/add-hosts.vue) 表单提交
   - 调用后端 `create_host` API
   - 添加到 hosts 列表

2. **实现系统 hosts 写入**
   - 在 Rust 后端实现 hosts 文件写入逻辑
   - 处理权限问题 (Linux/macOS 需要 sudo)
   - 添加启用/禁用切换逻辑

### 短期计划 (本月)

1. 完成远程 hosts 的完整支持
2. 实现导入/导出功能
3. 完善设置页面
4. 添加深色模式支持

### 中期计划 (下季度)

1. 全平台构建和测试
2. 性能优化
3. 测试覆盖
4. 文档完善

---

## 五、相关文档

- [产品需求文档](file:///workspace/.trae/documents/switch-hosts-r-product-document.md) - 产品功能设计
- [项目规范](file:///workspace/.trae/rules/project_rules.md) - 开发规范和约定
- [使用指南](file:///workspace/docs/guide) - 用户使用文档
- [API 文档](file:///workspace/docs/api) - 接口文档

---

## 六、版本历史

| 版本 | 日期 | 状态 |
|------|------|------|
| v0.0.1 | 2026-05-07 | 当前版本，基础框架搭建完成 |
