---
feature: product-docs-and-roadmap
role: ui-designer
status: draft
date: 2026-03-29
---

# SwitchHostsR -- UI 设计文档

## 1. 现有界面结构

### 1.1 整体布局

SwitchHostsR 采用经典的**顶部导航栏 + 左侧边栏 + 主内容区 + 底部栏**四区域布局（`App.vue`）：

```
+--------------------------------------------------+
|                  LHeader (顶部栏)                  |
|  [Logo + 应用名]              [+添加] [设置] [头像] |
+--------+-----------------------------------------+
|        |                                         |
| LSider |          主内容区                         |
| (侧边栏)|     (router-view + 页面切换动画)          |
| 64px   |                                         |
| 折叠态  |                                         |
|        |                                         |
|        |                                         |
| [更多]  |                                         |
+--------+-----------------------------------------+
|                  Footer (@2023 By Mark)           |
+--------------------------------------------------+
```

**关键布局参数**：

- 顶部栏：固定高度 56px（padding: 10px 25px），含 `data-tauri-drag-region` 支持窗口拖拽
- 侧边栏：折叠宽度 64px，展开时自适应，默认折叠状态（`collapsed = true`），可通过 bar 触发器展开/折叠
- 主内容区：宽度 100%，高度 `calc(100% - 46px)`，含页面切换的 fade 过渡动画（进入 0.7s / 离开 0.3s）
- 底部栏：固定高度约 46px，居中文本 "By Mark"
- 全局覆盖层：AddHosts 抽屉、Settings 模态框、GlobalModel 弹窗浮于主布局之上

### 1.2 各页面分析

#### 1.2.1 首页（Home / welcome.vue）

**功能定位**：应用仪表盘，展示概览信息和快速入口。

**布局结构**：
- 最大宽度 1200px 居中，内边距 24px
- **欢迎区域**：渐变背景卡片（紫色 `#667eea` -> `#764ba2`），包含 Logo（120px SVG）、标题、副标题和实时时钟
- **状态概览**：三列响应式网格（`grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))`），展示 Hosts 总数、已启用数、只读数
- **快速操作**：三列卡片网格（min 280px），包含管理 Hosts、添加配置、查看日志三个入口
- **最近使用**：三列卡片网格（min 300px），展示最近 3 个 hosts 文件信息

**已实现的响应式**：768px 断点下所有网格切换为单列，内边距缩小至 16px。

#### 1.2.2 Hosts 管理页（HostsList / hosts-list/index.vue）

**功能定位**：核心功能页，管理所有 hosts 配置文件的查看和编辑。

**布局结构**：
- 外层 `n-card` 全高无边框容器
- **卡片头部**：左侧标题 + 统计信息，右侧搜索框（宽 12rem）+ 刷新按钮 + 新建按钮（主要样式）+ 设置按钮
- **主体内容**：左侧 Tab 导航（`placement="left"`，宽 200px）+ 右侧编辑器区域
  - Tab 标签页：每个 hosts 项显示名称 + 状态指示灯（绿/灰色圆点）+ 状态/只读标签
  - 编辑器头部：hosts 名称 + 启用/禁用标签 + 操作按钮 + 文件路径信息
  - Monaco 编辑器：hosts 语法高亮，内容变化 300ms 防抖自动保存
- **空状态**：搜索无结果或无 hosts 文件时显示 `n-result` 组件 + 操作按钮
- 内容区高度：`calc(100vh - 200px)`

#### 1.2.3 编辑页（Edit / edit/index.vue）

**功能定位**：独立的 hosts 文件编辑器页面。

**布局结构**：
- 外层 `n-card` 全高无边框容器
- **卡片头部**：左侧标题 + 文件名 + 未保存标签，右侧按钮组（新建、导入、导出、保存、格式化）
- **编辑器容器**：纵向 flex 布局
  - 信息栏：浅灰背景，显示文件名和字符数
  - Monaco 编辑器：填充剩余空间
- 容器高度：`calc(100vh - 200px)`

**当前状态**：使用硬编码示例内容，未与后端 API 对接。

#### 1.2.4 登录页（Login / login/index.vue）

**功能定位**：用户认证入口，支持注册、登录、三方登录。

**布局结构**：
- 全屏布局，背景为循环播放的 MP4 视频（object-fit: cover）+ 半透明遮罩（rgba(0,0,0,0.3)）
- 居中登录卡片（宽 350px，绝对定位 + transform 居中，带阴影 `box-shadow: 0 0 10px rgba(0,0,0,0.35)`）
- 三 Tab 切换：登录（Login）、注册（Register）、三方登录（ThirdLogin）
- 已登录状态：显示用户头像（120px 圆形）+ 昵称

**表单设计**：
- 统一使用 Naive UI 表单组件，`label-placement="left"`，`size="medium"`
- 登录：邮箱 + 密码，支持邮箱格式验证
- 注册：昵称（可选）+ 邮箱 + 密码
- 三方登录：账号 + 密码 + 来源选择（GitHub / 微博，微博已禁用）
- 提交按钮：全宽 block 按钮，primary 类型，dashed 变体

#### 1.2.5 管理后台（Admin / admin/index.vue）

**功能定位**：超级管理员的用户管理和日志查看。

