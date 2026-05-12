//! 系统服务模块
//!
//! 提供系统信息相关的服务功能

use anyhow::Result;
use diesel::prelude::*;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::db::DbPool;
use super::BaseService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i32,
    pub file_name: String,
    #[serde(default)]
    pub app_version: String,
    #[serde(default)]
    pub schema_version: i32,
}

/// 系统信息结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: String,
    pub hostname: String,
    pub uptime: u64,
    pub memory_total: u64,
    pub memory_available: u64,
    pub cpu_count: u32,
}

/// 系统服务
#[derive(Debug, Clone)]
pub struct SystemService {
    pool: DbPool,
}

impl SystemService {
    /// 创建新的系统服务实例
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 获取系统信息
    pub async fn get_system_info(&self) -> Result<SystemInfo> {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        
        // 获取系统运行时间（简化实现）
        let uptime = 0; // 实际实现需要调用系统API
        
        // 获取内存信息（简化实现）
        let memory_total = 0;
        let memory_available = 0;
        
        // 获取CPU核心数
        let cpu_count = num_cpus::get() as u32;

        Ok(SystemInfo {
            os,
            arch,
            version,
            hostname,
            uptime,
            memory_total,
            memory_available,
            cpu_count,
        })
    }

    /// 检查系统健康状态
    pub async fn check_health(&self) -> Result<bool> {
        // 简单的健康检查
        // 可以检查数据库连接、磁盘空间等
        Ok(true)
    }

    /// 获取系统配置
    pub async fn get_system_config(&self, _auth: &crate::auth::AuthContext) -> Result<std::collections::HashMap<String, String>> {
        // 简化实现，返回空配置
        Ok(std::collections::HashMap::new())
    }

    /// 更新系统配置
    pub async fn update_system_config(&self, _auth: &crate::auth::AuthContext, _key: &str, _value: &str) -> Result<()> {
        // 简化实现，直接返回成功
        Ok(())
    }

    /// 清理日志
    pub async fn clean_logs(&self, _auth: &crate::auth::AuthContext, _days: i32) -> Result<i32> {
        // 简化实现，返回清理的日志数量
        Ok(0)
    }

    fn get_backups_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;
        let dir = home.join(".switchhostsr").join("backups");
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn get_backups_index_path() -> Result<PathBuf> {
        Ok(Self::get_backups_dir()?.join("backups.json"))
    }

    fn read_backups_index() -> Result<Vec<BackupInfo>> {
        let path = Self::get_backups_index_path()?;
        if !path.exists() {
            return Ok(vec![]);
        }
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str::<Vec<BackupInfo>>(&content).map_err(|e| e.into())
    }

