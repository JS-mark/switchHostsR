use rusqlite::{params, Connection, Error};
use serde::{Deserialize, Serialize};

pub struct Database {
    pub conn: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub menu_list: String,
    pub status: i32,     // 0: 启用，1: 未启用
    pub is_del: i32,     // 0: 删除，1: 未删除
    pub created_by: i32, // 用户id
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionParams {
    pub name: String,
    pub menu_list: String,
    pub status: i32,     // 0: 启用，1: 未启用
    pub is_del: i32,     // 0: 删除，1: 未删除
    pub created_by: i32, // 用户id
}

impl Database {
    pub fn new(file_path: impl AsRef<std::path::Path>) -> Result<Database, Error> {
        let conn = Connection::open(file_path)?;
        Database::init_database(&conn)?;
        Ok(Database { conn })
    }

    /**
     * 编辑数据
     */
    pub fn edit_permission(&mut self, permission: &Permission) -> Result<usize, Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE permission_list SET name=?, menu_list=?, status=?, updated_at=strftime('%s','now') WHERE id=?",
            params![
                permission.name,
                permission.menu_list,
                permission.status,
                permission.id
            ],
        )
    }

    /**
     * 插入 log
     */
    pub fn add_permission(&mut self, permission: &Permission) -> Result<Permission, Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO permission_list (name, menu_list, status, is_del, updated_at, created_at, created_by) VALUES (?,?,?,?,strftime('%s','now'),strftime('%s','now'), ?)",
            params![permission.name, permission.menu_list, permission.status, permission.is_del, permission.created_by],
        )?;
        let id: i32 = tx.last_insert_rowid() as i32;

        tx.commit()?;
        Ok(Permission {
            id,
            ..permission.clone()
        })
    }

    pub fn delete_permission(&mut self, is_del: i32) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE permission_list SET is_del=?, updated_at=strftime('%s','now') WHERE id=?",
            [is_del],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_permission(
        &mut self,
        user_id: i32,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Permission>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM permission_list WHERE permission_list.created_by = :user_id")?;
        println!("stmt is {:#?}", stmt);
        let permission_list_iter = stmt.query_map(
            &[
                (":user_id", &user_id),
                (":page", &page),
                (":page_size", &page_size),
            ],
            |row| {
                Ok(Permission {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    menu_list: row.get(2)?,
                    status: row.get(3)?,
                    is_del: row.get(4)?,
                    created_by: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )?;
        let mut permission_list = Vec::new();
        for permission_result in permission_list_iter {
            permission_list.push(permission_result?);
        }
        println!("{:?}", permission_list);
        Ok(permission_list)
    }

    /**
     * 初始化
     */
    pub fn init_database(conn: &Connection) -> Result<(), Error> {
        // 创建权限菜单
        conn.execute(
            "CREATE TABLE IF NOT EXISTS permission_list (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL, -- 权限名称
                menu_list TEXT NOT NULL, -- 权限菜单
                status INTEGER NOT NULL, -- 状态是否已启用
                is_del INTEGER DEFAULT 0 NOT NULL, -- 是否已删除, 0: 未删除, 1: 已删除
                created_by INTEGER,  -- 操作人员，默认为当前操作人员，system
                created_at INTEGER NOT NULL, -- 创建时间
                updated_at INTEGER NOT NULL, -- 更新时间
                FOREIGN KEY (created_by) REFERENCES users(id)  -- 外键约束引用users表的id
            );",
            [],
        )?;
        Ok(())
    }
}