**布局结构**：
- 外层 `n-card` 无边框容器
- 两 Tab 切换：用户管理 + 操作日志
- **用户管理**（user.vue）：`n-data-table` 数据表格，包含 ID、用户名、邮箱、三方账户标签、创建/修改时间、状态开关、操作按钮（编辑/删除）
- **操作日志**（logs.vue）：`n-data-table` 数据表格，支持行展开查看详情，包含 ID、日志类型标签（更新/删除/增加/未知）、用户名、头像（32px 圆形）、邮箱、创建时间

**分页配置**：
- 用户管理：每页 10 条，可选 15/20/30
- 操作日志：每页 5 条，可选 5/10/15/20/30

#### 1.2.6 关于页（About / about/index.vue）

**当前状态**：仅显示 "By Mark" 文本 + 一个点击触发 hosts 创建弹窗的链接。属于骨架实现，缺少版本信息、开源协议、依赖列表等内容。

#### 1.2.7 404 页面（404/404.vue）

**布局结构**：
- 全高居中的 flex 列布局
- 404 SVG 图标（300px）+ "你访问的页面不存在" 文本 + "返回首页"按钮（primary, round）

### 1.3 全局覆盖层组件

#### 1.3.1 偏好设置（Settings / settings/index.vue）

**呈现方式**：Naive UI Modal，宽度 600px，居中弹出

**内容结构**：四 Tab 切换（line 类型），固定面板高度 430px
- **通用**（general.vue）：语言选择、主题选择、写入模式（追加/覆盖 radio）、选择模式（单选/多选 radio）、托盘标题开关、启动隐藏开关，底部取消/确认按钮
- **命令**（cmd.vue）：内嵌 Monaco 编辑器（shell 语言），底部取消/确认按钮
- **代理**（proxy.vue）：代理开关、协议选择、主机输入、端口输入，底部取消/确认按钮
- **高级**（advanced.vue）：数据发送开关、hosts 文件路径（可打开）、数据存储路径（可打开/更改）

#### 1.3.2 添加 Hosts（AddHosts / add-hosts/add-hosts.vue）

**呈现方式**：Naive UI Drawer，右侧滑入，宽度 502px

**表单内容**：
- Hosts 类型选择（本地/远程）
- 名称输入
- 条件显示：远程地址输入 + 刷新类型选择（从不/1分钟/5分钟/15分钟/1小时/24小时/7天）
- 底部操作：取消（warning 圆角）+ 确认（primary 圆角）

#### 1.3.3 全局弹窗（GlobalModel / global-model.vue）

**当前状态**：骨架实现，固定宽度 600px 的卡片弹窗，仅包含占位文本 "ddd" 和一个空链接。

## 2. 设计系统分析

### 2.1 颜色体系

#### 2.1.1 主题色配置（provider.vue）

| 角色 | 色值 | 用途 |
|------|------|------|
| 主色 | `#1677ff`（Ant Design Blue） | 主要操作按钮、链接 |
| 主色悬停 | `#4096ff` | 主色按钮悬停 |
| 主色按钮背景 | `rgba(22, 119, 255, 0.8)` | 主色按钮默认透明度 |
| 错误色 | `#FF4D4F` | 错误提示、删除操作 |
| 错误色悬停 | `#ff7875` | 错误按钮悬停 |

#### 2.1.2 UnoCSS 自定义色（uno.config.ts）

| 角色 | 色值 | 备注 |
|------|------|------|
| primary | `#096`（绿色） | 与 Naive UI 主色（蓝色）冲突 |
| primary-dark | `#064` | 暗绿色变体 |

#### 2.1.3 页面内硬编码色值

| 色值 | 出现位置 | 用途 |
|------|---------|------|
| `#18a058` | welcome.vue（统计数字、操作图标） | Naive UI 成功色，直接硬编码 |
| `#2080f0` | welcome.vue（已启用图标） | Naive UI 信息色，直接硬编码 |
| `#f0a020` | welcome.vue（只读图标） | Naive UI 警告色，直接硬编码 |
| `#667eea -> #764ba2` | welcome.vue（欢迎区渐变） | 紫色渐变背景 |
| `#333` | l-header.vue, welcome.vue | 标题文字色 |
| `#666` | welcome.vue | 次要文字色 |
| `#ccc` | advanced.vue | 提示文字色 |
| `#718096` | general.vue | 提示文字色（Tailwind gray-500 变体） |
| `#fafafa` | hosts-list, edit | 编辑器头部背景 |
| `#e0e0e0` | hosts-list, edit | 边框色 |
| `#f5f5f5` | welcome.vue | 代码块背景 |
| `#10b981` | hosts-list（scoped 样式） | 启用状态绿色（Tailwind emerald-500） |
| `#9ca3af` | hosts-list（scoped 样式） | 禁用状态灰色 |
| `#6b7280` | hosts-list（scoped 样式） | 次要文本灰色 |

**问题**：颜色使用混乱，存在三套不同的颜色来源（Naive UI 主题变量、UnoCSS 主题色、硬编码色值），且 UnoCSS 的 primary 绿色与 Naive UI 的 primary 蓝色相互矛盾。

#### 2.1.4 深色模式

