# 常见问题 (FAQ)

这里收集了用户在使用 Switch Hosts R 过程中遇到的常见问题和解决方案。如果您的问题没有在这里找到答案，请通过 [GitHub Issues](https://github.com/switchhosts/switchhosts-r/issues) 或 [社区讨论](https://github.com/switchhosts/switchhosts-r/discussions) 联系我们。

## 🚀 安装和启动

### Q: 安装时提示需要管理员权限，为什么？

**A:** Switch Hosts R 需要修改系统的 Hosts 文件，这个文件位于系统保护目录中：
- **Windows**: `C:\Windows\System32\drivers\etc\hosts`
- **macOS**: `/etc/hosts`
- **Linux**: `/etc/hosts`

修改这些文件需要管理员权限来确保系统安全。应用只在必要时请求权限，不会进行其他系统级操作。

### Q: macOS 上提示「无法打开应用，因为它来自身份不明的开发者」

**A:** 这是 macOS 的安全机制。解决方法：

1. **方法一**：右键点击应用 → 选择「打开」→ 在弹出对话框中点击「打开」
2. **方法二**：
   ```bash
   sudo xattr -rd com.apple.quarantine /Applications/SwitchHostsR.app
   ```
3. **方法三**：在「系统偏好设置」→「安全性与隐私」→「通用」中点击「仍要打开」

### Q: Linux 上启动时提示缺少依赖

**A:** Switch Hosts R 基于 WebKit，需要以下依赖：

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install webkit2gtk-4.0 libgtk-3-0 libayatana-appindicator3-1

# CentOS/RHEL/Fedora
sudo dnf install webkit2gtk3 gtk3 libappindicator-gtk3

# Arch Linux
sudo pacman -S webkit2gtk gtk3
```

### Q: 启动后界面显示空白或加载失败

**A:** 可能的原因和解决方案：

1. **清除缓存**：
   - Windows: 删除 `%APPDATA%\com.switchhosts.switchhosts-r\cache`
   - macOS: 删除 `~/Library/Caches/com.switchhosts.switchhosts-r`
   - Linux: 删除 `~/.cache/switchhosts-r`

2. **重置配置**：
   - 重命名配置目录，让应用重新初始化
   - 配置目录位置见[数据迁移](/download/#数据迁移)部分

3. **检查防火墙**：确保防火墙没有阻止应用的网络访问

4. **更新显卡驱动**：确保显卡驱动是最新版本

## 🔧 功能使用

### Q: 如何快速切换 Hosts 配置？

**A:** Switch Hosts R 提供多种快速切换方式：

1. **主界面切换**：在主界面的配置列表中点击配置名称旁的开关
2. **快捷键**：
   - `Ctrl/Cmd + 1-9`：快速切换到对应编号的配置
   - `Ctrl/Cmd + Space`：打开快速切换面板
3. **系统托盘**：右键点击托盘图标，在菜单中选择配置
4. **命令行**：使用 CLI 工具（如果已安装）

### Q: 如何创建和管理 Hosts 分组？

**A:** 分组功能帮助您更好地组织 Hosts 配置：

1. **创建分组**：
   - 点击左侧面板的「+」按钮
   - 选择「新建分组」
   - 输入分组名称和描述

2. **移动配置到分组**：
   - 拖拽配置到目标分组
   - 或右键配置 → 「移动到分组」

3. **分组操作**：
   - 重命名：右键分组 → 「重命名」
   - 删除：右键分组 → 「删除」（配置会移到根目录）
   - 批量操作：选中分组 → 「启用/禁用所有配置」

### Q: 远程 Hosts 配置如何使用？

**A:** 远程 Hosts 配置允许您从网络获取 Hosts 规则：

1. **添加远程配置**：
   - 点击「+」→ 「远程 Hosts」
   - 输入 URL（支持 HTTP/HTTPS）
   - 设置更新频率和超时时间

2. **支持的格式**：
   - 标准 Hosts 格式
   - AdBlock Plus 格式
   - JSON 格式（自定义）

3. **自动更新**：
   - 可设置自动更新间隔（1小时到30天）
   - 支持手动强制更新
   - 更新失败时会显示错误信息

4. **常用远程源**：
   ```
   # 广告屏蔽
   https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts
   
   # 恶意软件防护
   https://someonewhocares.org/hosts/zero/hosts
   
   # 自定义企业规则
   https://your-company.com/internal-hosts.txt
   ```

### Q: 如何备份和恢复 Hosts 配置？

**A:** Switch Hosts R 提供多种备份方式：

1. **自动备份**：
   - 每次修改 Hosts 时自动创建备份
   - 保留最近 30 个备份文件
   - 备份文件位于 `backups/` 目录

2. **手动备份**：
   - 「文件」→「导出配置」
   - 选择导出格式（JSON/ZIP）
   - 可选择导出全部或部分配置

3. **恢复配置**：
   - 「文件」→「导入配置」
   - 支持从备份文件或其他 Hosts 管理工具导入
   - 导入前会提示是否覆盖现有配置

4. **云同步**（企业版）：
   - 支持 Google Drive、Dropbox、OneDrive
   - 多设备自动同步
   - 冲突解决机制

## 🔒 权限和安全

### Q: 为什么每次启动都要输入管理员密码？

**A:** 这取决于您的系统配置和安全策略：

1. **Windows**：
   - 如果启用了 UAC，每次修改 Hosts 都需要确认
   - 可以在 UAC 设置中调整提示级别
   - 不建议完全禁用 UAC

2. **macOS**：
   - 系统要求修改 `/etc/hosts` 时进行身份验证
   - 可以通过以下方式减少提示：
     ```bash
     # 给应用添加 Hosts 文件的写权限（谨慎使用）
     sudo chown $(whoami) /etc/hosts
     ```

3. **Linux**：
   - 可以配置 sudo 规则来避免重复输入密码：
     ```bash
     # 编辑 sudoers 文件
     sudo visudo
     # 添加规则（替换 username 为您的用户名）
     username ALL=(ALL) NOPASSWD: /usr/bin/tee /etc/hosts
     ```

### Q: Switch Hosts R 会收集我的数据吗？

**A:** Switch Hosts R 非常重视用户隐私：

1. **本地优先**：所有数据默认存储在本地，不会上传到服务器
2. **最小化收集**：只收集必要的匿名使用统计（可选择关闭）
3. **透明度**：
   - 开源代码，可审查所有功能
   - 详细的隐私政策说明
   - 数据收集设置完全由用户控制

4. **可选遥测**：
   - 应用启动次数
   - 功能使用频率
   - 错误报告（不包含个人信息）
   - 可在设置中完全禁用

### Q: 如何确保 Hosts 配置的安全性？

**A:** 保护 Hosts 配置安全的建议：

1. **定期备份**：启用自动备份功能
2. **验证来源**：只使用可信的远程 Hosts 源
3. **审查内容**：定期检查 Hosts 规则，删除不需要的条目
4. **访问控制**：
   - 设置应用密码（企业版功能）
   - 限制其他用户的访问权限
5. **监控变化**：启用 Hosts 文件监控，及时发现异常修改

## 🌐 网络和连接

### Q: 修改 Hosts 后网站仍然无法访问

**A:** 可能的原因和解决方案：

1. **DNS 缓存**：清除 DNS 缓存
   ```bash
   # Windows
   ipconfig /flushdns
   
   # macOS
   sudo dscacheutil -flushcache
   
   # Linux
   sudo systemctl restart systemd-resolved
   # 或
   sudo service network-manager restart
   ```

2. **浏览器缓存**：
   - 清除浏览器 DNS 缓存
   - Chrome: 访问 `chrome://net-internals/#dns` 点击 "Clear host cache"
   - Firefox: 重启浏览器

3. **代理设置**：检查是否使用了代理服务器
4. **防火墙**：确保防火墙没有阻止连接
5. **Hosts 语法**：检查 Hosts 规则语法是否正确

### Q: 如何测试 Hosts 配置是否生效？

**A:** 多种测试方法：

1. **命令行测试**：
   ```bash
   # 测试域名解析
   nslookup example.com
   
   # 测试连接
   ping example.com
   
   # 查看路由
   traceroute example.com  # macOS/Linux
   tracert example.com     # Windows
   ```

2. **浏览器测试**：
   - 访问目标网站
   - 检查开发者工具中的网络请求
   - 查看 IP 地址是否符合预期

3. **应用内测试**：
   - Switch Hosts R 提供内置的连接测试工具
   - 「工具」→「连接测试」
   - 输入域名查看解析结果

### Q: 某些应用不遵循 Hosts 设置

**A:** 一些应用可能绕过系统 Hosts 文件：

1. **使用自定义 DNS**：
   - 某些应用内置 DNS 服务器
   - 解决方案：在应用设置中禁用自定义 DNS

2. **使用 DoH/DoT**：
   - DNS over HTTPS 或 DNS over TLS
   - 解决方案：禁用安全 DNS 功能

3. **硬编码 IP**：
   - 应用直接使用 IP 地址
   - 解决方案：使用防火墙规则阻止

4. **常见应用处理**：
   - **Chrome**: 禁用 "使用安全 DNS"
   - **Firefox**: 设置 `network.trr.mode` 为 5
   - **Steam**: 添加启动参数 `-tcp`

## 📝 编辑和语法

### Q: Hosts 文件的语法规则是什么？

**A:** 标准 Hosts 文件语法：

```bash
# 基本格式：IP地址 域名 [别名...]
127.0.0.1 localhost
192.168.1.100 dev.example.com

# 支持多个别名
192.168.1.100 api.example.com api

# 注释行（以 # 开头）
# 这是注释

# 禁用域名（指向本地）
0.0.0.0 ads.example.com
127.0.0.1 tracker.example.com

# IPv6 支持
::1 localhost
2001:db8::1 ipv6.example.com

# 空行会被忽略

```

**语法要点**：
- IP 地址和域名之间用空格或制表符分隔
- 每行一个规则
- 支持 IPv4 和 IPv6
- 大小写不敏感
- 不支持通配符（但可以用工具生成）

### Q: 如何使用 Monaco Editor 的高级功能？

**A:** Switch Hosts R 集成了 Monaco Editor，提供丰富的编辑功能：

1. **语法高亮**：自动识别 IP 地址、域名、注释
2. **自动补全**：
   - 输入 IP 地址时提供常用选项
   - 域名自动补全历史记录
3. **错误检测**：
   - 无效 IP 地址会标红
   - 重复规则会警告
   - 语法错误会下划线提示
4. **快捷键**：
   - `Ctrl/Cmd + F`：查找
   - `Ctrl/Cmd + H`：替换
   - `Ctrl/Cmd + /`：注释/取消注释
   - `Alt + Up/Down`：移动行
   - `Shift + Alt + Up/Down`：复制行

### Q: 如何批量编辑 Hosts 规则？

**A:** 多种批量编辑方法：

1. **多选编辑**：
   - 按住 `Ctrl/Cmd` 点击多个位置
   - 同时编辑多个相同内容

2. **查找替换**：
   - `Ctrl/Cmd + H` 打开替换面板
   - 支持正则表达式
   - 可以替换全部或逐个确认

3. **列选择**：
   - 按住 `Alt` 拖拽选择矩形区域
   - 适合对齐格式或批量修改 IP

4. **脚本处理**：
   - 导出为文本文件
   - 使用脚本批量处理
   - 重新导入

## 🔄 同步和协作

### Q: 如何在多台设备间同步配置？

**A:** Switch Hosts R 提供多种同步方案：

1. **云存储同步**（企业版）：
   - 支持 Google Drive、Dropbox、OneDrive
   - 自动同步配置和设置
   - 冲突解决机制

2. **Git 同步**：
   - 将配置目录初始化为 Git 仓库
   - 推送到 GitHub/GitLab 私有仓库
   - 在其他设备上克隆仓库

3. **手动同步**：
   - 导出配置为 ZIP 文件
   - 通过网盘或 U 盘传输
   - 在目标设备上导入

4. **网络共享**：
   - 将配置目录放在网络共享文件夹
   - 多台设备访问同一配置
   - 注意文件锁定问题

### Q: 团队如何协作管理 Hosts 配置？

**A:** 团队协作的最佳实践：

1. **集中管理**：
   - 使用企业版的中央管理功能
   - 管理员统一分发配置
   - 权限控制和审批流程

2. **版本控制**：
   - 使用 Git 管理配置文件
   - 分支管理不同环境
   - Pull Request 审查机制

3. **远程配置**：
   - 在内网服务器托管 Hosts 文件
   - 团队成员订阅远程配置
   - 自动更新和通知

4. **文档化**：
   - 为每个配置添加详细说明
   - 维护变更日志
   - 建立使用规范

## 🚨 故障排除

### Q: 应用崩溃或无响应怎么办？

**A:** 故障排除步骤：

1. **收集信息**：
   - 记录崩溃时的操作
   - 查看错误日志
   - 截图错误信息

2. **基本排查**：
   - 重启应用
   - 重启系统
   - 检查系统资源使用情况

3. **重置配置**：
   - 备份当前配置
   - 重命名配置目录
   - 重新启动应用

4. **日志分析**：
   - 查看应用日志文件
   - 日志位置：配置目录下的 `logs/` 文件夹
   - 寻找 ERROR 或 FATAL 级别的消息

5. **报告问题**：
   - 访问 [GitHub Issues](https://github.com/switchhosts/switchhosts-r/issues)
   - 提供详细的错误信息和系统环境
   - 附上相关日志文件

### Q: 如何启用调试模式？

**A:** 调试模式可以提供更详细的日志信息：

1. **启动参数**：
   ```bash
   # Windows
   SwitchHostsR.exe --debug
   
   # macOS
   /Applications/SwitchHostsR.app/Contents/MacOS/SwitchHostsR --debug
   
   # Linux
   switchhosts-r --debug
   ```

2. **环境变量**：
   ```bash
   export RUST_LOG=debug
   export SWITCHHOSTS_DEBUG=1
   ```

3. **配置文件**：
   - 编辑 `config.json`
   - 设置 `"log_level": "debug"`

4. **开发者工具**：
   - 按 `F12` 或 `Ctrl/Cmd + Shift + I`
   - 查看控制台输出
   - 检查网络请求

### Q: 性能问题如何优化？

**A:** 性能优化建议：

1. **减少配置数量**：
   - 删除不需要的配置
   - 合并相似的配置
   - 使用分组管理

2. **优化 Hosts 规则**：
   - 删除重复规则
   - 移除无效域名
   - 使用更高效的 IP 地址

3. **系统优化**：
   - 关闭不必要的后台程序
   - 增加系统内存
   - 使用 SSD 硬盘

4. **应用设置**：
   - 禁用不需要的功能
   - 调整更新频率
   - 减少日志级别

## 📞 获取帮助

### Q: 如何联系技术支持？

**A:** 多种联系方式：

1. **GitHub Issues**：[https://github.com/switchhosts/switchhosts-r/issues](https://github.com/switchhosts/switchhosts-r/issues)
   - 适合：Bug 报告、功能请求
   - 响应时间：1-3 个工作日

2. **社区讨论**：[https://github.com/switchhosts/switchhosts-r/discussions](https://github.com/switchhosts/switchhosts-r/discussions)
   - 适合：使用问题、经验分享
   - 社区驱动，响应较快

3. **邮件支持**：[support@switchhosts.com](mailto:support@switchhosts.com)
   - 适合：企业用户、紧急问题
   - 响应时间：24 小时内

4. **在线文档**：
   - [用户指南](/guide/)
   - [API 文档](/api/)
   - [更新日志](/changelog/)

### Q: 如何提交有效的 Bug 报告？

**A:** 有效 Bug 报告应包含：

1. **环境信息**：
   - 操作系统版本
   - Switch Hosts R 版本
   - 系统架构（x64/ARM）

2. **重现步骤**：
   - 详细的操作步骤
   - 预期结果 vs 实际结果
   - 是否能稳定重现

3. **错误信息**：
   - 错误截图
   - 日志文件
   - 错误代码

4. **相关配置**：
   - Hosts 配置内容（脱敏处理）
   - 应用设置
   - 网络环境

**模板示例**：
```markdown
## 环境信息
- OS: Windows 11 22H2
- App Version: 1.0.0
- Architecture: x64

## 问题描述
应用在切换配置时崩溃

## 重现步骤
1. 打开应用
2. 点击配置 "开发环境"
3. 立即点击配置 "生产环境"
4. 应用崩溃

## 预期结果
配置正常切换

## 实际结果
应用崩溃，显示错误对话框

## 附加信息
- 错误截图：[附件]
- 日志文件：[附件]
- 配置文件：[附件]
```

---

**还有其他问题？**

如果您的问题没有在这里找到答案，请不要犹豫联系我们。我们致力于为所有用户提供最好的支持体验。

- 🔍 [搜索现有问题](https://github.com/switchhosts/switchhosts-r/issues)
- 💬 [加入社区讨论](https://github.com/switchhosts/switchhosts-r/discussions)
- 📧 [发送邮件](mailto:support@switchhosts.com)
- 📖 [查看文档](/guide/)

**文档更新**：本 FAQ 会根据用户反馈持续更新。如果您发现任何错误或有改进建议，欢迎提交 Pull Request。