    fn write_backups_index(list: &[BackupInfo]) -> Result<()> {
        let path = Self::get_backups_index_path()?;
        let content = serde_json::to_string_pretty(list)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub async fn create_backup(
        &self,
        _auth: &crate::auth::AuthContext,
        name: String,
        description: Option<String>,
    ) -> Result<BackupInfo> {
        let backups_dir = Self::get_backups_dir()?;
        let mut list = Self::read_backups_index()?;

        let created_at = chrono::Utc::now().timestamp() as i32;
        let mut id = created_at;
        while list.iter().any(|b| b.id == id) {
            id = id.saturating_add(1);
        }

        let file_name = format!("backup_{}_{}.db", id, uuid::Uuid::new_v4().simple());
        let backup_path = backups_dir.join(&file_name);
        let backup_path_str = backup_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("备份路径无效"))?
            .to_string();

        let mut conn = self.pool.get()?;
        diesel::sql_query("VACUUM INTO ?")
            .bind::<Text, _>(backup_path_str)
            .execute(&mut conn)?;

        let info = BackupInfo {
            id,
            name,
            description,
            created_at,
            file_name,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            schema_version: 1,
        };
        list.push(info.clone());
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.id.cmp(&a.id)));
        Self::write_backups_index(&list)?;

        Ok(info)
    }

    pub async fn get_backups(&self, _auth: &crate::auth::AuthContext) -> Result<Vec<BackupInfo>> {
        let mut list = Self::read_backups_index()?;
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.id.cmp(&a.id)));
        Ok(list)
    }

    pub async fn get_backup_dir(&self, _auth: &crate::auth::AuthContext) -> Result<String> {
        Ok(Self::get_backups_dir()?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("备份路径无效"))?
            .to_string())
    }

    pub async fn restore_backup(
        &self,
        _auth: &crate::auth::AuthContext,
        backup_id: i32,
    ) -> Result<()> {
        let backups_dir = Self::get_backups_dir()?;
        let list = Self::read_backups_index()?;
        let backup = list
            .into_iter()
            .find(|b| b.id == backup_id)
            .ok_or_else(|| anyhow::anyhow!("备份不存在"))?;

        let backup_path = backups_dir.join(backup.file_name);
        let backup_path_str = backup_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("备份路径无效"))?
            .to_string();

        let mut conn = self.pool.get()?;

        #[derive(QueryableByName)]
        struct PragmaColumnName {
            #[diesel(sql_type = Text)]
            name: String,
        }

        diesel::sql_query("PRAGMA foreign_keys=OFF;").execute(&mut conn)?;

        if let Err(e) = diesel::sql_query("ATTACH DATABASE ? AS backup;")
            .bind::<Text, _>(backup_path_str)
            .execute(&mut conn)
        {
            let _ = diesel::sql_query("PRAGMA foreign_keys=ON;").execute(&mut conn);
            return Err(e.into());
        }

        if let Err(e) = diesel::sql_query("BEGIN IMMEDIATE;").execute(&mut conn) {
            let _ = diesel::sql_query("DETACH DATABASE backup;").execute(&mut conn);
            let _ = diesel::sql_query("PRAGMA foreign_keys=ON;").execute(&mut conn);
            return Err(e.into());
        }

        let result: Result<()> = (|| {
            diesel::sql_query("DELETE FROM host_group_relations;").execute(&mut conn)?;
            diesel::sql_query("DELETE FROM host_groups;").execute(&mut conn)?;
            diesel::sql_query("DELETE FROM hosts;").execute(&mut conn)?;
            diesel::sql_query("DELETE FROM logs;").execute(&mut conn)?;
            diesel::sql_query("DELETE FROM users;").execute(&mut conn)?;

            diesel::sql_query(
                "INSERT INTO users (id, username, password, email, avatar, is_admin, created_at, updated_at)
                 SELECT id, username, password, email, avatar, is_admin, created_at, updated_at
                 FROM backup.users;",
            )
            .execute(&mut conn)?;

            diesel::sql_query(
                "INSERT INTO hosts (id, user_id, name, description, content, is_active, is_system, created_at, updated_at)
                 SELECT id, user_id, name, description, content, is_active, is_system, created_at, updated_at
                 FROM backup.hosts;",
            )
            .execute(&mut conn)?;

            diesel::sql_query(
                "INSERT INTO logs (id, user_id, action, target_type, target_id, details, created_at)
                 SELECT id, user_id, action, target_type, target_id, details, created_at
                 FROM backup.logs;",
            )
            .execute(&mut conn)?;

            let host_groups_columns = diesel::sql_query("SELECT name FROM backup.pragma_table_info('host_groups');")
                .load::<PragmaColumnName>(&mut conn)?
                .into_iter()
                .map(|c| c.name)
                .collect::<Vec<String>>();
            let has_is_active = host_groups_columns.iter().any(|c| c == "is_active");

            if has_is_active {
                diesel::sql_query(
                    "INSERT INTO host_groups (id, user_id, name, description, is_active, created_at, updated_at)
                     SELECT id, user_id, name, description, is_active, created_at, updated_at
                     FROM backup.host_groups;",
                )
                .execute(&mut conn)?;
            } else {
                diesel::sql_query(
                    "INSERT INTO host_groups (id, user_id, name, description, is_active, created_at, updated_at)
                     SELECT id, user_id, name, description, 1, created_at, updated_at
                     FROM backup.host_groups;",
                )
                .execute(&mut conn)?;
            }

            diesel::sql_query(
                "INSERT INTO host_group_relations (id, group_id, host_id, created_at)
                 SELECT id, group_id, host_id, created_at
                 FROM backup.host_group_relations;",
            )
            .execute(&mut conn)?;

            let _ = diesel::sql_query("DELETE FROM sqlite_sequence;").execute(&mut conn);
            let _ = diesel::sql_query("INSERT INTO sqlite_sequence SELECT * FROM backup.sqlite_sequence;")
                .execute(&mut conn);

            diesel::sql_query("COMMIT;").execute(&mut conn)?;
            Ok(())
        })();

        if result.is_err() {
            let _ = diesel::sql_query("ROLLBACK;").execute(&mut conn);
        }

        let _ = diesel::sql_query("DETACH DATABASE backup;").execute(&mut conn);
        let _ = diesel::sql_query("PRAGMA foreign_keys=ON;").execute(&mut conn);

        result
    }

    /// 恢复备份
    /// 重启服务
    pub async fn restart_service(&self, _auth: &crate::auth::AuthContext) -> Result<()> {
        // 简化实现，直接返回成功
        Ok(())
    }
}

impl BaseService for SystemService {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}
