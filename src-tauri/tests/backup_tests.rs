use app_lib::{
    auth::{permissions::PermissionManager, AuthContext, Role},
    db::{create_pool, services::system_service::SystemService},
};
use diesel::sql_types::{BigInt, Integer};
use diesel::RunQueryDsl;
use diesel::QueryableByName;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tempfile::TempDir;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn create_auth_context() -> AuthContext {
    let role = Role::Admin;
    let permissions = PermissionManager::get_role_permissions(&role);
    AuthContext {
        user_id: 1,
        username: "test_admin".to_string(),
        role,
        permissions,
        session_id: "test_session".to_string(),
        expires_at: chrono::Utc::now().timestamp() + 3600,
    }
}

fn setup_file_db() -> (app_lib::db::DbPool, TempDir) {
    let temp_dir = TempDir::new().expect("无法创建临时目录");
    let db_path = temp_dir.path().join("test.db");
    std::env::set_var("DATABASE_URL", db_path.to_string_lossy().to_string());
    std::env::set_var("HOME", temp_dir.path().to_string_lossy().to_string());
    std::env::set_var("USERPROFILE", temp_dir.path().to_string_lossy().to_string());

    let pool = create_pool().expect("无法创建数据库连接池");
    {
        let mut conn = pool.get().expect("无法获取数据库连接");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("无法运行迁移");
    }
    (pool, temp_dir)
}

#[derive(QueryableByName)]
struct CountRow {
    #[diesel(sql_type = BigInt)]
    cnt: i64,
}

#[derive(QueryableByName)]
struct IntRow {
    #[diesel(sql_type = Integer)]
    value: i32,
}

#[tokio::test]
async fn test_create_and_restore_backup() {
    let (pool, _temp_dir) = setup_file_db();
    let system_service = SystemService::new(pool.clone());
    let auth = create_auth_context();

    {
        let mut conn = pool.get().unwrap();
        diesel::sql_query(
            "INSERT INTO users (id, username, password, email, avatar, is_admin, created_at, updated_at)
             VALUES (1, 'u1', 'p', 'u1@example.com', NULL, 1, 1, 1);",
        )
        .execute(&mut conn)
        .unwrap();
        diesel::sql_query(
            "INSERT INTO hosts (id, user_id, name, description, content, is_active, is_system, created_at, updated_at)
             VALUES (1, 1, 'h1', NULL, '127.0.0.1 a.test', 1, 0, 1, 1);",
        )
        .execute(&mut conn)
        .unwrap();
    }

    let backup = system_service
        .create_backup(&auth, "b1".to_string(), None)
        .await
        .unwrap();
    let backups = system_service.get_backups(&auth).await.unwrap();
    assert!(!backups.is_empty());
    assert_eq!(backups[0].id, backup.id);

    {
        let mut conn = pool.get().unwrap();
        diesel::sql_query("DELETE FROM hosts;").execute(&mut conn).unwrap();
        let count = diesel::sql_query("SELECT COUNT(*) as cnt FROM hosts;")
            .get_result::<CountRow>(&mut conn)
            .unwrap();
        assert_eq!(count.cnt, 0);
    }

    system_service.restore_backup(&auth, backup.id).await.unwrap();

    {
        let mut conn = pool.get().unwrap();
        let count = diesel::sql_query("SELECT COUNT(*) as cnt FROM hosts;")
            .get_result::<CountRow>(&mut conn)
            .unwrap();
        assert_eq!(count.cnt, 1);
    }
}

#[tokio::test]
async fn test_restore_legacy_backup_without_host_group_is_active() {
    let (pool, temp_dir) = setup_file_db();
    let system_service = SystemService::new(pool.clone());
    let auth = create_auth_context();

    let backups_dir = temp_dir.path().join(".switchhostsr").join("backups");
    std::fs::create_dir_all(&backups_dir).unwrap();
    let legacy_backup_name = "legacy.db";
    let legacy_backup_path = backups_dir.join(legacy_backup_name);

    {
        use diesel::sqlite::SqliteConnection;
        use diesel::Connection;
        let mut conn = SqliteConnection::establish(&legacy_backup_path.to_string_lossy()).unwrap();

        diesel::sql_query(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                email TEXT,
                avatar TEXT,
                is_admin INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
        )
        .execute(&mut conn)
        .unwrap();

        diesel::sql_query(
            "CREATE TABLE hosts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                content TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 0,
                is_system INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
        )
        .execute(&mut conn)
        .unwrap();

        diesel::sql_query(
            "CREATE TABLE logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                action TEXT NOT NULL,
                target_type TEXT NOT NULL,
                target_id INTEGER,
                details TEXT,
                created_at INTEGER NOT NULL
            );",
        )
        .execute(&mut conn)
        .unwrap();

        diesel::sql_query(
            "CREATE TABLE host_groups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
        )
        .execute(&mut conn)
        .unwrap();

        diesel::sql_query(
            "CREATE TABLE host_group_relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                group_id INTEGER NOT NULL,
                host_id INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            );",
        )
        .execute(&mut conn)
        .unwrap();

        diesel::sql_query(
            "INSERT INTO users (id, username, password, email, avatar, is_admin, created_at, updated_at)
             VALUES (1, 'u1', 'p', 'u1@example.com', NULL, 1, 1, 1);",
        )
        .execute(&mut conn)
        .unwrap();
        diesel::sql_query(
            "INSERT INTO host_groups (id, user_id, name, description, created_at, updated_at)
             VALUES (1, 1, 'g1', NULL, 1, 1);",
        )
        .execute(&mut conn)
        .unwrap();
    }

    let index = serde_json::json!([
        {
            "id": 1,
            "name": "legacy",
            "description": null,
            "created_at": 1,
            "file_name": legacy_backup_name
        }
    ]);
    std::fs::write(backups_dir.join("backups.json"), index.to_string()).unwrap();

    system_service.restore_backup(&auth, 1).await.unwrap();

    {
        let mut conn = pool.get().unwrap();
        let row = diesel::sql_query("SELECT is_active as value FROM host_groups WHERE id = 1;")
            .get_result::<IntRow>(&mut conn)
            .unwrap();
        assert_eq!(row.value, 1);
    }
}
