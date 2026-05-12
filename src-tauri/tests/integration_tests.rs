//! 集成测试
//!
//! 对整个系统进行端到端测试，包括认证、权限、数据操作等

use app_lib::{
    api::{AppState},
    auth::{AuthContext, Role, permissions::PermissionManager},
    db::{
        models::{User},
        services::{
            auth_service::{LoginRequest, RegisterRequest},
            host_service::{CreateHostRequest, UpdateHostRequest},
            host_group_service::{CreateHostGroupRequest, UpdateHostGroupRequest},
            ServiceFactory,
        },
        create_pool,
    },
};
use chrono;
use tempfile::TempDir;

/// 创建测试数据库连接池
fn create_test_db() -> (app_lib::db::DbPool, TempDir) {
    use diesel::RunQueryDsl;
    
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    
    // 使用内存数据库
    let db_url = ":memory:";
    
    // 设置环境变量
    std::env::set_var("DATABASE_URL", db_url);
    
    let pool = create_pool().expect("无法创建数据库连接池");
    
    // 手动创建表结构（用于测试）
    let mut conn = pool.get().expect("无法获取数据库连接");
    
    // 创建用户表
    diesel::sql_query(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            email TEXT,
            avatar TEXT,
            is_admin INTEGER,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000)
        )"
    ).execute(&mut conn).expect("无法创建用户表");
    
    // 创建日志表
    diesel::sql_query(
        "CREATE TABLE logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            action TEXT NOT NULL,
            target_type TEXT NOT NULL,
            target_id INTEGER,
            details TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    ).execute(&mut conn).expect("无法创建日志表");
    
    // 创建主机表
    diesel::sql_query(
        "CREATE TABLE hosts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            content TEXT NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 0,
            is_system INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    ).execute(&mut conn).expect("无法创建主机表");
    
    // 创建主机组表
    diesel::sql_query(
        "CREATE TABLE host_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    ).execute(&mut conn).expect("无法创建主机组表");
    
    // 创建主机组关联表
    diesel::sql_query(
        "CREATE TABLE host_group_relations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL,
            host_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            FOREIGN KEY (group_id) REFERENCES host_groups(id) ON DELETE CASCADE,
            FOREIGN KEY (host_id) REFERENCES hosts(id) ON DELETE CASCADE,
            UNIQUE(group_id, host_id)
        )"
    ).execute(&mut conn).expect("无法创建主机组关联表");
    
    (pool, temp_dir)
}

/// 创建测试应用状态
fn create_test_app_state() -> (AppState, TempDir) {
    let (pool, temp_dir) = create_test_db();
    let service_factory = ServiceFactory::new(pool);
    let app_state = AppState::new(service_factory);
    (app_state, temp_dir)
}

/// 创建测试用户并返回认证上下文
async fn create_test_user_and_auth(
    app_state: &AppState,
    username: &str,
    is_admin: bool,
) -> (User, AuthContext) {
    let auth_service = app_state.service_factory.auth_service();
    
    // 注册用户
    let register_request = RegisterRequest {
        email: format!("{}@example.com", username),
        password: "test_password".to_string(),
        username: Some(username.to_string()),
        avatar: None,
    };
    
    let user = auth_service.register(register_request).unwrap();
    
    // 在测试环境中，直接在 AuthContext 中设置角色
    
    // 创建认证上下文
    let role = if is_admin { Role::Admin } else { Role::User };
    let permissions = PermissionManager::get_role_permissions(&role);
    
    let auth_context = AuthContext {
        user_id: user.id,
        username: user.username.clone(),
        role,
        permissions,
        session_id: "test_session".to_string(),
        expires_at: chrono::Utc::now().timestamp() + 3600,
    };
    
    (user, auth_context)
}

