//! 日志服务
//!
//! 处理系统日志管理相关操作

use super::BaseService;
use crate::auth::{AuthContext, AuthError, Permission};
use crate::db::{
    models::{logs::NewLog, Log},
    schema::logs::{self, dsl::*},
    DbPool,
};
use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// 日志查询参数
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LogQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub user_id: Option<i32>,
    pub action: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<i32>,
    pub start_date: Option<i32>,
    pub end_date: Option<i32>,
    pub search: Option<String>,
}

/// 日志列表响应
#[derive(Debug, Serialize)]
pub struct LogListResponse {
    pub logs: Vec<LogWithUser>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

/// 带用户信息的日志
#[derive(Debug, Serialize)]
pub struct LogWithUser {
    #[serde(flatten)]
    pub log: Log,
    pub username: String,
}

/// 日志统计信息
#[derive(Debug, Serialize)]
pub struct LogStats {
    pub total_logs: i32,
    pub today_logs: i32,
    pub week_logs: i32,
    pub month_logs: i32,
    pub action_stats: Vec<ActionStat>,
    pub user_stats: Vec<UserStat>,
}

/// 操作统计
#[derive(Debug, Serialize)]
pub struct ActionStat {
    pub action: String,
    pub count: i32,
}

/// 用户统计
#[derive(Debug, Serialize)]
pub struct UserStat {
    pub user_id: i32,
    pub username: String,
    pub count: i32,
}

/// 日志导出格式
#[derive(Debug, Serialize)]
pub struct LogExportData {
    pub logs: Vec<LogExportItem>,
    pub exported_at: i32,
    pub exported_by: String,
    pub filters: LogQueryParams,
}

#[derive(Debug, Serialize)]
pub struct LogExportItem {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<i32>,
    pub details: Option<String>,
    pub created_at: i32,
    pub created_at_formatted: String,
}

/// 日志服务
#[derive(Debug, Clone)]
pub struct LogService {
    pool: DbPool,
}

impl BaseService for LogService {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl LogService {
    /// 创建新的日志服务
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 创建日志记录
    pub fn create_log(
        &self,
        user_id_param: i32,
        action_param: String,
        target_type_param: String,
        target_id_param: Option<i32>,
        details_param: Option<String>,
    ) -> Result<Log> {
        let mut conn = self.pool.get()?;

        let new_log = NewLog {
            user_id: user_id_param,
            action: action_param,
            target_type: target_type_param,
            target_id: target_id_param,
            details: details_param,
            created_at: chrono::Utc::now().timestamp() as i32,
        };

        diesel::insert_into(logs)
            .values(&new_log)
            .execute(&mut conn)?;

        let created_log = logs.order(id.desc()).first::<Log>(&mut conn)?;

        Ok(created_log)
    }

    /// 获取日志列表
    pub fn get_logs(&self, auth: &AuthContext, params: LogQueryParams) -> Result<LogListResponse> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(50).clamp(1, 200);
        let offset = (page - 1) * page_size;

        // 预先创建搜索模式
        let search_pattern = params.search.as_ref().map(|term| format!("%{}%", term));

        let mut query = logs
            .inner_join(
                crate::db::schema::users::table.on(user_id.eq(crate::db::schema::users::id)),
            )
            .into_boxed();

        // 用户过滤
        if let Some(filter_user_id) = params.user_id {
            query = query.filter(user_id.eq(filter_user_id));
        }

        // 操作过滤
        if let Some(ref filter_action) = params.action {
            query = query.filter(action.eq(filter_action));
        }

        // 目标类型过滤
        if let Some(ref filter_target_type) = params.target_type {
            query = query.filter(target_type.eq(filter_target_type));
        }

        // 目标ID过滤
        if let Some(filter_target_id) = params.target_id {
            query = query.filter(target_id.eq(filter_target_id));
        }

        // 时间范围过滤
        if let Some(start_date) = params.start_date {
            query = query.filter(created_at.ge(start_date));
        }