- Naive UI 深色主题通过 `provider.vue` 中 `darkTheme` 切换
- Monaco 编辑器支持 `vs-dark` / `hosts-dark` 主题切换
- **但页面内大量硬编码的色值（#333、#666、#fafafa、#e0e0e0 等）在深色模式下不会自动反转**，将导致严重的可读性问题

### 2.2 字体排版

#### 2.2.1 全局字体（style.less）

```css
font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
font-size: 16px;
line-height: 24px;
font-weight: 400;
```

#### 2.2.2 特殊字体

- 时钟显示：`"Share Tech Mono", monospace`（welcome.vue）
- 文件路径：`monospace`（welcome.vue）

#### 2.2.3 字号使用情况

| 字号 | 对应 rem | 用途 |
|------|---------|------|
| 2.5rem (40px) | - | 欢迎标题 |
| 2rem (32px) | - | 统计数字、时钟时间 |
| 1.5rem (24px) | - | 章节标题 |
| 1.2rem (19.2px) | - | 欢迎副标题 |
| 1.1rem (17.6px) | - | 卡片标题、hosts 名称 |
| 18px | - | Logo 名称 |
| 16px | 1rem | 正文、昵称、页脚、404 文字 |
| 14px (0.9rem) | - | 标签文字、描述文字 |
| 12px (0.75rem) | - | 提示文字、标签名、Tab 名称、辅助信息 |
| 0.85rem | - | 路径文字 |

**问题**：字号混用 px 和 rem，缺乏统一的排版阶梯（typographic scale）。

### 2.3 组件使用

#### 2.3.1 Naive UI 组件清单

| 组件 | 用途 | 使用位置 |
|------|------|---------|
| `n-config-provider` | 全局主题配置 | provider.vue |
| `n-layout` / `n-layout-header` / `n-layout-sider` / `n-layout-content` / `n-layout-footer` | 页面布局框架 | App.vue |
| `n-menu` | 侧边栏导航菜单 | l-sider.vue |
| `n-card` | 页面容器、统计卡片、登录卡片 | 多处 |
| `n-tabs` / `n-tab-pane` | 页面内 Tab 切换 | hosts-list, admin, login, settings |
| `n-data-table` | 数据表格 | admin/user.vue, admin/logs.vue |
| `n-form` / `n-form-item` | 表单 | login, register, settings, add-hosts |
| `n-input` | 输入框 | 多处 |
| `n-select` | 下拉选择 | settings, add-hosts, third-login |
| `n-button` | 按钮（primary/secondary/tertiary/quaternary） | 全局 |
| `n-tag` | 状态标签 | hosts-list, admin |
| `n-icon` | 图标容器 | 全局 |
| `n-avatar` | 用户头像 | login, l-header |
| `n-dropdown` | 下拉菜单 | l-header, l-sider |
| `n-tooltip` | 提示气泡 | l-header, hosts-list, advanced |
| `n-divider` | 分割线 | l-header |
| `n-switch` | 开关 | settings, admin/user |
| `n-radio-group` / `n-radio` | 单选组 | general.vue |
| `n-checkbox` | 复选框 | advanced.vue |
| `n-modal` | 模态框 | settings, global-model |
| `n-drawer` / `n-drawer-content` | 抽屉面板 | add-hosts |
| `n-spin` | 加载状态 | login, hosts-list, editor |
| `n-result` | 结果页 | hosts-list（空状态）, editor（错误状态） |
| `n-scrollbar` | 自定义滚动条 | settings/general |
| `n-space` | 间距容器 | l-header, settings |
| `n-back-top` | 回到顶部 | App.vue |
| `n-loading-bar-provider` | 加载进度条 | provider.vue |
| `n-notification-provider` | 通知 | provider.vue |
| `n-message-provider` | 消息提示 | provider.vue |
| `n-dialog-provider` | 对话框 | provider.vue |
| `n-modal-provider` | 模态框 | provider.vue |

#### 2.3.2 自定义组件

| 组件 | 用途 | 设计特点 |
|------|------|---------|
| `SvgIcon` | SVG 图标渲染 | 通过 SVG Sprite 加载，支持自定义 size/color |
| `Editor` | Monaco 代码编辑器封装 | 支持多语言、格式化、主题切换、自适应尺寸 |
| `LHeader` | 顶部导航栏 | 左 Logo + 右操作区（添加/设置/头像） |
| `LSider` | 侧边导航栏 | 折叠式菜单 + 底部"更多"按钮 |
| `Settings` | 偏好设置弹窗 | 4 Tab 模态框 |
| `AddHosts` | 添加 Hosts 抽屉 | 右侧滑入表单 |
| `GlobalModel` | 全局弹窗 | 骨架实现 |
| `AdvancedCell` | 高级设置分区 | 简单的标题+插槽容器 |

### 2.4 图标体系

#### 2.4.1 SVG 自定义图标（src/assets/icons/）

共 16 个 SVG 图标：`logo`、`logo-header`、`home`、`about`、`settings`、`plus`、`import`、`export`、`import-remote`、`issues`、`updated`、`recycle`、`quit`、`404`、`vue`

通过 `vite-plugin-svg-icons` 打包为 SVG Sprite，通过 `SvgIcon` 组件引用。

#### 2.4.2 第三方图标库

