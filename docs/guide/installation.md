# 安装指南

本指南将帮助您在不同操作系统上安装和配置 Switch Hosts R。

## 📋 系统要求

### 最低系统要求

**Windows**
- Windows 10 版本 1903 或更高版本
- x64 或 ARM64 架构
- 至少 100MB 可用磁盘空间

**macOS**
- macOS 10.15 (Catalina) 或更高版本
- Intel 或 Apple Silicon (M1/M2) 处理器
- 至少 100MB 可用磁盘空间

**Linux**
- Ubuntu 18.04 LTS 或更高版本
- CentOS 8 或更高版本
- Debian 10 或更高版本
- x64 架构
- 至少 100MB 可用磁盘空间

### 推荐系统配置
- 内存：4GB RAM 或更多
- 处理器：双核 2.0GHz 或更高
- 网络：用于远程 hosts 文件同步

## 📥 下载安装包

### 官方下载渠道

1. **GitHub Releases**（推荐）
   - 访问 [GitHub Releases 页面](https://github.com/JS-mark/switchHostsR/releases)
   - 下载适合您系统的最新版本

2. **官方网站**
   - 访问 [官方下载页面](/download)
   - 选择对应的操作系统版本

### 文件说明

```bash
# Windows 安装包
SwitchHostsR_1.0.0_x64_en-US.msi     # Windows x64 安装程序
SwitchHostsR_1.0.0_x64.exe          # Windows x64 便携版

# macOS 安装包
SwitchHostsR_1.0.0_x64.dmg          # Intel Mac 安装包
SwitchHostsR_1.0.0_aarch64.dmg      # Apple Silicon Mac 安装包

# Linux 安装包
SwitchHostsR_1.0.0_amd64.deb        # Debian/Ubuntu 安装包
SwitchHostsR_1.0.0_amd64.rpm        # CentOS/RHEL 安装包
SwitchHostsR_1.0.0_amd64.AppImage   # 通用 Linux 应用
```

## 🖥️ Windows 安装

### 方式一：MSI 安装程序（推荐）

1. **下载安装程序**
   ```bash
   # 下载 MSI 安装包
   SwitchHostsR_1.0.0_x64_en-US.msi
   ```

2. **运行安装程序**
   - 双击下载的 `.msi` 文件
   - 如果出现安全警告，点击「更多信息」→「仍要运行」
   - 按照安装向导完成安装

3. **安装选项**
   - **安装路径**：默认为 `C:\Program Files\SwitchHostsR`
   - **开始菜单**：创建开始菜单快捷方式
   - **桌面快捷方式**：创建桌面图标
   - **自动启动**：开机自动启动（可选）

### 方式二：便携版

1. **下载便携版**
   ```bash
   # 下载便携版可执行文件
   SwitchHostsR_1.0.0_x64.exe
   ```

2. **运行应用**
   - 将文件放置在任意目录
   - 双击运行，无需安装
   - 配置文件将保存在同目录下

### Windows 权限配置

由于需要修改系统 hosts 文件，应用需要管理员权限：

```powershell
# 方式一：右键以管理员身份运行
右键点击应用图标 → 以管理员身份运行

# 方式二：设置始终以管理员身份运行
右键点击应用图标 → 属性 → 兼容性 → 以管理员身份运行此程序
```

## 🍎 macOS 安装

### 标准安装流程

1. **下载 DMG 文件**
   ```bash
   # Intel Mac
   SwitchHostsR_1.0.0_x64.dmg
   
   # Apple Silicon Mac
   SwitchHostsR_1.0.0_aarch64.dmg
   ```

2. **安装应用**
   - 双击下载的 `.dmg` 文件
   - 将 Switch Hosts R 拖拽到 Applications 文件夹
   - 弹出安装镜像

3. **首次运行**
   - 在 Launchpad 或 Applications 文件夹中找到应用
   - 首次运行可能需要在「系统偏好设置」→「安全性与隐私」中允许

### macOS 权限配置

**系统权限设置**
```bash
# 授予完全磁盘访问权限
系统偏好设置 → 安全性与隐私 → 隐私 → 完全磁盘访问权限 → 添加 Switch Hosts R

# 允许应用修改系统文件
sudo chmod +w /etc/hosts
```

**Homebrew 安装（可选）**
```bash
# 通过 Homebrew Cask 安装
brew install --cask switchhosts-r

# 更新应用
brew upgrade --cask switchhosts-r
```

## 🐧 Linux 安装

### Debian/Ubuntu 系统

```bash
# 下载 DEB 包
wget https://github.com/JS-mark/switchHostsR/releases/download/v1.0.0/SwitchHostsR_1.0.0_amd64.deb

# 安装应用
sudo dpkg -i SwitchHostsR_1.0.0_amd64.deb

# 修复依赖（如果需要）
sudo apt-get install -f

# 启动应用
switchhosts-r
```

### CentOS/RHEL 系统

```bash
# 下载 RPM 包
wget https://github.com/JS-mark/switchHostsR/releases/download/v1.0.0/SwitchHostsR_1.0.0_amd64.rpm

# 安装应用
sudo rpm -i SwitchHostsR_1.0.0_amd64.rpm

# 或使用 yum/dnf
sudo yum install SwitchHostsR_1.0.0_amd64.rpm

# 启动应用
switchhosts-r
```

### AppImage 通用版本

```bash
# 下载 AppImage
wget https://github.com/JS-mark/switchHostsR/releases/download/v1.0.0/SwitchHostsR_1.0.0_amd64.AppImage

# 添加执行权限
chmod +x SwitchHostsR_1.0.0_amd64.AppImage

# 运行应用
./SwitchHostsR_1.0.0_amd64.AppImage
```

### Linux 权限配置

```bash
# 确保 hosts 文件可写
sudo chmod 666 /etc/hosts

# 或者将用户添加到相应组
sudo usermod -a -G sudo $USER

# 创建桌面快捷方式
cat > ~/.local/share/applications/switchhosts-r.desktop << EOF
[Desktop Entry]
Name=Switch Hosts R
Comment=Modern hosts file manager
Exec=/path/to/SwitchHostsR
Icon=/path/to/icon.png
Terminal=false
Type=Application
Categories=Development;Network;
EOF
```

## 🔧 首次配置

### 初始化设置

1. **启动应用**
   - 首次启动会显示欢迎界面
   - 选择界面语言（中文/英文/日文）
   - 选择主题模式（浅色/深色/自动）

2. **权限确认**
   ```bash
   # 应用会请求以下权限：
   - 读取/写入 hosts 文件
   - 创建备份文件
   - 网络访问（用于远程 hosts 同步）
   ```

3. **基本配置**
   - **写入模式**：选择覆盖或追加模式
   - **自动备份**：启用自动备份功能
   - **启动设置**：配置开机自启动

### 数据目录

应用数据存储位置：

```bash
# Windows
%APPDATA%\SwitchHostsR

# macOS
~/Library/Application Support/SwitchHostsR

# Linux
~/.config/SwitchHostsR
```

目录结构：
```
SwitchHostsR/
├── config.json          # 应用配置
├── hosts.db            # hosts 数据库
├── backups/            # 备份文件
├── logs/               # 日志文件
└── themes/             # 自定义主题
```

## ✅ 安装验证

### 功能测试

1. **基本功能**
   ```bash
   # 检查应用是否正常启动
   # 检查是否能读取当前 hosts 文件
   # 测试创建新的 hosts 配置
   ```

2. **权限测试**
   ```bash
   # 测试是否能修改 hosts 文件
   # 测试备份功能是否正常
   # 测试恢复功能是否可用
   ```

### 常见问题排查

**应用无法启动**
```bash
# 检查系统要求是否满足
# 检查是否有足够的磁盘空间
# 查看错误日志文件
```

**权限问题**
```bash
# Windows: 确保以管理员身份运行
# macOS: 检查安全性与隐私设置
# Linux: 确保用户有 sudo 权限
```

**网络问题**
```bash
# 检查防火墙设置
# 确认网络连接正常
# 检查代理配置
```

## 🔄 更新升级

### 自动更新

应用内置自动更新功能：
- 启动时检查新版本
- 后台下载更新包
- 提示用户安装更新

### 手动更新

```bash
# 1. 备份当前配置
# 2. 下载新版本安装包
# 3. 卸载旧版本（可选）
# 4. 安装新版本
# 5. 恢复配置文件
```

## 🗑️ 卸载应用

### Windows 卸载
```bash
# 方式一：控制面板卸载
控制面板 → 程序和功能 → Switch Hosts R → 卸载

# 方式二：设置应用卸载
设置 → 应用 → Switch Hosts R → 卸载
```

### macOS 卸载
```bash
# 删除应用
rm -rf /Applications/SwitchHostsR.app

# 清理配置文件
rm -rf ~/Library/Application\ Support/SwitchHostsR
rm -rf ~/Library/Preferences/com.switchhosts.r.plist
```

### Linux 卸载
```bash
# DEB 包卸载
sudo apt remove switchhosts-r

# RPM 包卸载
sudo rpm -e switchhosts-r

# 清理配置文件
rm -rf ~/.config/SwitchHostsR
```

---

安装完成后，您可以继续阅读 [基本使用](/guide/basic-usage) 来学习如何使用 Switch Hosts R。