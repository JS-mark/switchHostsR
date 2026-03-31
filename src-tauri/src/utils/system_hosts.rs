//! 系统 hosts 文件管理
//!
//! 负责将数据库中激活的 hosts 合并并写入系统 hosts 文件，
//! 以及刷新 DNS 缓存。支持 macOS / Windows / Linux 三平台。

use anyhow::Result;
use log::{info, warn};
use serde::Serialize;
use std::process::Command;
use uuid::Uuid;

/// hosts 写入结果
#[derive(Debug, Serialize)]
pub struct ApplyHostsResult {
    /// 是否成功写入
    pub success: bool,
    /// 写入的 hosts 条目数
    pub entries_count: usize,
    /// 是否成功刷新 DNS
    pub dns_flushed: bool,
    /// 详细信息
    pub message: String,
}

/// 获取系统 hosts 文件路径
fn get_system_hosts_path() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        r"C:\Windows\System32\drivers\etc\hosts"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "/etc/hosts"
    }
}

/// SwitchHostsR 管理区域的标记
const MARKER_START: &str = "# ===== SwitchHostsR Managed Start =====";
const MARKER_END: &str = "# ===== SwitchHostsR Managed End =====";

/// 合并所有激活 hosts 内容
///
/// 保留系统 hosts 文件中 SwitchHostsR 标记之外的内容，
/// 将新的 hosts 内容插入到标记区域内。
fn build_merged_content(
    original_content: &str,
    active_hosts: &[(String, String)],
) -> String {
    let mut result = String::new();

    // 提取原始文件中非 SwitchHostsR 管理区域的内容
    let mut in_managed_section = false;
    for line in original_content.lines() {
        if line.trim() == MARKER_START {
            in_managed_section = true;
            continue;
        }
        if line.trim() == MARKER_END {
            in_managed_section = false;
            continue;
        }
        if !in_managed_section {
            result.push_str(line);
            result.push('\n');
        }
    }

    // 移除末尾多余的空行，保留一个换行
    let trimmed = result.trim_end();
    result = trimmed.to_string();
    if !result.is_empty() {
        result.push('\n');
    }
    result.push('\n');

    // 添加 SwitchHostsR 管理区域
    result.push_str(MARKER_START);
    result.push('\n');
    result.push_str("# 由 SwitchHostsR 自动生成，请勿手动修改此区域\n");
    result.push_str(&format!(
        "# 更新时间: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    result.push('\n');

    for (name, content) in active_hosts {
        result.push_str(&format!("# --- {} ---\n", name));
        for line in content.lines() {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() {
                result.push_str(trimmed_line);
                result.push('\n');
            }
        }
        result.push('\n');
    }

    result.push_str(MARKER_END);
    result.push('\n');

    result
}

/// 读取当前系统 hosts 文件内容
pub fn read_system_hosts() -> Result<String> {
    let path = get_system_hosts_path();
    std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("读取系统 hosts 文件失败: {}", e))
}

/// 写入系统 hosts 文件
///
/// 在 macOS/Linux 上需要 sudo 权限，使用 tauri-plugin-shell 或直接 Command。
/// `active_hosts`: Vec<(名称, 内容)>
pub fn write_system_hosts(active_hosts: &[(String, String)]) -> Result<ApplyHostsResult> {
    let hosts_path = get_system_hosts_path();

    // 读取现有内容
    let original_content = std::fs::read_to_string(hosts_path).unwrap_or_default();

    // 合并内容
    let merged = build_merged_content(&original_content, active_hosts);
    let entries_count = active_hosts.len();

    // 写入文件
    // macOS/Linux 需要提权，使用临时文件 + 平台提权方案
    #[cfg(not(target_os = "windows"))]
    {
        // 使用随机文件名避免并发竞态条件
        let temp_filename = format!("switchhostsr_hosts_{}", Uuid::new_v4());
        let temp_path = std::env::temp_dir().join(temp_filename);
        std::fs::write(&temp_path, &merged)
            .map_err(|e| anyhow::anyhow!("写入临时文件失败: {}", e))?;

        // macOS 使用 osascript 弹出系统权限对话框，比 sudo 更适合 GUI 环境
        #[cfg(target_os = "macos")]
        let output = {
            let script = format!(
                "do shell script \"cp '{}' '{}'\" with administrator privileges",
                temp_path.to_string_lossy(),
                hosts_path
            );
            Command::new("osascript")
                .args(["-e", &script])
                .output()
                .map_err(|e| anyhow::anyhow!("执行 osascript 提权失败: {}", e))?
        };

        // Linux 优先使用 pkexec（GUI 友好），回退到 sudo
        #[cfg(target_os = "linux")]
        let output = {
            let pkexec_result = Command::new("pkexec")
                .args(["cp", &temp_path.to_string_lossy(), hosts_path])
                .output();

            match pkexec_result {
                Ok(output) if output.status.success() => output,
                _ => {
                    // 回退到 sudo
                    warn!("pkexec 不可用，尝试使用 sudo");
                    Command::new("sudo")
                        .args(["cp", &temp_path.to_string_lossy(), hosts_path])
                        .output()
                        .map_err(|e| anyhow::anyhow!("执行 sudo cp 失败: {}", e))?
                }
            }
        };

        // 清理临时文件
        let _ = std::fs::remove_file(&temp_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "写入系统 hosts 文件失败。请确保已授予管理员权限。\n错误详情: {}",
                stderr
            ));
        }
    }

    #[cfg(target_os = "windows")]
    {
        std::fs::write(hosts_path, &merged)
            .map_err(|e| anyhow::anyhow!(
                "写入系统 hosts 文件失败。请以管理员身份运行应用程序。\n错误详情: {}",
                e
            ))?;
    }

    info!("系统 hosts 文件更新成功，共 {} 个规则组", entries_count);

    // 刷新 DNS 缓存
    let dns_flushed = flush_dns_cache();

    Ok(ApplyHostsResult {
        success: true,
        entries_count,
        dns_flushed,
        message: format!(
            "成功应用 {} 个 hosts 规则组{}",
            entries_count,
            if dns_flushed {
                "，DNS 缓存已刷新"
            } else {
                "，DNS 缓存刷新失败"
            }
        ),
    })
}