- **@vicons/ionicons5**：侧边栏菜单图标（Home, ListCircle, Text, ApertureOutline, CaretDownOutline, SettingsSharp）
- **@vicons/tabler**：功能按钮图标（Search, Plus, Refresh, Settings, Eye, EyeOff, Save, Download, Upload, FileText）
- **内联 SVG**：welcome.vue 中状态统计图标和快速操作图标直接内联 SVG path

**问题**：图标来源不统一（自定义 SVG + Ionicons5 + Tabler + 内联 SVG），风格可能不一致。

## 3. 交互模式

### 3.1 导航模式

- **侧边栏导航**：主导航通过 `n-menu` 实现，支持折叠/展开，使用 `vue-router` 的 `RouterLink` 渲染菜单项，当前页面高亮选中
- **Tab 导航**：Hosts 管理页的左侧 Tab 切换、管理后台的 Tab 切换、登录页的 Tab 切换、设置面板的 Tab 切换
- **页面跳转动画**：fade 过渡效果（translateX(20px) + opacity），进入 0.7s / 离开 0.3s
- **路由守卫**：未登录用户强制重定向到登录页，登录信息 7 天有效期

### 3.2 表单模式

- **标签位置**：统一 `label-placement="left"`，自动标签宽度
- **验证触发**：`['blur', 'input']` 双触发
- **必填标记**：`require-mark-placement="right-hanging"`
- **提交方式**：按钮点击或 Enter 键
- **防抖处理**：注册表单 300ms 防抖

### 3.3 编辑器交互

- **Monaco Editor**：自定义 hosts 语法高亮主题（明/暗两套）
- **实时保存**：内容变化 300ms 防抖后自动调用后端更新 API
- **格式化**：支持一键格式化文档（`editor.action.formatDocument`）
- **加载状态**：编辑器初始化期间显示 Spin 加载器
- **错误恢复**：初始化失败时显示 404 结果页 + 重试按钮
- **自适应**：`automaticLayout: true` 自动适应容器尺寸

### 3.4 弹窗/抽屉模式

- **设置弹窗**：Modal 形式，600px 固定宽度，ESC/遮罩点击/关闭按钮三种关闭方式
- **添加 Hosts 抽屉**：Drawer 右侧滑入，502px 固定宽度，带页头标题和页脚操作按钮
- **确认对话框**：hosts 启用/禁用操作前弹出 `dialog.warning` 二次确认

### 3.5 反馈机制

- **消息提示**：`useMessage()` 在操作成功/失败后弹出消息条（最多 1 条）
- **通知**：`useNotification()` 用于系统级提示（如编辑器初始化失败），最多 1 条
- **加载进度条**：`useLoadingBar()` 在页面切换和编辑器操作时显示
- **Spin 加载器**：数据加载时覆盖内容区
- **Tooltip**：悬停在 hosts Tab 上显示详细信息

### 3.6 下拉菜单体系

- **头部设置菜单**（rightMenus）：首页、关于、检查更新、issues、偏好设置、导出、本地导入、远程导入
- **头部用户菜单**（personalMenus）：退出
- **侧边栏底部菜单**（bottomMenus）：首页、关于、检查更新、issues、回收站
- 菜单项支持 Feature Flag 控制显隐（featureStore）

## 4. 现有 UI 评估

### 4.1 优点

1. **技术栈选型优秀**：Naive UI 提供了高质量的组件库，UnoCSS 提供灵活的原子化样式，Monaco Editor 提供专业的代码编辑体验，这些选型为构建优秀的桌面应用 UI 奠定了坚实基础。

2. **主布局结构合理**：Header + Sider + Content 的经典桌面应用布局，侧边栏可折叠节省空间，Tauri 拖拽区域已正确设置。

3. **i18n 完备**：三语言支持（中/英/日）覆盖了主要目标用户群体，所有 UI 文本均已国际化。

4. **主题切换机制已就绪**：通过 Naive UI 的 `n-config-provider` + `useOsTheme()` 实现了暗色/亮色/跟随系统三种主题模式的基础设施。

5. **编辑器封装良好**：Monaco Editor 的封装层（`useMonaco` + `useEditor`）设计合理，支持多实例管理、主题切换、格式化、加载/错误状态处理。

6. **交互反馈完整**：通过 Naive UI 的 Message、Notification、LoadingBar、Dialog、Spin 等组件提供了多层级的用户反馈机制。

7. **首页仪表盘设计有仪式感**：渐变色欢迎区 + 实时时钟 + 统计卡片 + 快速操作的设计给用户良好的第一印象。

8. **Feature Flag 机制**：通过 `featureStore` 控制功能的显隐，为渐进式功能发布提供了基础设施。

### 4.2 问题与改进空间

#### 4.2.1 设计一致性问题

1. **颜色体系混乱**：
   - Naive UI 主色 `#1677ff`（蓝色）与 UnoCSS 的 primary `#096`（绿色）冲突
   - welcome.vue 中使用 `#18a058` 作为强调色（Naive UI 的成功色），与 Naive UI 主色语义不一致
   - 大量硬编码色值散落在各组件的 scoped 样式中，无法通过主题系统统一管控
   - **建议**：统一颜色系统，移除 UnoCSS 自定义主色或与 Naive UI 对齐，将所有色值提取为 CSS 变量或 Naive UI 主题变量

