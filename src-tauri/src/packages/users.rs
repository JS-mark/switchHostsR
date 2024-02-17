use super::public_type::{List, ResultData};
use bcrypt::{hash, verify};
use rusqlite::{named_params, params, Connection, Error as RustsqliteError};
use serde::{Deserialize, Serialize};

pub struct Database {
    pub conn: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub status: i32,
    pub is_del: i32,
    pub user_level: i32,
    pub password: String,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResult {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub status: i32,
    pub is_del: i32,
    pub user_level: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    return hash(password, 12);
}

pub fn verify_password_hash(password: &str, password_hash: &str) -> bool {
    match verify(password, password_hash) {
        Ok(matches) => matches,
        Err(_) => false,
    }
}

impl Database {
    pub fn new(file_path: impl AsRef<std::path::Path>) -> Result<Database, RustsqliteError> {
        let conn = Connection::open(file_path)?;
        Database::init_database(&conn)?;
        Ok(Database { conn })
    }

    /**
     * 校验用户信息
     */
    pub fn verify_user(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<ResultData<UserResult>, RustsqliteError> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM users WHERE email = ? LIMIT 1")?;

        let mut rows = stmt.query(params![email])?;

        if let Some(row) = rows.next()? {
            let password_hash: String = row.get(5)?;

            if verify_password_hash(password, &password_hash) {
                // 注意处理 bcrypt::verify 返回的 Result
                let user = UserResult {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    status: row.get(3)?,
                    user_level: row.get(4)?,
                    is_del: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                };
                return Ok(ResultData {
                    code: 10000,
                    data: Some(user),
                    msg: "登录成功".to_string(),
                });
            } else {
                // 密码不匹配
                return Ok(ResultData {
                    code: 10001,
                    data: None,
                    msg: "用户名或密码错误".to_string(),
                });
            }
        }

        // 用户未找到
        Ok(ResultData {
            code: 10001,
            data: None,
            msg: "用户名或密码错误".to_string(),
        })
    }

    /**
     * 添加用户
     */
    pub fn add_user(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<ResultData<UserResult>, RustsqliteError> {
        let tx: rusqlite::Transaction<'_> = self.conn.transaction()?;

        let user_exists = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM users WHERE users.email = ?)",
                params![email],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if user_exists {
            // User already exists
            return Ok(ResultData {
                msg: "用户已存在".to_string(),
                data: None,
                code: 10001,
            });
        }

        let hashed = match hash_password(password) {
            Ok(h) => h,
            Err(e) => {
                println!("密码加密失败, {}", e);
                return Ok(ResultData {
                    code: 10002,
                    msg: "密码加密失败".to_string(),
                    data: None,
                });
            }
        };

        let insert_sql = "INSERT INTO users (name, email, status, is_del, user_level, password, created_at, updated_at) VALUES (:name, :email, 0, 0, 0, :password, strftime('%s','now'), strftime('%s','now'))";

        println!("{}", insert_sql);

        // 执行插入
        tx.execute(
            insert_sql,
            named_params! {
                ":name": name,
                ":email": email,
                ":password": hashed
            },
        )?;

        let id = tx.last_insert_rowid() as i32;
        // 查询数据
        let user: Result<UserResult, RustsqliteError> = tx.query_row(
            "SELECT * FROM users WHERE users.id = ?",
            params![id],
            |row| {
                Ok(UserResult {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    status: row.get(3)?,
                    user_level: row.get(4)?,
                    is_del: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        );

        // 提交事务
        tx.commit()?;

        println!("dasdasd {:?}", user);
        user.map(|u| ResultData {
            code: 10000,
            msg: "注册成功".to_string(),
            data: Some(u),
        })
        .map_err(|e| e.into()) // 在这里正确地处理查询结果或错误
    }

    pub fn edit_user_password(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<bool, RustsqliteError> {
        let tx = self.conn.transaction()?;

        let hashed =
            hash_password(password).map_err(|_| RustsqliteError::ExecuteReturnedResults)?;

        tx.execute(
            "UPDATE sessions SET password=? WHERE users.email=?",
            params![hashed, email],
        )?;

        tx.commit()?;
        Ok(true)
    }

    pub fn get_all_users(
        &mut self,
        page: i32,
        page_size: i32,
    ) -> Result<ResultData<List<Vec<UserResult>>>, RustsqliteError> {
        // 查询总量
        // 查询总数
        let total_count: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM users WHERE users.is_del=0",
            [],
            |row| row.get(0),
        )?;

        // 查询数据
        let mut stmt = self.conn.prepare("SELECT * FROM users WHERE users.is_del=0 ORDER BY created_at DESC LIMIT :page_size OFFSET (:page - 1) * :page_size;")?;

        let users_iter = stmt.query_map(
            named_params! {
                ":page": page,
                ":page_size": page_size,
            },
            |row| {
                Ok(UserResult {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    status: row.get(3)?,
                    user_level: row.get(4)?,
                    is_del: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        )?;
        let mut users = Vec::new();
        for user_result in users_iter {
            users.push(user_result?);
        }

        let data = List {
            list: users,
            total: total_count,
        };

        Ok(ResultData {
            msg: "success".to_string(),
            code: 10000,
            data: Some(data),
        })
    }

    pub fn init_database(conn: &Connection) -> Result<(), RustsqliteError> {
        // 初始化用户表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL, -- 用户名称
                email TEXT NOT NULL, -- 用户邮箱
                status INTEGER DEFAULT 0 NOT NULL, -- 用户状态, 0: 未启用, 1: 已启用, -1: 已禁用
                user_level INTEGER DEFAULT 0 NOT NULL, -- 操作类型 0: 系统用户, 1: 普通用户, 99: 超管用户
                password TEXT NOT NULL, -- 用户密码
                is_del INTEGER DEFAULT 0 NOT NULL, -- 是否已删除, 0: 未删除, 1: 已删除
                created_at TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL, -- 创建时间
                updated_at TEXT NOT NULL -- 更新时间
            )",
            [],
        )?;

        Ok(())
    }
}
