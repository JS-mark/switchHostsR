//! 用户 API 模块
//!
//! 提供与用户相关的前端 API 接口

use crate::db::handlers::UserHandler;
use crate::db::models::User;
use crate::ui::app_state::AppState;
use tauri::{State, command};

/// 用户登录
#[command]
pub async fn user_login(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<User, String> {
    let users_db = state.users_db.lock().await;
    users_db.login(username, password)
        .map_err(|e| e.to_string())
}

/// 用户登出
#[command]
pub async fn logout(
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 重置当前用户ID
    let mut current_user_id = state.current_user_id.lock().await;
    *current_user_id = 0;

    // 重新初始化数据库处理器
    let pool = state.db_pool.clone();

    let mut users_db = state.users_db.lock().await;
    *users_db = UserHandler::new(pool.clone(), 0)
        .map_err(|e| e.to_string())?;

    let mut hosts_db = state.hosts_db.lock().await;
    *hosts_db = crate::db::handlers::HostHandler::new(pool.clone(), 0)
        .map_err(|e| e.to_string())?;

    let mut host_groups_db = state.host_groups_db.lock().await;
    *host_groups_db = crate::db::handlers::HostGroupHandler::new(pool.clone(), 0)
        .map_err(|e| e.to_string())?;

    let mut logs_db = state.logs_db.lock().await;
    *logs_db = crate::db::handlers::LogHandler::new(pool.clone(), 0)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取当前用户
#[command]
pub async fn get_current_user(
    state: State<'_, AppState>,
) -> Result<User, String> {
    let current_user_id = *state.current_user_id.lock().await;
    if current_user_id == 0 {
        return Err("未登录".to_string());
    }

    let users_db = state.users_db.lock().await;
    users_db.get_user_by_id(current_user_id)
        .map_err(|e| e.to_string())
}

/// 获取所有用户（仅管理员）
#[command]
pub async fn get_all_users(
    state: State<'_, AppState>,
) -> Result<Vec<User>, String> {
    let users_db = state.users_db.lock().await;
    users_db.get_all_users()
        .map_err(|e| e.to_string())
}

/// 创建用户（仅管理员）
#[command]
pub async fn create_user(
    state: State<'_, AppState>,
    user: User,
) -> Result<User, String> {
    let users_db = state.users_db.lock().await;
    users_db.create_user(user)
        .map_err(|e| e.to_string())
}

/// 更新用户信息
#[command]
pub async fn update_user(
    state: State<'_, AppState>,
    user_id: i32,
    user: User,
) -> Result<User, String> {
    let users_db = state.users_db.lock().await;
    users_db.update_user(user_id, user)
        .map_err(|e| e.to_string())
}

/// 删除用户（仅管理员）
#[command]
pub async fn delete_user(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<(), String> {
    let users_db = state.users_db.lock().await;
    users_db.delete_user(user_id)
        .map_err(|e| e.to_string())
}

/// 修改密码
#[command]
pub async fn change_password(
    state: State<'_, AppState>,
    user_id: i32,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    let users_db = state.users_db.lock().await;
    users_db.change_password(user_id, old_password, new_password)
        .map_err(|e| e.to_string())
}