#[tokio::test]
async fn test_complete_user_workflow() {
    let (app_state, _temp_dir) = create_test_app_state();
    
    // 1. 用户注册
    let auth_service = app_state.service_factory.auth_service();
    let register_request = RegisterRequest {
        username: Some("test_user".to_string()),
        password: "test_password".to_string(),
        email: "test@example.com".to_string(),
        avatar: None,
    };
    
    let user = auth_service.register(register_request).unwrap();
    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    assert_eq!(user.is_admin, Some(false));
    
    // 2. 用户登录
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
        remember_me: Some(false),
    };
    
    let login_result = auth_service.login(login_request).unwrap();
    assert_eq!(login_result.user.username, "test_user");
    assert!(!login_result.tokens.access_token.token.is_empty());
    assert!(!login_result.tokens.refresh_token.token.is_empty());
    
    // 3. 验证令牌
    let user_info = auth_service.verify_token(&login_result.tokens.access_token.token).unwrap();
    assert_eq!(user_info.username, "test_user");
    
    // 4. 刷新令牌
    let new_tokens = auth_service.refresh_token(&login_result.tokens.refresh_token.token).unwrap();
    assert!(!new_tokens.access_token.token.is_empty());
    assert!(!new_tokens.refresh_token.token.is_empty());
}

