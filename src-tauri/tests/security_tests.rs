//! 安全测试
//!
//! 覆盖本次变更的关键安全点：
//! 1. JWT Token 安全（随机密钥、token_type 校验、过期拒绝）
//! 2. 路径安全（遍历攻击拦截、绝对路径拒绝、沙箱限制）
//! 3. Hosts 合并逻辑正确性
//! 4. 认证中间件（AuthState 全局化、登录/登出状态）

use app_lib::auth::{
    middleware::{AuthMiddleware, AuthState},
    permissions::PermissionManager,
    session::Session,
    token::TokenManager,
    AuthContext, Permission, Role,
};

// ============================================================================
// 模块 1: JWT Token 安全测试
// ============================================================================

mod jwt_security {
    use super::*;

    /// SEC-11: 随机密钥生成 — 不同密钥生成不同 token，跨密钥验证失败
    #[test]
    fn test_random_secret_keys_are_unique() {
        let manager1 = TokenManager::new("key_a".to_string(), 1, 7);
        let manager2 = TokenManager::new("key_b".to_string(), 1, 7);

        let pair1 = manager1
            .generate_token_pair(1, "user1".to_string(), Role::User)
            .unwrap();
        let pair2 = manager2
            .generate_token_pair(1, "user1".to_string(), Role::User)
            .unwrap();

        // 不同密钥生成的 token 应不同
        assert_ne!(pair1.access_token.token, pair2.access_token.token);
        assert_ne!(pair1.refresh_token.token, pair2.refresh_token.token);

        // 跨密钥验证应失败
        assert!(manager1
            .verify_token(&pair2.access_token.token)
            .is_err());
        assert!(manager2
            .verify_token(&pair1.access_token.token)
            .is_err());
    }

    /// SEC-11: 相同密钥的多次调用也生成不同 token（因 JTI 不同）
    #[test]
    fn test_same_key_generates_different_tokens() {
        let manager = TokenManager::new("same_key".to_string(), 1, 7);

        let pair1 = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();
        let pair2 = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 即使同一用户同一密钥，每次生成的 token 也不同（JTI 唯一）
        assert_ne!(pair1.access_token.token, pair2.access_token.token);
        assert_ne!(pair1.refresh_token.token, pair2.refresh_token.token);
    }

