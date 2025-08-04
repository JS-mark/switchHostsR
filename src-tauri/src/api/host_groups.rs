//! 主机组管理 API
//!
//! 处理主机组相关的 Tauri 命令

use crate::api::{ApiResult, AppState, PageData};
use crate::db::models::HostGroup;
use crate::db::services::host_group_service::*;
use crate::{require_auth, safe_execute};
use serde::Deserialize;
use tauri::State;

/// 主机组信息类型别名
pub type HostGroupInfo = crate::db::services::host_group_service::HostGroupWithStats;

/// 创建主机组请求
#[derive(Debug, Deserialize)]
pub struct CreateHostGroupRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

/// 更新主机组请求
#[derive(Debug, Deserialize)]
pub struct UpdateHostGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

/// 搜索主机组请求
#[derive(Debug, Deserialize)]
pub struct SearchHostGroupsRequest {
    pub keyword: String,
    pub is_active: Option<bool>,
    pub page: i32,
    pub page_size: i32,
}

/// 获取所有主机组
#[tauri::command]
pub async fn get_host_groups(
    state: State<'_, AppState>,
) -> Result<ApiResult<Vec<HostGroupInfo>>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();
    
    let params = crate::db::services::host_group_service::HostGroupQueryParams {
        page: None,
        page_size: None,
        search: None,
        keyword: None,
        active_only: None,
        user_id: None,
    };

    match host_group_service.get_host_groups(&auth, params) {
        Ok(response) => Ok(ApiResult::success(response.host_groups)),
        Err(e) => Ok(ApiResult::internal_error(e.to_string())),
    }
}

/// 分页获取主机组列表
#[tauri::command]
pub async fn get_host_groups_page(
    state: State<'_, AppState>,
    page: i32,
    page_size: i32,
) -> Result<ApiResult<PageData<HostGroupInfo>>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();
    
    let query_params = crate::db::services::host_group_service::HostGroupQueryParams {
        page: Some(page),
        page_size: Some(page_size),
        search: None,
        keyword: None,
        active_only: None,
        user_id: None,
    };

    match host_group_service.get_host_groups(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.host_groups,
                total: response.total,
            };
            Ok(ApiResult::success(page_data))
        }
        Err(e) => Ok(ApiResult::from(e)),
    }
}

/// 根据ID获取主机组
#[tauri::command]
pub async fn get_host_group(
    state: State<'_, AppState>,
    group_id: i32,
) -> Result<ApiResult<HostGroupDetailResponse>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(
        host_group_service.get_host_group_by_id(&auth, group_id)
    ))
}

/// 创建主机组
#[tauri::command]
pub async fn create_host_group(
    state: State<'_, AppState>,
    request: CreateHostGroupRequest,
) -> Result<ApiResult<HostGroup>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    let create_req = crate::db::services::host_group_service::CreateHostGroupRequest {
        name: request.name,
        description: request.description,
    };

    Ok(safe_execute!(
        host_group_service.create_host_group(&auth, create_req)
    ))
}

/// 更新主机组
#[tauri::command]
pub async fn update_host_group(
    state: State<'_, AppState>,
    group_id: i32,
    request: UpdateHostGroupRequest,
) -> Result<ApiResult<HostGroup>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    let update_req = crate::db::services::host_group_service::UpdateHostGroupRequest {
        name: request.name,
        description: request.description,
        is_active: request.is_active,
    };

    Ok(safe_execute!(
        host_group_service.update_host_group(&auth, group_id, update_req)
    ))
}

/// 删除主机组
#[tauri::command]
pub async fn delete_host_group(
    state: State<'_, AppState>,
    group_id: i32,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(
        host_group_service.delete_host_group(&auth, group_id)
    ))
}

/// 添加主机到组
#[tauri::command]
pub async fn add_host_to_group(
    state: State<'_, AppState>,
    group_id: i32,
    host_id: i32,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(
        host_group_service.add_host_to_group(&auth, group_id, host_id)
    ))
}

/// 从组中移除主机
#[tauri::command]
pub async fn remove_host_from_group(
    state: State<'_, AppState>,
    group_id: i32,
    host_id: i32,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(
        host_group_service.remove_host_from_group(&auth, group_id, host_id)
    ))
}

/// 切换组激活状态
#[tauri::command]
pub async fn toggle_group_active(
    state: State<'_, AppState>,
    group_id: i32,
) -> Result<ApiResult<HostGroup>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(
        host_group_service.toggle_host_group_active(&auth, group_id)
    ))
}

/// 获取组统计信息
#[tauri::command]
pub async fn get_group_stats(
    state: State<'_, AppState>,
) -> Result<ApiResult<HostGroupStats>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();

    Ok(safe_execute!(host_group_service.get_host_group_stats(&auth)))
}

/// 搜索主机组
#[tauri::command]
pub async fn search_host_groups(
    state: State<'_, AppState>,
    request: SearchHostGroupsRequest,
) -> Result<ApiResult<PageData<HostGroupInfo>>, ()> {
    let auth = require_auth!();
    let host_group_service = state.service_factory.host_group_service();
    
    let query_params = crate::db::services::host_group_service::HostGroupQueryParams {
        page: Some(request.page),
        page_size: Some(request.page_size),
        search: if request.keyword.is_empty() { None } else { Some(request.keyword) },
        keyword: None,
        active_only: request.is_active,
        user_id: None,
    };

    match host_group_service.get_host_groups(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.host_groups,
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
    use crate::db::services::ServiceFactory;
    use crate::db::tests::create_test_db;
    use tauri::Manager;

    fn create_test_app_state() -> AppState {
        let (pool, _temp_dir) = create_test_db();
        let service_factory = ServiceFactory::new(pool);
        AppState::new(service_factory)
    }

    #[tokio::test]
    async fn test_get_host_groups() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);

        // 这里需要模拟认证上下文
        // let result = get_host_groups(app.state()).await;
        // assert!(result.is_ok());
    }
}
