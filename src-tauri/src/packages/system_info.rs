use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use sysinfo::System;

pub fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    memory_total: f32,
    memory_used: f32,
    cpu_used: f32,
    swap_total: f32,
    swap_used: f32,
    hostname: String,
    cpus: usize,
    os_name: String,
    os_version: String,
    ip: String,
}

/**
 * 获取系统信息
 */
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo {
        // 内存总数
        memory_total: sys.total_memory() as f32 / (1024 * 1024 * 1024) as f32,
        memory_used: sys.used_memory() as f32 / (1024 * 1024 * 1024) as f32,
        cpu_used: sys.global_cpu_info().cpu_usage(),
        hostname: System::host_name().unwrap(),
        swap_total: sys.total_swap() as f32 / (1024 * 1024 * 1024) as f32,
        swap_used: sys.used_swap() as f32 / (1024 * 1024 * 1024) as f32,
        os_name: System::name().unwrap(),
        os_version: System::os_version().unwrap(),
        cpus: sys.cpus().len(),
        ip: get_ip().unwrap(),
    }
}