2. **样式技术混用**：
   - 同时使用 4 种样式方案：Less（style.less, welcome.vue, hosts-list, edit, editor）、Stylus（login, 404, l-header, l-sider, settings 子页面）、UnoCSS 原子类（部分组件中的 `flex`, `gap-2` 等）、内联样式
   - hosts-list 中甚至手动重新定义了大量 Tailwind/UnoCSS 原子类（`.flex`, `.gap-1`, `.text-xs` 等），而非使用 UnoCSS 自动生成
   - **建议**：统一为 UnoCSS + Less（或 SCSS），逐步移除 Stylus 文件，利用 UnoCSS 替代手工 utility 类

3. **组件编写风格不统一**：
   - 部分组件使用 `<script setup>`（welcome, hosts-list, editor, admin/logs, admin/user），部分使用 Options API 的 `defineComponent`（App.vue, login, register, third-login, about, 404, add-hosts, global-model）
   - **建议**：统一迁移到 `<script setup lang="ts">` 语法

#### 4.2.2 深色模式适配缺陷

1. **硬编码色值在深色模式下失效**：
   - 标题色 `#333` 在深色背景上不可见
   - 背景色 `#fafafa`、`#f5f5f5` 在深色模式下与页面背景对比度不足
   - 边框色 `#e0e0e0` 在深色模式下过于刺眼
   - **建议**：所有色值通过 Naive UI 主题变量（如 `var(--n-text-color)`、`var(--n-border-color)`）引用

2. **欢迎页渐变背景**：固定的紫色渐变在深色模式下可能过于鲜艳
   - **建议**：在深色模式下降低渐变色饱和度或使用不同的色板

#### 4.2.3 布局与间距问题

1. **固定高度计算**：多处使用 `calc(100vh - Npx)` 计算高度，当窗口结构变化时容易失效
   - `calc(100vh - 142px)`（welcome）、`calc(100vh - 200px)`（hosts-list, edit）
   - **建议**：改用 flex 布局的 `flex: 1` + `overflow: auto` 自动填充剩余空间

2. **hosts-list 编辑区域**：Tab 导航区固定 200px，对于长名称 hosts 项，最大显示宽度仅 120px，超出部分省略
   - **建议**：增加 Tab 导航宽度或支持宽度可调

3. **关于页面空洞**：仅有一行 "By Mark" 文本，浪费了整个页面空间
   - **建议**：重新设计为包含版本信息、更新日志、开源协议、贡献者列表、反馈入口的完整关于页

#### 4.2.4 交互体验问题

1. **登录页视频背景**：
   - 需要加载 bj.mp4 视频文件，增加包体积和启动时间
   - 视频循环播放消耗 CPU 资源
   - **建议**：改用静态渐变背景或 CSS 动画背景，或将视频改为可选

2. **添加 Hosts 功能未实现**：
   - hosts-list 中"新建"按钮和 l-header 中"+"按钮触发的功能不同（前者显示 message 提示，后者打开抽屉），用户可能困惑
   - **建议**：统一入口，均触发 AddHosts 抽屉

3. **编辑页面独立但割裂**：
   - Edit 页面（`/edit/:id`）与 HostsList 页面中内嵌的编辑器功能重叠
   - Edit 页面使用硬编码数据，未与后端对接
   - **建议**：明确两者定位 -- HostsList 内嵌编辑器用于快速编辑，独立 Edit 页面用于全屏沉浸编辑

4. **管理后台操作为桩实现**：
   - 用户编辑、删除按钮仅弹出 message 提示
   - **建议**：对接后端 API，添加确认对话框和操作反馈

5. **Footer 信息过时**：底部显示 "@2023 By Mark"，年份已过时
   - **建议**：动态生成年份，或改为更有用的信息（如版本号、快捷键提示）

#### 4.2.5 代码层面的 UI 问题

1. **全局样式类污染**：`style.less` 定义了 `.row`、`.column`、`.f-center` 等过于通用的全局类名，易与第三方库冲突
   - **建议**：使用 UnoCSS 的 `flex`、`flex-col`、`justify-center` 等标准原子类替代

2. **404 页面样式未 scoped**：使用 `<style scope>` 而非 `<style scoped>`（拼写错误），导致样式全局泄漏
   - **建议**：修正为 `<style scoped>`

3. **login 页面 stylus 语法使用 position 但未声明**：`.content` 缺少 `position: absolute` 的显式声明（Stylus 可能因缩进推断）
   - **建议**：改为 Less/SCSS 并显式声明

## 5. 后期功能 UI 设计建议

基于 PRD 文档中规划的后期功能，以下是各功能的 UI 设计建议：

### 5.1 Hosts 写入系统文件（WIP-01 / FUT-01，P0）

**交互流程**：
1. 用户在 HostsList 页面切换 hosts 启用/禁用状态
2. 系统自动合并所有已启用的 hosts 内容
3. 若需要管理员权限，弹出系统原生密码输入对话框
4. 写入完成后：
   - 成功：底部 Message 提示 "Hosts 已更新" + 可选的系统通知
   - 失败：弹出 Dialog 展示错误原因 + 重试按钮
5. 自动执行 DNS 缓存刷新（静默操作，成功后右下角显示小提示）

