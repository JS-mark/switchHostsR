//! 日志 API 模块
//!
//! 提供与日志相关的前端 API 接口

use crate::db::handlers::LogHandler;
use crate::db::models::Log;
use crate::ui::app_state::AppState;
use tauri::{State, command};

/// 获取所有日志
///
/// 需要管理员权限
#[command]
pub async fn get_all_logs(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<Log>, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.get_all_logs(limit.unwrap_or(100))
        .map_err(|e| e.to_string())
}

/// 获取用户的日志
///
/// 用户只能查看自己的日志，管理员可以查看所有用户的日志
#[command]
pub async fn get_user_logs(
    state: State<'_, AppState>,
    user_id: i32,
    limit: Option<i64>,
) -> Result<Vec<Log>, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.get_user_logs(user_id, limit.unwrap_or(50))
        .map_err(|e| e.to_string())
}

/// 添加日志
#[command]
pub async fn add_log(
    state: State<'_, AppState>,
    log: Log,
) -> Result<Log, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.add_log(log)
        .map_err(|e| e.to_string())
}

/// 清理旧日志（仅管理员）
#[command]
pub async fn clean_old_logs(
    state: State<'_, AppState>,
    keep_count: i64,
) -> Result<usize, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.clean_old_logs(keep_count)
        .map_err(|e| e.to_string())
}
