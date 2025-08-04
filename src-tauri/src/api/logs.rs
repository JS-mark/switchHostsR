//! 日志管理 API
//!
//! 处理日志相关的 Tauri 命令

use crate::api::{ApiResult, AppState};
use crate::db::services::log_service::*;
use crate::{require_auth, safe_execute};
use serde::Deserialize;
use tauri::State;

/// 日志信息类型别名
pub type LogInfo = LogWithUser;

/// 创建日志请求
#[derive(Debug, Deserialize)]
pub struct CreateLogRequest {
    pub action: String,
    pub target_type: String,
    pub target_id: Option<i32>,
    pub details: Option<String>,
}

/// 搜索日志请求
#[derive(Debug, Deserialize)]
pub struct SearchLogsRequest {
    pub keyword: Option<String>,
    pub user_id: Option<i32>,
    pub action: Option<String>,
    pub target_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// 批量删除日志请求
#[derive(Debug, Deserialize)]
pub struct BatchDeleteLogsRequest {
    pub log_ids: Vec<i32>,
}

/// 清理旧日志请求
#[derive(Debug, Deserialize)]
pub struct CleanOldLogsRequest {
    pub days: i32,
}

/// 获取所有日志
#[tauri::command]
pub async fn get_logs(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<LogInfo>>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    let query_params = LogQueryParams {
        page: Some(1),
        page_size: Some(1000),
        action: None,
        target_type: None,
        target_id: None,
        user_id: None,
        start_date: None,
        end_date: None,
        search: None,
    };
    let result = log_service.get_logs(&auth, query_params).map_err(|_| ())?;
    Ok(ApiResult::success(result.logs))
}

/// 根据ID获取日志
#[tauri::command]
pub async fn get_log(
    state: State<'_, AppState>,
    log_id: i32,
) -> Result<ApiResult<LogInfo>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.get_log_by_id(&auth, log_id)))
}

/// 创建日志
#[tauri::command]
pub async fn create_log(
    state: State<'_, AppState>,
    request: CreateLogRequest,
) -> Result<ApiResult<LogInfo>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    let log = log_service.create_log(
        auth.user_id,
        request.action,
        request.target_type,
        request.target_id,
        request.details,
    ).map_err(|_| ())?;
    
    // 将 Log 转换为 LogWithUser
    let log_with_user = LogWithUser {
        log,
        username: auth.username.clone(),
    };
    
    Ok(ApiResult::success(log_with_user))
}

/// 删除日志
#[tauri::command]
pub async fn delete_log(
    state: State<'_, AppState>,
    log_id: i32,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.delete_log(&auth, log_id)))
}

/// 批量删除日志
#[tauri::command]
pub async fn batch_delete_logs(
    state: State<'_, AppState>,
    request: BatchDeleteLogsRequest,
) -> Result<ApiResult<i32>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.batch_delete_logs(&auth, request.log_ids)))
}

/// 清理旧日志
#[tauri::command]
pub async fn clean_old_logs(
    state: State<'_, AppState>,
    request: CleanOldLogsRequest,
) -> Result<ApiResult<i32>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.cleanup_old_logs(&auth, request.days)))
}

/// 获取日志统计信息
#[tauri::command]
pub async fn get_log_stats(
    state: State<'_, AppState>,
) -> Result<ApiResult<LogStats>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.get_log_stats(&auth)))
}

/// 获取用户日志
#[tauri::command]
pub async fn get_user_logs(
    state: State<'_, AppState>,
    user_id: i32,
    params: LogQueryParams,
) -> Result<ApiResult<LogListResponse>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.get_user_logs(&auth, user_id, params)))
}

/// 导出日志
#[tauri::command]
pub async fn export_logs(
    state: State<'_, AppState>,
    params: LogQueryParams,
) -> Result<ApiResult<crate::db::services::log_service::LogExportData>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.export_logs(&auth, params)))
}

/// 搜索日志
#[tauri::command]
pub async fn search_logs(
    state: State<'_, AppState>,
    request: SearchLogsRequest,
) -> Result<ApiResult<Vec<LogInfo>>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    let query_params = crate::db::services::log_service::LogQueryParams {
        page: request.page.map(|p| p as i32),
        page_size: request.page_size.map(|p| p as i32),
        user_id: request.user_id,
        action: request.action,
        target_type: request.target_type,
        target_id: None,
        start_date: None, // 需要转换字符串日期为时间戳
        end_date: None,   // 需要转换字符串日期为时间戳
        search: request.keyword,
    };
    
    let result = log_service.get_logs(&auth, query_params).map_err(|_| ())?;
    Ok(ApiResult::success(result.logs))
}

/// 获取操作类型列表
#[tauri::command]
pub async fn get_operation_types(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<String>>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.get_action_types(&auth)))
}

/// 获取目标类型列表
#[tauri::command]
pub async fn get_target_types(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<String>>, ()> {
    let auth = require_auth!();
    let log_service = state.service_factory.log_service();
    
    Ok(safe_execute!(log_service.get_target_types(&auth)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::create_test_db;
    use crate::db::services::ServiceFactory;
    use tauri::Manager;
    
    fn create_test_app_state() -> AppState {
        let (pool, _temp_dir) = create_test_db();
        let service_factory = ServiceFactory::new(pool);
        AppState::new(service_factory)
    }
    
    #[tokio::test]
    async fn test_get_logs() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);
        
        // 这里需要模拟认证上下文
        // let result = get_logs(app.state()).await;
        // assert!(result.is_ok());
    }
}