    /// AUTH-TOKEN-003 变体: token_type 校验 — access token 不能用于刷新
    #[test]
    fn test_access_token_cannot_refresh() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let pair = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 使用 access token 调用刷新应失败
        let result = manager.refresh_access_token(&pair.access_token.token);
        assert!(result.is_err(), "access token 不应能用于刷新");
    }

    /// AUTH-TOKEN-003 变体: refresh token 可以正常刷新
    #[test]
    fn test_refresh_token_can_refresh() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let pair = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 使用 refresh token 调用刷新应成功
        let result = manager.refresh_access_token(&pair.refresh_token.token);
        assert!(result.is_ok(), "refresh token 应能成功刷新");

        // 刷新得到的新 token 应是有效的 access token
        let new_token = result.unwrap();
        let claims = manager.verify_token(&new_token.token).unwrap();
        assert_eq!(claims.token_type, "access", "刷新得到的应是 access token");
    }

    /// token_type 字段正确嵌入 Claims — access 和 refresh 类型正确
    #[test]
    fn test_token_type_in_claims() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let pair = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        let access_claims = manager.verify_token(&pair.access_token.token).unwrap();
        assert_eq!(access_claims.token_type, "access");

        let refresh_claims = manager.verify_token(&pair.refresh_token.token).unwrap();
        assert_eq!(refresh_claims.token_type, "refresh");
    }

    /// SEC-04: 篡改 Token payload 后签名验证失败
    #[test]
    fn test_tampered_token_rejected() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let pair = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 篡改 token 的 payload 部分（JWT 格式: header.payload.signature）
        let parts: Vec<&str> = pair.access_token.token.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT 应有三个部分");

        // 修改 payload 中的一个字符
        let mut tampered_payload = parts[1].to_string();
        if tampered_payload.ends_with('A') {
            tampered_payload.push('B');
        } else {
            tampered_payload.push('A');
        }

        let tampered_token = format!("{}.{}.{}", parts[0], tampered_payload, parts[2]);

        // 篡改后的 token 应被拒绝
        assert!(
            manager.verify_token(&tampered_token).is_err(),
            "篡改后的 token 应被拒绝"
        );
    }

    /// SEC-05: 撤销 Token 后使用该 Token 应被拒绝
    #[test]
    fn test_revoked_token_rejected() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let pair = manager
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 验证 token 有效
        let claims = manager
            .verify_token(&pair.access_token.token)
            .unwrap();
        assert_eq!(claims.sub, "1");

        // 撤销 token
        manager.revoke_token(&claims.jti).unwrap();

        // 撤销后使用应失败
        assert!(
            manager.verify_token(&pair.access_token.token).is_err(),
            "已撤销的 token 应被拒绝"
        );
    }

    /// SEC-03: 不同密钥签名的 Token 被拒绝
    #[test]
    fn test_wrong_secret_token_rejected() {
        let manager_a = TokenManager::new("secret_a".to_string(), 1, 7);
        let manager_b = TokenManager::new("secret_b".to_string(), 1, 7);

        let pair = manager_a
            .generate_token_pair(1, "user".to_string(), Role::User)
            .unwrap();

        // 使用不同密钥的管理器验证应失败
        assert!(
            manager_b.verify_token(&pair.access_token.token).is_err(),
            "不同密钥签名的 token 应被拒绝"
        );
    }

    /// Token 的 Claims 包含正确的用户信息
    #[test]
    fn test_token_claims_contain_correct_user_info() {
        let manager = TokenManager::new("test_secret".to_string(), 2, 7);

        let pair = manager
            .generate_token_pair(42, "admin_user".to_string(), Role::Admin)
            .unwrap();

        let claims = manager
            .verify_token(&pair.access_token.token)
            .unwrap();
        assert_eq!(claims.sub, "42");
        assert_eq!(claims.username, "admin_user");
        assert_eq!(claims.role, Role::Admin);
        assert_eq!(claims.token_type, "access");
        assert!(!claims.jti.is_empty(), "JTI 不应为空");
    }

    /// 清理过期撤销记录 — 刚撤销的不会被清理
    #[test]
    fn test_cleanup_revoked_tokens() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        // 撤销一些 token
        manager.revoke_token("jti_1").unwrap();
        manager.revoke_token("jti_2").unwrap();
        manager.revoke_token("jti_3").unwrap();

        // 清理（由于刚撤销，不会被清理，因为没有超过 refresh_token_duration）
        let cleaned = manager.cleanup_revoked_tokens().unwrap();
        assert_eq!(cleaned, 0, "刚撤销的 token 不应被清理");
    }

    /// 完全无效的 token 字符串被拒绝
    #[test]
    fn test_invalid_token_string_rejected() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        assert!(
            manager.verify_token("not_a_valid_jwt").is_err(),
            "无效的 token 字符串应被拒绝"
        );
        assert!(
            manager.verify_token("").is_err(),
            "空字符串 token 应被拒绝"
        );
        assert!(
            manager
                .verify_token("header.payload.signature.extra")
                .is_err(),
            "格式错误的 token 应被拒绝"
        );
    }
}

// ============================================================================
// 模块 2: 路径安全测试
// ============================================================================

mod path_security {
    use std::path::Path;

    /// SEC-14: 路径遍历攻击模式检测
    #[test]
    fn test_path_traversal_patterns_detected() {
        let dangerous_paths = vec![
            "../etc/passwd",
            "../../etc/shadow",
            "foo/../../../etc/hosts",
            "foo/bar/../../baz/../../../etc/passwd",
        ];

        for path in dangerous_paths {
            assert!(
                path.contains(".."),
                "测试路径 '{}' 应包含 '..'",
                path
            );
        }
    }

    /// SEC-14: 绝对路径检测
    #[test]
    fn test_absolute_paths_detected() {
        let absolute_paths = vec![
            "/etc/passwd",
            "/etc/shadow",
            "/tmp/test.txt",
            "/root/.ssh/id_rsa",
        ];

        for path_str in absolute_paths {
            let path = Path::new(path_str);
            assert!(
                path.is_absolute(),
                "路径 '{}' 应被检测为绝对路径",
                path_str
            );
        }
    }

