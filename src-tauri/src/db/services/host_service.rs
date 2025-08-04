//! 主机服务
//!
//! 处理主机管理相关操作

use super::BaseService;
use crate::auth::{AuthContext, AuthError, Permission};
use crate::db::{
    models::{hosts::NewHost, logs::NewLog, Host},
    schema::hosts::dsl::{
            content, created_at, description, hosts, id, is_active, name, updated_at,
            user_id,
        },
    DbPool,
};
use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// 创建主机请求
#[derive(Debug, Deserialize)]
pub struct CreateHostRequest {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub is_active: Option<bool>,
}

/// 更新主机请求
#[derive(Debug, Deserialize)]
pub struct UpdateHostRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}

/// 主机查询参数
#[derive(Debug, Deserialize, Default)]
pub struct HostQueryParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub active_only: Option<bool>,
    pub user_id: Option<i32>,
}

/// 主机列表响应
#[derive(Debug, Serialize)]
pub struct HostListResponse {
    pub hosts: Vec<Host>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

/// 主机统计信息
#[derive(Debug, Serialize)]
pub struct HostStats {
    pub total_hosts: i32,
    pub active_hosts: i32,
    pub inactive_hosts: i32,
    pub user_hosts: i32, // 当前用户的主机数
}

/// 主机导入/导出格式
#[derive(Debug, Serialize, Deserialize)]
pub struct HostExportData {
    pub hosts: Vec<HostExportItem>,
    pub exported_at: i32,
    pub exported_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostExportItem {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
    pub is_active: bool,
}

/// 主机导入结果
#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub imported_hosts: Vec<Host>,
    pub errors: Vec<String>,
}

/// 主机服务
#[derive(Debug, Clone)]
pub struct HostService {
    pool: DbPool,
}

impl BaseService for HostService {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

impl HostService {
    /// 创建新的主机服务
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 创建主机
    pub fn create_host(&self, auth: &AuthContext, request: CreateHostRequest) -> Result<Host> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::HostCreate)?;

        let mut conn = self.pool.get()?;

        // 检查名称是否已存在
        let existing_host = hosts
            .filter(user_id.eq(auth.user_id))
            .filter(name.eq(&request.name))
            .first::<Host>(&mut conn)
            .optional()?;

        if existing_host.is_some() {
            return Err(AuthError::Other("主机名称已存在".to_string()).into());
        }

        let now = chrono::Utc::now().timestamp() as i32;
        let new_host = NewHost {
            user_id: auth.user_id,
            name: request.name,
            description: request.description,
            content: request.content,
            is_active: if request.is_active.unwrap_or(false) {
                1
            } else {
                0
            },
            is_system: 0,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(hosts)
            .values(&new_host)
            .execute(&mut conn)?;

        let created_host = hosts.order(id.desc()).first::<Host>(&mut conn)?;

        // 记录操作日志
        self.log_host_operation(
            auth.user_id,
            "create_host",
            created_host.id,
            Some(format!("创建主机 {}", created_host.name)),
        )?;

        Ok(created_host)
    }

    /// 获取所有主机（简单列表）
    pub fn get_all_hosts(&self, auth: &AuthContext) -> Result<Vec<Host>> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;
        let mut query = hosts.into_boxed();

        // 权限检查：普通用户只能看到自己的主机，管理员可以看到所有主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            query = query.filter(user_id.eq(auth.user_id));
        }

        let host_list = query
            .select(Host::as_select())
            .order(created_at.desc())
            .load::<Host>(&mut conn)?;

