//! 主机组数据处理模块
//!
//! 该模块包含所有与主机组相关的数据库操作。

use crate::db::models::{Host, HostGroup, HostGroupRelation, User};
use crate::db::schema::host_group_relations::dsl::*;
use crate::db::schema::host_groups::dsl::*;
use crate::db::schema::hosts::dsl as hosts_dsl;
use crate::db::schema::users::dsl as users_dsl;
use crate::db::DbPool;
use crate::utils::time;
use anyhow::{Context, Result};
use diesel::prelude::*;
use thiserror::Error;

/// 主机组操作错误
#[derive(Debug, Error)]
pub enum HostGroupError {
    #[error("未登录")]
    NotLoggedIn,

    #[error("主机组不存在")]
    GroupNotFound,

    #[error("主机不存在")]
    HostNotFound,

    #[error("权限不足")]
    PermissionDenied,

    #[error("主机已在组中")]
    HostAlreadyInGroup,

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

/// 主机组数据库操作
pub struct HostGroupHandler {
    pool: DbPool,
    current_user_id: i32,
}

impl HostGroupHandler {
    /// 创建新的主机组处理器
    pub fn new(pool: DbPool, current_user_id: i32) -> Result<Self, HostGroupError> {
        Ok(Self {
            pool,
            current_user_id,
        })
    }

    /// 检查是否已登录
    fn check_logged_in(&self) -> Result<(), HostGroupError> {
        if self.current_user_id == 0 {
            return Err(HostGroupError::NotLoggedIn);
        }
        Ok(())
    }

    /// 检查是否有权限操作指定主机组
    fn check_group_permission(&self, group_id: i32) -> Result<(), HostGroupError> {
        if group_id == 0 {
            return Ok(());
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostGroupError::Other(e.to_string()))?;

        // 获取主机组信息
        let group = host_groups
            .find(group_id)
            .first::<HostGroup>(&mut conn)
            .map_err(|_| HostGroupError::GroupNotFound)?;

        // 检查是否是当前用户的主机组
        if group.user_id == self.current_user_id {
            return Ok(());
        }

        // 检查当前用户是否是管理员
        let current_user = users_dsl::users
            .find(self.current_user_id)
            .first::<User>(&mut conn)
            .map_err(|_| HostGroupError::Other("获取用户信息失败".to_string()))?;

        if current_user.is_admin.unwrap_or(false) {
            return Ok(());
        }

        Err(HostGroupError::PermissionDenied)
    }

    /// 创建主机组
    pub fn create_group(&self, new_group: HostGroup) -> Result<HostGroup, HostGroupError> {
        self.check_logged_in()?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| HostGroupError::Other(e.to_string()))?;
        let now_time = time::now();

        let group = HostGroup {
            id: None,
            user_id: self.current_user_id, // 使用当前用户ID
            name: new_group.name,
            description: new_group.description,
            created_at: now_time,
            updated_at: now_time,
        };

        diesel::insert_into(host_groups)
            .values(&group)
            .execute(&mut conn)
            .map_err(HostGroupError::DatabaseError)?;

        host_groups
            .order(id.desc())
            .first(&mut conn)
            .map_err(HostGroupError::DatabaseError)
    }

    // ... 其他方法 ...
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::create_test_db;

    #[test]
    fn test_create_group() {
        let (pool, _temp_dir) = create_test_db();

        // 创建一个已登录的处理器
        let handler = HostGroupHandler::new(pool.clone(), 1).unwrap();

        // 创建测试主机组
        let new_group = HostGroup {
            id: None,
            user_id: 0, // 这个值会被忽略
            name: "测试组".to_string(),
            description: Some("测试描述".to_string()),
            created_at: 0,
            updated_at: 0,
        };

        // 测试创建主机组
        let created_group = handler.create_group(new_group).unwrap();

        // 验证结果
        assert_eq!(created_group.name, "测试组");
        assert_eq!(created_group.description, Some("测试描述".to_string()));
        assert_eq!(created_group.user_id, 1); // 应该使用当前用户ID
        assert!(created_group.id.is_some());
    }

    // ... 其他测试 ...
}
