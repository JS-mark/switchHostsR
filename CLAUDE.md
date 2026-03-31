# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SwitchHostsR 是一个跨平台桌面应用，用于管理系统 hosts 文件。基于 Tauri 2.x（Rust 原生层）+ Vue 3（前端 UI 层）构建，支持 macOS / Windows / Linux。

## Common Commands

```bash
# 安装依赖
pnpm install

# 前端开发（Tauri 模式，端口 1420）
pnpm dev:tauri

# 前端开发（uTools 模式）
pnpm dev:utool

# 构建生产包
pnpm build:tauri
pnpm build:utool

# Lint
pnpm lint:eslint
pnpm lint:stylelint

# Rust 后端测试（在 src-tauri/ 目录下执行）
cd src-tauri && cargo test                                    # 全部测试
cd src-tauri && cargo test test_hosts_api -- --nocapture       # 单个测试
cd src-tauri && cargo test test_users_api -- --nocapture
cd src-tauri && cargo test test_host_groups_api -- --nocapture
cd src-tauri && cargo test test_logs_api -- --nocapture

# 文档
pnpm docs:dev       # 开发文档站
pnpm docs:build     # 构建文档

# 应用图标生成（需先放置 592x592 的 app-icon.png）
npx tauri icon
```

## Architecture

### Frontend ↔ Backend 通信

前端通过 Tauri command RPC 调用 Rust 后端，所有业务逻辑和数据持久化在 Rust 层完成，前端是纯 UI 层。

```
Vue 3 (src/)  --[invoke]-→  Tauri Commands (src-tauri/src/api/)  --→  Services  --→  DB (SQLite/Diesel)
```

### 前端 (`src/`)

| 目录 | 用途 |
|------|------|
| `apis/` | Tauri command 的 TypeScript 封装 |
| `pages/` | 页面组件（home, login, hosts-list, edit, admin, about, 404） |
| `components/` | 可复用组件（editor/Monaco编辑器, settings, layout, add-hosts） |
| `store/` | Pinia 状态管理（useUser, useHosts, useSettings, useLocal, featureStore） |
| `router/` | Vue Router 4 (hash mode)，含登录鉴权守卫 |
| `langs/` | Vue i18n 国际化（zh-cn, en, ja） |
| `plugins/` | Vue 插件初始化（pinia, router, ui, request/alova） |
| `utils/bridge/` | 平台抽象层（tauri.ts / utools.ts），统一封装不同运行环境的 API |

### 后端 (`src-tauri/src/`)

| 目录 | 用途 |
|------|------|
| `api/` | Tauri command handlers（hosts, host_groups, users, logs, system, commands） |
| `db/models/` | Diesel ORM 数据模型 |
| `db/handlers/` | 底层数据库操作 |
| `db/services/` | 业务逻辑层，通过 ServiceFactory 注入 |
| `db/schema.rs` | Diesel schema（自动生成，勿手动修改） |
| `auth/` | JWT token + RBAC 角色权限 + Session 管理 |
| `ui/` | 系统托盘 (tray.rs) + 窗口管理 (window.rs) |

### 关键设计决策

- **双平台支持**: 通过 `utils/bridge/` 抽象层同时支持 Tauri 桌面端和 uTools 插件端
- **单实例控制**: 启动时通过 pgrep/tasklist 检测已运行实例
- **数据库**: SQLite + Diesel ORM + r2d2 连接池，数据库文件在用户本地
- **状态持久化**: Pinia + pinia-plugin-persistedstate 自动持久化到 localStorage

## Path Aliases

```
@     → src/
@utils → src/utils/
/#    → types/
```

## Code Conventions

- 所有回答使用中文
- Vue 组件使用 `<script setup lang="ts">` 语法
- Props/Emits 必须使用 TypeScript 类型声明
- 优先使用 `interface` 而非 `type`，避免 `enum`（用 `const enum` 或对象映射）
- 禁止使用 `any`
- 函数优先使用箭头函数
- 命名：组件 PascalCase，文件 kebab-case，变量/函数 camelCase，常量 UPPER_SNAKE_CASE，布尔值 is/has/should 前缀
- 样式使用 SCSS + BEM 命名，优先使用 Naive UI 主题变量，避免硬编码颜色值
- 优先使用命名导出

## Tech Stack Quick Reference

- **Desktop**: Tauri 2.x | **Frontend**: Vue 3 + TypeScript | **UI**: Naive UI
- **Styling**: UnoCSS (Wind v3 preset) + SCSS/LESS | **State**: Pinia
- **HTTP**: Alova | **Editor**: Monaco Editor | **i18n**: Vue i18n
- **Backend**: Rust + Diesel 2.1 (SQLite) | **Build**: Vite 7 + pnpm
- **Docs**: Rspress | **Node**: >=22.0.0

## Git Hooks

pre-commit 通过 simple-git-hooks + lint-staged 自动对 `*.{vue,js,ts,tsx}` 运行 eslint --fix。
