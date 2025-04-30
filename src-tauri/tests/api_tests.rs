use switchhosts_r::{
    api::{host_groups, hosts, logs, users},
    db::{
        create_pool,
        models::{Host, HostGroup, Log, User},
    },
    ui::app_state::create_app_state,
};
use tauri::State;

#[tokio::test]
async fn test_logs_api() {
    // 创建应用状态
    let app_state = create_app_state();

    // 创建测试日志
    let test_log = Log {
        id: None,
        user_id: 1,
        action: "test_action".to_string(),
        target_type: "test_target".to_string(),
        target_id: Some(1),
        details: Some("测试日志详情".to_string()),
        created_at: 0,
    };

    // 测试添加日志
    let result = logs::add_log(State::new(app_state.clone()), test_log.clone()).await;
    assert!(result.is_ok(), "添加日志失败: {:?}", result.err());

    let added_log = result.unwrap();
    assert_eq!(added_log.action, "test_action");
    assert_eq!(added_log.target_type, "test_target");
    assert_eq!(added_log.details, Some("测试日志详情".to_string()));

    // 测试获取用户日志
    let result = logs::get_user_logs(State::new(app_state.clone()), 1, Some(10)).await;
    assert!(result.is_ok(), "获取用户日志失败: {:?}", result.err());

    let user_logs = result.unwrap();
    assert!(!user_logs.is_empty(), "用户日志不应为空");
    assert_eq!(user_logs[0].user_id, 1);
}

#[tokio::test]
async fn test_users_api() {
    // 创建应用状态
    let app_state = create_app_state();

    // 创建测试用户
    let test_user = User {
        id: None,
        username: "test_user".to_string(),
        password: "test_password".to_string(),
        email: Some("test@example.com".to_string()),
        avatar: None,
        is_admin: Some(false),
        created_at: 0,
        updated_at: 0,
    };

    // 测试创建用户
    let result = users::create_user(State::new(app_state.clone()), test_user.clone()).await;
    assert!(result.is_ok(), "创建用户失败: {:?}", result.err());

    let created_user = result.unwrap();
    assert_eq!(created_user.username, "test_user");
    assert_eq!(created_user.email, Some("test@example.com".to_string()));

    // 测试用户登录
    let result = users::user_login(
        State::new(app_state.clone()),
        "test_user".to_string(),
        "test_password".to_string(),
    )
    .await;
    assert!(result.is_ok(), "用户登录失败: {:?}", result.err());
}

#[tokio::test]
async fn test_hosts_api() {
    // 创建应用状态
    let app_state = create_app_state();

    // 创建测试主机
    let test_host = Host {
        id: None,
        user_id: 1,
        name: "test_host".to_string(),
        description: Some("测试主机描述".to_string()),
        content: "127.0.0.1 localhost".to_string(),
        is_active: 1,
        is_system: 0,
        created_at: 0,
        updated_at: 0,
    };

    // 测试创建主机
    let result = hosts::create_host(State::new(app_state.clone()), test_host.clone()).await;
    assert!(result.is_ok(), "创建主机失败: {:?}", result.err());

    let created_host = result.unwrap();
    assert_eq!(created_host.name, "test_host");
    assert_eq!(created_host.description, Some("测试主机描述".to_string()));
    assert_eq!(created_host.content, "127.0.0.1 localhost");
}

#[tokio::test]
async fn test_host_groups_api() {
    // 创建应用状态
    let app_state = create_app_state();

    // 创建测试主机组
    let test_group = HostGroup {
        id: None,
        user_id: 1,
        name: "test_group".to_string(),
        description: Some("测试主机组描述".to_string()),
        created_at: 0,
        updated_at: 0,
    };

    // 测试创建主机组
    let result =
        host_groups::create_host_group(State::new(app_state.clone()), test_group.clone()).await;
    assert!(result.is_ok(), "创建主机组失败: {:?}", result.err());

    let created_group = result.unwrap();
    assert_eq!(created_group.name, "test_group");
    assert_eq!(
        created_group.description,
        Some("测试主机组描述".to_string())
    );
}
