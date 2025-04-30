use tauri::menu::{MenuBuilder, SystemTrayMenuItemBuilder};
use tauri::{AppHandle, Manager, Runtime};

/// 打开主页
fn open_home<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(window) = app.get_window("home") {
        if !window.is_visible()? {
            window.show()?;
        }
        if window.is_minimized()? {
            window.unminimize()?;
        }
        window.set_focus()?;
    }
    Ok(())
}

/// 还原图标
fn red_icon<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.tray_handle()?;
    handle.set_icon(tauri::Icon::Raw(
        include_bytes!("../../icons/icon.ico").to_vec(),
    ))?;
    Ok(())
}

/// 创建系统托盘菜单
pub fn create_tray_menu<R: Runtime>() -> tauri::menu::SystemTrayMenu {
    MenuBuilder::new()
        .item(&SystemTrayMenuItemBuilder::new("打开主页").build())
        .separator()
        .item(&SystemTrayMenuItemBuilder::new("退出").build())
        .build()
}

/// 托盘事件处理
pub fn handle_tray_event<R: Runtime>(
    app: &AppHandle<R>,
    event: tauri::menu::SystemTrayEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    match event {
        tauri::menu::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "打开主页" => {
                open_home(app)?;
            }
            "退出" => {
                app.exit(0);
            }
            _ => {}
        },
        tauri::menu::SystemTrayEvent::LeftClick { .. } => {
            if let Some(login_window) = app.get_window("login") {
                if login_window.is_minimized()? {
                    login_window.unminimize()?;
                } else {
                    login_window.set_focus()?;
                }
            } else if let Some(tray_window) = app.get_window("tray") {
                tray_window.emit("stop", false)?;
                open_home(app)?;
                red_icon(app)?;
            }
        }
        tauri::menu::SystemTrayEvent::RightClick { .. } => {
            if let Some(tray_window) = app.get_window("tray") {
                if let Ok(position) = tray_window.outer_position() {
                    let new_position = tauri::Position::Physical(tauri::PhysicalPosition {
                        x: position.x,
                        y: position.y - tray_window.outer_size()?.height as i32,
                    });
                    tray_window.set_position(new_position)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