        Ok(host_list)
    }

    /// 获取主机列表
    pub fn get_hosts(
        &self,
        auth: &AuthContext,
        params: HostQueryParams,
    ) -> Result<HostListResponse> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(50).clamp(1, 200);
        let offset = (page - 1) * page_size;

        // 预先创建搜索模式
        let search_pattern = params.search.as_ref().map(|term| format!("%{}%", term));

        let mut query = hosts.into_boxed();

        // 权限检查：普通用户只能看到自己的主机，管理员可以看到所有主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            query = query.filter(user_id.eq(auth.user_id));
        } else if let Some(filter_user_id) = params.user_id {
            query = query.filter(user_id.eq(filter_user_id));
        }

        // 搜索过滤
        if let Some(ref pattern) = search_pattern {
            query = query.filter(
                name.like(pattern)
                    .or(description.like(pattern))
                    .or(content.like(pattern)),
            );
        }

        // 活跃状态过滤
        if let Some(active_filter) = params.active_only {
            let active_value = if active_filter { 1 } else { 0 };
            query = query.filter(is_active.eq(active_value));
        }

        // 获取总数 - 重新构建查询
        let mut count_query = hosts.into_boxed();

        // 应用相同的过滤条件
        if self.check_permission(auth, Permission::HostRead).is_err() {
            count_query = count_query.filter(user_id.eq(auth.user_id));
        } else if let Some(filter_user_id) = params.user_id {
            count_query = count_query.filter(user_id.eq(filter_user_id));
        }

        if let Some(ref pattern) = search_pattern {
            count_query = count_query.filter(
                name.like(pattern)
                    .or(description.like(pattern))
                    .or(content.like(pattern)),
            );
        }

        if let Some(active_filter) = params.active_only {
            let active_value = if active_filter { 1 } else { 0 };
            count_query = count_query.filter(is_active.eq(active_value));
        }

        let total = count_query.count().get_result::<i64>(&mut conn)? as i32;

        // 获取分页数据
        let host_list = query
            .select(Host::as_select())
            .order(created_at.desc())
            .limit(page_size.into())
            .offset(offset.into())
            .load::<Host>(&mut conn)?;

        let total_pages = (total + page_size - 1) / page_size;

        Ok(HostListResponse {
            hosts: host_list,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// 根据ID获取主机
    pub fn get_host_by_id(&self, auth: &AuthContext, host_id: i32) -> Result<Host> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;
        let host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| AuthError::Other("主机不存在".to_string()))?;

        // 权限检查：用户只能访问自己的主机，管理员可以访问所有主机
        if host.user_id != auth.user_id {
            self.check_permission(auth, Permission::HostRead)?;
        }

        Ok(host)
    }

    /// 更新主机
    pub fn update_host(
        &self,
        auth: &AuthContext,
        host_id: i32,
        request: UpdateHostRequest,
    ) -> Result<Host> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        // 检查主机是否存在并验证权限
        let existing_host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| AuthError::Other("主机不存在".to_string()))?;

        // 权限检查：用户只能修改自己的主机，管理员可以修改所有主机
        if existing_host.user_id != auth.user_id {
            self.check_permission(auth, Permission::HostUpdate)?;
        }

        // 检查主机名是否与其他主机冲突
        if let Some(ref new_name) = request.name {
            if new_name != &existing_host.name {
                let name_exists = hosts
                    .filter(user_id.eq(existing_host.user_id))
                    .filter(name.eq(new_name))
                    .filter(id.ne(host_id))
                    .first::<Host>(&mut conn)
                    .optional()?;

                if name_exists.is_some() {
                    return Err(AuthError::Other("主机名已存在".to_string()).into());
                }
            }
        }

        let now = chrono::Utc::now().timestamp() as i32;

        diesel::update(hosts.find(host_id))
            .set((
                name.eq(request.name.unwrap_or(existing_host.name)),
                content.eq(request.content.unwrap_or(existing_host.content)),
                description.eq(request.description.or(existing_host.description)),
                is_active.eq(
                    if request.is_active.unwrap_or(existing_host.is_active == 1) {
                        1
                    } else {
                        0
                    },
                ),
                updated_at.eq(now),
            ))
            .execute(&mut conn)?;

        let updated_host = hosts.find(host_id).first::<Host>(&mut conn)?;

        // 记录操作日志
        self.log_host_operation(
            auth.user_id,
            "update_host",
            host_id,
            Some(format!("更新主机 {}", updated_host.name)),
        )?;

        Ok(updated_host)
    }

    /// 删除主机
    pub fn delete_host(&self, auth: &AuthContext, host_id: i32) -> Result<()> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        // 检查主机是否存在并验证权限
        let existing_host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| AuthError::Other("主机不存在".to_string()))?;

        // 权限检查：用户只能删除自己的主机，管理员可以删除所有主机
        if existing_host.user_id != auth.user_id {
            self.check_permission(auth, Permission::HostDelete)?;
        }

        // 删除主机
        diesel::delete(hosts.find(host_id)).execute(&mut conn)?;

        // 记录操作日志
        self.log_host_operation(
            auth.user_id,
            "delete_host",
            host_id,
            Some(format!("删除主机 {}", existing_host.name)),
        )?;

        Ok(())
    }

    /// 切换主机活跃状态
    pub fn toggle_host_active(&self, auth: &AuthContext, host_id: i32) -> Result<Host> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        // 检查主机是否存在并验证权限
        let existing_host = hosts
            .find(host_id)
            .first::<Host>(&mut conn)
            .map_err(|_| AuthError::Other("主机不存在".to_string()))?;

        // 权限检查：用户只能修改自己的主机，管理员可以修改所有主机
        if existing_host.user_id != auth.user_id {
            self.check_permission(auth, Permission::HostUpdate)?;
        }

        let new_active_status = if existing_host.is_active == 0 { 1 } else { 0 };
        let now = chrono::Utc::now().timestamp() as i32;

        diesel::update(hosts.find(host_id))
            .set((is_active.eq(new_active_status), updated_at.eq(now)))
            .execute(&mut conn)?;

        let updated_host = hosts.find(host_id).first::<Host>(&mut conn)?;

        // 记录操作日志
        let action_desc = if new_active_status == 1 {
            format!("激活主机 {}", updated_host.name)
        } else {
            format!("停用主机 {}", updated_host.name)
        };

        self.log_host_operation(
            auth.user_id,
            "toggle_host_active",
            host_id,
            Some(action_desc),
        )?;

        Ok(updated_host)
    }

    /// 获取活跃主机列表
    pub fn get_active_hosts(&self, auth: &AuthContext) -> Result<Vec<Host>> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        let mut query = hosts.filter(is_active.eq(1)).into_boxed();

        // 权限检查：普通用户只能看到自己的主机，管理员可以看到所有主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            query = query.filter(user_id.eq(auth.user_id));
        }

        let active_hosts = query
            .select(Host::as_select())
            .order(name.asc())
            .load::<Host>(&mut conn)?;

        Ok(active_hosts)
    }

    /// 获取主机统计信息
    pub fn get_host_stats(&self, auth: &AuthContext) -> Result<HostStats> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        let mut total_query = hosts.into_boxed();
        let mut active_query = hosts.filter(is_active.eq(1)).into_boxed();
        let mut inactive_query = hosts.filter(is_active.eq(0)).into_boxed();

        // 如果不是管理员，只统计自己的主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            total_query = total_query.filter(user_id.eq(auth.user_id));
            active_query = active_query.filter(user_id.eq(auth.user_id));
            inactive_query = inactive_query.filter(user_id.eq(auth.user_id));
        }

        let total_hosts = total_query.count().get_result::<i64>(&mut conn)? as i32;
        let active_hosts = active_query.count().get_result::<i64>(&mut conn)? as i32;
        let inactive_hosts = inactive_query.count().get_result::<i64>(&mut conn)? as i32;

        // 当前用户的主机数
        let user_hosts = hosts
            .filter(user_id.eq(auth.user_id))
            .count()
            .get_result::<i64>(&mut conn)? as i32;

        Ok(HostStats {
            total_hosts,
            active_hosts,
            inactive_hosts,
            user_hosts,
        })
    }

    /// 导出主机数据
    pub fn export_hosts(&self, auth: &AuthContext) -> Result<HostExportData> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;

        let mut query = hosts.into_boxed();

        // 权限检查：普通用户只能导出自己的主机，管理员可以导出所有主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            query = query.filter(user_id.eq(auth.user_id));
        }

        let host_list = query
            .select(Host::as_select())
            .order(name.asc())
            .load::<Host>(&mut conn)?;

        let export_items: Vec<HostExportItem> = host_list
            .into_iter()
            .map(|host| HostExportItem {
                name: host.name,
                content: host.content,
                description: host.description,
                is_active: host.is_active == 1,
            })
            .collect();

        Ok(HostExportData {
            hosts: export_items,
            exported_at: chrono::Utc::now().timestamp() as i32,
            exported_by: auth.username.clone(),
        })
    }

    /// 导入主机数据
    pub fn import_hosts(
        &self,
        auth: &AuthContext,
        import_data: HostExportData,
        overwrite_existing: bool,
    ) -> Result<Vec<Host>> {
        self.validate_auth(auth)?;
        self.check_permission(auth, Permission::HostCreate)?;

        let mut conn = self.pool.get()?;
        let mut imported_hosts = Vec::new();
        let now = chrono::Utc::now().timestamp() as i32;

        for import_item in import_data.hosts {
            // 检查是否已存在相同名称的主机
            let existing_host = hosts
                .filter(user_id.eq(auth.user_id))
                .filter(name.eq(&import_item.name))
                .first::<Host>(&mut conn)
                .optional()?;

            if let Some(existing) = existing_host {
                if overwrite_existing {
                    // 更新现有主机
                    diesel::update(hosts.find(existing.id))
                        .set((
                            description.eq(&import_item.description),
                            content.eq(&import_item.content),
                            is_active.eq(if import_item.is_active { 1 } else { 0 }),
                            updated_at.eq(now),
                        ))
                        .execute(&mut conn)?;

                    let updated_host = hosts.find(existing.id).first::<Host>(&mut conn)?;

                    imported_hosts.push(updated_host);
                }
                // 如果不覆盖，跳过已存在的主机
            } else {
                // 创建新主机
                let new_host = NewHost {
                    user_id: auth.user_id,
                    name: import_item.name,
                    description: import_item.description,
                    content: import_item.content,
                    is_active: if import_item.is_active { 1 } else { 0 },
                    is_system: 0,
                    created_at: now,
                    updated_at: now,
                };

                diesel::insert_into(hosts)
                    .values(&new_host)
                    .execute(&mut conn)?;

                let created_host = hosts.order(id.desc()).first::<Host>(&mut conn)?;

                imported_hosts.push(created_host);
            }
        }

        // 记录导入操作日志
        self.log_host_operation(
            auth.user_id,
            "import_hosts",
            0, // 使用0表示批量操作
            Some(format!("导入 {} 个主机", imported_hosts.len())),
        )?;

        Ok(imported_hosts)
    }

    /// 搜索主机
    pub fn search_hosts(
        &self,
        auth: &AuthContext,
        search_term: &str,
        limit: Option<i32>,
    ) -> Result<Vec<Host>> {
        self.validate_auth(auth)?;

        let mut conn = self.pool.get()?;
        let search_pattern = format!("%{}%", search_term);
        let limit = limit.unwrap_or(10).clamp(1, 50);

        let mut query = hosts
            .filter(
                name.like(&search_pattern)
                    .or(description.like(&search_pattern))
                    .or(content.like(&search_pattern)),
            )
            .into_boxed();

        // 权限检查：普通用户只能搜索自己的主机，管理员可以搜索所有主机
        if self.check_permission(auth, Permission::HostRead).is_err() {
            query = query.filter(user_id.eq(auth.user_id));
        }

        let search_results = query
            .select(Host::as_select())
            .order(name.asc())
            .limit(limit.into())
            .load::<Host>(&mut conn)?;

        Ok(search_results)
    }

    /// 记录主机操作日志
    fn log_host_operation(
        &self,
        operator_id: i32,
        action: &str,
        target_host_id: i32,
        details: Option<String>,
    ) -> Result<()> {
        use crate::db::schema::logs;

        let mut conn = self.pool.get()?;

        let log_entry = NewLog {
            user_id: operator_id,
            action: action.to_string(),
            target_type: "host".to_string(),
            target_id: Some(target_host_id),
            details,
            created_at: chrono::Utc::now().timestamp() as i32,
        };

        // 如果日志表不存在则忽略错误（用于测试环境）
        if let Err(e) = diesel::insert_into(logs::table)
            .values(&log_entry)
            .execute(&mut conn) {
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
    use crate::auth::{Role};
    use crate::db::tests::create_test_db;

    fn create_test_auth_context(test_user_id: i32, role: Role) -> AuthContext {
        let permissions = crate::auth::permissions::PermissionManager::get_role_permissions(&role);
        AuthContext {
            user_id: test_user_id,
            username: "test_user".to_string(),
            role,
            permissions,
            session_id: "test_session".to_string(),
            expires_at: chrono::Utc::now().timestamp() as i64 + 3600,
        }
    }

    #[test]
    fn test_create_host() {
        let (pool, _temp_dir) = create_test_db();
        let host_service = HostService::new(pool.clone());

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

        // 测试创建主机
        let create_request = CreateHostRequest {
            name: "Test Host".to_string(),
            description: Some("Test host description".to_string()),
            content: "192.168.1.100 test.local".to_string(),
            is_active: Some(true),
        };

        let host = host_service.create_host(&auth, create_request).unwrap();
        assert_eq!(host.name, "Test Host");
        assert_eq!(host.content, "192.168.1.100 test.local");
        assert_eq!(host.user_id, user.id);
        assert_eq!(host.is_active, 1);
    }

    #[test]
    fn test_get_host_by_id() {
        let (pool, _temp_dir) = create_test_db();
        let host_service = HostService::new(pool.clone());

        // 创建测试用户和主机
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            username: Some("test_user".to_string()),
            password: "test_password".to_string(),
            email: "test@example.com".to_string(),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();
        let auth = create_test_auth_context(user.id, Role::User);

        let create_request = CreateHostRequest {
            name: "Test Host".to_string(),
            description: None,
            content: "192.168.1.100 test.local".to_string(),
            is_active: Some(false),
        };

        let host = host_service.create_host(&auth, create_request).unwrap();

        // 测试获取主机
        let retrieved_host = host_service
            .get_host_by_id(&auth, host.id)
            .unwrap();
        assert_eq!(retrieved_host.name, "Test Host");
        assert_eq!(retrieved_host.user_id, user.id);
    }

    #[test]
    fn test_toggle_host_active() {
        let (pool, _temp_dir) = create_test_db();
        let host_service = HostService::new(pool.clone());

        // 创建测试用户和主机
        let auth_service = crate::db::services::auth_service::AuthService::new(pool.clone());
        let register_request = crate::db::services::auth_service::RegisterRequest {
            username: Some("test_user".to_string()),
            password: "test_password".to_string(),
            email: "test@example.com".to_string(),
            avatar: None,
        };

        let user = auth_service.register(register_request).unwrap();
        let auth = create_test_auth_context(user.id, Role::User);

        let create_request = CreateHostRequest {
            name: "Test Host".to_string(),
            description: None,
            content: "192.168.1.100 test.local".to_string(),
            is_active: Some(false),
        };

        let host = host_service.create_host(&auth, create_request).unwrap();
        assert_eq!(host.is_active, 0);

        // 测试切换活跃状态
        let toggled_host = host_service
            .toggle_host_active(&auth, host.id)
            .unwrap();
        assert_eq!(toggled_host.is_active, 1);

        // 再次切换
        let toggled_again = host_service
            .toggle_host_active(&auth, host.id)
            .unwrap();
        assert_eq!(toggled_again.is_active, 0);
    }
}