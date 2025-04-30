use crate::db::models::{Host, Log, User};
use crate::ui::app_state::AppState;
use tauri::State;

// 用户相关命令
#[tauri::command]
pub async fn user_login(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<User, String> {
    let user_db = state.user_db.lock().await;
    let user = user_db.get_user_by_username(&username)?;

    // 验证密码
    if bcrypt::verify(&password, &user.password).map_err(|e| e.to_string())? {
        // 记录登录日志
        let logs_db = state.logs_db.lock().await;
        let log = Log {
            id: None,
            user_id: user.id.unwrap(),
            action: "login".to_string(),
            target_type: "user".to_string(),
            target_id: user.id,
            details: Some("用户登录".to_string()),
            created_at: 0, // 会在数据库层自动设置
        };
        logs_db.add_log(log)?;

        Ok(user)
    } else {
        Err("密码错误".to_string())
    }
}

#[tauri::command]
pub async fn get_all_users(state: State<'_, AppState>) -> Result<Vec<User>, String> {
    let user_db = state.user_db.lock().await;
    user_db.get_all_users()
}

#[tauri::command]
pub async fn edit_user(
    state: State<'_, AppState>,
    user_id: i32,
    user: User,
) -> Result<User, String> {
    let user_db = state.user_db.lock().await;
    user_db.update_user(user_id, user)
}

#[tauri::command]
pub async fn debug_user(state: State<'_, AppState>, username: String) -> Result<User, String> {
    let user_db = state.user_db.lock().await;
    user_db.get_user_by_username(&username)
}

// Hosts 相关命令
#[tauri::command]
pub async fn get_all_hosts_data(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<Vec<Host>, String> {
    let hosts_db = state.hosts_db.lock().await;
    hosts_db.get_hosts_by_user(user_id)
}

#[tauri::command]
pub async fn update_host(
    state: State<'_, AppState>,
    host_id: i32,
    host: Host,
) -> Result<Host, String> {
    let hosts_db = state.hosts_db.lock().await;
    let updated_host = hosts_db.update_host(host_id, host)?;

    // 记录操作日志
    let logs_db = state.logs_db.lock().await;
    let log = Log {
        id: None,
        user_id: updated_host.user_id,
        action: "update".to_string(),
        target_type: "host".to_string(),
        target_id: Some(host_id),
        details: Some(format!("更新 Host: {}", updated_host.name)),
        created_at: 0, // 会在数据库层自动设置
    };
    logs_db.add_log(log)?;

    Ok(updated_host)
}

// 日志相关命令
#[tauri::command]
pub async fn get_all_logs(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<Log>, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.get_all_logs(limit.unwrap_or(100))
}

#[tauri::command]
pub async fn add_log(state: State<'_, AppState>, log: Log) -> Result<Log, String> {
    let logs_db = state.logs_db.lock().await;
    logs_db.add_log(log)
}

// 系统相关命令
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    use crate::utils::system_info;
    let info = system_info::get_system_info();
    Ok(serde_json::to_value(info).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>, user_id: i32) -> Result<(), String> {
    // 记录登出日志
    let logs_db = state.logs_db.lock().await;
    let log = Log {
        id: None,
        user_id,
        action: "logout".to_string(),
        target_type: "user".to_string(),
        target_id: Some(user_id),
        details: Some("用户登出".to_string()),
        created_at: 0,
    };
    logs_db.add_log(log)?;

    Ok(())
}

// 文件操作命令
#[tauri::command]
pub async fn read_text_file(path: String) -> Result<String, String> {
    use std::fs;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<(), String> {
    use std::fs;
    fs::write(path, content).map_err(|e| e.to_string())
}
