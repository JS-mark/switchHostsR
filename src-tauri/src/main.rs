#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod packages;
use packages::logs;
use packages::permission;
use packages::public_type::{List, ResultData};
use packages::users;
use packages::users::UserResult;
use std::sync::Mutex;

fn main() {
    let users_db =
        users::Database::new("database.db").expect("Unable to create database connection");
    let logs_db = logs::Database::new("database.db").expect("Unable to create database connection");
    let permisson_db =
        permission::Database::new("database.db").expect("Unable to create database connection");
    let app_state = AppState::new(
        Mutex::new(users_db),
        Mutex::new(logs_db),
        Mutex::new(permisson_db),
    );
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_users,
            get_all_logs,
            add_log,
            edit_user_password,
            third_account_login,
            add_user,
            user_login
        ])
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppState {
    user_db: Mutex<users::Database>,
    logs_db: Mutex<logs::Database>,
    permission_db: Mutex<permission::Database>,
}

impl AppState {
    pub fn new(
        user_db: Mutex<users::Database>,
        logs_db: Mutex<logs::Database>,
        permission_db: Mutex<permission::Database>,
    ) -> Self {
        Self {
            user_db: user_db,
            logs_db: logs_db,
            permission_db: permission_db,
        }
    }
}

#[tauri::command]
fn get_all_users(
    state: tauri::State<AppState>,
    page: i32,
    page_size: i32,
) -> Result<ResultData<List<Vec<UserResult>>>, String> {
    println!("page {}, pageSize {}", page, page_size);
    state
        .user_db
        .lock()
        .unwrap()
        .get_all_users(page, page_size)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_all_logs(
    state: tauri::State<AppState>,
    user_id: i32,
    page: i32,
    page_size: i32,
) -> Result<ResultData<List<Vec<logs::Log>>>, String> {
    state
        .logs_db
        .lock()
        .unwrap()
        .get_all_logs(user_id, page, page_size)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn add_log(
    state: tauri::State<AppState>,
    user_id: i32,
    log: logs::LogData,
) -> Result<ResultData<i32>, String> {
    state
        .logs_db
        .lock()
        .unwrap()
        .add_log(user_id, log)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn add_user(
    state: tauri::State<AppState>,
    name: &str,
    email: &str,
    password: &str,
) -> Result<ResultData<UserResult>, String> {
    state
        .user_db
        .lock()
        .unwrap()
        .add_user(name, email, "", password)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn third_account_login(
    state: tauri::State<AppState>,
    name: &str,
    email: &str,
    account: &str,
    password: &str,
    uid: &str,
    avatar_url: &str,
    created_at: &str,
    updated_at: &str,
) -> Result<ResultData<UserResult>, String> {
    state
        .user_db
        .lock()
        .unwrap()
        .third_account_login(
            name, uid, email, account, password, avatar_url, created_at, updated_at,
        )
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn user_login(
    state: tauri::State<AppState>,
    email: &str,
    password: &str,
) -> Result<ResultData<UserResult>, String> {
    state
        .user_db
        .lock()
        .unwrap()
        .verify_user(email, password)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn edit_user_password(
    state: tauri::State<AppState>,
    username: &str,
    password: &str,
) -> Result<bool, String> {
    state
        .user_db
        .lock()
        .unwrap()
        .edit_user_password(username, password)
        .map_err(|err| err.to_string())
}
