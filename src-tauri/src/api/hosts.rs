//! 主机 API 模块
//!
//! 提供与主机相关的前端 API 接口

use crate::db::handlers::HostHandler;
use crate::db::models::Host;
use crate::ui::app_state::AppState;
use tauri::{command, State};

/// 创建主机
#[command]
pub async fn create_host(state: State<'_, AppState>, host: Host) -> Result<Host, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db.create_host(host).map_err(|e| e.to_string())
}

/// 获取主机详情
#[command]
pub async fn get_host(state: State<'_, AppState>, host_id: i32) -> Result<Host, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db.get_host_by_id(host_id).map_err(|e| e.to_string())
}

/// 获取用户的所有主机
#[command]
pub async fn get_user_hosts(state: State<'_, AppState>, user_id: i32) -> Result<Vec<Host>, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db
        .get_hosts_by_user(user_id)
        .map_err(|e| e.to_string())
}

/// 更新主机
#[command]
pub async fn update_host(
    state: State<'_, AppState>,
    host_id: i32,
    host: Host,
) -> Result<Host, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db
        .update_host(host_id, host)
        .map_err(|e| e.to_string())
}

/// 激活/停用主机
#[command]
pub async fn toggle_host_active(
    state: State<'_, AppState>,
    host_id: i32,
    active: bool,
) -> Result<(), String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db
        .toggle_host_active(host_id, active)
        .map_err(|e| e.to_string())
}

/// 删除主机
#[command]
pub async fn delete_host(state: State<'_, AppState>, host_id: i32) -> Result<(), String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db.delete_host(host_id).map_err(|e| e.to_string())
}

/// 获取所有激活的主机
#[command]
pub async fn get_active_hosts(state: State<'_, AppState>) -> Result<Vec<Host>, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db.get_active_hosts().map_err(|e| e.to_string())
}