**UI 元素**：
- 在 HostsList 顶部工具栏添加 "应用到系统" 按钮（绿色 success 类型，带刷新图标）
- 在状态栏或底部添加 "系统 Hosts 同步状态" 指示器（绿色圆点 = 已同步，黄色 = 有未应用的变更）

### 5.2 主机组管理界面（WIP-03，P1）

**建议新增页面或区域**：
- 在侧边栏添加 "分组管理" 菜单项
- 页面布局：左侧分组树（支持拖拽排序）+ 右侧分组详情
- 分组卡片展示：组名、描述、包含的 hosts 数量、启用状态
- 拖拽 hosts 到分组的交互
- 批量操作工具栏：全选、启用选中、禁用选中、删除选中

### 5.3 Hosts 语法检查（FUT-02，P1）

**编辑器增强**：
- 利用 Monaco Editor 的 Diagnostics API 实现实时语法检查
- 在编辑器左侧行号区域显示错误/警告标记（红/黄色圆点）
- 错误详情通过悬停 Tooltip 展示
- 底部状态栏显示错误/警告总数
- 错误类型：无效 IP 格式、无效域名格式、重复条目、注释格式不规范

### 5.4 配置版本历史（FUT-03，P1）

**UI 设计**：
- 在编辑器工具栏添加 "历史版本" 按钮
- 点击后在右侧滑出 Drawer，展示版本时间线列表
- 每个版本项：时间戳、操作用户、变更摘要、"查看" / "回滚" 按钮
- 点击 "查看" 进入差异对比视图

### 5.5 配置差异对比（FUT-04，P1）

**UI 设计**：
- 利用 Monaco Editor 的 `DiffEditor` 组件
- 双栏并排显示（左旧右新），差异行高亮标记
- 顶部工具栏：版本选择下拉框 x 2、"仅显示差异" 切换、"复制变更" 按钮
- 可从版本历史页面跳转进入

### 5.6 全局快捷键（FUT-08，P1）

**建议快捷键方案**：

| 快捷键 | 功能 | 作用域 |
|--------|------|--------|
| `Cmd/Ctrl + N` | 新建 Hosts 配置 | 全局 |
| `Cmd/Ctrl + F` | 搜索 Hosts | 全局 |
| `Cmd/Ctrl + S` | 保存当前编辑 | 编辑器 |
| `Cmd/Ctrl + ,` | 打开偏好设置 | 全局 |
| `Cmd/Ctrl + 1-9` | 切换 Hosts Tab | HostsList |
| `Cmd/Ctrl + Shift + Space` | 全局唤起（系统级） | 系统 |

**UI 支持**：
- 在设置面板增加 "快捷键" Tab，展示所有快捷键映射
- 支持自定义修改快捷键
- 在菜单项右侧显示对应的快捷键标注

### 5.7 关于页面重设计（WIP-13，P2）

**建议布局**：
```
+------------------------------------------+
|     [App Logo 80px]                      |
|     SwitchHostsR                         |
|     v0.1.0 (Build 2026.03.29)            |
|     [检查更新] 按钮                       |
+------------------------------------------+
|  [更新日志]  [开源协议]  [致谢]  [反馈]    |
+------------------------------------------+
|  内容区（根据 Tab 切换）                   |
|  - 更新日志：时间线形式展示版本变更         |
|  - 开源协议：MIT License 全文              |
|  - 致谢：依赖库列表                        |
|  - 反馈：GitHub Issues 链接 + 邮箱         |
+------------------------------------------+
|  Copyright 2023-2026 Mark                |
|  Built with Tauri + Vue 3 + Rust         |
+------------------------------------------+
```

## 6. 用户体验改进

### 6.1 首次使用引导

**当前问题**：新用户打开应用后被强制跳转到登录页，没有产品介绍或功能说明。

**建议**：
1. 添加首次启动引导流程（Onboarding）：
   - 第 1 步：欢迎页 + 产品简介
   - 第 2 步：选择是否使用登录功能（本地模式 vs 云模式）
   - 第 3 步：创建第一个 Hosts 配置
2. 在关键功能处添加引导提示（如 Tooltip 气泡或 Coach Mark）

### 6.2 空状态设计

**当前问题**：多处空状态使用简单的 `n-result` 404 图标，不够友好。

**建议**：
- 首页无 Hosts 时：展示 "开始您的第一个 Hosts 配置" 引导卡片
- HostsList 空列表：展示简洁的插图 + "创建第一个 Hosts 文件" CTA
- 管理后台无日志：展示 "暂无操作记录" + 说明文字

### 6.3 操作确认与撤销

**当前问题**：启用/禁用 hosts 需要弹出 dialog 确认，流程偏重。

**建议**：
- 启用/禁用操作改为直接执行 + 底部显示 "已禁用 xxx" + 5 秒内可撤销的 Snackbar
- 删除操作保留 Dialog 二次确认
- 支持 Cmd/Ctrl + Z 全局撤销

### 6.4 搜索体验优化

**当前问题**：搜索仅在前端过滤已加载的列表数据。

**建议**：
- 添加全局搜索（Cmd/Ctrl + K 唤起命令面板）
- 支持搜索 hosts 名称、hosts 内容中的域名和 IP
- 搜索结果高亮匹配文本
- 搜索历史记录

