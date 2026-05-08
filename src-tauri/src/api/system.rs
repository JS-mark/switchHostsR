//! 系统管理 API
//!
//! 处理系统相关的 Tauri 命令

use crate::api::{ApiResult, AppState};
use crate::{require_auth, safe_execute};
use serde::Deserialize;
use tauri::State;

pub type BackupInfo = crate::db::services::system_service::BackupInfo;

/// 系统配置更新请求
#[derive(Debug, Deserialize)]
pub struct UpdateSystemConfigRequest {
    pub config_key: String,
    pub config_value: String,
}

/// 备份创建请求
#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub name: String,
    pub description: Option<String>,
}

/// 备份恢复请求
#[derive(Debug, Deserialize)]
pub struct RestoreBackupRequest {
    pub backup_id: i32,
}

// get_system_info 已移至 api/commands.rs

/// 获取系统配置
#[tauri::command]
pub async fn get_system_config(
    state: State<'_, AppState>,
) -> Result<ApiResult<std::collections::HashMap<String, String>>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(system_service.get_system_config(&auth).await))
}

/// 更新系统配置
#[tauri::command]
pub async fn update_system_config(
    state: State<'_, AppState>,
    request: UpdateSystemConfigRequest,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(
        system_service
            .update_system_config(&auth, &request.config_key, &request.config_value)
            .await
    ))
}

/// 健康检查
#[tauri::command]
pub async fn health_check(state: State<'_, AppState>) -> Result<ApiResult<bool>, ()> {
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(system_service.check_health().await))
}

/// 清理日志
#[tauri::command]
pub async fn clean_logs(state: State<'_, AppState>, days: i32) -> Result<ApiResult<i32>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(system_service.clean_logs(&auth, days).await))
}

/// 创建备份
#[tauri::command]
pub async fn create_backup(
    state: State<'_, AppState>,
    request: CreateBackupRequest,
) -> Result<ApiResult<BackupInfo>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(
        system_service
            .create_backup(&auth, request.name, request.description)
            .await
    ))
}

/// 获取备份列表
#[tauri::command]
pub async fn get_backups(state: State<'_, AppState>) -> Result<ApiResult<Vec<BackupInfo>>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(system_service.get_backups(&auth).await))
}

/// 恢复备份
#[tauri::command]
pub async fn restore_backup(
    state: State<'_, AppState>,
    request: RestoreBackupRequest,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(
        system_service
            .restore_backup(&auth, request.backup_id)
            .await
    ))
}

/// 重启服务
#[tauri::command]
pub async fn restart_service(state: State<'_, AppState>) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let system_service = state.service_factory.system_service();

    Ok(safe_execute!(system_service.restart_service(&auth).await))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::services::ServiceFactory;
    use crate::db::tests::create_test_db;
    use tauri::Manager;

    fn create_test_app_state() -> AppState {
        let (pool, _temp_dir) = create_test_db();
        let service_factory = ServiceFactory::new(pool);
        AppState::new(service_factory)
    }

    #[tokio::test]
    async fn test_get_system_info() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);

        // 这里需要模拟认证上下文
        // let result = get_system_info(app.state()).await;
        // assert!(result.is_ok());
    }
}
