# 更新日志

记录 Switch Hosts R 的版本更新历史和重要变更。

## 版本说明

- **主版本号**：不兼容的 API 修改
- **次版本号**：向下兼容的功能性新增
- **修订号**：向下兼容的问题修正

---

## 🚧 开发中版本

### v0.1.0 (开发中)

#### 🎯 目标
实现核心 hosts 管理功能，提供可用的基础版本。

#### ✨ 新功能

- **👤 用户认证系统**
  - 邮箱注册和登录
  - JWT Token 认证
  - 第三方登录 (GitHub OAuth)
  - 会话管理

- **📁 Hosts 管理**
  - Hosts 文件列表展示
  - Monaco Editor 代码编辑器
  - Hosts 启用/禁用切换
  - 主机组管理
  - 操作日志记录

- **⚙️ 系统功能**
  - SQLite 数据库存储
  - 系统托盘图标
  - 多语言支持 (中文、英文、日文)
  - 基础设置面板

#### 🏗️ 技术架构

- **Tauri 2.x** 桌面应用框架
- **Vue 3 + TypeScript** 前端架构
- **Naive UI** 组件库
- **Rust** 后端服务
- **Diesel ORM** 数据库操作
- **Rspress** 文档站

#### 🐛 待修复

- 新建 Hosts 功能待完善
- 系统 hosts 文件写入待实现
- 远程 hosts 同步待集成

---

## 📅 版本历史

### v0.0.1 (2026-05-07)

#### 🎉 初始版本

- 项目基础框架搭建
- 前端页面结构搭建
- 后端 API 基础架构
- 数据库表结构设计
- 文档站初始化

---

## 🔮 未来计划

### v0.2.0 (规划中)

- 远程 hosts 支持 (HTTP/HTTPS)
- 导入/导出功能
- 备份与恢复
- 快捷键支持
- 深色模式

### v0.3.0 (规划中)

- 全平台构建测试
- 性能优化
- 安全性审计
- 文档完善

### v1.0.0 (规划中)

- 正式版本发布
- 多平台安装包
- 完整的发布说明

---

## 📞 反馈和支持

### 问题报告

如果您在使用过程中遇到问题：

1. **检查已知问题**：查看本页面的修复记录
2. **搜索现有 Issues**：[GitHub Issues](https://github.com/switchhosts/switchhosts-r/issues)
3. **提交新问题**：提供详细的环境信息和重现步骤

### 功能建议

我们欢迎您的功能建议：

1. **社区讨论**：[GitHub Discussions](https://github.com/switchhosts/switchhosts-r/discussions)
2. **功能投票**：为您关心的功能投票
3. **贡献代码**：提交 Pull Request

### 联系方式

- 📧 **邮件**：[support@switchhosts.com](mailto:support@switchhosts.com)
- 💬 **社区**：[GitHub Discussions](https://github.com/switchhosts/switchhosts-r/discussions)
- 🐛 **Bug 报告**：[GitHub Issues](https://github.com/switchhosts/switchhosts-r/issues)
- 📖 **文档**：[在线文档](https://switchhosts.github.io/switchhosts-r/)

---

**感谢您使用 Switch Hosts R！** 🎉

我们致力于为用户提供最好的 Hosts 管理体验。您的反馈和建议是我们持续改进的动力。