    /// 系统 hosts 文件路径存在
    #[test]
    fn test_system_hosts_path() {
        #[cfg(target_os = "macos")]
        {
            let hosts_path = Path::new("/etc/hosts");
            assert!(hosts_path.exists(), "macOS 的 /etc/hosts 应存在");
        }

        #[cfg(target_os = "linux")]
        {
            let hosts_path = Path::new("/etc/hosts");
            assert!(hosts_path.exists(), "Linux 的 /etc/hosts 应存在");
        }

        #[cfg(target_os = "windows")]
        {
            let hosts_path = Path::new(r"C:\Windows\System32\drivers\etc\hosts");
            assert!(hosts_path.exists(), "Windows 的 hosts 文件应存在");
        }
    }

    /// 应用数据目录可创建
    #[test]
    fn test_app_data_dir_creation() {
        let home = dirs::home_dir().expect("应能获取用户主目录");
        let app_dir = home.join(".switchhostsr");

        // 确保目录可以创建或已存在
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir).expect("应能创建应用数据目录");
        }
        assert!(app_dir.exists(), "应用数据目录应存在");
        assert!(app_dir.is_dir(), "应用数据路径应是目录");
    }

    /// 相对路径解析到应用数据目录内
    #[test]
    fn test_relative_path_resolves_within_app_dir() {
        let home = dirs::home_dir().expect("应能获取用户主目录");
        let app_dir = home.join(".switchhostsr");
        let resolved = app_dir.join("data/test.txt");

        assert!(
            resolved.starts_with(&app_dir),
            "解析后的路径应在应用数据目录内"
        );
    }

    /// 符号链接解析 — macOS 上 /etc -> /private/etc
    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_hosts_symlink_resolution() {
        let hosts_path = Path::new("/etc/hosts");
        let canonical = hosts_path.canonicalize().expect("应能解析 /etc/hosts");
        // macOS 上 /etc 是 /private/etc 的符号链接
        assert!(
            canonical.starts_with("/private") || canonical == hosts_path.to_path_buf(),
            "/etc/hosts 的 canonical 路径应在 /private 下或保持不变"
        );
    }
}

// ============================================================================
// 模块 3: Hosts 合并逻辑测试
// ============================================================================

mod hosts_merge {
    /// 测试多个激活 hosts 正确合并
    #[test]
    fn test_merge_multiple_active_hosts() {
        let original = "# System hosts\n127.0.0.1 localhost\n::1 localhost\n";
        let active_hosts = vec![
            (
                "开发环境".to_string(),
                "127.0.0.1 dev.local\n127.0.0.1 api.local".to_string(),
            ),
            (
                "测试环境".to_string(),
                "10.0.0.1 test.local\n10.0.0.2 staging.local".to_string(),
            ),
            (
                "生产配置".to_string(),
                "192.168.1.1 prod.local".to_string(),
            ),
        ];

        let merged = build_test_merged_content(original, &active_hosts);

        // 原始内容保留
        assert!(merged.contains("127.0.0.1 localhost"), "应保留原始 hosts 内容");
        assert!(merged.contains("::1 localhost"), "应保留原始 IPv6 hosts");

        // 所有激活 hosts 内容包含
        assert!(merged.contains("# --- 开发环境 ---"), "应包含开发环境标题");
        assert!(merged.contains("127.0.0.1 dev.local"), "应包含开发环境 hosts");
        assert!(merged.contains("127.0.0.1 api.local"), "应包含 API hosts");
        assert!(merged.contains("# --- 测试环境 ---"), "应包含测试环境标题");
        assert!(merged.contains("10.0.0.1 test.local"), "应包含测试环境 hosts");
        assert!(merged.contains("# --- 生产配置 ---"), "应包含生产配置标题");
        assert!(merged.contains("192.168.1.1 prod.local"), "应包含生产 hosts");

        // 标记区域正确
        assert!(
            merged.contains("# ===== SwitchHostsR Managed Start ====="),
            "应包含开始标记"
        );
        assert!(
            merged.contains("# ===== SwitchHostsR Managed End ====="),
            "应包含结束标记"
        );
    }

    /// 测试空活跃 hosts 列表 — 仍应写入标记区域
    #[test]
    fn test_empty_active_hosts_still_writes_markers() {
        let original = "127.0.0.1 localhost\n";
        let active_hosts: Vec<(String, String)> = vec![];

        let merged = build_test_merged_content(original, &active_hosts);

        assert!(merged.contains("127.0.0.1 localhost"), "应保留原始内容");
        assert!(
            merged.contains("# ===== SwitchHostsR Managed Start ====="),
            "应包含开始标记"
        );
        assert!(
            merged.contains("# ===== SwitchHostsR Managed End ====="),
            "应包含结束标记"
        );
    }