/// 刷新 DNS 缓存
///
/// 针对不同操作系统执行对应的 DNS 缓存刷新命令
fn flush_dns_cache() -> bool {
    let result = flush_dns_cache_internal();
    match result {
        Ok(_) => {
            info!("DNS 缓存已成功刷新");
            true
        }
        Err(e) => {
            warn!("DNS 缓存刷新失败: {}", e);
            false
        }
    }
}

fn flush_dns_cache_internal() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("sudo")
            .args(["dscacheutil", "-flushcache"])
            .output()
            .map_err(|e| anyhow::anyhow!("执行 dscacheutil 失败: {}", e))?;

        if !output.status.success() {
            warn!(
                "dscacheutil 执行失败: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // macOS 还需要重启 mDNSResponder
        let output2 = Command::new("sudo")
            .args(["killall", "-HUP", "mDNSResponder"])
            .output()
            .map_err(|e| anyhow::anyhow!("执行 killall mDNSResponder 失败: {}", e))?;

        if !output2.status.success() {
            warn!(
                "killall mDNSResponder 执行失败: {}",
                String::from_utf8_lossy(&output2.stderr)
            );
        }

        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("ipconfig")
            .arg("/flushdns")
            .output()
            .map_err(|e| anyhow::anyhow!("执行 ipconfig /flushdns 失败: {}", e))?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "ipconfig /flushdns 失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        // 尝试 systemd-resolve
        let output = Command::new("sudo")
            .args(["systemd-resolve", "--flush-caches"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return Ok(());
            }
        }

        // 尝试 resolvectl (systemd 新版命令)
        let output = Command::new("sudo")
            .args(["resolvectl", "flush-caches"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return Ok(());
            }
        }

        // 尝试 nscd
        let output = Command::new("sudo")
            .args(["service", "nscd", "restart"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return Ok(());
            }
        }

        warn!("Linux DNS 缓存刷新：未找到可用的 DNS 缓存服务");
        return Ok(());
    }

    #[allow(unreachable_code)]
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_merged_content() {
        let original = "# 系统 hosts\n127.0.0.1 localhost\n::1 localhost\n";
        let active = vec![
            ("开发环境".to_string(), "127.0.0.1 dev.local\n127.0.0.1 api.local".to_string()),
            ("测试环境".to_string(), "192.168.1.100 test.local".to_string()),
        ];

        let merged = build_merged_content(original, &active);

        assert!(merged.contains("127.0.0.1 localhost"));
        assert!(merged.contains(MARKER_START));
        assert!(merged.contains(MARKER_END));
        assert!(merged.contains("# --- 开发环境 ---"));
        assert!(merged.contains("127.0.0.1 dev.local"));
        assert!(merged.contains("# --- 测试环境 ---"));
        assert!(merged.contains("192.168.1.100 test.local"));
    }

    #[test]
    fn test_build_merged_content_replaces_existing_section() {
        let original = format!(
            "# 系统 hosts\n127.0.0.1 localhost\n\n{}\n# 旧内容\n127.0.0.1 old.local\n{}\n",
            MARKER_START, MARKER_END
        );
        let active = vec![
            ("新规则".to_string(), "127.0.0.1 new.local".to_string()),
        ];

        let merged = build_merged_content(&original, &active);

        assert!(merged.contains("127.0.0.1 localhost"));
        assert!(!merged.contains("127.0.0.1 old.local"));
        assert!(merged.contains("127.0.0.1 new.local"));
        // 确保只有一对标记
        assert_eq!(merged.matches(MARKER_START).count(), 1);
        assert_eq!(merged.matches(MARKER_END).count(), 1);
    }

    #[test]
    fn test_build_merged_content_empty_active() {
        let original = "127.0.0.1 localhost\n";
        let active: Vec<(String, String)> = vec![];

        let merged = build_merged_content(original, &active);

        assert!(merged.contains("127.0.0.1 localhost"));
        assert!(merged.contains(MARKER_START));
        assert!(merged.contains(MARKER_END));
    }

    #[test]
    fn test_get_system_hosts_path() {
        let path = get_system_hosts_path();
        #[cfg(target_os = "windows")]
        assert!(path.contains("drivers"));
        #[cfg(not(target_os = "windows"))]
        assert_eq!(path, "/etc/hosts");
    }
}
