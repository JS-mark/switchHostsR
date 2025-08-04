use crate::db::{self, handlers};
use std::fmt;
use std::sync::{Mutex, MutexGuard};
use tauri::State;

// 手动实现 Debug trait 而不是使用 derive
use crate::db::handlers::{HostGroupHandler, HostHandler, LogHandler, UserHandler};

pub struct AppState {
    pub user_id: Mutex<i32>,
    pub user_db: Mutex<UserHandler>,
    pub logs_db: Mutex<LogHandler>,
    pub hosts_db: Mutex<HostHandler>,
    pub host_groups_db: Mutex<HostGroupHandler>,
}

// 手动实现 Debug trait
impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("user_id", &self.user_id)
            // 跳过不支持 Debug 的字段
            .field("user_db", &"<UserHandler>")
            .field("logs_db", &"<LogHandler>")
            .field("hosts_db", &"<HostHandler>")
            .field("host_groups_db", &"<HostGroupHandler>")
            .finish()
    }
}

// 实现 Send + Sync 特性，确保可以在线程间安全传递
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub fn new(
        user_id: Mutex<i32>,
        user_db: Mutex<handlers::UserHandler>,
        logs_db: Mutex<handlers::LogHandler>,
        hosts_db: Mutex<handlers::HostHandler>,
        host_groups_db: Mutex<handlers::HostGroupHandler>,
    ) -> Self {
        Self {
            user_id,
            user_db,
            logs_db,
            hosts_db,
            host_groups_db,
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

    pub fn get_user_db(&self) -> MutexGuard<handlers::UserHandler> {
        self.user_db.lock().unwrap()
    }

    pub fn get_logs_db(&self) -> MutexGuard<handlers::LogHandler> {
        self.logs_db.lock().unwrap()
    }

    pub fn get_hosts_db(&self) -> MutexGuard<handlers::HostHandler> {
        self.hosts_db.lock().unwrap()
    }

    pub fn get_host_groups_db(&self) -> MutexGuard<handlers::HostGroupHandler> {
        self.host_groups_db.lock().unwrap()
    }

    // 为 Tauri 添加一个从 State 获取 AppState 的方法
    pub fn from_state<'a>(state: State<'a, Self>) -> State<'a, Self> {
        state
    }
}

pub fn create_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    // 创建数据库连接
    let conn = db::create_pool()?;

    // 创建 db
    let user_db = handlers::UserHandler::new(conn.clone(), 0)?;
    let logs_db = handlers::LogHandler::new(conn.clone(), 0)?;
    let hosts_db = handlers::HostHandler::new(conn.clone(), 0)?;
    let host_groups_db = handlers::HostGroupHandler::new(conn.clone(), 0)?;

    // 创建应用状态
    Ok(AppState::new(
        Mutex::new(0),
        Mutex::new(user_db),
        Mutex::new(logs_db),
        Mutex::new(hosts_db),
        Mutex::new(host_groups_db),
    ))
}