### 6.5 编辑器体验增强

**建议**：
- 添加行号区域的断点/书签功能
- 支持多光标编辑
- 添加 "查找和替换" 快捷入口
- 在编辑器底部状态栏显示：当前行/列、字符编码、hosts 条目数
- 添加右键上下文菜单：复制行、删除行、注释/取消注释、跳转到行

### 6.6 通知与状态同步

**建议**：
- 在顶部栏添加通知铃铛图标，展示未读通知列表
- 当后端检测到远程 Hosts 更新时，推送通知
- 在系统托盘菜单中显示当前启用的 Hosts 方案名称

## 7. 响应式与多平台适配

### 7.1 当前状态分析

- **桌面端**：当前 UI 主要面向桌面端设计，但窗口最小尺寸未限制
- **响应式**：仅 welcome.vue 有 768px 断点的响应式处理，其他页面均为固定布局
- **UnoCSS 断点**：已配置但未广泛使用（sm: 640px, md: 768px, lg: 1024px, xl: 1280px, 2xl: 1536px）

### 7.2 桌面端窗口适配建议

1. **设定最小窗口尺寸**：在 `tauri.conf.json` 中设置 `minWidth: 800`、`minHeight: 600`
2. **侧边栏自适应**：
   - 窗口宽度 < 900px 时自动折叠侧边栏
   - 窗口宽度 < 700px 时隐藏侧边栏，改为汉堡菜单
3. **编辑器区域**：保持自适应，但设置最小宽度确保编辑体验
4. **HostsList Tab 面板**：窗口较窄时将左侧 Tab 改为顶部 Tab 或下拉选择

### 7.3 uTools 插件端适配

**当前状态**：Bridge 抽象层已就位，但 uTools 端 UI 未做针对性适配。

**建议**：
- uTools 插件窗口通常较小（约 800x600），需要精简布局
- 考虑为 uTools 模式设计独立的紧凑布局
- 隐藏顶部导航栏（uTools 有自己的窗口控制）
- 直接展示 HostsList 作为主页面
- 设置面板改为全屏而非弹窗

### 7.4 多平台视觉适配

1. **macOS**：
   - 已设置 `data-tauri-drag-region`，支持标题栏拖拽
   - 考虑添加 traffic light 按钮区域的间距
   - 字体渲染：macOS 已启用 `-webkit-font-smoothing: antialiased`

2. **Windows**：
   - 需要处理 DPI 缩放问题
   - Windows 原生窗口控件与自定义顶栏的交互
   - 考虑 Windows 11 Mica/Acrylic 材质背景

3. **Linux**：
   - 确保 GTK/Qt 主题不干扰应用样式
   - 测试不同桌面环境（GNOME, KDE, XFCE）下的表现

## 8. 无障碍访问

### 8.1 当前状态

- SvgIcon 组件设置了 `aria-hidden="true"`（正确，装饰性图标应隐藏）
- 全局弹窗设置了 `role="dialog"` 和 `aria-modal="true"`
- **但整体无障碍支持非常有限**，缺乏系统性的 a11y 设计

### 8.2 改进建议

#### 8.2.1 键盘导航

1. **Tab 顺序**：确保所有交互元素可通过 Tab 键访问，顺序合理
2. **焦点可见**：所有可聚焦元素需有明确的焦点指示环（Naive UI 已有基础支持）
3. **键盘操作**：
   - Escape 关闭弹窗/抽屉（部分已实现）
   - Enter 确认操作
   - 方向键在菜单/列表中导航

#### 8.2.2 ARIA 属性补充

1. **导航区域**：侧边栏添加 `role="navigation"` 和 `aria-label="主导航"`
2. **主内容区**：添加 `role="main"` 和 `aria-label="主内容"`
3. **状态变更**：hosts 启用/禁用操作后使用 `aria-live="polite"` 通告状态变化
4. **表格增强**：`n-data-table` 补充 `aria-label` 描述表格内容
5. **图标按钮**：所有仅含图标的按钮添加 `aria-label`（如 "+" 按钮应标注为 "添加新 Hosts"）

#### 8.2.3 色彩对比度

1. **当前问题**：
   - 提示文字色 `#ccc` 在白色背景上对比度不足（约 1.6:1，远低于 WCAG 4.5:1 要求）
   - 提示文字色 `#718096` 在白色背景上对比度约 4.8:1（勉强达标）
   - `#666` 在白色背景上对比度约 5.7:1（达标）
2. **建议**：所有文本确保对比度 >= 4.5:1（AA 级），大文本 >= 3:1

#### 8.2.4 屏幕阅读器支持

1. 为重要状态变更添加 `aria-live` 区域
2. 为表格、表单添加说明性 `aria-label`
3. 为 Tab 面板提供 `aria-controls` 关联

## 9. 设计规范建议

### 9.1 统一颜色规范

建议建立以下颜色 Token 体系，统一在 Naive UI 主题覆盖和 CSS 变量中定义：

