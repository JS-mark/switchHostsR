//! 通用命令 API
//!
//! 处理文件操作和系统信息等通用 Tauri 命令

use crate::api::{ApiResult, AppState};
use crate::{require_auth, safe_execute};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::State;

/// 文件读取请求
#[derive(Debug, Deserialize)]
pub struct ReadFileRequest {
    pub file_path: String,
}

/// 文件写入请求
#[derive(Debug, Deserialize)]
pub struct WriteFileRequest {
    pub file_path: String,
    pub content: String,
}

/// 系统信息响应
#[derive(Debug, Serialize)]
pub struct SystemInfoResponse {
    pub os: String,
    pub kernel: String,
    pub memory: MemoryInfo,
    pub cpu_count: usize,
}

/// 内存信息
#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

/// 获取系统信息
#[tauri::command]
pub async fn get_system_info(
    _state: State<'_, AppState>,
) -> Result<ApiResult<SystemInfoResponse>, ()> {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = SystemInfoResponse {
        os: sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel: sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        memory: MemoryInfo {
            total: sys.total_memory(),
            used: sys.used_memory(),
            available: sys.available_memory(),
        },
        cpu_count: sys.cpus().len(),
    };

    Ok(ApiResult::success(info))
}

/// 读取文本文件
#[tauri::command]
pub async fn read_text_file(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<String>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!({
        use std::fs;
        match fs::read_to_string(&file_path) {
            Ok(content) => Ok::<String, anyhow::Error>(content),
            Err(e) => Err(anyhow::anyhow!("读取文件失败: {}", e)),
        }
    });
    Ok(result)
}

/// 写入文本文件
#[tauri::command]
pub async fn write_text_file(
    _state: State<'_, AppState>,
    file_path: String,
    content: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!({
        use std::fs;
        match fs::write(&file_path, content) {
            Ok(_) => Ok::<(), anyhow::Error>(()),
            Err(e) => Err(anyhow::anyhow!("写入文件失败: {}", e)),
        }
    });
    Ok(result)
}

/// 检查文件是否存在
#[tauri::command]
pub async fn file_exists(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<bool>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!({
        let exists = std::path::Path::new(&file_path).exists();
        Ok::<bool, anyhow::Error>(exists)
    });
    Ok(result)
}

/// 创建目录
#[tauri::command]
pub async fn create_directory(
    _state: State<'_, AppState>,
    dir_path: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!({
        use std::fs;
        match fs::create_dir_all(&dir_path) {
            Ok(_) => Ok::<(), anyhow::Error>(()),
            Err(e) => Err(anyhow::anyhow!("创建目录失败: {}", e)),
        }
    });
    Ok(result)
}

/// 删除文件
#[tauri::command]
pub async fn delete_file(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!({
        use std::fs;
        match fs::remove_file(&file_path) {
            Ok(_) => Ok::<(), anyhow::Error>(()),
            Err(e) => Err(anyhow::anyhow!("删除文件失败: {}", e)),
        }
    });
    Ok(result)
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_info(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<Value>, ()> {
    let result = safe_execute!({
        use std::fs;
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                let info = serde_json::json!({
                    "size": metadata.len(),
                    "is_file": metadata.is_file(),
                    "is_dir": metadata.is_dir(),
                    "modified": metadata.modified()
                        .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                        .unwrap_or(0),
                    "created": metadata.created()
                        .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                        .unwrap_or(0)
                });
                Ok::<Value, anyhow::Error>(info)
            }
            Err(e) => Err(anyhow::anyhow!("获取文件信息失败: {}", e)),
        }
    });
    Ok(result)
}

/// 列出目录内容
#[tauri::command]
pub async fn list_directory(
    _state: State<'_, AppState>,
    dir_path: String,
) -> Result<ApiResult<Vec<Value>>, ()> {
    let result = safe_execute!({
        use std::fs;

        // 使用闭包来处理错误
        (|| -> Result<Vec<Value>, anyhow::Error> {
            let entries =
                fs::read_dir(&dir_path).map_err(|e| anyhow::anyhow!("读取目录失败: {}", e))?;

            let mut items = Vec::new();
            for entry_result in entries {
                let entry = entry_result.map_err(|e| anyhow::anyhow!("读取目录项失败: {}", e))?;

                let metadata = entry
                    .metadata()
                    .map_err(|e| anyhow::anyhow!("获取文件元数据失败: {}", e))?;

                let item = serde_json::json!({
                    "name": entry.file_name().to_string_lossy(),
                    "path": entry.path().to_string_lossy(),
                    "is_file": metadata.is_file(),
                    "is_dir": metadata.is_dir(),
                    "size": metadata.len(),
                    "modified": metadata.modified()
                        .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                        .unwrap_or(0)
                });

                items.push(item);
            }

            Ok(items)
        })()
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::services::ServiceFactory;
    use crate::db::tests::create_test_db;
    use tauri::Manager;

    fn create_test_app_state() -> AppState {
        let (pool, _temp_dir) = create_test_db();
        let service_factory = ServiceFactory::new(pool);
        AppState::new(service_factory)
    }

    #[tokio::test]
    async fn test_get_system_info() {
        let app_state = create_test_app_state();
        let app = tauri::test::mock_app();
        app.manage(app_state);

        let result = get_system_info(app.state::<AppState>()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        if response.code == 200 {
            if let Some(data) = response.data {
                // 测试成功，验证数据存在
                assert!(!data.os.is_empty());
                assert!(data.cpu_count > 0);
            } else {
                panic!("Expected data but got None");
            }
        } else {
            panic!("Expected success but got error: {}", response.msg);
        }
    }
}
