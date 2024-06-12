use super::app_state::AppState;
use super::{
    public_type::{List, ResultData},
    system_info,
    utils::now_time,
};
use crate::sqlite_db::models::users::{EditUser, UserData};
use crate::sqlite_db::models::{
    hosts::{EditHostData, HostData},
    logs::{AddLog, AddLogData, LogsList},
};

#[tauri::command]
pub fn get_system_info() -> ResultData<system_info::SystemInfo> {
    return ResultData {
        code: 10000,
        data: Some(system_info::get_system_info()),
        msg: "success".to_string(),
    };
}

/**
 * 获取用户 id
 */
pub fn get_user_id(state: tauri::State<'_, AppState>) -> Result<i32, String> {
    let user_id = state.get_user_id();
    Ok(*user_id)
}

/**
 * 设置用户登录 id
 */
pub fn set_user_id(state: tauri::State<'_, AppState>, new_id: i32) -> Result<(), String> {
    // 直接调用AppState的方法，不需要直接处理Mutex
    state.inner().set_user_id(new_id);
    Ok(())
}

/**
 * 用户退出登录s
 */
#[tauri::command]
pub fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    // 直接调用AppState的方法，不需要直接处理Mutex
    let data = set_user_id(state.clone(), 0).map_err(|err| err.to_string());
    Ok(data.unwrap())
}

#[tauri::command]
pub fn update_host(
    state: tauri::State<AppState>,
    id: i32,
    host: EditHostData,
) -> Result<ResultData<()>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;
    if user_id == 0 {
        return Err("User not found".to_string());
    }

    return Ok(state
        .get_hosts_db()
        .update_host(id, host)
        .map_err(|err| err.to_string())?);
}

#[tauri::command]
pub fn get_all_hosts_data(
    state: tauri::State<AppState>,
    page: i64,
    page_size: i64,
) -> Result<ResultData<List<HostData>>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;
    if user_id == 0 {
        return Err("User not found".to_string());
    }

    // 更新 user_id
    state.get_hosts_db().update_user_id(user_id);

    return Ok(state
        .get_hosts_db()
        .get_all_hosts(page, page_size)
        .map_err(|err| err.to_string())?);
}

#[tauri::command]
pub fn debug_user(state: tauri::State<AppState>, user_id: i32) -> Result<(), String> {
    println!("User ID: {}", user_id);
    if user_id <= 0 {
        return Ok(());
    }
    println!("User ID: {}", user_id);
    // 这里可以添加调试代码
    let data = set_user_id(state.clone(), user_id).map_err(|err| err.to_string());
    Ok(data.unwrap())
}

/**
 * 查询所有用户
 */
#[tauri::command]
pub fn get_all_users(
    state: tauri::State<AppState>,
    page: i64,
    page_size: i64,
) -> Result<ResultData<List<UserData>>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;
    if user_id == 0 {
        return Err("User not found".to_string());
    }

    state.get_user_db().update_user_id(user_id);

    return Ok(state
        .get_user_db()
        .get_all_users(page, page_size)
        .map_err(|err| err.to_string())?);
}

/**
 * 获取所有日志
 */
#[tauri::command]
pub fn get_all_logs(
    state: tauri::State<AppState>,
    page: i64,
    page_size: i64,
) -> Result<ResultData<List<LogsList>>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;

    if user_id == 0 {
        return Err("User not found".to_string());
    }

    let db = &mut state.get_logs_db();

    // 更新 user_id
    db.update_user_id(user_id);

    return Ok(db
        .get_all_logs(page, page_size)
        .map_err(|err| err.to_string())?);
}

/**
 * 添加日志
 */
#[tauri::command]
pub fn add_log(
    state: tauri::State<AppState>,
    log_options: AddLogData,
) -> Result<ResultData<usize>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;
    if user_id == 0 {
        return Err("User not found".to_string());
    }

    let db = &mut state.get_logs_db();

    // 更新 user_id
    db.update_user_id(user_id);

    let data = AddLog {
        log_type: log_options.log_type,
        content: log_options.content,
        created_by: Some(user_id),
        created_at: now_time().to_string(),
    };

    return db.add_log(data);
}

// #[tauri::command]
// fn add_user(
//     state: tauri::State<AppState>,
//     name: &str,
//     email: &str,
//     password: &str,
// ) -> Result<ResultData<UserResult>, String> {
//     let user_info = state
//         .user_db
//         .lock()
//         .unwrap()
//         .add_user(name, email, "", password)
//         .map_err(|err| err.to_string());
//     {
//         match user_info {
//             Ok(user_info_res) => {
//                 let res = user_info_res.clone();
//                 let id = res.data.unwrap().id;
//                 // 更新 user_id
//                 let _ = set_user_id(state, id).map_err(|err| err.to_string());

//                 Ok(user_info_res)
//             }
//             Err(err) => Err(err),
//         }
//     } // 锁在这个代码块结束时自动释放
// }

// #[tauri::command]
// fn third_account_login(
//     state: tauri::State<AppState>,
//     name: &str,
//     email: &str,
//     account: &str,
//     password: &str,
//     uid: &str,
//     avatar_url: &str,
//     created_at: &str,
//     updated_at: &str,
// ) -> Result<ResultData<UserResult>, String> {
//     let user_info = state
//         .user_db
//         .lock()
//         .unwrap()
//         .third_account_login(
//             name, uid, email, account, password, avatar_url, created_at, updated_at,
//         )
//         .map_err(|err| err.to_string());

//     {
//         match user_info {
//             Ok(user_info_res) => {
//                 let res = user_info_res.clone();
//                 let id = res.data.unwrap().id;
//                 // 更新 user_id
//                 let _ = set_user_id(state, id).map_err(|err| err.to_string());

//                 Ok(user_info_res)
//             }
//             Err(err) => Err(err),
//         }
//     } // 锁在这个代码块结束时自动释放
// }

#[tauri::command]
pub fn user_login(
    state: tauri::State<AppState>,
    email: &str,
    password: &str,
) -> Result<ResultData<UserData>, String> {
    let db = &mut state.get_user_db();
    let user_info = db
        .verify_user(email, password)
        .map_err(|err| err.to_string());

    {
        match user_info {
            Ok(user_info_res) => {
                let id = user_info_res.data.as_ref().unwrap().id.as_ref().unwrap();

                // 更新 user_id
                state.inner().set_user_id(*id);

                Ok(user_info_res)
            }
            Err(err) => Err(err),
        }
    } // 锁在这个代码块结束时自动释放
}

/**
 * 编辑用户
 */
#[tauri::command]
pub fn edit_user(
    state: tauri::State<AppState>,
    user: EditUser,
) -> Result<ResultData<usize>, String> {
    let user_id = get_user_id(state.clone()).map_err(|err| err.to_string())?;

    if user_id == 0 {
        return Err("User not found".to_string());
    }

    let db = &mut state.get_user_db();
    return db.edit_user(user).map_err(|err| err.to_string());
}
