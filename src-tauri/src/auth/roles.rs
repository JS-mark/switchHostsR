//! 角色管理模块
//!
//! 定义用户角色的层级关系和权限继承

use super::{Permission, Role};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub role: Role,
    pub name: String,
    pub description: String,
    pub level: u8,                   // 角色等级，数字越大权限越高
    pub inherits_from: Option<Role>, // 继承自哪个角色
}

/// 角色管理器
pub struct RoleManager;

impl RoleManager {
    /// 获取所有角色信息
    pub fn get_all_roles() -> Vec<RoleInfo> {
        vec![
            RoleInfo {
                role: Role::Guest,
                name: "访客".to_string(),
                description: "只能查看基本系统信息".to_string(),
                level: 0,
                inherits_from: None,
            },
            RoleInfo {
                role: Role::User,
                name: "普通用户".to_string(),
                description: "可以管理自己的主机和主机组".to_string(),
                level: 1,
                inherits_from: Some(Role::Guest),
            },
            RoleInfo {
                role: Role::Admin,
                name: "管理员".to_string(),
                description: "可以管理所有用户和系统配置".to_string(),
                level: 2,
                inherits_from: Some(Role::User),
            },
            RoleInfo {
                role: Role::SuperAdmin,
                name: "超级管理员".to_string(),
                description: "拥有系统的所有权限".to_string(),
                level: 3,
                inherits_from: Some(Role::Admin),
            },
        ]
    }

    /// 获取角色信息
    pub fn get_role_info(role: &Role) -> Option<RoleInfo> {
        Self::get_all_roles()
            .into_iter()
            .find(|info| info.role == *role)
    }

    /// 获取角色等级
    pub fn get_role_level(role: &Role) -> u8 {
        Self::get_role_info(role)
            .map(|info| info.level)
            .unwrap_or(0)
    }

    /// 检查角色是否有足够的权限等级
    pub fn has_sufficient_level(user_role: &Role, required_role: &Role) -> bool {
        Self::get_role_level(user_role) >= Self::get_role_level(required_role)
    }

    /// 检查是否可以管理目标角色
    pub fn can_manage_role(manager_role: &Role, target_role: &Role) -> bool {
        // 超级管理员可以管理所有角色
        if *manager_role == Role::SuperAdmin {
            return true;
        }

        // 管理员可以管理普通用户和访客
        if *manager_role == Role::Admin {
            return matches!(target_role, Role::User | Role::Guest);
        }

        // 其他角色不能管理任何角色
        false
    }

    /// 获取角色的所有权限（包括继承的权限）
    pub fn get_inherited_permissions(role: &Role) -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        let mut current_role = Some(role.clone());

        // 递归获取继承的权限
        while let Some(role) = current_role {
            let role_permissions =
                crate::auth::permissions::PermissionManager::get_role_permissions(&role);
            permissions.extend(role_permissions);

            // 获取父角色
            current_role = Self::get_role_info(&role).and_then(|info| info.inherits_from);
        }

        permissions
    }

    /// 检查角色升级是否合法
    pub fn is_valid_role_upgrade(from: &Role, to: &Role) -> bool {
        let from_level = Self::get_role_level(from);
        let to_level = Self::get_role_level(to);

        // 只能升级到更高等级的角色
        to_level > from_level
    }

    /// 检查角色降级是否合法
    pub fn is_valid_role_downgrade(from: &Role, to: &Role) -> bool {
        let from_level = Self::get_role_level(from);
        let to_level = Self::get_role_level(to);

        // 只能降级到更低等级的角色
        to_level < from_level
    }

    /// 获取可以分配的角色列表
    pub fn get_assignable_roles(manager_role: &Role) -> Vec<Role> {
        let all_roles = Self::get_all_roles();

        match manager_role {
            Role::SuperAdmin => {
                // 超级管理员可以分配所有角色
                all_roles.into_iter().map(|info| info.role).collect()
            }
            Role::Admin => {
                // 管理员可以分配普通用户和访客角色
                vec![Role::User, Role::Guest]
            }
            _ => {
                // 其他角色不能分配任何角色
                vec![]
            }
        }
    }

    /// 获取默认角色
    pub fn get_default_role() -> Role {
        Role::User
    }

    /// 检查是否为系统角色（不能被删除或修改）
    pub fn is_system_role(_role: &Role) -> bool {
        // 所有角色都是系统角色，不能被删除
        true
    }

    /// 获取角色的显示名称
    pub fn get_role_display_name(role: &Role) -> String {
        Self::get_role_info(role)
            .map(|info| info.name)
            .unwrap_or_else(|| "未知角色".to_string())
    }

    /// 获取角色的描述
    pub fn get_role_description(role: &Role) -> String {
        Self::get_role_info(role)
            .map(|info| info.description)
            .unwrap_or_else(|| "无描述".to_string())
    }

    /// 验证角色变更权限
    pub fn validate_role_change(
        operator_role: &Role,
        target_current_role: &Role,
        target_new_role: &Role,
    ) -> Result<(), String> {
        // 检查操作者是否有权限管理目标用户的当前角色
        if !Self::can_manage_role(operator_role, target_current_role) {
            return Err("没有权限管理该用户的当前角色".to_string());
        }

        // 检查操作者是否有权限分配新角色
        let assignable_roles = Self::get_assignable_roles(operator_role);
        if !assignable_roles.contains(target_new_role) {
            return Err("没有权限分配该角色".to_string());
        }

        // 防止用户将自己的角色降级为无法管理的角色
        if Self::get_role_level(target_new_role) > Self::get_role_level(operator_role) {
            return Err("不能分配比自己更高级别的角色".to_string());
        }

        Ok(())
    }
}

