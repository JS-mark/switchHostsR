//! 用户服务
//!
//! 处理用户管理相关操作

use super::BaseService;
use crate::api::users::UpdateUserRequest;
use crate::auth::{AuthContext, AuthError, Permission};
use crate::db::{
    models::{logs::NewLog, User},
    schema::users::dsl::*,
    DbPool,
};
use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// 用户查询参数
#[derive(Debug, Deserialize, Default)]
pub struct UserQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub role_filter: Option<String>,
}

/// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

/// 用户统计信息
#[derive(Debug, Serialize)]
pub struct UserStats {
    pub total_users: i32,
    pub admin_users: i32,
    pub regular_users: i32,
    pub recent_registrations: i32, // 最近7天注册的用户数
}

/// 用户服务
#[derive(Debug, Clone)]
pub struct UserService {
    pool: DbPool,
}

impl BaseService for UserService {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl UserService {
    /// 创建新的用户服务
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 获取所有用户（简单列表）
    pub fn get_all_users(&self, auth: &AuthContext) -> Result<Vec<User>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserRead)?;

        let mut conn = self.pool.get()?;
        let user_list = users
            .select(User::as_select())
            .order(created_at.desc())
            .load::<User>(&mut conn)?;

        Ok(user_list)
    }

    /// 获取用户列表
    pub fn get_users(
        &self,
        auth: &AuthContext,
        params: UserQueryParams,
    ) -> Result<UserListResponse> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserList)?;

        let mut conn = self.pool.get()?;

        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(50).clamp(1, 200);
        let offset = (page - 1) * page_size;

        // 预先创建搜索模式
        let search_pattern = params.search.as_ref().map(|term| format!("%{}%", term));

        let mut query = users.into_boxed();

        // 搜索过滤
        if let Some(ref pattern) = search_pattern {
            query = query.filter(username.like(pattern).or(email.like(pattern)));
        }

        // 角色过滤
        if let Some(role_filter) = &params.role_filter {
            match role_filter.as_str() {
                "admin" => query = query.filter(is_admin.eq(true)),
                "user" => query = query.filter(is_admin.eq(false).or(is_admin.is_null())),
                _ => {}
            }
        }

        // 获取总数 - 重新构建查询
        let mut count_query = users.into_boxed();

        // 应用相同的过滤条件
        if let Some(ref pattern) = search_pattern {
            count_query = count_query.filter(username.like(pattern).or(email.like(pattern)));
        }

        if let Some(role_filter) = &params.role_filter {
            match role_filter.as_str() {
                "admin" => count_query = count_query.filter(is_admin.eq(true)),
                "user" => {
                    count_query = count_query.filter(is_admin.eq(false).or(is_admin.is_null()))
                }
                _ => {}
            }
        }

        let total = count_query.count().get_result::<i64>(&mut conn)? as i32;

        // 获取分页数据
        let user_list = query
            .select(User::as_select())
            .order(created_at.desc())
            .limit(page_size.into())
            .offset(offset.into())
            .load::<User>(&mut conn)?;

        let total_pages = (total + page_size - 1) / page_size;

        Ok(UserListResponse {
            users: user_list,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// 根据ID获取用户
    pub fn get_user_by_id(&self, auth: &AuthContext, user_id: i32) -> Result<User> {
        self.validate_auth(auth)?;

        // 用户可以查看自己的信息，管理员可以查看所有用户信息
        if auth.user_id != user_id {
            self.check_permission(auth, Permission::UserList)?;
        }

        let mut conn = self.pool.get()?;
        let user = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        Ok(user)
    }

    /// 更新用户信息
    pub fn update_user(
        &self,
        auth: &AuthContext,
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<User> {
        self.validate_auth(auth)?;

        // 用户可以更新自己的基本信息，但不能修改管理员状态
        // 管理员可以更新所有用户信息
        let can_update_admin_status =
            auth.user_id != user_id && self.check_permission(auth, Permission::UserUpdate).is_ok();

        if auth.user_id != user_id {
            self.check_permission(auth, Permission::UserUpdate)?;
        }

        let mut conn = self.pool.get()?;

        // 检查用户是否存在
        let existing_user = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 检查用户名是否已被其他用户使用
        if let Some(ref new_username) = request.username {
            if new_username != &existing_user.username {
                let username_exists = users
                    .filter(username.eq(new_username))
                    .filter(id.ne(user_id))
                    .first::<User>(&mut conn)
                    .optional()?;

                if username_exists.is_some() {
                    return Err(AuthError::Other("用户名已存在".to_string()).into());
                }
            }
        }

        // 检查邮箱是否已被其他用户使用
        if let Some(ref new_email) = request.email {
            if Some(new_email) != existing_user.email.as_ref() {
                let email_exists = users
                    .filter(email.eq(new_email))
                    .filter(id.ne(user_id))
                    .first::<User>(&mut conn)
                    .optional()?;

                if email_exists.is_some() {
                    return Err(AuthError::Other("邮箱已存在".to_string()).into());
                }
            }
        }

        let now = chrono::Utc::now().timestamp() as i32;

        // 构建更新字段
        let mut update_fields = vec![];

        if let Some(ref new_username) = request.username {
            update_fields.push(("username", new_username.clone()));
        }

        if let Some(ref new_email) = request.email {
            update_fields.push(("email", new_email.clone()));
        }

        if let Some(ref new_avatar) = request.avatar {
            update_fields.push(("avatar", new_avatar.clone()));
        }

        // 只有管理员可以修改其他用户的角色
        let role_update = if let Some(ref new_role) = request.role {
            if can_update_admin_status {
                Some(new_role.clone())
            } else {
                None
            }
        } else {
            None
        };

        // 执行更新
        diesel::update(users.find(user_id))
            .set((
                username.eq(request.username.unwrap_or(existing_user.username.clone())),
                password.eq(existing_user.password.clone()), // 保持现有密码不变
                email.eq(request.email.or(existing_user.email.clone())),
                avatar.eq(request.avatar.or(existing_user.avatar.clone())),
                is_admin.eq(role_update
                    .map(|v| v == "admin")
                    .unwrap_or(existing_user.is_admin.unwrap_or(false))),
                updated_at.eq(now),
            ))
            .execute(&mut conn)?;

        let updated_user = users.find(user_id).first::<User>(&mut conn)?;

        // 记录操作日志
        self.log_user_operation(
            auth.user_id,
            "update_user",
            user_id,
            Some(format!("更新用户 {} 信息", updated_user.username)),
        )?;

        Ok(updated_user)
    }

    /// 删除用户
    pub fn delete_user(&self, auth: &AuthContext, user_id: i32) -> Result<()> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserDelete)?;

        // 不能删除自己
        if auth.user_id == user_id {
            return Err(AuthError::Other("不能删除自己的账户".to_string()).into());
        }

        let mut conn = self.pool.get()?;

        // 检查用户是否存在
        let user_to_delete = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 删除用户
        diesel::delete(users.find(user_id)).execute(&mut conn)?;

        // 记录操作日志
        self.log_user_operation(
            auth.user_id,
            "delete_user",
            user_id,
            Some(format!("删除用户 {}", user_to_delete.username)),
        )?;

        Ok(())
    }

    /// 重置用户密码
    pub fn reset_user_password(
        &self,
        auth: &AuthContext,
        user_id: i32,
        new_password: String,
    ) -> Result<()> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserDelete)?;

        let mut conn = self.pool.get()?;

        // 检查用户是否存在
        let user_to_reset = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 加密新密码
        let hashed_password = hash(&new_password, DEFAULT_COST)?;

        // 更新密码
        let now = chrono::Utc::now().timestamp() as i32;
        diesel::update(users.find(user_id))
            .set((
                username.eq(user_to_reset.username.clone()),
                password.eq(hashed_password),
                email.eq(user_to_reset.email.clone()),
                avatar.eq(user_to_reset.avatar.clone()),
                is_admin.eq(user_to_reset.is_admin),
                updated_at.eq(now)
            ))
            .execute(&mut conn)?;

        // 记录操作日志
        self.log_user_operation(
            auth.user_id,
            "reset_password",
            user_id,
            Some(format!("重置用户 {} 密码", user_to_reset.username)),
        )?;

        Ok(())
    }

    /// 切换用户管理员状态
    pub fn toggle_admin_status(&self, auth: &AuthContext, user_id: i32) -> Result<User> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserDelete)?;

        // 不能修改自己的管理员状态
        if auth.user_id == user_id {
            return Err(AuthError::Other("不能修改自己的管理员状态".to_string()).into());
        }

        let mut conn = self.pool.get()?;

        // 获取当前用户信息
        let current_user = users
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        let new_admin_status = !current_user.is_admin.unwrap_or(false);
        let now = chrono::Utc::now().timestamp() as i32;

        // 更新管理员状态
        diesel::update(users.find(user_id))
            .set((
                username.eq(current_user.username.clone()),
                password.eq(current_user.password.clone()),
                email.eq(current_user.email.clone()),
                avatar.eq(current_user.avatar.clone()),
                is_admin.eq(new_admin_status),
                updated_at.eq(now)
            ))
            .execute(&mut conn)?;

        let updated_user = users.find(user_id).first::<User>(&mut conn)?;

        // 记录操作日志
        let action_desc = if new_admin_status {
            format!("将用户 {} 设置为管理员", updated_user.username)
        } else {
            format!("取消用户 {} 的管理员权限", updated_user.username)
        };

        self.log_user_operation(auth.user_id, "toggle_admin", user_id, Some(action_desc))?;

        Ok(updated_user)
    }

    /// 获取用户统计信息
    pub fn get_user_stats(&self, auth: &AuthContext) -> Result<UserStats> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserRead)?;

        let mut conn = self.pool.get()?;

        // 总用户数
        let total_users = users.count().get_result::<i64>(&mut conn)? as i32;

        // 管理员用户数
        let admin_users = users
            .filter(is_admin.eq(true))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        // 普通用户数
        let regular_users = total_users - admin_users;

        // 最近7天注册的用户数
        let seven_days_ago = chrono::Utc::now().timestamp() as i32 - (7 * 24 * 60 * 60);
        let recent_registrations = users
            .filter(created_at.gt(seven_days_ago))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        Ok(UserStats {
            total_users,
            admin_users,
            regular_users,
            recent_registrations,
        })
    }

    /// 搜索用户
    pub fn search_users(
        &self,
        auth: &AuthContext,
        search_term: &str,
        limit: Option<i32>,
    ) -> Result<Vec<User>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::UserRead)?;

        let mut conn = self.pool.get()?;
        let search_pattern = format!("%{}%", search_term);
        let limit = limit.unwrap_or(10).clamp(1, 50) as i64;

        let search_results = users
            .select(User::as_select())
            .filter(
                username
                    .like(&search_pattern)
                    .or(email.like(&search_pattern)),
            )
            .order(username.asc())
            .limit(limit.into())
            .load::<User>(&mut conn)?;

        Ok(search_results)
    }

    /// 记录用户操作日志
    fn log_user_operation(
        &self,
        operator_id: i32,
        action: &str,
        target_user_id: i32,
        details: Option<String>,
    ) -> Result<()> {
        use crate::db::schema::logs;

        let mut conn = self.pool.get()?;

        let log_entry = NewLog {
            user_id: operator_id,
            action: action.to_string(),
            target_type: "user".to_string(),
            target_id: Some(target_user_id),
            details,
            created_at: chrono::Utc::now().timestamp() as i32,
        };

        // 如果日志表不存在则忽略错误（用于测试环境）
        if let Err(e) = diesel::insert_into(logs::table)
            .values(&log_entry)
            .execute(&mut conn)
        {
            // 在日志表不存在时忽略错误
            if e.to_string().contains("no such table: logs") {
                return Ok(());
            }
            return Err(e.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{Permission, Role};
    use crate::db::tests::create_test_db;

    fn create_test_auth_context(user_id: i32, role: Role) -> AuthContext {
        let permissions = crate::auth::permissions::PermissionManager::get_role_permissions(&role);
        AuthContext {
            user_id,
            username: "test_user".to_string(),
            role,
            permissions,
            session_id: "test_session".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        }
    }

    #[test]
    fn test_get_user_by_id() {
        let (pool, _temp_dir) = create_test_db();
        let user_service = UserService::new(pool.clone());

        // 创建测试用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
            username: Some("test_user".to_string()),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();
        let auth = create_test_auth_context(user.id, Role::User);

        // 测试获取用户信息
        let retrieved_user = user_service.get_user_by_id(&auth, user.id).unwrap();
        assert_eq!(retrieved_user.username, "test_user");
    }

    #[test]
    fn test_update_user() {
        let (pool, _temp_dir) = create_test_db();
        let user_service = UserService::new(pool.clone());

        // 创建测试用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            username: Some("test_user".to_string()),
            password: "test_password".to_string(),
            email: "test@example.com".to_string(),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();
        let auth = create_test_auth_context(user.id, Role::User);

        // 测试更新用户信息
        let update_request = crate::api::users::UpdateUserRequest {
            username: Some("updated_user".to_string()),
            email: Some("updated@example.com".to_string()),
            avatar: Some("new_avatar.png".to_string()),
            role: None, // 普通用户不能修改角色
            is_active: None,
        };

        let updated_user = user_service
            .update_user(&auth, user.id, update_request)
            .unwrap();

        assert_eq!(updated_user.username, "updated_user");
        assert_eq!(updated_user.email, Some("updated@example.com".to_string()));
        assert_eq!(updated_user.avatar, Some("new_avatar.png".to_string()));
    }

    #[test]
    fn test_admin_operations() {
        let (pool, _temp_dir) = create_test_db();
        let user_service = UserService::new(pool.clone());

        // 创建管理员用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            email: "admin@example.com".to_string(),
            password: "admin_password".to_string(),
            username: Some("admin_user".to_string()),
            avatar: None,
        };

        let admin_user = auth_service.register(register_request).unwrap();

        // 手动设置为管理员（在实际应用中这应该通过其他方式完成）
        let mut conn = pool.get().unwrap();
        diesel::update(users.find(admin_user.id))
            .set(is_admin.eq(true))
            .execute(&mut conn)
            .unwrap();

        let admin_auth = create_test_auth_context(admin_user.id, Role::Admin);

        // 测试获取用户统计信息
        let stats = user_service.get_user_stats(&admin_auth).unwrap();
        assert_eq!(stats.total_users, 1);
        assert_eq!(stats.admin_users, 1);
        assert_eq!(stats.regular_users, 0);
    }
}
