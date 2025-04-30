//! 主机组 API 模块
//!
//! 提供与主机组相关的前端 API 接口

use crate::db::models::{HostGroup, HostGroupRelation};
use crate::ui::app_state::AppState;
use tauri::{command, State};

/// 创建主机组
#[command]
pub fn create_host_group(state: State<AppState>, group: HostGroup) -> Result<HostGroup, String> {
    // 直接使用 state 的方法获取数据库处理器
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .create_group(group)
        .map_err(|e| e.to_string())
}

/// 获取用户的所有主机组
#[command]
pub fn get_user_host_groups(
    state: State<AppState>,
    user_id: i32,
) -> Result<Vec<HostGroup>, String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .get_groups_by_user(user_id)
        .map_err(|e| e.to_string())
}

/// 获取主机组详情
#[command]
pub fn get_host_group(state: State<AppState>, group_id: i32) -> Result<HostGroup, String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .get_group_by_id(group_id)
        .map_err(|e| e.to_string())
}

/// 更新主机组
#[command]
pub fn update_host_group(
    state: State<AppState>,
    group_id: i32,
    group: HostGroup,
) -> Result<HostGroup, String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .update_group(group_id, group)
        .map_err(|e| e.to_string())
}

/// 删除主机组
#[command]
pub fn delete_host_group(state: State<AppState>, group_id: i32) -> Result<(), String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .delete_group(group_id)
        .map_err(|e| e.to_string())
}

/// 向主机组添加主机
#[command]
pub fn add_host_to_group(
    state: State<AppState>,
    group_id: i32,
    host_id: i32,
) -> Result<HostGroupRelation, String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .add_host_to_group(group_id, host_id)
        .map_err(|e| e.to_string())
}

/// 从主机组移除主机
#[command]
pub fn remove_host_from_group(
    state: State<AppState>,
    group_id: i32,
    host_id: i32,
) -> Result<(), String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .remove_host_from_group(group_id, host_id)
        .map_err(|e| e.to_string())
}

/// 获取主机组中的所有主机
#[command]
pub fn get_hosts_in_group(
    state: State<AppState>,
    group_id: i32,
) -> Result<Vec<crate::db::models::Host>, String> {
    let host_groups_db = state.get_host_groups_db();
    host_groups_db
        .get_hosts_in_group(group_id)
        .map_err(|e| e.to_string())
}
