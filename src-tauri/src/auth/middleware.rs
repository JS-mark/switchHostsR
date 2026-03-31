//! 认证中间件模块
//!
//! 提供 API 调用前的认证和授权检查

use super::session::{get_session_manager, Session};
use super::{AuthContext, AuthError, Permission};
use anyhow::Result;
use std::sync::Arc;
use tauri::{command, State};

/// 认证中间件
#[derive(Debug, Clone)]
pub struct AuthMiddleware {
    current_session: Option<Arc<Session>>,
}

impl AuthMiddleware {
    /// 创建新的认证中间件
    pub fn new() -> Self {
        Self {
            current_session: None,
        }
    }

    /// 设置当前会话
    pub fn set_session(&mut self, session: Session) {
        self.current_session = Some(Arc::new(session));
    }

    /// 清除当前会话
    pub fn clear_session(&mut self) {
        self.current_session = None;
    }

    /// 获取当前认证上下文
    pub fn get_auth_context(&self) -> Result<AuthContext> {
        let session = self
            .current_session
            .as_ref()
            .ok_or(AuthError::NotAuthenticated)?;

        if session.is_expired() {
            return Err(AuthError::SessionExpired.into());
        }

        Ok(session.to_auth_context())
    }

    /// 验证权限
    pub fn check_permission(&self, permission: Permission) -> Result<AuthContext> {
        let auth_context = self.get_auth_context()?;

        if !auth_context.has_permission(&permission) {
            return Err(AuthError::PermissionDenied.into());
        }

        Ok(auth_context)
    }

    /// 验证用户资源访问权限
    pub fn check_user_resource_access(
        &self,
        target_user_id: i32,
        permission: Permission,
    ) -> Result<AuthContext> {
        let auth_context = self.check_permission(permission)?;

        if !auth_context.can_access_user_resource(target_user_id) {
            return Err(AuthError::PermissionDenied.into());
        }

        Ok(auth_context)
    }

    /// 更新会话活动时间
    pub fn update_activity(&self) -> Result<()> {
        if let Some(session) = &self.current_session {
            get_session_manager(None).update_session_activity(&session.id)?;
        }
        Ok(())
    }
}

/// 认证状态管理
#[derive(Debug)]
pub struct AuthState {
    pub middleware: std::sync::RwLock<AuthMiddleware>,
}

impl AuthState {
    /// 创建新的认证状态
    pub fn new() -> Self {
        Self {
            middleware: std::sync::RwLock::new(AuthMiddleware::new()),
        }
    }

    /// 用户登录
    pub fn login(&self, session: Session) -> Result<()> {
        let mut middleware = self
            .middleware
            .write()
            .map_err(|e| AuthError::Other(format!("获取认证锁失败: {}", e)))?;

        middleware.set_session(session);
        Ok(())
    }

    /// 用户登出
    pub fn logout(&self) -> Result<()> {
        let mut middleware = self
            .middleware
            .write()
            .map_err(|e| AuthError::Other(format!("获取认证锁失败: {}", e)))?;

        // 如果有当前会话，从会话管理器中移除
        if let Ok(auth_context) = middleware.get_auth_context() {
            get_session_manager(None).remove_session(&auth_context.session_id)?;
        }

        middleware.clear_session();
        Ok(())
    }

    /// 获取当前认证上下文
    pub fn get_auth_context(&self) -> Result<AuthContext> {
        let middleware = self
            .middleware
            .read()
            .map_err(|e| AuthError::Other(format!("获取认证锁失败: {}", e)))?;

        let auth_context = middleware.get_auth_context()?;

        // 更新活动时间
        if let Err(e) = middleware.update_activity() {
            log::warn!("更新会话活动时间失败: {}", e);
        }

        Ok(auth_context)
    }

    /// 检查权限
    pub fn check_permission(&self, permission: Permission) -> Result<AuthContext> {
        let middleware = self
            .middleware
            .read()
            .map_err(|e| AuthError::Other(format!("获取认证锁失败: {}", e)))?;

        middleware.check_permission(permission)
    }

    /// 检查用户资源访问权限
    pub fn check_user_resource_access(
        &self,
        target_user_id: i32,
        permission: Permission,
    ) -> Result<AuthContext> {
        let middleware = self
            .middleware
            .read()
            .map_err(|e| AuthError::Other(format!("获取认证锁失败: {}", e)))?;

        middleware.check_user_resource_access(target_user_id, permission)
    }
}

/// 全局认证状态实例
static GLOBAL_AUTH_STATE: std::sync::OnceLock<AuthState> = std::sync::OnceLock::new();

/// 获取全局认证状态
pub fn get_global_auth_state() -> &'static AuthState {
    GLOBAL_AUTH_STATE.get_or_init(|| AuthState::new())
}

/// 用户资源访问检查装饰器宏
#[macro_export]
macro_rules! require_user_resource {
    ($auth_state:expr, $user_id:expr, $permission:expr) => {
        $auth_state
            .check_user_resource_access($user_id, $permission)
            .map_err(|e| e.to_string())?
    };
}

/// 认证命令 - 获取当前用户信息
#[command]
pub async fn get_current_auth_context(
    auth_state: State<'_, AuthState>,
) -> Result<AuthContext, String> {
    auth_state.get_auth_context().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{session::Session, Role};

    #[test]
    fn test_auth_middleware() {
        let mut middleware = AuthMiddleware::new();

        // 未登录状态
        assert!(middleware.get_auth_context().is_err());

        // 设置会话
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);
        middleware.set_session(session);

        // 已登录状态
        let auth_context = middleware.get_auth_context().unwrap();
        assert_eq!(auth_context.user_id, 1);
        assert_eq!(auth_context.username, "test_user");

        // 清除会话
        middleware.clear_session();
        assert!(middleware.get_auth_context().is_err());
    }

    #[test]
    fn test_permission_check() {
        let mut middleware = AuthMiddleware::new();
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);
        middleware.set_session(session);

        // 用户应该有读取主机的权限
        assert!(middleware.check_permission(Permission::HostRead).is_ok());

        // 用户不应该有创建用户的权限
        assert!(middleware.check_permission(Permission::UserCreate).is_err());
    }

    #[test]
    fn test_user_resource_access() {
        let mut middleware = AuthMiddleware::new();
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);
        middleware.set_session(session);

        // 用户可以访问自己的资源
        assert!(middleware
            .check_user_resource_access(1, Permission::HostRead)
            .is_ok());

        // 用户不能访问其他用户的资源
        assert!(middleware
            .check_user_resource_access(2, Permission::HostRead)
            .is_err());
    }

    #[test]
    fn test_auth_state() {
        let auth_state = AuthState::new();

        // 未登录状态
        assert!(auth_state.get_auth_context().is_err());

        // 登录
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);
        auth_state.login(session).unwrap();

        // 已登录状态
        let auth_context = auth_state.get_auth_context().unwrap();
        assert_eq!(auth_context.user_id, 1);

        // 登出
        auth_state.logout().unwrap();
        assert!(auth_state.get_auth_context().is_err());
    }
}
