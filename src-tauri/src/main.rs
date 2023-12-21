#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands;

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                println!("close");
                api.prevent_close()
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![commands::hi])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
