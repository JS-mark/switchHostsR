#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod db;
pub mod ui;
pub mod utils;

use api::commands::{debug_user, edit_user, get_system_info, read_text_file, write_text_file};
use api::hosts::{get_all_hosts_data, update_host};
use api::logs::add_log;
use api::users::{get_all_users, logout, user_login};
use log::{error, info};
use tauri::WindowEvent;
use ui::app_state::create_app_state;
use ui::tray::{create_tray_menu, handle_tray_event};

pub type SetupHook =
    Box<dyn FnOnce(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    env_logger::init();

    let mut app = tauri::Builder::default()
        .setup(|app| {
            ui::window::set_window_attribute(app);

            // 创建主窗口
            let window =
                WindowBuilder::new(app, "home", tauri::WindowUrl::App("index.html".into()))
                    .title("switch-hosts-r")
                    .inner_size(800.0, 600.0)
                    .center()
                    .decorations(false)
                    .transparent(true)
                    .build()?;

            // 设置窗口事件处理
            window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { .. } => {
                    if let Err(e) = window.hide() {
                        error!("Failed to hide window: {}", e);
                    }
                }
                _ => {}
            });

            info!("Main window created successfully");
            Ok(())
        })
        // 设置系统托盘
        .system_tray(tauri::SystemTray::new().with_menu(create_tray_menu()))
        .on_system_tray_event(handle_tray_event)
        // 在 invoke_handler 部分添加新的命令
        .invoke_handler(tauri::generate_handler![
            // 日志相关命令
            add_log,
            api::logs::get_all_logs,
            api::logs::get_user_logs,
            api::logs::clean_old_logs,
            // 用户相关命令
            debug_user,
            edit_user,
            user_login,
            logout,
            get_all_users,
            api::users::get_current_user,
            api::users::create_user,
            api::users::update_user,
            api::users::delete_user,
            api::users::change_password,
            // 主机相关命令
            get_all_hosts_data,
            update_host,
            api::hosts::create_host,
            api::hosts::get_host,
            api::hosts::get_user_hosts,
            api::hosts::toggle_host_active,
            api::hosts::delete_host,
            api::hosts::get_active_hosts,
            // 主机组相关命令
            api::host_groups::create_host_group,
            api::host_groups::get_user_host_groups,
            api::host_groups::get_host_group,
            api::host_groups::update_host_group,
            api::host_groups::delete_host_group,
            api::host_groups::add_host_to_group,
            api::host_groups::remove_host_from_group,
            api::host_groups::get_hosts_in_group,
            // 系统相关命令
            get_system_info,
            read_text_file,
            write_text_file,
        ])
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .manage(create_app_state());

    // 运行应用
    if let Err(e) = app.build(tauri::generate_context!()) {
        error!("Failed to build application: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }) {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}
