use app_lib::{
    auth::{permissions::PermissionManager, AuthContext, Role},
    db::{
        create_pool,
        models::User,
        services::{
            auth_service::{LoginRequest, RegisterRequest},
            host_group_service::{CreateHostGroupRequest, HostGroupQueryParams},
            host_service::{CreateHostRequest, HostQueryParams},
            log_service::LogQueryParams,
            ServiceFactory,
        },
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
        )",
    )
    .execute(&mut conn)
    .expect("无法创建用户表");

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
        )",
    )
    .execute(&mut conn)
    .expect("无法创建日志表");

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
        )",
    )
    .execute(&mut conn)
    .expect("无法创建主机表");

    // 创建主机组表
    diesel::sql_query(
        "CREATE TABLE host_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
    )
    .execute(&mut conn)
    .expect("无法创建主机组表");

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
        )",
    )
    .execute(&mut conn)
    .expect("无法创建主机组关联表");

    (pool, temp_dir)
}

/// 创建测试服务工厂
fn create_test_service_factory() -> (ServiceFactory, TempDir) {
    let (pool, temp_dir) = create_test_db();
    let service_factory = ServiceFactory::new(pool);
    (service_factory, temp_dir)
}

/// 创建测试用户并返回认证上下文
async fn create_test_user_and_auth(
    service_factory: &ServiceFactory,
    username: &str,
    is_admin: bool,
) -> (User, AuthContext) {
    let auth_service = service_factory.auth_service();

    // 注册用户
    let register_request = RegisterRequest {
        username: Some(username.to_string()),
        password: "test_password".to_string(),
        email: format!("{}@example.com", username),
        avatar: None,
    };

    let user = auth_service.register(register_request).unwrap();

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
async fn test_logs_service() {
    let (service_factory, _temp_dir) = create_test_service_factory();
    let (_user, auth) = create_test_user_and_auth(&service_factory, "log_user", false).await;

    let log_service = service_factory.log_service();

    // 测试创建日志
    let log = log_service
        .create_log(
            auth.user_id,
            "test_action".to_string(),
            "test_target".to_string(),
            Some(1),
            Some("测试日志详情".to_string()),
        )
        .unwrap();

    assert_eq!(log.action, "test_action");
    assert_eq!(log.target_type, "test_target");
    assert_eq!(log.target_id, Some(1));
    assert_eq!(log.details, Some("测试日志详情".to_string()));
    assert_eq!(log.user_id, auth.user_id);

    // 测试获取用户日志
    let log_params = LogQueryParams {
        page: Some(1),
        page_size: Some(10),
        user_id: Some(auth.user_id),
        ..Default::default()
    };

    let log_result = log_service.get_logs(&auth, log_params).unwrap();
    assert!(!log_result.logs.is_empty(), "用户日志不应为空");
    assert_eq!(log_result.logs[0].log.user_id, auth.user_id);
    assert_eq!(log_result.logs[0].log.action, "test_action");
}

#[tokio::test]
async fn test_users_service() {
    let (service_factory, _temp_dir) = create_test_service_factory();

    let auth_service = service_factory.auth_service();

    // 测试用户注册
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

    // 测试用户登录
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
        remember_me: Some(false),
    };

    let login_result = auth_service.login(login_request).unwrap();
    assert_eq!(login_result.user.username, "test_user");
    assert!(!login_result.tokens.access_token.token.is_empty());
    assert!(!login_result.tokens.refresh_token.token.is_empty());

    // 测试令牌验证
    let user_info = auth_service
        .verify_token(&login_result.tokens.access_token.token)
        .unwrap();
    assert_eq!(user_info.username, "test_user");
}

#[tokio::test]
async fn test_hosts_service() {
    let (service_factory, _temp_dir) = create_test_service_factory();
    let (_user, auth) = create_test_user_and_auth(&service_factory, "host_user", false).await;

    let host_service = service_factory.host_service();

    // 测试创建主机
    let create_request = CreateHostRequest {
        name: "test_host".to_string(),
        content: "127.0.0.1 localhost".to_string(),
        description: Some("测试主机描述".to_string()),
        is_active: Some(true),
    };

    let host = host_service.create_host(&auth, create_request).unwrap();
    assert_eq!(host.name, "test_host");
    assert_eq!(host.content, "127.0.0.1 localhost");
    assert_eq!(host.description, Some("测试主机描述".to_string()));
    assert_eq!(host.user_id, auth.user_id);
    assert_eq!(host.is_active, 1);

    // 测试获取主机
    let retrieved_host = host_service.get_host_by_id(&auth, host.id).unwrap();
    assert_eq!(retrieved_host.id, host.id);
    assert_eq!(retrieved_host.name, host.name);

    // 测试获取主机列表
    let query_params = HostQueryParams {
        page: None,
        page_size: None,
        search: None,
        active_only: None,
        user_id: None,
    };
    let host_list = host_service.get_hosts(&auth, query_params).unwrap();
    assert_eq!(host_list.hosts.len(), 1);
    assert_eq!(host_list.total, 1);
}

#[tokio::test]
async fn test_host_groups_service() {
    let (service_factory, _temp_dir) = create_test_service_factory();
    let (_user, auth) = create_test_user_and_auth(&service_factory, "group_user", false).await;

    let host_group_service = service_factory.host_group_service();

    // 测试创建主机组
    let create_request = CreateHostGroupRequest {
        name: "test_group".to_string(),
        description: Some("测试主机组描述".to_string()),
    };

    let group = host_group_service
        .create_host_group(&auth, create_request)
        .unwrap();
    assert_eq!(group.name, "test_group");
    assert_eq!(group.description, Some("测试主机组描述".to_string()));
    assert_eq!(group.user_id, auth.user_id);

    // 测试获取主机组
    let retrieved_group = host_group_service
        .get_host_group_by_id(&auth, group.id)
        .unwrap();
    assert_eq!(retrieved_group.host_group.id, group.id);
    assert_eq!(retrieved_group.host_group.name, group.name);

    // 测试获取主机组列表
    let query_params = HostGroupQueryParams {
        page: None,
        page_size: None,
        search: None,
        keyword: None,
        active_only: None,
        user_id: None,
    };
    let group_list = host_group_service
        .get_host_groups(&auth, query_params)
        .unwrap();
    assert_eq!(group_list.host_groups.len(), 1);
    assert_eq!(group_list.total, 1);
}
