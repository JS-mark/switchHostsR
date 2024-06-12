// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod packages;
mod sqlite_db;
use packages::app_state::create_app_state;
use packages::command::{
    add_log, debug_user, edit_user, get_all_hosts_data, get_all_logs, get_all_users,
    get_system_info, logout, update_host, user_login,
};
use tauri::SystemTray;

fn main() {
    let system_tray = SystemTray::new();
    tauri::Builder::default()
        .setup(|app| {
            packages::window::set_window_attribute(app);
            Ok(())
        })
        .menu(tauri::Menu::new()) // 使用空菜单来替换默认的操作系统菜单
        .system_tray(system_tray) // 将 `tauri.conf.json` 上配置的图标添加到系统托盘
        .invoke_handler(tauri::generate_handler![
            get_all_logs,
            get_all_users,
            add_log,
            get_system_info,
            get_all_hosts_data,
            update_host,
            debug_user,
            user_login,
            edit_user,
            logout
        ])
        .manage(create_app_state())
        .on_system_tray_event(packages::tray::handler) // 注册系统托盘事件处理程序
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        }) // 阻止默认关闭行为
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
