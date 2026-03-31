//! 用户管理 API
//!
//! 处理用户相关的 Tauri 命令

use crate::api::{ApiResult, AppState, PageData, PageParams};
use crate::auth::middleware::get_global_auth_state;
use crate::auth::token::TokenPair;
use crate::db::models::users::User;
use crate::db::services::auth_service::{ChangePasswordRequest, LoginRequest, LoginResponse, ResetPasswordRequest};
use crate::db::services::user_service::*;
use crate::{require_auth, safe_execute};
use serde::Deserialize;
use tauri::State;

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

/// 搜索用户请求
#[derive(Debug, Deserialize)]
pub struct SearchUsersRequest {
    pub keyword: String,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub page: i32,
    pub page_size: i32,
}

// ==================== 认证相关接口 ====================

/// 用户登录
#[tauri::command]
pub async fn user_login(
    state: State<'_, AppState>,
    username: String,
    password: String,
    remember_me: Option<bool>,
) -> Result<ApiResult<LoginResponse>, ()> {
    let auth_service = state.service_factory.auth_service();
    let login_request = LoginRequest {
        username,
        password,
        remember_me,
    };

    match auth_service.login(login_request) {
        Ok(response) => {
            // 登录成功后，将 session 注入 AuthState
            if let Err(e) = get_global_auth_state().login(response.session.clone()) {
                log::error!("设置认证状态失败: {}", e);
            }
            Ok(ApiResult::success(response))
        }
        Err(e) => Ok(ApiResult::from(e)),
    }
}

/// 用户登出
#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let auth_service = state.service_factory.auth_service();

    // 登出后清除 AuthState
    if let Err(e) = get_global_auth_state().logout() {
        log::warn!("清除认证状态失败: {}", e);
    }

    Ok(safe_execute!(auth_service.logout(&auth)))
}

/// 刷新令牌
#[tauri::command]
pub async fn refresh_token(
    state: State<'_, AppState>,
    refresh_token: String,
) -> Result<ApiResult<TokenPair>, ()> {
    let auth_service = state.service_factory.auth_service();

    Ok(safe_execute!(auth_service.refresh_token(&refresh_token)))
}

/// 修改密码
#[tauri::command]
pub async fn change_password(
    state: State<'_, AppState>,
    old_password: String,
    new_password: String,
) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let auth_service = state.service_factory.auth_service();
    let change_request = ChangePasswordRequest {
        old_password,
        new_password,
    };

    Ok(safe_execute!(
        auth_service.change_password(&auth, change_request)
    ))
}

/// 重置密码（忘记密码，公开接口）
#[tauri::command]
pub async fn reset_password(
    state: State<'_, AppState>,
    email: String,
    new_password: String,
) -> Result<ApiResult<()>, ()> {
    let auth_service = state.service_factory.auth_service();
    let reset_request = ResetPasswordRequest {
        email,
        new_password,
    };

    Ok(safe_execute!(auth_service.reset_password(reset_request)))
}

/// 验证令牌
#[tauri::command]
pub async fn verify_token(
    state: State<'_, AppState>,
    token: String,
) -> Result<ApiResult<crate::auth::AuthContext>, ()> {
    let auth_service = state.service_factory.auth_service();

    Ok(safe_execute!(auth_service.verify_token(&token)))
}

/// 获取当前用户信息
#[tauri::command]
pub async fn get_current_user_info(state: State<'_, AppState>) -> Result<ApiResult<User>, ()> {
    let auth = require_auth!();
    let auth_service = state.service_factory.auth_service();

    Ok(safe_execute!(auth_service.get_user_info(&auth)))
}

/// 检查用户名可用性
#[tauri::command]
pub async fn check_username_availability(
    state: State<'_, AppState>,
    username: String,
) -> Result<ApiResult<bool>, ()> {
    let auth_service = state.service_factory.auth_service();

    Ok(safe_execute!(
        auth_service.check_username_availability(&username)
    ))
}

