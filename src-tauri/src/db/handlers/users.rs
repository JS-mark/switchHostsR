//! 用户数据处理模块
//!
//! 该模块包含所有与用户相关的数据库操作。

use crate::db::models::{users::NewUser, User};
use crate::db::schema::users::dsl::*;
use crate::db::DbPool;
use crate::utils::time;
use anyhow::Result;
use diesel::prelude::*;
use thiserror::Error;

/// 用户操作错误
#[derive(Debug, Error)]
pub enum UserError {
    #[error("未登录")]
    NotLoggedIn,

    #[error("用户不存在")]
    UserNotFound,

    #[error("权限不足")]
    PermissionDenied,

    #[error("用户名已存在")]
    UsernameExists,

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

/// 用户数据库操作
pub struct UserHandler {
    pool: DbPool,
    current_user_id: i32,
}

impl UserHandler {
    /// 获取数据库连接池
    pub fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    pub fn change_password(
        &self,
        user_id: i32,
        old_password: String,
        new_password: String,
    ) -> Result<(), UserError> {
        self.check_logged_in()?;
        self.check_permission(user_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;

        // 获取用户信息
        let user = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| UserError::UserNotFound)?;

        // 验证旧密码
        if user.password != old_password {
            return Err(UserError::Other("旧密码不正确".to_string()));
        }

        // 更新密码
        let now_time = time::now();
        diesel::update(users.find(user_id))
            .set((password.eq(new_password), updated_at.eq(now_time)))
            .execute(&mut conn)
            .map_err(UserError::DatabaseError)?;

        Ok(())
    }
    /// 创建新的用户处理器
    pub fn new(pool: DbPool, current_user_id: i32) -> Result<Self, UserError> {
        // 验证用户ID是否有效（非零值表示已登录）
        if current_user_id == 0 {
            return Ok(Self {
                pool,
                current_user_id,
            });
        }

        // 验证用户是否存在
        let mut conn = pool.get().map_err(|e| UserError::Other(e.to_string()))?;
        let user_exists = users.find(current_user_id).first::<User>(&mut conn).is_ok();

        if !user_exists {
            return Err(UserError::UserNotFound);
        }

        Ok(Self {
            pool,
            current_user_id,
        })
    }

    /// 检查是否已登录
    fn check_logged_in(&self) -> Result<(), UserError> {
        if self.current_user_id == 0 {
            return Err(UserError::NotLoggedIn);
        }
        Ok(())
    }

    /// 检查是否有权限操作指定用户
    fn check_permission(&self, target_user_id: i32) -> Result<(), UserError> {
        // 只能操作自己的账户，除非是管理员
        if self.current_user_id != target_user_id && target_user_id != 0 {
            // 检查当前用户是否是管理员
            let mut conn = self
                .pool
                .get()
                .map_err(|e| UserError::Other(e.to_string()))?;
            let current_user = users
                .find(self.current_user_id)
                .first::<User>(&mut conn)
                .map_err(|_| UserError::UserNotFound)?;

            if !current_user.is_admin.unwrap_or(false) {
                return Err(UserError::PermissionDenied);
            }
        }
        Ok(())
    }

    /// 创建用户
    pub fn create_user(&self, new_user: NewUser) -> Result<User, UserError> {
        // 创建用户时，如果已登录，需要检查是否有管理员权限
        if self.current_user_id != 0 {
            self.check_permission(0)?; // 传入0表示需要管理员权限
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        let now_time = time::now();

        let new_user_record = NewUser {
            username: new_user.username.clone(),
            password: new_user.password,
            email: new_user.email,
            avatar: new_user.avatar,
            is_admin: new_user.is_admin,
            created_at: now_time,
            updated_at: now_time,
        };

        // 检查用户名是否已存在
        let username_exists = users
            .filter(username.eq(&new_user_record.username))
            .first::<User>(&mut conn)
            .is_ok();

        if username_exists {
            return Err(UserError::UsernameExists);
        }

        diesel::insert_into(users)
            .values(&new_user_record)
            .execute(&mut conn)
            .map_err(UserError::DatabaseError)?;

        users
            .order(id.desc())
            .first::<User>(&mut conn)
            .map_err(UserError::DatabaseError)
    }

    // 根据ID获取用户
    pub fn get_user_by_id(&self, user_id: i32) -> Result<User, UserError> {
        self.check_logged_in()?;
        self.check_permission(user_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| UserError::UserNotFound)
    }

    // 根据用户名获取用户 - 登录时使用，不需要权限检查
    pub fn get_user_by_username(&self, user_name: &str) -> Result<User, UserError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        users
            .filter(username.eq(user_name))
            .first::<User>(&mut conn)
            .map_err(|_| UserError::UserNotFound)
    }

