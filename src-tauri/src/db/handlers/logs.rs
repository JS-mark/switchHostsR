//! 日志数据处理模块
//!
//! 该模块包含所有与日志相关的数据库操作。

use crate::db::models::logs::NewLog;
use crate::db::models::Log;
use crate::db::models::User;
use crate::db::schema::logs::dsl::*;
use crate::db::schema::users::dsl as users_dsl;
use crate::db::DbPool;
use crate::utils::time;
use diesel::prelude::*;
use thiserror::Error;

/// 日志操作错误
#[derive(Debug, Error)]
pub enum LogError {
    #[error("未登录")]
    NotLoggedIn,

    #[error("日志不存在")]
    LogNotFound,

    #[error("权限不足")]
    PermissionDenied,

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("其他错误: {0}")]
    Other(String),
}

/// 日志数据库操作
pub struct LogHandler {
    pool: DbPool,
    current_user_id: i32,
}

impl LogHandler {
    /// 创建新的日志处理器
    pub fn new(pool: DbPool, current_user_id: i32) -> Result<Self, LogError> {
        Ok(Self {
            pool,
            current_user_id,
        })
    }

    /// 检查是否已登录
    fn check_logged_in(&self) -> Result<(), LogError> {
        if self.current_user_id == 0 {
            return Err(LogError::NotLoggedIn);
        }
        Ok(())
    }

    /// 检查是否是管理员
    fn check_admin(&self) -> Result<(), LogError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| LogError::Other(e.to_string()))?;
        let current_user = users_dsl::users
            .find(self.current_user_id)
            .first::<User>(&mut conn)
            .map_err(|_| LogError::Other("获取用户信息失败".to_string()))?;

        if !current_user.is_admin.unwrap_or(false) {
            return Err(LogError::PermissionDenied);
        }

        Ok(())
    }

    /// 添加日志
    pub fn add_log(&self, new_log: NewLog) -> Result<Log, LogError> {
        // 添加日志不需要登录检查，因为系统也可能记录日志

        let mut conn = self
            .pool
            .get()
            .map_err(|e| LogError::Other(e.to_string()))?;
        let now_time = time::now();

        let new_log_record = NewLog {
            user_id: if self.current_user_id != 0 {
                self.current_user_id
            } else {
                new_log.user_id
            },
            action: new_log.action,
            target_type: new_log.target_type,
            target_id: new_log.target_id,
            details: new_log.details,
            created_at: now_time,
        };

        diesel::insert_into(logs)
            .values(&new_log_record)
            .execute(&mut conn)
            .map_err(LogError::DatabaseError)?;

        logs.order(id.desc())
            .first::<Log>(&mut conn)
            .map_err(LogError::DatabaseError)
    }

    /// 获取所有日志（需要管理员权限）
    pub fn get_all_logs(&self, limit_count: i64) -> Result<Vec<Log>, LogError> {
        self.check_logged_in()?;
        self.check_admin()?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| LogError::Other(e.to_string()))?;
        logs.select(Log::as_select())
            .order(created_at.desc())
            .limit(limit_count.into())
            .load::<Log>(&mut conn)
            .map_err(LogError::DatabaseError)
    }

    /// 获取用户的日志
    pub fn get_user_logs(&self, uid: i32, limit_count: i64) -> Result<Vec<Log>, LogError> {
        self.check_logged_in()?;

        // 如果查询的不是当前用户的日志，需要检查权限
        if uid != self.current_user_id {
            self.check_admin()?;
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| LogError::Other(e.to_string()))?;
        logs.select(Log::as_select())
            .filter(user_id.eq(uid))
            .order(created_at.desc())
            .limit(limit_count.into())
            .load::<Log>(&mut conn)
            .map_err(LogError::DatabaseError)
    }

    /// 清除旧日志（保留最近的n条）
    pub fn clean_old_logs(&self, keep_count: i64) -> Result<usize, LogError> {
        self.check_logged_in()?;
        self.check_admin()?;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| LogError::Other(e.to_string()))?;

        // 获取要保留的日志ID
        let keep_ids: Vec<i32> = logs
            .order(created_at.desc())
            .limit(keep_count.into())
            .select(id)
            .load::<i32>(&mut conn)
            .map_err(LogError::DatabaseError)?;

        if keep_ids.is_empty() {
            return Ok(0);
        }

        // 删除不在保留列表中的日志
        let deleted = diesel::delete(logs.filter(id.ne_all(keep_ids)))
            .execute(&mut conn)
            .map_err(LogError::DatabaseError)?;

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::create_test_db;

    #[test]
    fn test_add_log() {
        let (pool, _temp_dir) = create_test_db();

        // 创建一个处理器
        let handler = LogHandler::new(pool.clone(), 1).unwrap();

        // 创建测试日志
        let new_log = NewLog {
            user_id: 1,
            action: "login".to_string(),
            target_type: "user".to_string(),
            target_id: Some(1),
            details: Some("用户登录".to_string()),
            created_at: 0,
        };

        // 测试添加日志
        let created_log = handler.add_log(new_log).unwrap();

        // 验证结果
        assert_eq!(created_log.action, "login");
        assert_eq!(created_log.target_type, "user");
        assert_eq!(created_log.details, Some("用户登录".to_string()));
        assert!(created_log.id > 0);
    }
}
