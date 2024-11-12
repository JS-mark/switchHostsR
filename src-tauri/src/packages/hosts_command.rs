use std::sync::Arc;
use tauri::api::fs::read_to_string;
use tauri::async_runtime::Mutex;
use tauri::AppHandle;
use tauri::Window;

#[tauri::command]
pub async fn read_text_file(
    app: AppHandle,
    window: Window,
    path: String,
) -> Result<String, String> {
    let file_content = read_to_string(path).await;
    match file_content {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("读取文件出错: {}", e)),
    }
}

#[tauri::command]
async fn write_text_file(
    app: AppHandle,
    window: Window,
    path: String,
    content: String,
) -> Result<(), String> {
    let write_result = write(path, content).await;
    match write_result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("写入文件出错: {}", e)),
    }
}