```
# 品牌色
--color-brand-primary:     #1677ff    (主操作、链接、选中态)
--color-brand-primary-hover: #4096ff  (主色悬停)
--color-brand-primary-bg:  rgba(22, 119, 255, 0.08)  (主色淡背景)

# 功能色
--color-success:           #18a058    (成功、已启用)
--color-warning:           #f0a020    (警告、只读)
--color-error:             #FF4D4F    (错误、删除)
--color-info:              #2080f0    (信息、提示)

# 中性色（亮色模式）
--color-text-primary:      #1f2937    (主文本)
--color-text-secondary:    #4b5563    (次要文本)
--color-text-tertiary:     #6b7280    (辅助文本)
--color-text-placeholder:  #9ca3af    (占位文本)
--color-border:            #e5e7eb    (边框)
--color-divider:           #f0f0f0    (分割线)
--color-bg-primary:        #ffffff    (主背景)
--color-bg-secondary:      #f9fafb    (次要背景)
--color-bg-tertiary:       #f3f4f6    (三级背景)

# 中性色（暗色模式）-- 自动由 Naive UI darkTheme 管理
```

### 9.2 统一间距规范

采用 4px 基准的间距系统（与 UnoCSS 的 spacing 对齐）：

| Token | 值 | 用途 |
|-------|-----|------|
| `spacing-1` | 4px | 紧凑间距（标签内间距） |
| `spacing-2` | 8px | 小间距（图标与文本间距） |
| `spacing-3` | 12px | 中小间距（表单项间距） |
| `spacing-4` | 16px | 标准间距（卡片间距、内边距） |
| `spacing-6` | 24px | 大间距（区块间距） |
| `spacing-8` | 32px | 较大间距（页面区域间距） |
| `spacing-10` | 40px | 特大间距（欢迎区域内边距） |

### 9.3 统一字号规范

建立从小到大的字号阶梯：

| Token | 值 | 行高 | 用途 |
|-------|-----|------|------|
| `text-xs` | 12px | 16px | 辅助文本、标签、提示 |
| `text-sm` | 14px | 20px | 次要文本、描述 |
| `text-base` | 16px | 24px | 正文（基准） |
| `text-lg` | 18px | 28px | 小标题 |
| `text-xl` | 20px | 28px | 页面标题 |
| `text-2xl` | 24px | 32px | 章节标题 |
| `text-3xl` | 30px | 36px | 大标题 |
| `text-4xl` | 36px | 40px | 超大标题 |

### 9.4 统一圆角规范

| Token | 值 | 用途 |
|-------|-----|------|
| `rounded-sm` | 4px | 小型元素（标签、徽章） |
| `rounded` | 6px | 标准元素（输入框、按钮、卡片） |
| `rounded-lg` | 12px | 大型元素（操作图标背景） |
| `rounded-xl` | 16px | 特大元素（欢迎区域卡片） |
| `rounded-full` | 9999px | 圆形（头像、状态指示器） |

### 9.5 统一阴影规范

| Token | 值 | 用途 |
|-------|-----|------|
| `shadow-sm` | `0 1px 3px rgba(0,0,0,0.08)` | 卡片默认 |
| `shadow` | `0 4px 15px rgba(0,0,0,0.1)` | 卡片悬停 |
| `shadow-lg` | `0 8px 25px rgba(0,0,0,0.1)` | 弹窗 |
| `shadow-xl` | `0 0 10px rgba(0,0,0,0.35)` | 登录卡片 |

### 9.6 动画与过渡规范

| 场景 | 时长 | 缓动函数 |
|------|------|---------|
| 按钮悬停 | 200ms | ease |
| 卡片悬停 | 300ms | ease |
| 页面进入 | 300ms | ease-out |
| 页面离开 | 200ms | ease-in |
| 弹窗进入 | 250ms | ease-out |
| 弹窗离开 | 200ms | ease-in |
| 抽屉进入 | 300ms | ease-out |
| 侧边栏折叠 | 300ms | ease |

**建议**：减少当前 welcome 页面进入动画的 0.7s 时长，改为 0.3s 以提升流畅感。

### 9.7 组件使用规范

1. **按钮层级**：
   - Primary：每个视图中最多 1 个主操作按钮
   - Secondary/Default：次要操作
   - Tertiary/Quaternary：低优先级操作
   - Text：内联操作

2. **表单规范**：
   - 标签统一左对齐，宽度自适应
   - 验证消息统一在输入框下方显示
   - 操作按钮统一右对齐（取消在左，确认在右）

3. **图标使用**：
   - 统一使用 `@vicons/tabler` 作为主图标库（线性风格，与桌面应用气质匹配）
   - 品牌图标和特殊图标保留 SVG Sprite 方式
   - 淘汰 Ionicons5 和内联 SVG 的使用

### 9.8 样式技术规范

1. **优先级**：UnoCSS 原子类 > Less scoped 样式 > 全局样式
2. **色值引用**：禁止硬编码色值，统一通过 CSS 变量或 Naive UI 主题变量引用
3. **布局**：优先使用 UnoCSS 的 flex/grid 工具类，复杂布局使用 scoped Less
4. **单位**：字号使用 px（与设计稿对齐），间距使用 UnoCSS 的间距单位
5. **文件格式**：统一使用 Less（移除 Stylus 依赖）
6. **命名**：自定义类名使用 BEM 命名法（如 `.editor-header__title`）

---

*本文档基于对 SwitchHostsR 前端代码（截至 2026-03-29，dev 分支）的完整分析编写。所有界面描述和问题诊断均来源于实际代码审阅。*
