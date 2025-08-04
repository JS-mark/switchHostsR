//! 认证服务
//!
//! 处理用户认证、授权和会话管理

use super::BaseService;
use crate::auth::{
    session::{get_session_manager, Session},
    token::{get_token_manager, TokenPair},
    AuthContext, AuthError, Role,
};
use crate::db::{
    models::{logs::NewLog, users::NewUser, User},
    schema::users::dsl::*,
    DbPool,
};
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub session: Session,
    pub tokens: TokenPair,
}

/// 注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}

/// 密码修改请求
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

/// 认证服务
#[derive(Debug, Clone)]
pub struct AuthService {
    pool: DbPool,
}

impl BaseService for AuthService {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl AuthService {
    /// 创建新的认证服务
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 用户登录
    pub fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        let mut conn = self.pool.get()?;

        // 查找用户
        let user = users
            .filter(username.eq(&request.username))
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 验证密码
        if !verify(&request.password, &user.password)? {
            return Err(AuthError::InvalidPassword.into());
        }

        // 确定用户角色
        let user_role = if user.is_admin.unwrap_or(false) {
            Role::Admin
        } else {
            Role::User
        };

        // 创建会话
        let session_duration: i64 = if request.remember_me.unwrap_or(false) {
            24 * 7 // 7天
        } else {
            24 // 24小时
        };

        let session = get_session_manager(Some(session_duration)).create_session(
            user.id,
            user.username.clone(),
            user_role.clone(),
        )?;

        // 生成令牌
        let tokens =
            get_token_manager().generate_token_pair(user.id, user.username.clone(), user_role)?;

        // 记录登录日志
        self.log_auth_event(
            user.id,
            "login",
            Some(format!("用户 {} 登录成功", user.username)),
        )?;

        Ok(LoginResponse {
            user,
            session,
            tokens,
        })
    }

    /// 用户注册
    pub fn register(&self, request: RegisterRequest) -> Result<User> {
        let mut conn = self.pool.get()?;

        // 检查邮箱是否已存在
        let existing_email = users
            .filter(email.eq(&request.email))
            .first::<User>(&mut conn)
            .optional()?;

        if existing_email.is_some() {
            return Err(AuthError::Other("邮箱已存在".to_string()).into());
        }

        // 检查用户名是否已存在（如果提供了用户名）
        if let Some(ref username_val) = request.username {
            let existing_user = users
                .filter(username.eq(username_val))
                .first::<User>(&mut conn)
                .optional()?;

            if existing_user.is_some() {
                return Err(AuthError::Other("用户名已存在".to_string()).into());
            }
        }

        // 加密密码
        let hashed_password = hash(&request.password, DEFAULT_COST)?;

        // 如果没有提供用户名，使用邮箱前缀作为默认用户名
        let default_username = request.username.clone().unwrap_or_else(|| {
            request.email.split('@').next().unwrap_or(&request.email).to_string()
        });

        // 创建用户
        let now = chrono::Utc::now().timestamp() as i32;
        let new_user = NewUser {
            username: default_username.clone(),
            password: hashed_password,
            email: Some(request.email.clone()),
            avatar: None,
            is_admin: Some(false),
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)?;

        let created_user = users
            .filter(email.eq(&request.email))
            .first::<User>(&mut conn)?;

        // 记录注册日志
        self.log_auth_event(
            created_user.id,
            "register",
            Some(format!("用户 {} 注册成功", created_user.email.as_ref().unwrap_or(&created_user.username))),
        )?;

        Ok(created_user)
    }

    /// 用户登出
    pub fn logout(&self, auth: &AuthContext) -> Result<()> {
        self.validate_auth(auth)?;

        // 移除会话
        get_session_manager(None).remove_session(&auth.session_id)?;

        // 记录登出日志
        self.log_auth_event(
            auth.user_id,
            "logout",
            Some(format!("用户 {} 登出", auth.username)),
        )?;

        Ok(())
    }

    /// 刷新令牌
    pub fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair> {
        // 验证刷新令牌
        let claims = get_token_manager().verify_token(refresh_token)?;

        // 获取用户信息
        let mut conn = self.pool.get()?;
        let user = users
            .find(claims.sub.parse::<i32>()?)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 生成新的令牌对
        let tokens =
            get_token_manager().generate_token_pair(user.id, user.username, claims.role)?;

        Ok(tokens)
    }

