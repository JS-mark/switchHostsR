#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod api;
pub mod auth;
pub mod db;
pub mod ui;
pub mod utils;

// 移除过时的命令导入，现在使用模块化的 API
use crate::api::AppState;
use crate::db::services::{get_service_factory, init_service_factory};
use log::{error, info};
use std::process::Command;
use tauri::{WebviewWindowBuilder, WindowEvent};

// 检查是否有其他实例正在运行
fn is_another_instance_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("pgrep")
            .arg("-f")
            .arg("switch-hosts-r")
            .output();

        if let Ok(output) = output {
            let processes = String::from_utf8_lossy(&output.stdout);
            let process_count = processes.lines().count();
            // 如果有超过1个进程（当前进程），说明有其他实例在运行
            return process_count > 1;
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq switch-hosts-r.exe")
            .output();

        if let Ok(output) = output {
            let processes = String::from_utf8_lossy(&output.stdout);
            return processes.contains("switch-hosts-r.exe");
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("pgrep")
            .arg("-f")
            .arg("switch-hosts-r")
            .output();

        if let Ok(output) = output {
            let processes = String::from_utf8_lossy(&output.stdout);
            let process_count = processes.lines().count();
            return process_count > 1;
        }
    }

    false
}

pub type SetupHook =
    Box<dyn FnOnce(&mut tauri::App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    env_logger::init();

    // 单实例控制 - 检查是否已有实例运行
    if is_another_instance_running() {
        error!("Another instance of the application is already running");
        std::process::exit(1);
    }

    // 初始化数据库连接池和服务工厂
    let pool = match crate::db::create_pool() {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    // 初始化服务工厂
    init_service_factory(pool);
    info!("Database and service factory initialized successfully");

    let app = tauri::Builder::default()
        .setup(|app| {
            // 创建主窗口
            let window =
                WebviewWindowBuilder::new(app, "home", tauri::WebviewUrl::App("index.html".into()))
                    .title("switch-hosts-r")
                    .inner_size(800.0, 600.0)
                    .min_inner_size(800.0, 600.0)
                    .center()
                    .decorations(false)
                    .transparent(true)
                    .resizable(true)
                    .maximizable(true)
                    .build()?;

            // 设置 window
            // let _ = ui::window::set_window_attribute(&window);

            // 设置窗口事件处理
            let window_clone = window.clone();
            window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { .. } => {
                    if let Err(e) = window_clone.hide() {
                        error!("Failed to hide window: {}", e);
                    }
                }
                _ => {}
            });

            info!("Main window created successfully");

            // 创建系统托盘
            ui::tray::create_tray(app.handle())?;

            Ok(())
        })
        // 在 invoke_handler 部分添加新的命令
        .invoke_handler(tauri::generate_handler![
            // 系统相关
            api::system::get_system_config,
            api::system::update_system_config,
            api::system::health_check,
            api::system::clean_logs,
            api::system::create_backup,
            api::system::get_backups,
            api::system::restore_backup,
            api::system::restart_service,
            // 文件操作和通用命令
            api::commands::get_system_info,
            api::commands::read_text_file,
            api::commands::write_text_file,
            api::commands::file_exists,
            api::commands::create_directory,
            api::commands::delete_file,
            api::commands::get_file_info,
            api::commands::list_directory,
            // 认证相关
            api::users::user_login,
            api::users::logout,
            api::users::refresh_token,
            api::users::change_password,
            api::users::verify_token,
            api::users::get_current_user_info,
            api::users::check_username_availability,
            api::users::check_email_availability,
            // 公开注册
            api::users::register_user,
            // 重置密码（忘记密码）
            api::users::reset_password,
            // 用户管理
            api::users::get_users,
            api::users::get_user,
            api::users::create_user,
            api::users::update_user,
            api::users::delete_user,
            api::users::get_user_stats,
            api::users::search_users,
            // 主机管理
            api::hosts::get_hosts,
            api::hosts::get_host,
            api::hosts::create_host,
            api::hosts::update_host,
            api::hosts::delete_host,
            api::hosts::toggle_host_active,
            api::hosts::get_active_hosts,
            api::hosts::get_host_stats,
            api::hosts::export_hosts,
            api::hosts::import_hosts,
            api::hosts::search_hosts,
            api::hosts::apply_hosts_to_system,
            api::hosts::read_system_hosts_file,
            // 主机组管理
            api::host_groups::get_host_groups,
            api::host_groups::get_host_group,
            api::host_groups::create_host_group,
            api::host_groups::update_host_group,
            api::host_groups::delete_host_group,
            api::host_groups::add_host_to_group,
            api::host_groups::remove_host_from_group,
            api::host_groups::toggle_group_active,
            api::host_groups::get_group_stats,
            api::host_groups::search_host_groups,
            // 日志管理
            api::logs::get_logs,
            api::logs::get_log,
            api::logs::create_log,
            api::logs::delete_log,
            api::logs::batch_delete_logs,
            api::logs::clean_old_logs,
            api::logs::get_log_stats,
            api::logs::get_user_logs,
            api::logs::export_logs,
            api::logs::search_logs,
            api::logs::get_operation_types,
            api::logs::get_target_types,
        ])
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState::new(get_service_factory().clone()));

    // 加载配置
    // 构建并运行应用
    let app = match app.build(tauri::generate_context!()) {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to build application: {}", e);
            std::process::exit(1);
        }
    };

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    })
}
