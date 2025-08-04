use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

/// 打开主页
fn open_home<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(window) = app.get_webview_window("home") {
        if !window.is_visible()? {
            // window.show()?;
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
    let _tray = TrayIconBuilder::with_id("main")
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
        .on_tray_icon_event(|_tray, event| match event {
            TrayIconEvent::Click {
                button,
                button_state,
                ..
            } => {
                println!(
                    "Tray icon clicked with {:?} button in {:?} state",
                    button, button_state
                );
            }
            TrayIconEvent::DoubleClick { button, .. } => {
                println!("Tray icon double-clicked with {:?} button", button);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