#[tokio::test]
async fn test_complete_host_workflow() {
    let (app_state, _temp_dir) = create_test_app_state();
    let (user, auth) = create_test_user_and_auth(&app_state, "host_user", false).await;
    
    let host_service = app_state.service_factory.host_service();
    
    // 1. 创建主机记录
    let create_request = CreateHostRequest {
        name: "Test Host".to_string(),
        content: "192.168.1.100 test.example.com".to_string(),
        description: Some("Test host description".to_string()),
        is_active: Some(true),
    };
    
    let host = host_service.create_host(&auth, create_request).unwrap();
    assert_eq!(host.name, "Test Host");
    assert_eq!(host.content, "192.168.1.100 test.example.com");
    assert_eq!(host.user_id, user.id);
    assert_eq!(host.is_active, 1);
    
    // 2. 获取主机记录
    let retrieved_host = host_service.get_host_by_id(&auth, host.id).unwrap();
    assert_eq!(retrieved_host.id, host.id);
    assert_eq!(retrieved_host.name, host.name);
    
    // 3. 更新主机记录
    let update_request = UpdateHostRequest {
        name: Some("Updated Test Host".to_string()),
        content: Some("192.168.1.101 updated.example.com".to_string()),
        description: Some("Updated description".to_string()),
        is_active: Some(false),
    };
    
    let updated_host = host_service.update_host(&auth, host.id, update_request).unwrap();
    assert_eq!(updated_host.name, "Updated Test Host");
    assert_eq!(updated_host.content, "192.168.1.101 updated.example.com");
    assert_eq!(updated_host.is_active, 0);
    
    // 4. 切换主机状态
    let toggled_host = host_service.toggle_host_active(&auth, host.id).unwrap();
    assert_eq!(toggled_host.is_active, 1); // 应该从 0 变为 1
    
    // 5. 获取用户的主机列表
    let host_list = host_service.get_hosts(&auth, Default::default()).unwrap();
    assert_eq!(host_list.hosts.len(), 1);
    assert_eq!(host_list.total, 1);
    
    // 6. 删除主机记录
    host_service.delete_host(&auth, host.id).unwrap();
    
    // 验证删除
    let result = host_service.get_host_by_id(&auth, host.id);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_complete_host_group_workflow() {
    let (app_state, _temp_dir) = create_test_app_state();
    let (_user, auth) = create_test_user_and_auth(&app_state, "group_user", false).await;
    
    let host_service = app_state.service_factory.host_service();
    let host_group_service = app_state.service_factory.host_group_service();
    
    // 1. 创建主机记录
    let create_host_request = CreateHostRequest {
        name: "Group Test Host".to_string(),
        content: "192.168.1.200 group-test.example.com".to_string(),
        description: None,
        is_active: Some(true),
    };
    
    let host = host_service.create_host(&auth, create_host_request).unwrap();
    
    // 2. 创建主机组
    let create_group_request = CreateHostGroupRequest {
        name: "Test Group".to_string(),
        description: Some("Test group description".to_string()),
        is_active: None,
    };
    
    let group = host_group_service.create_host_group(&auth, create_group_request).unwrap();
    assert_eq!(group.name, "Test Group");
    assert_eq!(group.user_id, auth.user_id);
    assert!(!group.name.is_empty());
    
    // 3. 添加主机到组
    host_group_service.add_host_to_group(&auth, group.id, host.id).unwrap();
    
    // 4. 获取组的主机列表
    let group_detail = host_group_service.get_host_group_by_id(&auth, group.id).unwrap();
    let group_hosts = group_detail.hosts;
    assert_eq!(group_hosts.len(), 1);
    assert_eq!(group_hosts[0].id, host.id);
    
    // 5. 更新主机组
    let update_group_request = UpdateHostGroupRequest {
        name: Some("Updated Test Group".to_string()),
        description: Some("Updated description".to_string()),
        is_active: Some(false),
    };
    
    let updated_group = host_group_service.update_host_group(&auth, group.id, update_group_request).unwrap();
    assert_eq!(updated_group.name, "Updated Test Group");
    assert!(!updated_group.name.is_empty());
    assert_eq!(updated_group.is_active, 0);

    let updated_host = host_service.get_host_by_id(&auth, host.id).unwrap();
    assert_eq!(updated_host.is_active, 0);
    
    // 6. 从组中移除主机
    host_group_service.remove_host_from_group(&auth, group.id, host.id).unwrap();
    
    // 验证主机已从组中移除
    let empty_group_detail = host_group_service.get_host_group_by_id(&auth, group.id).unwrap();
    let empty_group_hosts = empty_group_detail.hosts;
    assert_eq!(empty_group_hosts.len(), 0);
    
    // 7. 删除主机组
    host_group_service.delete_host_group(&auth, group.id).unwrap();
    
    // 验证删除
    let result = host_group_service.get_host_group_by_id(&auth, group.id);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_permission_system() {
    let (app_state, _temp_dir) = create_test_app_state();
    
    // 创建普通用户和管理员用户
    let (normal_user, normal_auth) = create_test_user_and_auth(&app_state, "normal_user", false).await;
    let (admin_user, admin_auth) = create_test_user_and_auth(&app_state, "admin_user", true).await;
    
    let user_service = app_state.service_factory.user_service();
    
    // 1. 测试普通用户权限
    // 普通用户应该能够查看自己的信息
    let user_info = user_service.get_user_by_id(&normal_auth, normal_user.id).unwrap();
    assert_eq!(user_info.id, normal_user.id);
    
    // 普通用户不应该能够查看其他用户的信息
    let result = user_service.get_user_by_id(&normal_auth, admin_user.id);
    assert!(result.is_err());
    
    // 普通用户不应该能够获取用户列表
    let result = user_service.get_users(&normal_auth, Default::default());
    assert!(result.is_err());
    
    // 2. 测试管理员权限
    // 管理员应该能够查看所有用户信息
    let admin_user_info = user_service.get_user_by_id(&admin_auth, normal_user.id).unwrap();
    assert_eq!(admin_user_info.id, normal_user.id);
    
    // 管理员应该能够获取用户列表
    let user_list = user_service.get_users(&admin_auth, Default::default()).unwrap();
    assert!(user_list.total >= 2); // 至少有两个用户
    
    // 管理员应该能够创建新用户
    let register_request = RegisterRequest {
        username: Some("created_user".to_string()),
        password: "password123".to_string(),
        email: "created@example.com".to_string(),
        avatar: None,
    };
    
    let auth_service = app_state.service_factory.auth_service();
    let created_user = auth_service.register(register_request).unwrap();
    assert_eq!(created_user.username, "created_user");
    
    // 管理员应该能够删除用户
    user_service.delete_user(&admin_auth, created_user.id).unwrap();
    
    // 验证用户已删除
    let result = user_service.get_user_by_id(&admin_auth, created_user.id);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logging_system() {
    let (app_state, _temp_dir) = create_test_app_state();
    let (user, auth) = create_test_user_and_auth(&app_state, "log_user", true).await;
    
    let log_service = app_state.service_factory.log_service();
    let host_service = app_state.service_factory.host_service();
    
    // 1. 创建一些操作以生成日志
    let create_host_request = CreateHostRequest {
        name: "Log Test Host".to_string(),
        content: "192.168.1.300 log-test.example.com".to_string(),
        description: None,
        is_active: Some(true),
    };
    
    let host = host_service.create_host(&auth, create_host_request).unwrap();
    
    // 2. 手动创建日志记录
    let log = log_service.create_log(
        user.id,
        "test_action".to_string(),
        "test_target".to_string(),
        Some(host.id),
        Some("Test log entry".to_string()),
    ).unwrap();
    
    assert_eq!(log.user_id, user.id);
    assert_eq!(log.action, "test_action");
    assert_eq!(log.target_type, "test_target");
    assert_eq!(log.target_id, Some(host.id));
    
    // 3. 获取日志列表
    let log_list = log_service.get_logs(&auth, Default::default()).unwrap();
    assert!(log_list.total >= 1);
    assert!(!log_list.logs.is_empty());
    
    // 4. 根据ID获取日志
    let retrieved_log = log_service.get_log_by_id(&auth, log.id).unwrap();
    assert_eq!(retrieved_log.log.id, log.id);
    assert_eq!(retrieved_log.username, user.username);
    
    // 5. 获取日志统计信息
    let log_stats = log_service.get_log_stats(&auth).unwrap();
    assert!(log_stats.total_logs >= 1);
    assert!(log_stats.today_logs >= 1);
    
    // 6. 搜索日志
    let search_results = log_service.search_logs(&auth, "test_action", Some(10)).unwrap();
    assert!(!search_results.is_empty());
    assert!(search_results.iter().any(|l| l.log.action == "test_action"));
}

#[tokio::test]
async fn test_data_consistency() {
    let (app_state, _temp_dir) = create_test_app_state();
    let (_user, auth) = create_test_user_and_auth(&app_state, "consistency_user", false).await;
    
    let host_service = app_state.service_factory.host_service();
    let host_group_service = app_state.service_factory.host_group_service();
    
    // 1. 创建主机和主机组
    let create_host_request = CreateHostRequest {
        name: "Consistency Test Host".to_string(),
        content: "192.168.1.400 consistency-test.example.com".to_string(),
        description: None,
        is_active: Some(true),
    };
    
    let host = host_service.create_host(&auth, create_host_request).unwrap();
    
    let create_group_request = CreateHostGroupRequest {
        name: "Consistency Test Group".to_string(),
        description: None,
        is_active: None,
    };
    
    let group = host_group_service.create_host_group(&auth, create_group_request).unwrap();
    
    // 2. 将主机添加到组
    host_group_service.add_host_to_group(&auth, group.id, host.id).unwrap();
    
    // 3. 验证关联关系
    let group_detail = host_group_service.get_host_group_by_id(&auth, group.id).unwrap();
    let group_hosts = group_detail.hosts;
    assert_eq!(group_hosts.len(), 1);
    assert_eq!(group_hosts[0].id, host.id);
    
    // 4. 删除主机，应该自动清理关联关系
    host_service.delete_host(&auth, host.id).unwrap();
    
    // 5. 验证关联关系已清理
    let empty_group_detail = host_group_service.get_host_group_by_id(&auth, group.id).unwrap();
    let empty_group_hosts = empty_group_detail.hosts;
    assert_eq!(empty_group_hosts.len(), 0);
    
    // 6. 删除主机组
    host_group_service.delete_host_group(&auth, group.id).unwrap();
}

#[tokio::test]
async fn test_concurrent_operations() {
    let (app_state, _temp_dir) = create_test_app_state();
    let (_user, auth) = create_test_user_and_auth(&app_state, "concurrent_user", false).await;
    
    let host_service = app_state.service_factory.host_service();
    
    // 并发创建多个主机记录
    let mut handles = vec![];
    
    for i in 0..10 {
        let auth_clone = auth.clone();
        let host_service_clone = host_service.clone();
        
        let handle = tokio::spawn(async move {
            let create_request = CreateHostRequest {
                name: format!("Concurrent Host {}", i),
                content: format!("192.168.1.{} concurrent-{}.example.com", 100 + i, i),
                description: None,
                is_active: Some(true),
            };
            
            host_service_clone.create_host(&auth_clone, create_request)
        });
        
        handles.push(handle);
    }
    
    // 等待所有操作完成
    let mut created_hosts = vec![];
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
        created_hosts.push(result.unwrap());
    }
    
    // 验证所有主机都已创建
    assert_eq!(created_hosts.len(), 10);
    
    // 验证主机名称唯一性
    let mut names: Vec<String> = created_hosts.iter().map(|h| h.name.clone()).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), 10); // 所有名称应该是唯一的
    
    // 清理创建的主机
    for host in created_hosts {
        host_service.delete_host(&auth, host.id).unwrap();
    }
}
