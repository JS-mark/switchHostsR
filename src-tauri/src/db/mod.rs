//! 数据库模块
//!
//! 该模块包含所有与数据库相关的操作，包括连接管理、模型定义和数据处理。

pub mod handlers;
pub mod models;
pub mod schema;

use anyhow::{Context, Result};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;
use std::{env, error::Error};

/// 数据库连接池类型
pub type DbPool = Arc<Pool<ConnectionManager<SqliteConnection>>>;

/// 创建数据库连接池
///
/// # Errors
///
/// 如果无法创建连接池，将返回错误。可能的错误原因包括：
/// - 环境变量 DATABASE_URL 未设置
/// - 数据库连接失败
/// - 连接池配置错误
pub fn create_pool() -> Result<DbPool, Box<dyn Error>> {
    // 尝试加载 .env 文件，但不强制要求
    dotenv::dotenv().ok();

    // 获取数据库 URL
    let database_url = env::var("DATABASE_URL").map_err(|_| "环境变量 DATABASE_URL 未设置")?;

    // 创建连接管理器
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // 构建连接池
    let pool = Pool::builder()
        .build(manager)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(Arc::new(pool))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::TempDir;

    /// 创建测试数据库连接池
    pub fn create_test_db() -> (DbPool, TempDir) {
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        let db_path = temp_dir.path().join("test.db");
        let db_url = format!("sqlite:{}", db_path.to_str().unwrap());

        // 设置环境变量
        env::set_var("DATABASE_URL", &db_url);

        // 创建连接池
        let pool = create_pool().expect("无法创建测试数据库连接池");

        // 运行迁移
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

        let mut conn = pool.get().expect("无法获取数据库连接");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("无法运行迁移");

        (pool, temp_dir)
    }
}
