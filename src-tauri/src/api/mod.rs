//! API 模块
//!
//! 提供 Tauri 命令接口，处理前端请求

pub mod commands;
pub mod host_groups;
pub mod hosts;
pub mod logs;
pub mod public_type;
pub mod system;
pub mod users;

use crate::auth::middleware::get_global_auth_state;
use crate::auth::AuthContext;
use crate::db::services::ServiceFactory;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 重新导出统一的响应类型
pub use crate::api::public_type::{ApiResult, PageData, PageParams};

/// 应用状态
#[derive(Debug)]
pub struct AppState {
    pub service_factory: Arc<ServiceFactory>,
}

impl AppState {
    pub fn new(service_factory: ServiceFactory) -> Self {
        Self {
            service_factory: Arc::new(service_factory),
        }
    }
}

/// API 响应结果（保持向后兼容）
pub type ApiResponse<T> = ApiResult<T>;

/// 分页请求参数
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// 搜索请求参数
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub search: Option<String>,
    pub limit: Option<i32>,
}

/// 批量操作请求
#[derive(Debug, Deserialize)]
pub struct BatchRequest<T> {
    pub items: Vec<T>,
}

/// 批量操作响应
#[derive(Debug, Serialize)]
pub struct BatchResponse {
    pub success_count: i32,
    pub failed_count: i32,
    pub errors: Vec<String>,
}

/// 从错误转换为 API 响应
impl<T> From<anyhow::Error> for ApiResult<T> {
    fn from(error: anyhow::Error) -> Self {
        // 检查是否是认证错误
        if let Some(auth_error) = error.downcast_ref::<crate::auth::AuthError>() {
            match auth_error {
                crate::auth::AuthError::NotAuthenticated => {
                    Self::auth_error("用户未登录".to_string())
                }
                crate::auth::AuthError::SessionExpired => {
                    Self::auth_error("会话已过期".to_string())
                }
                crate::auth::AuthError::PermissionDenied => {
                    Self::permission_error("权限不足".to_string())
                }
                crate::auth::AuthError::InvalidPassword => Self::auth_error("凭据无效".to_string()),
                crate::auth::AuthError::UserNotFound => {
                    Self::not_found_error("用户不存在".to_string())
                }
                crate::auth::AuthError::TokenExpired => Self::auth_error("令牌已过期".to_string()),
                crate::auth::AuthError::InvalidToken => Self::auth_error("令牌无效".to_string()),
                crate::auth::AuthError::Other(msg) => Self::param_error(msg.clone()),
            }
        } else {
            Self::internal_error(error.to_string())
        }
    }
}

/// 获取当前认证上下文的辅助函数
/// 通过全局 AuthState 获取当前登录用户的认证信息
pub fn get_auth_context() -> Result<AuthContext, crate::auth::AuthError> {
    get_global_auth_state().get_auth_context().map_err(|e| {
        // 尝试将 anyhow::Error 转为 AuthError
        match e.downcast::<crate::auth::AuthError>() {
            Ok(auth_err) => auth_err,
            Err(_) => crate::auth::AuthError::NotAuthenticated,
        }
    })
}

/// 验证认证上下文的宏
/// 使用全局 AuthState 获取认证上下文，无需依赖局部变量
#[macro_export]
macro_rules! require_auth {
    () => {
        match $crate::api::get_auth_context() {
            Ok(auth) => auth,
            Err(e) => return Ok($crate::api::ApiResult::from(anyhow::Error::from(e))),
        }
    };
}

// 权限验证宏已移至 auth/mod.rs

/// 安全地执行异步操作的宏
#[macro_export]
macro_rules! safe_execute {
    ($operation:expr) => {
        match $operation {
            Ok(result) => $crate::api::ApiResult::success(result),
            Err(e) => $crate::api::ApiResult::from(e),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_result_success() {
        let response = ApiResult::success("test data");
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
        assert_eq!(response.data.as_deref(), Some("test data"));
    }

    #[test]
    fn test_api_result_error() {
        let response: ApiResult<String> = ApiResult::internal_error("test error".to_string());
        assert_eq!(response.code, 500);
        assert_eq!(response.msg, "test error");
        assert_eq!(response.data, None);
    }

    #[test]
    fn test_api_result_auth_error() {
        let response: ApiResult<String> = ApiResult::auth_error("auth error".to_string());
        assert_eq!(response.code, 401);
        assert_eq!(response.msg, "auth error");
        assert_eq!(response.data, None);
    }

    #[test]
    fn test_page_params() {
        let params = PageParams::new(Some(2), Some(20));
        assert_eq!(params.page, 2);
        assert_eq!(params.page_size, 20);
        assert_eq!(params.offset(), 20);
        assert_eq!(params.limit(), 20);
    }
}
