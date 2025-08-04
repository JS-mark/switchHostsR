//! 数据库服务层
//!
//! 提供高级的数据访问接口，封装复杂的业务逻辑

pub mod auth_service;
pub mod host_group_service;
pub mod host_service;
pub mod log_service;
pub mod system_service;
pub mod user_service;

use crate::auth::AuthContext;
use crate::db::DbPool;
use anyhow::Result;

/// 服务基类特征
pub trait BaseService {
    /// 获取数据库连接池
    fn get_pool(&self) -> &DbPool;

    /// 验证认证上下文
    fn validate_auth(&self, auth: &AuthContext) -> Result<()> {
        if auth.is_expired() {
            return Err(crate::auth::AuthError::SessionExpired.into());
        }
        Ok(())
    }

    /// 检查权限
    fn check_permission(
        &self,
        auth: &AuthContext,
        permission: crate::auth::Permission,
    ) -> Result<()> {
        if !auth.has_permission(&permission) {
            return Err(crate::auth::AuthError::PermissionDenied.into());
        }
        Ok(())
    }

    /// 检查用户资源访问权限
    fn check_user_resource(&self, auth: &AuthContext, resource_user_id: i32) -> Result<()> {
        if !auth.can_access_user_resource(resource_user_id) {
            return Err(crate::auth::AuthError::PermissionDenied.into());
        }
        Ok(())
    }
}

/// 服务工厂
#[derive(Debug, Clone)]
pub struct ServiceFactory {
    pool: DbPool,
}

impl ServiceFactory {
    /// 创建新的服务工厂
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 创建用户服务
    pub fn user_service(&self) -> user_service::UserService {
        user_service::UserService::new(self.pool.clone())
    }

    /// 创建主机服务
    pub fn host_service(&self) -> host_service::HostService {
        host_service::HostService::new(self.pool.clone())
    }

    /// 创建主机组服务
    pub fn host_group_service(&self) -> host_group_service::HostGroupService {
        host_group_service::HostGroupService::new(self.pool.clone())
    }

    /// 创建日志服务
    pub fn log_service(&self) -> log_service::LogService {
        log_service::LogService::new(self.pool.clone())
    }

    /// 创建认证服务
    pub fn auth_service(&self) -> auth_service::AuthService {
        auth_service::AuthService::new(self.pool.clone())
    }

    /// 创建系统服务
    pub fn system_service(&self) -> system_service::SystemService {
        system_service::SystemService::new(self.pool.clone())
    }
}

/// 全局服务工厂实例
static SERVICE_FACTORY: std::sync::OnceLock<ServiceFactory> = std::sync::OnceLock::new();

/// 获取全局服务工厂
pub fn get_service_factory() -> &'static ServiceFactory {
    SERVICE_FACTORY.get().expect("服务工厂未初始化")
}

/// 初始化服务工厂
pub fn init_service_factory(pool: DbPool) {
    SERVICE_FACTORY
        .set(ServiceFactory::new(pool))
        .map_err(|_| "服务工厂已经初始化")
        .unwrap();
}
