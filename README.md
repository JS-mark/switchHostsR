# SwitchHostsR

> 该项目是采用 `tarui` 作为底层能力，`Vue3` 作为UI层的hybird应用。主要用于学习使用，涉及功能目前免费；

- 生成app icon 资源
  - 在根目录节点放置`app-icon.png`资源, 图片大小 `592x592` 执行命令

```bash
npx tauri icon
```

## 项目结构

```text
src-tauri/
├── src/
│   ├── main.rs           # 应用入口点
│   ├── lib.rs            # 库入口点
│   ├── api/              # API 模块 (前端调用的接口)
│   │   ├── mod.rs
│   │   ├── hosts.rs
│   │   ├── users.rs
│   │   └── logs.rs
│   ├── db/               # 数据库模块
│   │   ├── mod.rs
│   │   ├── models/       # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── users.rs
│   │   │   ├── hosts.rs
│   │   │   └── logs.rs
│   │   ├── schema.rs     # 数据库表结构
│   │   └── handlers/     # 数据库操作处理
│   │       ├── mod.rs
│   │       ├── users.rs
│   │       ├── hosts.rs
│   │       └── logs.rs
│   ├── utils/            # 工具函数
│   │   ├── mod.rs
│   │   ├── time.rs
│   │   └── security.rs
│   └── ui/               # UI 相关
│       ├── mod.rs
│       ├── window.rs
│       └── tray.rs
├── tests/                # 集成测试
│   ├── db_tests.rs
│   └── api_tests.rs
├── Cargo.toml            # Rust 项目配置文件
└── tauri.conf.json       # Tauri 配置文件
```

## 开发

### 单元测试

```bash
# 只运行日志 API 测试
cargo test test_logs_api -- --nocapture

# 只运行用户 API 测试
cargo test test_users_api -- --nocapture

# 只运行主机 API 测试
cargo test test_hosts_api -- --nocapture

# 只运行主机组 API 测试
cargo test test_host_groups_api -- --nocapture
```

### 集成测试

```bash
cargo test --nocapture
```
