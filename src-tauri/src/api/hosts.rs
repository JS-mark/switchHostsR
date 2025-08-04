//! 主机管理 API
//!
//! 处理主机相关的 Tauri 命令

use crate::api::{ApiResult, AppState, PageData};
use crate::db::models::Host;
use crate::db::services::host_service::{*, ImportResult};
use crate::{require_auth, safe_execute};
use serde::Deserialize;
use tauri::State;

/// 主机信息类型别名
pub type HostInfo = Host;

/// 创建主机请求
#[derive(Debug, Deserialize)]
pub struct CreateHostRequest {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub group_id: Option<i32>,
}

/// 更新主机请求
#[derive(Debug, Deserialize)]
pub struct UpdateHostRequest {
    pub name: Option<String>,
    pub content: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub group_id: Option<i32>,
}

/// 搜索主机请求
#[derive(Debug, Deserialize)]
pub struct SearchHostsRequest {
    pub keyword: String,
    pub group_id: Option<i32>,
    pub is_active: Option<bool>,
    pub page: i32,
    pub page_size: i32,
}

/// 导入主机请求
#[derive(Debug, Deserialize)]
pub struct ImportHostsRequest {
    pub hosts: Vec<CreateHostRequest>,
    pub overwrite: Option<bool>,
}

/// 获取所有主机
#[tauri::command]
pub async fn get_hosts(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<HostInfo>>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.get_all_hosts(&auth)))
}

/// 分页获取主机列表
#[tauri::command]
pub async fn get_hosts_page(
    state: State<'_, AppState>,
    page: i32,
    page_size: i32,
) -> Result<ApiResult<PageData<HostInfo>>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();
    
    let query_params = crate::db::services::host_service::HostQueryParams {
        page: Some(page),
        page_size: Some(page_size),
        search: None,
        active_only: None,
        user_id: None,
    };

    match host_service.get_hosts(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.hosts,
                total: response.total,
            };
            Ok(ApiResult::success(page_data))
        }
        Err(e) => Ok(ApiResult::from(e)),
    }
}

/// 根据ID获取主机
#[tauri::command]
pub async fn get_host(
    state: State<'_, AppState>,
    host_id: i32,
) -> Result<ApiResult<HostInfo>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.get_host_by_id(&auth, host_id)))
}

/// 创建主机
#[tauri::command]
pub async fn create_host(
    state: State<'_, AppState>,
    request: CreateHostRequest,
) -> Result<ApiResult<HostInfo>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    let create_req = crate::db::services::host_service::CreateHostRequest {
        name: request.name,
        content: request.content,
        description: request.description,
        is_active: request.is_active,
    };

    Ok(safe_execute!(host_service.create_host(&auth, create_req)))
}

/// 更新主机
#[tauri::command]
pub async fn update_host(
    state: State<'_, AppState>,
    host_id: i32,
    request: UpdateHostRequest,
) -> Result<ApiResult<HostInfo>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    let update_req = crate::db::services::host_service::UpdateHostRequest {
        name: request.name,
        content: request.content,
        description: request.description,
        is_active: request.is_active,
    };

    Ok(safe_execute!(host_service.update_host(&auth, host_id, update_req)))
}

/// 删除主机
#[tauri::command]
pub async fn delete_host(
    state: State<'_, AppState>,
    host_id: i32,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.delete_host(&auth, host_id)))
}

/// 切换主机激活状态
#[tauri::command]
pub async fn toggle_host_active(
    state: State<'_, AppState>,
    host_id: i32,
) -> Result<ApiResult<HostInfo>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.toggle_host_active(&auth, host_id)))
}

/// 获取激活的主机
#[tauri::command]
pub async fn get_active_hosts(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<HostInfo>>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.get_active_hosts(&auth)))
}

/// 获取主机统计信息
#[tauri::command]
pub async fn get_host_stats(
    state: State<'_, AppState>,
) -> Result<ApiResult<HostStats>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.get_host_stats(&auth)))
}

/// 导出主机
#[tauri::command]
pub async fn export_hosts(
    state: State<'_, AppState>,
    _format: String,
) -> Result<ApiResult<crate::db::services::host_service::HostExportData>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    Ok(safe_execute!(host_service.export_hosts(&auth)))
}

/// 导入主机
#[tauri::command]
pub async fn import_hosts(
    state: State<'_, AppState>,
    request: ImportHostsRequest,
) -> Result<ApiResult<ImportResult>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();

    let import_data = crate::db::services::host_service::HostExportData {
        hosts: request.hosts.into_iter().map(|h| {
            crate::db::services::host_service::HostExportItem {
                name: h.name,
                content: h.content,
                description: h.description,
                is_active: h.is_active.unwrap_or(true),
            }
        }).collect(),
        exported_at: chrono::Utc::now().timestamp() as i32,
        exported_by: auth.username.clone(),
    };

    let overwrite = request.overwrite.unwrap_or(false);

    let imported_hosts_result = safe_execute!(host_service.import_hosts(&auth, import_data, overwrite));

    let imported_hosts = if imported_hosts_result.code == 200 {
        imported_hosts_result.data.unwrap_or_default()
    } else {
        let _error_result = ImportResult {
            success_count: 0,
            failed_count: 0,
            imported_hosts: vec![],
            errors: vec![imported_hosts_result.msg.clone()],
        };
        return Ok(ApiResult::error(imported_hosts_result.code, imported_hosts_result.msg));
    };

    let result = ImportResult {
        success_count: imported_hosts.len(),
        failed_count: 0,
        imported_hosts,
        errors: vec![],
    };

    Ok(ApiResult::success(result))
}

/// 搜索主机
#[tauri::command]
pub async fn search_hosts(
    state: State<'_, AppState>,
    request: SearchHostsRequest,
) -> Result<ApiResult<PageData<HostInfo>>, ()> {
    let auth = require_auth!();
    let host_service = state.service_factory.host_service();
    
    let query_params = crate::db::services::host_service::HostQueryParams {
        page: Some(request.page),
        page_size: Some(request.page_size),
        search: if request.keyword.is_empty() { None } else { Some(request.keyword) },
        active_only: request.is_active,
        user_id: None,
    };

    match host_service.get_hosts(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.hosts,
                total: response.total,
            };
            Ok(ApiResult::success(page_data))
        }
        Err(e) => Ok(ApiResult::from(e)),
    }
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
    async fn test_get_hosts() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);

        // 这里需要模拟认证上下文
        // let result = get_hosts(app.state()).await;
        // assert!(result.is_ok());
    }
}
