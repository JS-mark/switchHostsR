//! 会话管理模块
//!
//! 处理用户会话的创建、验证和销毁

use super::{AuthContext, AuthError, Role};
use crate::auth::permissions::PermissionManager;
use anyhow::Result;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// 会话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub username: String,
    pub role: Role,
    pub created_at: i64,
    pub expires_at: i64,
    pub last_activity: i64,
}

impl Session {
    /// 创建新会话
    pub fn new(user_id: i32, username: String, role: Role, duration_hours: i64) -> Self {
        let now = Utc::now().timestamp();
        let expires_at = now + Duration::hours(duration_hours).num_seconds();

        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            username,
            role,
            created_at: now,
            expires_at,
            last_activity: now,
        }
    }

    /// 检查会话是否过期
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > self.expires_at
    }

    /// 更新最后活动时间
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now().timestamp();
    }

    /// 延长会话时间
    pub fn extend(&mut self, duration_hours: i64) {
        let now = Utc::now().timestamp();
        self.expires_at = now + Duration::hours(duration_hours).num_seconds();
        self.last_activity = now;
    }

    /// 转换为认证上下文
    pub fn to_auth_context(&self) -> AuthContext {
        let permissions = PermissionManager::get_role_permissions(&self.role);

        AuthContext {
            user_id: self.user_id,
            username: self.username.clone(),
            role: self.role.clone(),
            permissions,
            session_id: self.id.clone(),
            expires_at: self.expires_at,
        }
    }
}

/// 会话管理器
#[derive(Debug)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    default_duration_hours: i64,
}

impl SessionManager {
    /// 创建新的会话管理器
    pub fn new(default_duration_hours: i64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_duration_hours,
        }
    }

    /// 创建新会话
    pub fn create_session(&self, user_id: i32, username: String, role: Role) -> Result<Session> {
        let session = Session::new(user_id, username, role, self.default_duration_hours);

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        sessions.insert(session.id.clone(), session.clone());

        Ok(session)
    }

    /// 获取会话
    pub fn get_session(&self, session_id: &str) -> Result<Session> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        let session = sessions
            .get(session_id)
            .ok_or(AuthError::InvalidToken)?
            .clone();

        if session.is_expired() {
            drop(sessions);
            self.remove_session(session_id)?;
            return Err(AuthError::SessionExpired.into());
        }

        Ok(session)
    }

    /// 更新会话活动时间
    pub fn update_session_activity(&self, session_id: &str) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        if let Some(session) = sessions.get_mut(session_id) {
            session.update_activity();
        }

        Ok(())
    }

    /// 延长会话时间
    pub fn extend_session(&self, session_id: &str, duration_hours: i64) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        if let Some(session) = sessions.get_mut(session_id) {
            session.extend(duration_hours);
        }

        Ok(())
    }

    /// 移除会话
    pub fn remove_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        sessions.remove(session_id);

        Ok(())
    }

    /// 移除用户的所有会话
    pub fn remove_user_sessions(&self, user_id: i32) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        sessions.retain(|_, session| session.user_id != user_id);

        Ok(())
    }

    /// 清理过期会话
    pub fn cleanup_expired_sessions(&self) -> Result<usize> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        let initial_count = sessions.len();
        sessions.retain(|_, session| !session.is_expired());
        let final_count = sessions.len();

        Ok(initial_count - final_count)
    }

    /// 获取活跃会话数量
    pub fn get_active_session_count(&self) -> Result<usize> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        Ok(sessions.len())
    }

    /// 获取用户的活跃会话数量
    pub fn get_user_session_count(&self, user_id: i32) -> Result<usize> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| AuthError::Other(format!("获取会话锁失败: {}", e)))?;

        let count = sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_expired())
            .count();

        Ok(count)
    }
}

/// 全局会话管理器实例
static SESSION_MANAGER: std::sync::OnceLock<SessionManager> = std::sync::OnceLock::new();

/// 获取全局会话管理器
pub fn get_session_manager(duration_hour: Option<i64>) -> &'static SessionManager {
    SESSION_MANAGER.get_or_init(|| SessionManager::new(duration_hour.unwrap_or(24)))
    // 默认24小时过期
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);

        assert_eq!(session.user_id, 1);
        assert_eq!(session.username, "test_user");
        assert_eq!(session.role, Role::User);
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_expiry() {
        let mut session = Session::new(1, "test_user".to_string(), Role::User, 0);
        session.expires_at = Utc::now().timestamp() - 1; // 设置为已过期

        assert!(session.is_expired());
    }

    #[test]
    fn test_session_manager() {
        let manager = SessionManager::new(24);

        // 创建会话
        let session = manager
            .create_session(1, "test_user".to_string(), Role::User)
            .unwrap();

        // 获取会话
        let retrieved_session = manager.get_session(&session.id).unwrap();
        assert_eq!(retrieved_session.user_id, 1);

        // 移除会话
        manager.remove_session(&session.id).unwrap();

        // 确认会话已被移除
        assert!(manager.get_session(&session.id).is_err());
    }

    #[test]
    fn test_auth_context_conversion() {
        let session = Session::new(1, "test_user".to_string(), Role::User, 24);
        let auth_context = session.to_auth_context();

        assert_eq!(auth_context.user_id, 1);
        assert_eq!(auth_context.username, "test_user");
        assert_eq!(auth_context.role, Role::User);
        assert!(!auth_context.permissions.is_empty());
    }
}