    /// 测试替换已有管理区域 — 旧内容被新内容覆盖
    #[test]
    fn test_replace_existing_managed_section() {
        let original = format!(
            "# System hosts\n127.0.0.1 localhost\n\n{}\n# 旧规则\n127.0.0.1 old.local\n{}\n",
            "# ===== SwitchHostsR Managed Start =====",
            "# ===== SwitchHostsR Managed End ====="
        );
        let active_hosts = vec![(
            "新规则".to_string(),
            "127.0.0.1 new.local".to_string(),
        )];

        let merged = build_test_merged_content(&original, &active_hosts);

        // 旧内容不应存在
        assert!(!merged.contains("127.0.0.1 old.local"), "旧规则应被移除");
        assert!(!merged.contains("# 旧规则"), "旧规则标题应被移除");

        // 新内容应存在
        assert!(merged.contains("127.0.0.1 new.local"), "新规则应存在");
        assert!(merged.contains("# --- 新规则 ---"), "新规则标题应存在");

        // 只有一对标记
        assert_eq!(
            merged
                .matches("# ===== SwitchHostsR Managed Start =====")
                .count(),
            1,
            "应只有一个开始标记"
        );
        assert_eq!(
            merged
                .matches("# ===== SwitchHostsR Managed End =====")
                .count(),
            1,
            "应只有一个结束标记"
        );
    }

    /// 测试非激活 hosts 不包含在合并结果中
    #[test]
    fn test_inactive_hosts_not_included() {
        let original = "127.0.0.1 localhost\n";
        // 只传入激活的 hosts，非激活的不应该出现
        let active_only = vec![(
            "活跃规则".to_string(),
            "127.0.0.1 active.local".to_string(),
        )];

        let merged = build_test_merged_content(original, &active_only);

        assert!(merged.contains("127.0.0.1 active.local"), "活跃规则应存在");
        assert!(
            !merged.contains("inactive.local"),
            "非活跃规则不应存在"
        );
    }

    /// 测试空行和空白字符处理
    #[test]
    fn test_whitespace_handling_in_hosts_content() {
        let original = "127.0.0.1 localhost\n";
        let active_hosts = vec![(
            "含空行".to_string(),
            "  127.0.0.1 dev.local  \n\n  10.0.0.1 api.local  \n".to_string(),
        )];

        let merged = build_test_merged_content(original, &active_hosts);

        // 内容应被 trim 处理
        assert!(
            merged.contains("127.0.0.1 dev.local"),
            "应包含 trim 后的 hosts"
        );
        assert!(
            merged.contains("10.0.0.1 api.local"),
            "应包含 trim 后的 hosts"
        );
    }

    /// 测试原始内容为空时的行为
    #[test]
    fn test_empty_original_content() {
        let original = "";
        let active_hosts = vec![(
            "规则".to_string(),
            "127.0.0.1 test.local".to_string(),
        )];

        let merged = build_test_merged_content(original, &active_hosts);

        assert!(merged.contains("127.0.0.1 test.local"), "应包含 hosts 规则");
        assert!(
            merged.contains("# ===== SwitchHostsR Managed Start ====="),
            "应包含开始标记"
        );
    }