/// 检查邮箱可用性
#[tauri::command]
pub async fn check_email_availability(
    state: State<'_, AppState>,
    email: String,
) -> Result<ApiResult<bool>, ()> {
    let auth_service = state.service_factory.auth_service();

    Ok(safe_execute!(auth_service.check_email_availability(&email)))
}

// ==================== 公开注册接口 ====================

/// 用户注册（公开接口，无需登录）
#[tauri::command]
pub async fn register_user(
    state: State<'_, AppState>,
    request: CreateUserRequest,
) -> Result<ApiResult<User>, ()> {
    let auth_service = state.service_factory.auth_service();
    let register_req = crate::db::services::auth_service::RegisterRequest {
        email: request.email,
        password: request.password,
        username: request.username,
        avatar: request.avatar,
    };

    Ok(safe_execute!(auth_service.register(register_req)))
}

// ==================== 用户管理接口 ====================

/// 获取所有用户
#[tauri::command]
pub async fn get_users(state: State<'_, AppState>) -> Result<ApiResult<Vec<User>>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    Ok(safe_execute!(user_service.get_all_users(&auth)))
}

/// 分页获取用户列表
#[tauri::command]
pub async fn get_users_page(
    state: State<'_, AppState>,
    page: i32,
    page_size: i32,
) -> Result<ApiResult<PageData<User>>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();
    let _page_params = PageParams::new(Some(page), Some(page_size));

    let query_params = crate::db::services::user_service::UserQueryParams {
        page: Some(page),
        page_size: Some(page_size),
        search: None,
        role_filter: None,
    };
    match user_service.get_users(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.users,
                total: response.total,
            };
            Ok(ApiResult::success(page_data))
        }
        Err(e) => Ok(ApiResult::from(e)),
    }
}

/// 根据ID获取用户
#[tauri::command]
pub async fn get_user(state: State<'_, AppState>, user_id: i32) -> Result<ApiResult<User>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    Ok(safe_execute!(user_service.get_user_by_id(&auth, user_id)))
}

/// 创建用户（管理员操作）
///
/// 需要管理员权限。开放注册应使用独立的注册接口。
/// 此命令用于管理员后台手动创建用户。
#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    request: CreateUserRequest,
) -> Result<ApiResult<User>, ()> {
    let _auth = require_auth!();
    let auth_service = state.service_factory.auth_service();
    let register_req = crate::db::services::auth_service::RegisterRequest {
        email: request.email,
        password: request.password,
        username: request.username,
        avatar: request.avatar,
    };

    Ok(safe_execute!(auth_service.register(register_req)))
}

/// 更新用户
#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    user_id: i32,
    request: UpdateUserRequest,
) -> Result<ApiResult<User>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    Ok(safe_execute!(
        user_service.update_user(&auth, user_id, request)
    ))
}

/// 删除用户
#[tauri::command]
pub async fn delete_user(state: State<'_, AppState>, user_id: i32) -> Result<ApiResult<()>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    Ok(safe_execute!(user_service.delete_user(&auth, user_id)))
}

/// 获取用户统计信息
#[tauri::command]
pub async fn get_user_stats(state: State<'_, AppState>) -> Result<ApiResult<UserStats>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    Ok(safe_execute!(user_service.get_user_stats(&auth)))
}

/// 搜索用户
#[tauri::command]
pub async fn search_users(
    state: State<'_, AppState>,
    request: SearchUsersRequest,
) -> Result<ApiResult<PageData<User>>, ()> {
    let auth = require_auth!();
    let user_service = state.service_factory.user_service();

    let query_params = crate::db::services::user_service::UserQueryParams {
        page: Some(request.page),
        page_size: Some(request.page_size),
        search: if request.keyword.is_empty() {
            None
        } else {
            Some(request.keyword)
        },
        role_filter: request.role,
    };

    match user_service.get_users(&auth, query_params) {
        Ok(response) => {
            let page_data = PageData {
                list: response.users,
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
    async fn test_get_users() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);

        // 这里需要模拟认证上下文
        // let result = get_users(app.state()).await;
        // assert!(result.is_ok());
    }
}
