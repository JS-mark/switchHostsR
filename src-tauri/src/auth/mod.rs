//! 认证和授权模块
//!
//! 提供用户认证、权限管理和访问控制功能

pub mod middleware;
pub mod permissions;
pub mod roles;
pub mod session;
pub mod token;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

/// 认证错误类型
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("未登录")]
    NotAuthenticated,

    #[error("权限不足")]
    PermissionDenied,

    #[error("无效的令牌")]
    InvalidToken,

    #[error("令牌已过期")]
    TokenExpired,

    #[error("用户不存在")]
    UserNotFound,

    #[error("密码错误")]
    InvalidPassword,

    #[error("会话已过期")]
    SessionExpired,

    #[error("其他错误: {0}")]
    Other(String),
}

/// 用户权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    // 用户管理权限
    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,
    UserList,

    // 主机管理权限
    HostCreate,
    HostRead,
    HostUpdate,
    HostDelete,
    HostList,
    HostActivate,

    // 主机组管理权限
    HostGroupCreate,
    HostGroupRead,
    HostGroupUpdate,
    HostGroupDelete,
    HostGroupList,

    // 日志管理权限
    LogRead,
    LogList,
    LogDelete,

    // 系统管理权限
    SystemConfig,
    SystemInfo,

    // 管理员权限
    AdminAll,
}

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    /// 超级管理员 - 拥有所有权限
    SuperAdmin,
    /// 管理员 - 拥有大部分管理权限
    Admin,
    /// 普通用户 - 只能管理自己的资源
    User,
    /// 访客 - 只读权限
    Guest,
}

/// 认证上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub user_id: i32,
    pub username: String,
    pub role: Role,
    pub permissions: HashSet<Permission>,
    pub session_id: String,
    pub expires_at: i64,
}

impl AuthContext {
    /// 检查是否拥有指定权限
    pub fn has_permission(&self, permission: &Permission) -> bool {
        // 超级管理员拥有所有权限
        if self.role == Role::SuperAdmin {
            return true;
        }

        // 检查是否拥有管理员全部权限
        if self.permissions.contains(&Permission::AdminAll) {
            return true;
        }

        // 检查具体权限
        self.permissions.contains(permission)
    }

    /// 检查是否可以访问指定用户的资源
    pub fn can_access_user_resource(&self, target_user_id: i32) -> bool {
        // 超级管理员和管理员可以访问所有用户资源
        if matches!(self.role, Role::SuperAdmin | Role::Admin) {
            return true;
        }

        // 普通用户只能访问自己的资源
        self.user_id == target_user_id
    }

    /// 检查会话是否过期
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as i64;
        now > self.expires_at
    }
}

/// 权限检查宏
#[macro_export]
macro_rules! require_permission {
    ($auth_context:expr, $permission:expr) => {
        if !$auth_context.has_permission(&$permission) {
            return Err(crate::auth::AuthError::PermissionDenied.into());
        }
    };
}

/// 用户资源访问检查宏
#[macro_export]
macro_rules! require_user_access {
    ($auth_context:expr, $user_id:expr) => {
        if !$auth_context.can_access_user_resource($user_id) {
            return Err(crate::auth::AuthError::PermissionDenied.into());
        }
    };
}