    /// 修改密码
    pub fn change_password(
        &self,
        auth: &AuthContext,
        request: ChangePasswordRequest,
    ) -> Result<()> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        // 获取当前用户
        let user = users
            .find(auth.user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 验证旧密码
        if !verify(&request.old_password, &user.password)? {
            return Err(AuthError::InvalidPassword.into());
        }

        // 加密新密码
        let hashed_new_password = hash(&request.new_password, DEFAULT_COST)?;

        // 更新密码
        let now = chrono::Utc::now().timestamp() as i32;
        diesel::update(users.find(auth.user_id))
            .set((password.eq(hashed_new_password), updated_at.eq(now)))
            .execute(&mut conn)?;

        // 记录密码修改日志
        self.log_auth_event(
            auth.user_id,
            "change_password",
            Some("用户修改密码".to_string()),
        )?;

        // 移除用户的所有会话，强制重新登录
        get_session_manager(None).remove_user_sessions(auth.user_id)?;

        Ok(())
    }

    /// 验证令牌
    pub fn verify_token(&self, token: &str) -> Result<AuthContext> {
        let claims = get_token_manager().verify_token(token)?;

        // 获取用户信息验证用户是否仍然存在
        let mut conn = self.pool.get()?;
        let user = users
            .find(claims.sub.parse::<i32>()?)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        // 创建认证上下文
        let permissions =
            crate::auth::permissions::PermissionManager::get_role_permissions(&claims.role);

        Ok(AuthContext {
            user_id: user.id,
            username: user.username,
            role: claims.role,
            permissions,
            session_id: claims.jti,
            expires_at: claims.exp,
        })
    }

    /// 获取用户信息
    pub fn get_user_info(&self, auth: &AuthContext) -> Result<User> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;
        let user = users
            .find(auth.user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::UserNotFound)?;

        Ok(user)
    }

    /// 检查用户名是否可用
    pub fn check_username_availability(&self, username_to_check: &str) -> Result<bool> {
        let mut conn = self.pool.get()?;

        let existing_user = users
            .filter(username.eq(username_to_check))
            .first::<User>(&mut conn)
            .optional()?;

        Ok(existing_user.is_none())
    }

    /// 检查邮箱是否可用
    pub fn check_email_availability(&self, email_to_check: &str) -> Result<bool> {
        let mut conn = self.pool.get()?;

        let existing_user = users
            .filter(email.eq(email_to_check))
            .first::<User>(&mut conn)
            .optional()?;

        Ok(existing_user.is_none())
    }

    /// 记录认证事件日志
    fn log_auth_event(&self, user_id: i32, action: &str, details: Option<String>) -> Result<()> {
        use crate::db::schema::logs;

        let mut conn = self.pool.get()?;

        let log_entry = NewLog {
            user_id,
            action: action.to_string(),
            target_type: "auth".to_string(),
            target_id: Some(user_id),
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
    use crate::db::tests::create_test_db;

    #[test]
    fn test_user_registration() {
        let (pool, _temp_dir) = create_test_db();
        let auth_service = AuthService::new(pool.clone());

        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
            username: Some("test_user".to_string()),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();
        assert_eq!(user.username, "test_user");
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(user.is_admin, Some(false));
    }

    #[test]
    fn test_user_login() {
        let (pool, _temp_dir) = create_test_db();
        let auth_service = AuthService::new(pool.clone());

        // 先注册用户
        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "test_password".to_string(),
            username: Some("test_user".to_string()),
            avatar: None,
        };

        auth_service.register(register_request).unwrap();

        // 然后登录
        let login_request = LoginRequest {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
            remember_me: Some(false),
        };

        let login_response = auth_service.login(login_request).unwrap();
        assert_eq!(login_response.user.username, "test_user");
        assert!(!login_response.session.is_expired());
    }

    #[test]
    fn test_invalid_login() {
        let (pool, _temp_dir) = create_test_db();
        let auth_service = AuthService::new(pool.clone());

        let login_request = LoginRequest {
            username: "nonexistent_user".to_string(),
            password: "wrong_password".to_string(),
            remember_me: Some(false),
        };

        let result = auth_service.login(login_request);
        assert!(result.is_err());
    }

    #[test]
    fn test_username_availability() {
        let (pool, _temp_dir) = create_test_db();
        let auth_service = AuthService::new(pool.clone());

        // 检查不存在的用户名
        assert!(auth_service
            .check_username_availability("new_user")
            .unwrap());

        // 注册用户
        let register_request = RegisterRequest {
            username: Some("test_user".to_string()),
            password: "test_password".to_string(),
            email: "test@example.com".to_string(),
            avatar: None,
        };

        auth_service.register(register_request).unwrap();

        // 检查已存在的用户名
        assert!(!auth_service
            .check_username_availability("test_user")
            .unwrap());
    }
}