    // 更新用户信息
    pub fn update_user(&self, user_id: i32, updated_user: User) -> Result<User, UserError> {
        self.check_logged_in()?;
        self.check_permission(user_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        let now_time = time::now();

        // 检查用户名是否已被其他用户使用
        let username_conflict = users
            .filter(username.eq(&updated_user.username))
            .filter(id.ne(user_id))
            .first::<User>(&mut conn)
            .is_ok();

        if username_conflict {
            return Err(UserError::UsernameExists);
        }

        diesel::update(users.find(user_id))
            .set((
                username.eq(updated_user.username),
                email.eq(updated_user.email),
                avatar.eq(updated_user.avatar),
                updated_at.eq(now_time),
            ))
            .execute(&mut conn)
            .map_err(UserError::DatabaseError)?;

        users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(UserError::DatabaseError)
    }

    // 更新用户密码
    pub fn update_password(&self, user_id: i32, new_password: &str) -> Result<(), UserError> {
        self.check_logged_in()?;
        self.check_permission(user_id)?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        let now_time = time::now();

        diesel::update(users.find(user_id))
            .set((password.eq(new_password), updated_at.eq(now_time)))
            .execute(&mut conn)
            .map_err(UserError::DatabaseError)?;

        Ok(())
    }

    // 获取所有用户 - 只有管理员可以
    pub fn get_all_users(&self) -> Result<Vec<User>, UserError> {
        self.check_logged_in()?;
        self.check_permission(0)?; // 传入0表示需要管理员权限

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        users
            .select(User::as_select())
            .load::<User>(&mut conn)
            .map_err(UserError::DatabaseError)
    }

    // 删除用户
    pub fn delete_user(&self, user_id: i32) -> Result<(), UserError> {
        self.check_logged_in()?;
        self.check_permission(user_id)?;

        // 不允许删除自己的账户
        if self.current_user_id == user_id {
            return Err(UserError::Other("不能删除当前登录的账户".to_string()));
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| UserError::Other(e.to_string()))?;
        diesel::delete(users.find(user_id))
            .execute(&mut conn)
            .map_err(UserError::DatabaseError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::create_test_db;

    #[test]
    fn test_create_user() {
        let (pool, _temp_dir) = create_test_db();

        // 创建一个未登录的处理器
        let handler = UserHandler::new(pool.clone(), 0).unwrap();

        // 创建测试用户
        let new_user = NewUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
            email: Some("test@example.com".to_string()),
            avatar: None,
            is_admin: Some(false),
            created_at: 0,
            updated_at: 0,
        };

        // 测试创建用户
        let created_user = handler.create_user(new_user).unwrap();

        // 验证结果
        assert_eq!(created_user.username, "testuser");
        assert_eq!(created_user.email, Some("test@example.com".to_string()));
        assert!(created_user.id > 0);

        // 测试重复用户名
        let duplicate_user = NewUser {
            username: "testuser".to_string(),
            password: "password456".to_string(),
            email: Some("another@example.com".to_string()),
            avatar: None,
            is_admin: Some(false),
            created_at: 0,
            updated_at: 0,
        };

        // 应该返回错误
        let result = handler.create_user(duplicate_user);
        assert!(matches!(result, Err(UserError::UsernameExists)));
    }
}
