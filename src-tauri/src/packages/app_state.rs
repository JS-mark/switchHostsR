use crate::sqlite_db::{create_pool, handler};
use std::sync::{Mutex, MutexGuard};

pub struct AppState {
    user_id: Mutex<i32>,
    user_db: Mutex<handler::users::Database>,
    logs_db: Mutex<handler::logs::Database>,
    hosts_db: Mutex<handler::hosts::Database>,
}

impl AppState {
    pub fn new(
        user_id: Mutex<i32>,
        user_db: Mutex<handler::users::Database>,
        logs_db: Mutex<handler::logs::Database>,
        hosts_db: Mutex<handler::hosts::Database>,
    ) -> Self {
        Self {
            user_id: user_id,
            user_db: user_db,
            logs_db: logs_db,
            hosts_db: hosts_db,
        }
    }

    // 注意，这里返回的是一个锁的守护者，允许访问内部的i32值
    pub fn get_user_id(&self) -> MutexGuard<i32> {
        self.user_id.lock().unwrap()
    }

    // 修改方法以使用Mutex的lock方法来更新user_id
    pub fn set_user_id(&self, user_id: i32) {
        // 错误处理: 这里简单地使用unwrap来处理潜在的错误。
        // 在实际应用中，你可能希望更优雅地处理这个错误。
        *self.user_id.lock().unwrap() = user_id; // 如果使用 Mutex
    }

    pub fn get_user_db(&self) -> MutexGuard<handler::users::Database> {
        self.user_db.lock().unwrap()
    }

    pub fn get_logs_db(&self) -> MutexGuard<handler::logs::Database> {
        self.logs_db.lock().unwrap()
    }

    pub fn get_hosts_db(&self) -> MutexGuard<handler::hosts::Database> {
        self.hosts_db.lock().unwrap()
    }
}

pub fn create_app_state() -> AppState {
    // 创建数据库连接
    let conn = create_pool();
    // 创建 db
    let user_db =
        handler::users::Database::new(conn.clone(), 0).expect("Unable to create database connection");
    let logs_db = handler::logs::Database::new(conn.clone(), 0)
        .expect("Unable to create database connection");
    let hosts_db = handler::hosts::Database::new(conn.clone(), 0)
        .expect("Unable to create database connection");

    // 创建应用状态
    return AppState::new(
        Mutex::new(0),
        Mutex::new(user_db),
        Mutex::new(logs_db),
        Mutex::new(hosts_db),
    );
}
