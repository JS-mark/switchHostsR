//! 权限管理模块
//!
//! 定义角色权限映射和权限检查逻辑

use super::{Permission, Role};
use std::collections::HashSet;

/// 权限管理器
pub struct PermissionManager;

impl PermissionManager {
    /// 获取角色对应的权限集合
    pub fn get_role_permissions(role: &Role) -> HashSet<Permission> {
        match role {
            Role::SuperAdmin => Self::get_super_admin_permissions(),
            Role::Admin => Self::get_admin_permissions(),
            Role::User => Self::get_user_permissions(),
            Role::Guest => Self::get_guest_permissions(),
        }
    }

    /// 超级管理员权限
    fn get_super_admin_permissions() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::AdminAll);
        permissions
    }

    /// 管理员权限
    fn get_admin_permissions() -> HashSet<Permission> {
        let mut permissions = HashSet::new();

        // 用户管理权限
        permissions.insert(Permission::UserCreate);
        permissions.insert(Permission::UserRead);
        permissions.insert(Permission::UserUpdate);
        permissions.insert(Permission::UserDelete);
        permissions.insert(Permission::UserList);

        // 主机管理权限
        permissions.insert(Permission::HostCreate);
        permissions.insert(Permission::HostRead);
        permissions.insert(Permission::HostUpdate);
        permissions.insert(Permission::HostDelete);
        permissions.insert(Permission::HostList);
        permissions.insert(Permission::HostActivate);

        // 主机组管理权限
        permissions.insert(Permission::HostGroupCreate);
        permissions.insert(Permission::HostGroupRead);
        permissions.insert(Permission::HostGroupUpdate);
        permissions.insert(Permission::HostGroupDelete);
        permissions.insert(Permission::HostGroupList);

        // 日志管理权限
        permissions.insert(Permission::LogRead);
        permissions.insert(Permission::LogList);
        permissions.insert(Permission::LogDelete);

        // 系统管理权限
        permissions.insert(Permission::SystemConfig);
        permissions.insert(Permission::SystemInfo);

        permissions
    }

    /// 普通用户权限
    fn get_user_permissions() -> HashSet<Permission> {
        let mut permissions = HashSet::new();

        // 用户信息管理权限（仅限自己的信息）
        permissions.insert(Permission::UserRead);
        permissions.insert(Permission::UserUpdate);

        // 主机管理权限（仅限自己的主机）
        permissions.insert(Permission::HostCreate);
        permissions.insert(Permission::HostRead);
        permissions.insert(Permission::HostUpdate);
        permissions.insert(Permission::HostDelete);
        permissions.insert(Permission::HostList);
        permissions.insert(Permission::HostActivate);

        // 主机组管理权限（仅限自己的主机组）
        permissions.insert(Permission::HostGroupCreate);
        permissions.insert(Permission::HostGroupRead);
        permissions.insert(Permission::HostGroupUpdate);
        permissions.insert(Permission::HostGroupDelete);
        permissions.insert(Permission::HostGroupList);

        // 日志查看权限（仅限自己的日志）
        permissions.insert(Permission::LogRead);
        permissions.insert(Permission::LogList);

        // 系统信息查看权限
        permissions.insert(Permission::SystemInfo);

        permissions
    }

    /// 访客权限
    fn get_guest_permissions() -> HashSet<Permission> {
        let mut permissions = HashSet::new();

        // 只有基本的查看权限
        permissions.insert(Permission::SystemInfo);

        permissions
    }

    /// 检查权限是否需要用户资源访问控制
    pub fn requires_user_resource_check(permission: &Permission) -> bool {
        matches!(
            permission,
            Permission::UserRead
                | Permission::UserUpdate
                | Permission::HostCreate
                | Permission::HostRead
                | Permission::HostUpdate
                | Permission::HostDelete
                | Permission::HostList
                | Permission::HostActivate
                | Permission::HostGroupCreate
                | Permission::HostGroupRead
                | Permission::HostGroupUpdate
                | Permission::HostGroupDelete
                | Permission::HostGroupList
                | Permission::LogRead
                | Permission::LogList
        )
    }

    /// 检查权限是否为管理员专用
    pub fn is_admin_only_permission(permission: &Permission) -> bool {
        matches!(
            permission,
            Permission::UserCreate
                | Permission::UserDelete
                | Permission::UserList
                | Permission::LogDelete
                | Permission::SystemConfig
                | Permission::AdminAll
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_admin_permissions() {
        let permissions = PermissionManager::get_role_permissions(&Role::SuperAdmin);
        assert!(permissions.contains(&Permission::AdminAll));
    }

    #[test]
    fn test_admin_permissions() {
        let permissions = PermissionManager::get_role_permissions(&Role::Admin);
        assert!(permissions.contains(&Permission::UserCreate));
        assert!(permissions.contains(&Permission::HostCreate));
        assert!(permissions.contains(&Permission::SystemConfig));
        assert!(!permissions.contains(&Permission::AdminAll));
    }

    #[test]
    fn test_user_permissions() {
        let permissions = PermissionManager::get_role_permissions(&Role::User);
        assert!(permissions.contains(&Permission::HostCreate));
        assert!(permissions.contains(&Permission::UserRead));
        assert!(!permissions.contains(&Permission::UserCreate));
        assert!(!permissions.contains(&Permission::SystemConfig));
    }

    #[test]
    fn test_guest_permissions() {
        let permissions = PermissionManager::get_role_permissions(&Role::Guest);
        assert!(permissions.contains(&Permission::SystemInfo));
        assert!(!permissions.contains(&Permission::HostCreate));
        assert!(!permissions.contains(&Permission::UserRead));
    }

    #[test]
    fn test_permission_checks() {
        assert!(PermissionManager::requires_user_resource_check(
            &Permission::HostRead
        ));
        assert!(!PermissionManager::requires_user_resource_check(
            &Permission::SystemInfo
        ));

        assert!(PermissionManager::is_admin_only_permission(
            &Permission::UserCreate
        ));
        assert!(!PermissionManager::is_admin_only_permission(
            &Permission::HostCreate
        ));
    }
}