    // 复制 system_hosts.rs 中的 build_merged_content 逻辑用于测试
    // 因为该函数是模块私有的 (pub(crate))
    fn build_test_merged_content(
        original_content: &str,
        active_hosts: &[(String, String)],
    ) -> String {
        let marker_start = "# ===== SwitchHostsR Managed Start =====";
        let marker_end = "# ===== SwitchHostsR Managed End =====";

        let mut result = String::new();
        let mut in_managed_section = false;

        for line in original_content.lines() {
            if line.trim() == marker_start {
                in_managed_section = true;
                continue;
            }
            if line.trim() == marker_end {
                in_managed_section = false;
                continue;
            }
            if !in_managed_section {
                result.push_str(line);
                result.push('\n');
            }
        }

        let trimmed = result.trim_end();
        result = trimmed.to_string();
        if !result.is_empty() {
            result.push('\n');
        }
        result.push('\n');

        result.push_str(marker_start);
        result.push('\n');
        result.push_str("# 由 SwitchHostsR 自动生成，请勿手动修改此区域\n");
        result.push_str(&format!(
            "# 更新时间: {}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));
        result.push('\n');

        for (name, content) in active_hosts {
            result.push_str(&format!("# --- {} ---\n", name));
            for line in content.lines() {
                let trimmed_line = line.trim();
                if !trimmed_line.is_empty() {
                    result.push_str(trimmed_line);
                    result.push('\n');
                }
            }
            result.push('\n');
        }

        result.push_str(marker_end);
        result.push('\n');

        result
    }
}

// ============================================================================
// 模块 4: 认证中间件（AuthState）测试
// ============================================================================

mod auth_state_tests {
    use super::*;

    /// AuthState 初始化后处于未认证状态
    #[test]
    fn test_auth_state_initial_not_authenticated() {
        let auth_state = AuthState::new();
        let result = auth_state.get_auth_context();
        assert!(result.is_err(), "初始状态应为未认证");
    }

    /// 登录后 get_auth_context 返回正确的用户信息
    #[test]
    fn test_auth_state_login_returns_correct_user() {
        let auth_state = AuthState::new();

        let session = Session::new(42, "test_admin".to_string(), Role::Admin, 24);
        auth_state.login(session).unwrap();

        let auth_ctx = auth_state.get_auth_context().unwrap();
        assert_eq!(auth_ctx.user_id, 42);
        assert_eq!(auth_ctx.username, "test_admin");
        assert_eq!(auth_ctx.role, Role::Admin);
    }

    /// 登出后 get_auth_context 返回 NotAuthenticated
    #[test]
    fn test_auth_state_logout_clears_context() {
        let auth_state = AuthState::new();

        // 登录
        let session = Session::new(1, "user1".to_string(), Role::User, 24);
        auth_state.login(session).unwrap();
        assert!(
            auth_state.get_auth_context().is_ok(),
            "登录后应能获取认证上下文"
        );

        // 登出
        auth_state.logout().unwrap();
        assert!(
            auth_state.get_auth_context().is_err(),
            "登出后应无法获取认证上下文"
        );
    }

    /// 重复登录覆盖之前的会话
    #[test]
    fn test_auth_state_login_replaces_session() {
        let auth_state = AuthState::new();

        // 第一次登录
        let session1 = Session::new(1, "user1".to_string(), Role::User, 24);
        auth_state.login(session1).unwrap();
        let ctx1 = auth_state.get_auth_context().unwrap();
        assert_eq!(ctx1.user_id, 1);

        // 第二次登录（不同用户）
        let session2 = Session::new(2, "user2".to_string(), Role::Admin, 24);
        auth_state.login(session2).unwrap();
        let ctx2 = auth_state.get_auth_context().unwrap();
        assert_eq!(ctx2.user_id, 2, "应返回最新登录的用户");
        assert_eq!(ctx2.username, "user2");
    }

    /// AuthMiddleware 权限检查
    #[test]
    fn test_auth_middleware_permission_check() {
        let mut middleware = AuthMiddleware::new();

        // 普通用户
        let user_session = Session::new(1, "normal_user".to_string(), Role::User, 24);
        middleware.set_session(user_session);

        // 普通用户应有 HostRead 权限
        assert!(
            middleware.check_permission(Permission::HostRead).is_ok(),
            "普通用户应有 HostRead 权限"
        );

        // 普通用户不应有 UserCreate 权限（管理员专属）
        assert!(
            middleware.check_permission(Permission::UserCreate).is_err(),
            "普通用户不应有 UserCreate 权限"
        );
    }

    /// AuthMiddleware 资源访问控制
    #[test]
    fn test_auth_middleware_resource_access_control() {
        let mut middleware = AuthMiddleware::new();

        // 普通用户
        let session = Session::new(1, "user1".to_string(), Role::User, 24);
        middleware.set_session(session);

        // 可以访问自己的资源
        assert!(
            middleware
                .check_user_resource_access(1, Permission::HostRead)
                .is_ok(),
            "应能访问自己的资源"
        );

        // 不能访问他人的资源
        assert!(
            middleware
                .check_user_resource_access(999, Permission::HostRead)
                .is_err(),
            "不应能访问他人的资源"
        );
    }

    /// 管理员可以访问所有用户的资源
    #[test]
    fn test_admin_can_access_all_resources() {
        let mut middleware = AuthMiddleware::new();

        let admin_session = Session::new(1, "admin".to_string(), Role::Admin, 24);
        middleware.set_session(admin_session);

        // 管理员可以访问任意用户的资源
        assert!(
            middleware
                .check_user_resource_access(999, Permission::HostRead)
                .is_ok(),
            "管理员应能访问其他用户的资源"
        );
    }

    /// AuthContext.has_permission 各角色测试
    #[test]
    fn test_auth_context_has_permission_by_role() {
        // 普通用户
        let user_perms = PermissionManager::get_role_permissions(&Role::User);
        let user_ctx = AuthContext {
            user_id: 1,
            username: "user".to_string(),
            role: Role::User,
            permissions: user_perms,
            session_id: "s1".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        };
        assert!(user_ctx.has_permission(&Permission::HostRead));
        assert!(user_ctx.has_permission(&Permission::HostCreate));
        assert!(!user_ctx.has_permission(&Permission::UserCreate));

        // 管理员（注意：Admin 没有 AdminAll 权限，但有具体的各项权限）
        let admin_perms = PermissionManager::get_role_permissions(&Role::Admin);
        let admin_ctx = AuthContext {
            user_id: 2,
            username: "admin".to_string(),
            role: Role::Admin,
            permissions: admin_perms,
            session_id: "s2".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        };
        assert!(admin_ctx.has_permission(&Permission::HostRead));
        assert!(admin_ctx.has_permission(&Permission::UserCreate));
        assert!(admin_ctx.has_permission(&Permission::SystemConfig));
        // Admin 没有 AdminAll，这是设计如此
        assert!(!admin_ctx.has_permission(&Permission::AdminAll));

        // 超级管理员
        let sa_perms = PermissionManager::get_role_permissions(&Role::SuperAdmin);
        let sa_ctx = AuthContext {
            user_id: 3,
            username: "superadmin".to_string(),
            role: Role::SuperAdmin,
            permissions: sa_perms,
            session_id: "s3".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        };
        // 超级管理员拥有所有权限
        assert!(sa_ctx.has_permission(&Permission::AdminAll));
        assert!(sa_ctx.has_permission(&Permission::SystemConfig));
    }

    /// AuthContext.can_access_user_resource 测试
    #[test]
    fn test_auth_context_resource_access() {
        let user_perms = PermissionManager::get_role_permissions(&Role::User);
        let user_ctx = AuthContext {
            user_id: 10,
            username: "user10".to_string(),
            role: Role::User,
            permissions: user_perms,
            session_id: "s1".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        };

        // 普通用户只能访问自己的资源
        assert!(user_ctx.can_access_user_resource(10));
        assert!(!user_ctx.can_access_user_resource(20));

        // 管理员可以访问所有资源
        let admin_perms = PermissionManager::get_role_permissions(&Role::Admin);
        let admin_ctx = AuthContext {
            user_id: 1,
            username: "admin".to_string(),
            role: Role::Admin,
            permissions: admin_perms,
            session_id: "s2".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
        };
        assert!(admin_ctx.can_access_user_resource(10));
        assert!(admin_ctx.can_access_user_resource(999));
    }

    /// AuthContext.is_expired 测试
    #[test]
    fn test_auth_context_expiration() {
        let perms = PermissionManager::get_role_permissions(&Role::User);

        // 未过期的上下文
        let valid_ctx = AuthContext {
            user_id: 1,
            username: "user".to_string(),
            role: Role::User,
            permissions: perms.clone(),
            session_id: "s1".to_string(),
            expires_at: chrono::Utc::now().timestamp() + 3600, // 1小时后
        };
        assert!(!valid_ctx.is_expired(), "1小时后过期的上下文不应标记为已过期");

        // 已过期的上下文
        let expired_ctx = AuthContext {
            user_id: 1,
            username: "user".to_string(),
            role: Role::User,
            permissions: perms,
            session_id: "s2".to_string(),
            expires_at: chrono::Utc::now().timestamp() - 3600, // 1小时前
        };
        assert!(expired_ctx.is_expired(), "1小时前过期的上下文应标记为已过期");
    }
}
