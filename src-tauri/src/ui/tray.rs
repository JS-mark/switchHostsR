use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

/// 打开主页
fn open_home<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(window) = app.get_webview_window("home") {
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

/// 创建系统托盘
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // 创建菜单项
    let open_item = MenuItem::with_id(app, "open", "打开主页", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // 创建菜单
    let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

    // 创建托盘图标
    let mut builder = TrayIconBuilder::with_id("main")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                if let Err(e) = open_home(app) {
                    eprintln!("Failed to open home: {}", e);
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(move |tray, event| match event {
            TrayIconEvent::Click {
                button,
                button_state,
                ..
            } => {
                use tauri::tray::{MouseButton, MouseButtonState};
                println!(
                    "Tray icon clicked with {:?} button in {:?} state",
                    button, button_state
                );

                // 左键点击显示主页
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    let app = tray.app_handle();
                    if let Err(e) = open_home(app) {
                        eprintln!("Failed to open home: {}", e);
                    }
                }
            }
            TrayIconEvent::DoubleClick { button, .. } => {
                println!("Tray icon double-clicked with {:?} button", button);
            }
            _ => {}
        });

    // 设置图标（如果存在）
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;

    Ok(())
}
