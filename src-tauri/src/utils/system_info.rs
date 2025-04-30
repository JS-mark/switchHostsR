use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use sysinfo::System;

/// 获取本地 IP 地址
pub fn get_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

/// 系统信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    /// 总内存 (GB)
    memory_total: f32,
    /// 已使用内存 (GB)
    memory_used: f32,
    /// CPU 使用率 (%)
    cpu_used: f32,
    /// 交换分区总大小 (GB)
    swap_total: f32,
    /// 已使用交换分区 (GB)
    swap_used: f32,
    /// 主机名
    hostname: String,
    /// CPU 核心数
    cpus: usize,
    /// 操作系统名称
    os_name: String,
    /// 操作系统版本
    os_version: String,
    /// IP 地址
    ip: String,
}

/// 获取系统信息
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 计算 CPU 使用率
    let mut cpu_used = 0.0;
    for cpu in sys.cpus() {
        cpu_used += cpu.cpu_usage();
    }
    cpu_used /= sys.cpus().len() as f32;

    // 计算内存信息（转换为 GB）
    let memory_total = sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
    let memory_used = sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
    let swap_total = sys.total_swap() as f32 / 1024.0 / 1024.0 / 1024.0;
    let swap_used = sys.used_swap() as f32 / 1024.0 / 1024.0 / 1024.0;

    // 获取系统信息
    let hostname = sysinfo::System::host_name().unwrap_or_else(|| "未知".to_string());
    let os_name = sysinfo::System::name().unwrap_or_else(|| "未知".to_string());
    let os_version = sysinfo::System::os_version().unwrap_or_else(|| "未知".to_string());
    let ip = get_ip().unwrap_or_else(|| "未知".to_string());

    SystemInfo {
        memory_total,
        memory_used,
        cpu_used,
        swap_total,
        swap_used,
        hostname,
        cpus: sys.cpus().len(),
        os_name,
        os_version,
        ip,
    }
}

/// 获取格式化的系统信息字符串
pub fn get_system_info_string() -> String {
    let info = get_system_info();
    format!(
        "系统信息:\n\
        主机名: {}\n\
        IP: {}\n\
        操作系统: {} {}\n\
        CPU 核心数: {}\n\
        CPU 使用率: {:.2}%\n\
        内存: {:.2}/{:.2} GB ({:.2}%)\n\
        交换分区: {:.2}/{:.2} GB ({:.2}%)",
        info.hostname,
        info.ip,
        info.os_name,
        info.os_version,
        info.cpus,
        info.cpu_used,
        info.memory_used,
        info.memory_total,
        if info.memory_total > 0.0 {
            info.memory_used / info.memory_total * 100.0
        } else {
            0.0
        },
        info.swap_used,
        info.swap_total,
        if info.swap_total > 0.0 {
            info.swap_used / info.swap_total * 100.0
        } else {
            0.0
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_info() {
        let info = get_system_info();

        // 验证系统信息是否合理
        assert!(info.memory_total > 0.0, "总内存应该大于0");
        assert!(
            info.memory_used <= info.memory_total,
            "已用内存不应超过总内存"
        );
        assert!(
            info.cpu_used >= 0.0 && info.cpu_used <= 100.0,
            "CPU使用率应在0-100%之间"
        );
        assert!(info.cpus > 0, "CPU核心数应该大于0");
        assert!(!info.hostname.is_empty(), "主机名不应为空");
        assert!(!info.os_name.is_empty(), "操作系统名称不应为空");
    }

    #[test]
    fn test_get_ip() {
        // IP地址可能获取不到，所以只测试函数不崩溃
        let _ = get_ip();
    }
}