/// 角色比较特征
impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_level = RoleManager::get_role_level(self);
        let other_level = RoleManager::get_role_level(other);
        self_level.partial_cmp(&other_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_levels() {
        assert_eq!(RoleManager::get_role_level(&Role::Guest), 0);
        assert_eq!(RoleManager::get_role_level(&Role::User), 1);
        assert_eq!(RoleManager::get_role_level(&Role::Admin), 2);
        assert_eq!(RoleManager::get_role_level(&Role::SuperAdmin), 3);
    }

    #[test]
    fn test_role_management_permissions() {
        // 超级管理员可以管理所有角色
        assert!(RoleManager::can_manage_role(
            &Role::SuperAdmin,
            &Role::Admin
        ));
        assert!(RoleManager::can_manage_role(&Role::SuperAdmin, &Role::User));
        assert!(RoleManager::can_manage_role(
            &Role::SuperAdmin,
            &Role::Guest
        ));

        // 管理员可以管理普通用户和访客
        assert!(RoleManager::can_manage_role(&Role::Admin, &Role::User));
        assert!(RoleManager::can_manage_role(&Role::Admin, &Role::Guest));
        assert!(!RoleManager::can_manage_role(
            &Role::Admin,
            &Role::SuperAdmin
        ));

        // 普通用户不能管理任何角色
        assert!(!RoleManager::can_manage_role(&Role::User, &Role::Guest));
        assert!(!RoleManager::can_manage_role(&Role::User, &Role::Admin));
    }

    #[test]
    fn test_assignable_roles() {
        let super_admin_roles = RoleManager::get_assignable_roles(&Role::SuperAdmin);
        assert_eq!(super_admin_roles.len(), 4);

        let admin_roles = RoleManager::get_assignable_roles(&Role::Admin);
        assert_eq!(admin_roles.len(), 2);
        assert!(admin_roles.contains(&Role::User));
        assert!(admin_roles.contains(&Role::Guest));

        let user_roles = RoleManager::get_assignable_roles(&Role::User);
        assert!(user_roles.is_empty());
    }

    #[test]
    fn test_role_validation() {
        // 超级管理员可以将管理员降级为普通用户
        assert!(
            RoleManager::validate_role_change(&Role::SuperAdmin, &Role::Admin, &Role::User).is_ok()
        );

        // 管理员不能将普通用户升级为超级管理员
        assert!(
            RoleManager::validate_role_change(&Role::Admin, &Role::User, &Role::SuperAdmin)
                .is_err()
        );

        // 普通用户不能修改任何角色
        assert!(RoleManager::validate_role_change(&Role::User, &Role::Guest, &Role::User).is_err());
    }

    #[test]
    fn test_inherited_permissions() {
        let user_permissions = RoleManager::get_inherited_permissions(&Role::User);
        let guest_permissions = RoleManager::get_inherited_permissions(&Role::Guest);

        // 用户权限应该包含访客权限
        for permission in &guest_permissions {
            assert!(user_permissions.contains(permission));
        }

        // 用户权限应该比访客权限多
        assert!(user_permissions.len() > guest_permissions.len());
    }

    #[test]
    fn test_role_comparison() {
        assert!(Role::SuperAdmin > Role::Admin);
        assert!(Role::Admin > Role::User);
        assert!(Role::User > Role::Guest);
    }
}
