//! 主机数据处理模块
//!
//! 该模块包含所有与主机相关的数据库操作。

use crate::db::models::hosts::NewHost;
use crate::db::models::{Host, User};
use crate::db::schema::hosts::dsl::*;
use crate::db::schema::users::dsl as users_dsl;
use crate::db::DbPool;
use crate::utils::time;
use anyhow::Result;
use diesel::prelude::*;
use thiserror::Error;

/// 主机操作错误
#[derive(Debug, Error)]
pub enum HostError {
    #[error("未登录")]
    NotLoggedIn,

    #[error("主机不存在")]
    HostNotFound,

    #[error("权限不足")]
    PermissionDenied,

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

/// 主机数据库操作
pub struct HostHandler {
    pool: DbPool,
    current_user_id: i32,
}

impl HostHandler {
    /// 创建新的主机处理器
    pub fn new(pool: DbPool, current_user_id: i32) -> Result<Self, HostError> {
        Ok(Self {
            pool,
            current_user_id,
        })
    }

    /// 检查是否已登录
    fn check_logged_in(&self) -> Result<(), HostError> {
        if self.current_user_id == 0 {
            return Err(HostError::NotLoggedIn);
        }
        Ok(())
    }

    /// 检查是否有权限操作指定主机
    fn check_host_permission(&self, host_id: i32) -> Result<(), HostError> {
        if host_id == 0 {
            return Ok(());
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;

        // 获取主机信息
        let host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| HostError::HostNotFound)?;

        // 检查是否是系统主机或者是当前用户的主机
        if host.is_system == 1 || host.user_id == self.current_user_id {
            return Ok(());
        }

        // 检查当前用户是否是管理员
        let current_user = users_dsl::users
            .find(self.current_user_id)
            .first::<User>(&mut conn)
            .map_err(|_| HostError::Other("获取用户信息失败".to_string()))?;

        if current_user.is_admin.unwrap_or(false) {
            return Ok(());
        }

        Err(HostError::PermissionDenied)
    }

    /// 创建主机
    pub fn create_host(&self, new_host: NewHost) -> Result<Host, HostError> {
        self.check_logged_in()?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        let now_time = time::now();

        let new_host_record = NewHost {
            user_id: self.current_user_id, // 使用当前用户ID
            name: new_host.name,
            description: new_host.description,
            content: new_host.content,
            is_active: new_host.is_active,
            is_system: 0, // 用户创建的主机不是系统主机
            created_at: now_time,
            updated_at: now_time,
        };

        diesel::insert_into(hosts)
            .values(&new_host_record)
            .execute(&mut conn)
            .map_err(HostError::DatabaseError)?;

        hosts
            .order(id.desc())
            .first::<Host>(&mut conn)
            .map_err(HostError::DatabaseError)
    }

    /// 根据ID获取主机
    pub fn get_host_by_id(&self, host_id: i32) -> Result<Host, HostError> {
        self.check_logged_in()?;
        self.check_host_permission(host_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| HostError::HostNotFound)
    }

    /// 获取用户的所有主机
    pub fn get_hosts_by_user(&self, uid: i32) -> Result<Vec<Host>, HostError> {
        self.check_logged_in()?;

        // 如果查询的不是当前用户的主机，需要检查权限
        if uid != self.current_user_id {
            // 检查当前用户是否是管理员
            let mut conn = self
                .pool
                .get()
                .map_err(|e| HostError::Other(e.to_string()))?;
            let current_user = users_dsl::users
                .find(self.current_user_id)
                .first::<User>(&mut conn)
                .map_err(|_| HostError::Other("获取用户信息失败".to_string()))?;

            if !current_user.is_admin.unwrap_or(false) {
                return Err(HostError::PermissionDenied);
            }
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        hosts
            .select(Host::as_select())
            .filter(user_id.eq(uid))
            .load::<Host>(&mut conn)
            .map_err(HostError::DatabaseError)
    }

    /// 更新主机
    pub fn update_host(&self, host_id: i32, updated_host: Host) -> Result<Host, HostError> {
        self.check_logged_in()?;
        self.check_host_permission(host_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        let now_time = time::now();

        diesel::update(hosts.find(host_id))
            .set((
                name.eq(updated_host.name),
                description.eq(updated_host.description),
                content.eq(updated_host.content),
                updated_at.eq(now_time),
            ))
            .execute(&mut conn)
            .map_err(HostError::DatabaseError)?;

        hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(HostError::DatabaseError)
    }

    /// 激活/停用主机
    pub fn toggle_host_active(&self, host_id: i32, active: bool) -> Result<(), HostError> {
        self.check_logged_in()?;
        self.check_host_permission(host_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        let now_time = time::now();
        let active_value = if active { 1 } else { 0 };

        diesel::update(hosts.find(host_id))
            .set((is_active.eq(active_value), updated_at.eq(now_time)))
            .execute(&mut conn)
            .map_err(HostError::DatabaseError)?;

        Ok(())
    }

    /// 删除主机
    pub fn delete_host(&self, host_id: i32) -> Result<(), HostError> {
        self.check_logged_in()?;
        self.check_host_permission(host_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;

        // 检查是否是系统主机
        let host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| HostError::HostNotFound)?;

        if host.is_system == 1 {
            return Err(HostError::Other("不能删除系统主机".to_string()));
        }

        diesel::delete(hosts.find(host_id))
            .execute(&mut conn)
            .map_err(HostError::DatabaseError)?;
        Ok(())
    }

    /// 获取所有激活的主机
    pub fn get_active_hosts(&self) -> Result<Vec<Host>, HostError> {
        self.check_logged_in()?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostError::Other(e.to_string()))?;
        hosts
            .select(Host::as_select())
            .filter(is_active.eq(1))
            .filter(user_id.eq(self.current_user_id).or(is_system.eq(1)))
            .load::<Host>(&mut conn)
            .map_err(HostError::DatabaseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::create_test_db;

    #[test]
    fn test_create_host() {
        let (pool, _temp_dir) = create_test_db();

        // 创建一个已登录的处理器
        let handler = HostHandler::new(pool.clone(), 1).unwrap();

        // 创建测试主机
        let new_host = NewHost {
            user_id: 0, // 这个值会被忽略
            name: "测试主机".to_string(),
            description: Some("测试描述".to_string()),
            content: "127.0.0.1 localhost".to_string(),
            is_active: 1,
            is_system: 0,
            created_at: 0,
            updated_at: 0,
        };

        // 测试创建主机
        let created_host = handler.create_host(new_host).unwrap();

        // 验证结果
        assert_eq!(created_host.name, "测试主机");
        assert_eq!(created_host.description, Some("测试描述".to_string()));
        assert_eq!(created_host.content, "127.0.0.1 localhost");
        assert_eq!(created_host.user_id, 1); // 应该使用当前用户ID
        assert!(created_host.id > 0);
    }
}