        if let Some(end_date) = params.end_date {
            query = query.filter(created_at.le(end_date));
        }

        // 搜索过滤
        if let Some(ref pattern) = search_pattern {
            query = query.filter(
                action
                    .like(pattern)
                    .or(target_type.like(pattern))
                    .or(details.like(pattern))
                    .or(crate::db::schema::users::username.like(pattern)),
            );
        }

        // 获取总数 - 重新构建查询
        let mut count_query = logs
            .inner_join(
                crate::db::schema::users::table.on(user_id.eq(crate::db::schema::users::id)),
            )
            .into_boxed();

        // 应用相同的过滤条件
        if let Some(filter_user_id) = params.user_id {
            count_query = count_query.filter(user_id.eq(filter_user_id));
        }

        if let Some(ref filter_action) = params.action {
            count_query = count_query.filter(action.eq(filter_action));
        }

        if let Some(ref filter_target_type) = params.target_type {
            count_query = count_query.filter(target_type.eq(filter_target_type));
        }

        if let Some(filter_target_id) = params.target_id {
            count_query = count_query.filter(target_id.eq(filter_target_id));
        }

        if let Some(start_date) = params.start_date {
            count_query = count_query.filter(created_at.ge(start_date));
        }

        if let Some(end_date) = params.end_date {
            count_query = count_query.filter(created_at.le(end_date));
        }

        if let Some(ref pattern) = search_pattern {
            count_query = count_query.filter(
                action
                    .like(pattern)
                    .or(target_type.like(pattern))
                    .or(details.like(pattern))
                    .or(crate::db::schema::users::username.like(pattern)),
            );
        }

        let total = count_query.count().get_result::<i64>(&mut conn)? as i32;

        // 获取分页数据
        let log_data = query
            .select((logs::all_columns, crate::db::schema::users::username))
            .order(created_at.desc())
            .limit(page_size.into())
            .offset(offset.into())
            .load::<(Log, String)>(&mut conn)?;

        let logs_with_user: Vec<LogWithUser> = log_data
            .into_iter()
            .map(|(log, username)| LogWithUser { log, username })
            .collect();

        let total_pages = (total + page_size - 1) / page_size;

        Ok(LogListResponse {
            logs: logs_with_user,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// 根据ID获取日志
    pub fn get_log_by_id(&self, auth: &AuthContext, log_id: i32) -> Result<LogWithUser> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let (log, username) = logs
            .inner_join(
                crate::db::schema::users::table.on(user_id.eq(crate::db::schema::users::id)),
            )
            .select((logs::all_columns, crate::db::schema::users::username))
            .filter(id.eq(log_id))
            .first::<(Log, String)>(&mut conn)
            .map_err(|_| AuthError::Other("日志不存在".to_string()))?;

        Ok(LogWithUser { log, username })
    }

    /// 删除日志
    pub fn delete_log(&self, auth: &AuthContext, log_id: i32) -> Result<()> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogDelete)?;

        let mut conn = self.pool.get()?;

        let deleted_count = diesel::delete(logs.find(log_id)).execute(&mut conn)?;

        if deleted_count == 0 {
            return Err(AuthError::Other("日志不存在".to_string()).into());
        }

        // 记录删除日志的操作
        self.create_log(
            auth.user_id,
            "delete_log".to_string(),
            "log".to_string(),
            Some(log_id),
            Some(format!("删除日志记录 {}", log_id)),
        )?;

        Ok(())
    }

    /// 批量删除日志
    pub fn batch_delete_logs(&self, auth: &AuthContext, log_ids: Vec<i32>) -> Result<i32> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogDelete)?;

        let mut conn = self.pool.get()?;

        let deleted_count =
            diesel::delete(logs.filter(id.eq_any(&log_ids))).execute(&mut conn)? as i32;

