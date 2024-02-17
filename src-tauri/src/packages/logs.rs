use super::public_type::{self, List, ResultData};
use rusqlite::{params, Connection, Error};
use serde::{Deserialize, Serialize};

pub struct Database {
    pub conn: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: i32,
    pub content: String,
    pub log_type: i32,   //  -- 操作类型 0:更新,1:删除,2:增加,-1:未知
    pub created_by: i32, // 用户id
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogData {
    pub content: String,
    pub log_type: i32, //  -- 操作类型 0:更新,1:删除,2:增加,-1:未知
}

impl Database {
    pub fn new(file_path: impl AsRef<std::path::Path>) -> Result<Database, Error> {
        let conn = Connection::open(file_path)?;
        Database::init_database(&conn)?;
        Ok(Database { conn })
    }

    /**
     * 插入 log
     */
    pub fn add_log(&mut self, user_id: i32, log: LogData) -> Result<ResultData<i32>, Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO logs (log_type, content, created_at, created_by) VALUES (?,?,strftime('%s','now'), ?)",
            params![log.log_type, log.content, user_id],
        )?;
        let id: i32 = tx.last_insert_rowid() as i32;

        tx.commit()?;
        Ok(ResultData {
            data: Some(id),
            code: 10000,
            msg: "success".to_string(),
        })
    }

    pub fn get_all_logs(
        &mut self,
        user_id: i32,
        page: i32,
        page_size: i32,
    ) -> Result<public_type::ResultData<public_type::List<Vec<Log>>>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM logs WHERE created_by = :user_id ORDER BY created_at DESC LIMIT :page_size OFFSET (:page - 1) * :page_size;"
        )?;
        let logs_iter = stmt.query_map(
            &[
                (":user_id", &user_id),
                (":page", &page),
                (":page_size", &page_size),
            ],
            |row| {
                Ok(Log {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    log_type: row.get(2)?,
                    created_by: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )?;
        let mut logs = Vec::new();
        for logs_result in logs_iter {
            logs.push(logs_result?);
        }

        let list = List {
            list: logs,
            total: 0,
        };

        Ok(ResultData {
            code: 10000,
            data: Some(list),
            msg: "success".to_string(),
        })
    }

    pub fn init_database(conn: &Connection) -> Result<(), Error> {
        // 创建权限菜单
        conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                log_type INTEGER DEFAULT 0 NOT NULL, -- 操作类型 0:更新,1:删除,2:增加,-1:未知
                content TEXT NOT NULL,  -- JSON数据存储为文本
                created_at INTEGER NOT NULL, -- 创建时间
                created_by INTEGER,  -- 操作人员，默认为当前操作人员，system
                FOREIGN KEY (created_by) REFERENCES users(id)  -- 外键约束引用users表的id
            );",
            [],
        )?;
        Ok(())
    }
}