        // 记录批量删除操作
        self.create_log(
            auth.user_id,
            "batch_delete_logs".to_string(),
            "log".to_string(),
            None,
            Some(format!("批量删除 {} 条日志记录", deleted_count)),
        )?;

        Ok(deleted_count)
    }

    /// 清理过期日志
    pub fn cleanup_old_logs(&self, auth: &AuthContext, days_to_keep: i32) -> Result<i32> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogDelete)?;

        let mut conn = self.pool.get()?;

        let cutoff_timestamp =
            chrono::Utc::now().timestamp() as i32 - (days_to_keep as i32 * 24 * 60 * 60);

        let deleted_count =
            diesel::delete(logs.filter(created_at.lt(cutoff_timestamp))).execute(&mut conn)? as i32;

        // 记录清理操作
        self.create_log(
            auth.user_id,
            "cleanup_old_logs".to_string(),
            "system".to_string(),
            None,
            Some(format!(
                "清理 {} 天前的日志，删除 {} 条记录",
                days_to_keep, deleted_count
            )),
        )?;

        Ok(deleted_count)
    }

    /// 获取日志统计信息
    pub fn get_log_stats(&self, auth: &AuthContext) -> Result<LogStats> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let now = chrono::Utc::now().timestamp() as i32;
        let today_start = now - (now % (24 * 60 * 60));
        let week_start = now - (7 * 24 * 60 * 60);
        let month_start = now - (30 * 24 * 60 * 60);

        // 总日志数
        let total_logs = logs.count().get_result::<i64>(&mut conn)? as i32;

        // 今日日志数
        let today_logs = logs
            .filter(created_at.ge(today_start))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        // 本周日志数
        let week_logs = logs
            .filter(created_at.ge(week_start))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        // 本月日志数
        let month_logs = logs
            .filter(created_at.ge(month_start))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        // 操作统计（最近30天）
        let action_stats_data: Vec<(String, i64)> = logs
            .filter(created_at.ge(month_start))
            .group_by(action)
            .select((action, diesel::dsl::count(id)))
            .order(diesel::dsl::count(id).desc())
            .limit(10)
            .load(&mut conn)?;

        let action_stats: Vec<ActionStat> = action_stats_data
            .into_iter()
            .map(|(action_name, count)| ActionStat {
                action: action_name,
                count: count as i32,
            })
            .collect();

        // 用户统计（最近30天）
        let user_stats_data: Vec<(i32, i64)> = logs
            .filter(created_at.ge(month_start))
            .group_by(user_id)
            .select((user_id, diesel::dsl::count(logs::id)))
            .order(diesel::dsl::count(logs::id).desc())
            .limit(10)
            .load(&mut conn)?;

        let mut user_stats: Vec<UserStat> = Vec::new();
        for (uid, count) in user_stats_data {
            // 获取用户名
            let username = crate::db::schema::users::table
                .filter(crate::db::schema::users::id.eq(uid))
                .select(crate::db::schema::users::username)
                .first::<String>(&mut conn)
                .unwrap_or_else(|_| "Unknown".to_string());

            user_stats.push(UserStat {
                user_id: uid,
                username,
                count: count as i32,
            });
        }

        Ok(LogStats {
            total_logs,
            today_logs,
            week_logs,
            month_logs,
            action_stats,
            user_stats,
        })
    }

    /// 获取用户操作日志
    pub fn get_user_logs(
        &self,
        auth: &AuthContext,
        target_user_id: i32,
        params: LogQueryParams,
    ) -> Result<LogListResponse> {
        self.validate_auth(auth)?;

        // 用户可以查看自己的日志，管理员可以查看所有用户的日志
        if auth.user_id != target_user_id {
            self.check_permission(auth, Permission::LogRead)?;
        }

        let mut modified_params = params;
        modified_params.user_id = Some(target_user_id);

        self.get_logs(auth, modified_params)
    }

    /// 导出日志数据
    pub fn export_logs(&self, auth: &AuthContext, params: LogQueryParams) -> Result<LogExportData> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let mut query = logs
            .inner_join(
                crate::db::schema::users::table.on(user_id.eq(crate::db::schema::users::id)),
            )
            .into_boxed();

        // 应用过滤条件（与 get_logs 相同的逻辑）
        if let Some(filter_user_id) = params.user_id {
            query = query.filter(user_id.eq(filter_user_id));
        }

        if let Some(ref filter_action) = params.action {
            query = query.filter(action.eq(filter_action));
        }

        if let Some(ref filter_target_type) = params.target_type {
            query = query.filter(target_type.eq(filter_target_type));
        }

        if let Some(filter_target_id) = params.target_id {
            query = query.filter(target_id.eq(filter_target_id));
        }

        if let Some(start_date) = params.start_date {
            query = query.filter(created_at.ge(start_date));
        }

        if let Some(end_date) = params.end_date {
            query = query.filter(created_at.le(end_date));
        }

        if let Some(search_term) = &params.search {
            let pattern = format!("%{}%", search_term);
            query = query.filter(
                action
                    .like(pattern.clone())
                    .or(target_type.like(pattern.clone()))
                    .or(details.like(pattern.clone()))
                    .or(crate::db::schema::users::username.like(pattern)),
            );
        }

        // 获取所有匹配的日志（限制最大导出数量）
        let log_data = query
            .select((logs::all_columns, crate::db::schema::users::username))
            .order(created_at.desc())
            .limit(10000) // 限制最大导出10000条
            .load::<(Log, String)>(&mut conn)?;

        let export_items: Vec<LogExportItem> = log_data
            .into_iter()
            .map(|(log, username)| {
                let created_at_formatted =
                    chrono::DateTime::from_timestamp(log.created_at.into(), 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                        .unwrap_or_else(|| "Invalid timestamp".to_string());

                LogExportItem {
                    id: log.id,
                    user_id: log.user_id,
                    username,
                    action: log.action,
                    target_type: log.target_type,
                    target_id: log.target_id,
                    details: log.details,
                    created_at: log.created_at,
                    created_at_formatted,
                }
            })
            .collect();

        // 记录导出操作
        self.create_log(
            auth.user_id,
            "export_logs".to_string(),
            "system".to_string(),
            None,
            Some(format!("导出 {} 条日志记录", export_items.len())),
        )?;

        Ok(LogExportData {
            logs: export_items,
            exported_at: chrono::Utc::now().timestamp() as i32,
            exported_by: auth.username.clone(),
            filters: params,
        })
    }

    /// 搜索日志
    pub fn search_logs(
        &self,
        auth: &AuthContext,
        search_term: &str,
        limit: Option<i32>,
    ) -> Result<Vec<LogWithUser>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;
        let search_pattern = format!("%{}%", search_term);
        let limit = limit.unwrap_or(20).clamp(1, 100) as i64;

        let log_data = logs
            .inner_join(
                crate::db::schema::users::table.on(user_id.eq(crate::db::schema::users::id)),
            )
            .filter(
                action
                    .like(&search_pattern)
                    .or(target_type.like(&search_pattern))
                    .or(details.like(&search_pattern))
                    .or(crate::db::schema::users::username.like(&search_pattern)),
            )
            .select((logs::all_columns, crate::db::schema::users::username))
            .order(created_at.desc())
            .limit(limit.into())
            .load::<(Log, String)>(&mut conn)?;

        let search_results: Vec<LogWithUser> = log_data
            .into_iter()
            .map(|(log, username)| LogWithUser { log, username })
            .collect();

        Ok(search_results)
    }

    /// 获取操作类型列表
    pub fn get_action_types(&self, auth: &AuthContext) -> Result<Vec<String>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let action_types = logs
            .select(action)
            .distinct()
            .order(action.asc())
            .load::<String>(&mut conn)?;

        Ok(action_types)
    }

    /// 获取目标类型列表
    pub fn get_target_types(&self, auth: &AuthContext) -> Result<Vec<String>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::LogRead)?;

        let mut conn = self.pool.get()?;

        let target_types = logs
            .select(target_type)
            .distinct()
            .order(target_type.asc())
            .load::<String>(&mut conn)?;

        Ok(target_types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{Permission, Role};
    use crate::db::tests::create_test_db;

    fn create_test_auth_context(test_user_id: i32, role: Role) -> AuthContext {
        let permissions = crate::auth::permissions::PermissionManager::get_role_permissions(&role);
        AuthContext {
            user_id: test_user_id,
            username: "test_user".to_string(),
            role,
            permissions,
            session_id: "test_session".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        }
    }

    #[test]
    fn test_create_log() {
        let (pool, _temp_dir) = create_test_db();
        let log_service = LogService::new(pool.clone());

        // 创建测试用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            username: Some("test_user".to_string()),
            password: "test_password".to_string(),
            email: "test@example.com".to_string(),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();

        // 测试创建日志
        let log = log_service
            .create_log(
                user.id,
                "test_action".to_string(),
                "test_target".to_string(),
                Some(123),
                Some("Test log details".to_string()),
            )
            .unwrap();

        assert_eq!(log.user_id, user.id);
        assert_eq!(log.action, "test_action");
        assert_eq!(log.target_type, "test_target");
        assert_eq!(log.target_id, Some(123));
        assert_eq!(log.details, Some("Test log details".to_string()));
    }

    #[test]
    fn test_get_logs() {
        let (pool, _temp_dir) = create_test_db();
        let log_service = LogService::new(pool.clone());

        // 创建测试用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            email: "admin@example.com".to_string(),
            password: "admin_password".to_string(),
            username: Some("admin_user".to_string()),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();

        // 手动设置为管理员
        let mut conn = pool.get().unwrap();
        diesel::update(crate::db::schema::users::table.find(user.id))
            .set(crate::db::schema::users::is_admin.eq(true))
            .execute(&mut conn)
            .unwrap();

        let auth = create_test_auth_context(user.id, Role::Admin);

        // 创建一些测试日志
        log_service
            .create_log(
                user.id,
                "test_action_1".to_string(),
                "test_target".to_string(),
                Some(1),
                Some("Test log 1".to_string()),
            )
            .unwrap();

        log_service
            .create_log(
                user.id,
                "test_action_2".to_string(),
                "test_target".to_string(),
                Some(2),
                Some("Test log 2".to_string()),
            )
            .unwrap();

        // 测试获取日志列表
        let params = LogQueryParams {
            page: Some(1),
            page_size: Some(10),
            user_id: None,
            action: None,
            target_type: None,
            target_id: None,
            start_date: None,
            end_date: None,
            search: None,
        };

        let log_list = log_service.get_logs(&auth, params).unwrap();
        assert!(log_list.total >= 2);
        assert!(!log_list.logs.is_empty());
    }

    #[test]
    fn test_log_stats() {
        let (pool, _temp_dir) = create_test_db();
        let log_service = LogService::new(pool.clone());

        // 创建测试用户
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            username: Some("admin_user".to_string()),
            password: "admin_password".to_string(),
            email: "admin@example.com".to_string(),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();

        // 手动设置为管理员
        let mut conn = pool.get().unwrap();
        diesel::update(crate::db::schema::users::table.find(user.id))
            .set(crate::db::schema::users::is_admin.eq(true))
            .execute(&mut conn)
            .unwrap();

        let auth = create_test_auth_context(user.id, Role::Admin);

        // 创建一些测试日志
        log_service
            .create_log(
                user.id,
                "login".to_string(),
                "auth".to_string(),
                None,
                Some("User login".to_string()),
            )
            .unwrap();

        // 测试获取统计信息
        let stats = log_service.get_log_stats(&auth).unwrap();
        assert!(stats.total_logs >= 1);
        assert!(stats.today_logs >= 1);
    }
}