//! 通用命令 API
//!
//! 处理文件操作和系统信息等通用 Tauri 命令
//! 所有文件操作均限制在应用数据目录内，防止路径遍历攻击。
//! 仅对系统 hosts 文件提供只读访问。

use crate::api::{ApiResult, AppState};
use crate::{require_auth, safe_execute};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
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

/// 获取应用数据目录
///
/// 返回 `~/.switchhostsr/` 作为安全沙箱目录
fn get_app_data_dir() -> Result<PathBuf, anyhow::Error> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;
    let app_dir = home.join(".switchhostsr");
    // 确保目录存在
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| anyhow::anyhow!("创建应用数据目录失败: {}", e))?;
    }
    Ok(app_dir)
}

/// 获取系统 hosts 文件路径
fn get_system_hosts_path() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        r"C:\Windows\System32\drivers\etc\hosts"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "/etc/hosts"
    }
}

/// 检查路径是否为系统 hosts 文件（只读访问白名单）
fn is_system_hosts_path(path: &Path) -> bool {
    // 比较两个路径的 canonicalize 结果（处理 macOS 上 /etc -> /private/etc 的符号链接）
    let hosts_canonical = match Path::new(get_system_hosts_path()).canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };
    match path.canonicalize() {
        Ok(canonical) => canonical == hosts_canonical,
        Err(_) => false,
    }
}

/// 验证文件路径是否在安全目录内
///
/// 安全规则：
/// 1. 拒绝包含 `..` 的路径（防止目录遍历）
/// 2. 拒绝绝对路径（只接受相对于应用数据目录的路径）
/// 3. 将相对路径解析为应用数据目录下的绝对路径
/// 4. 使用 canonicalize 验证最终路径确实在安全目录内
fn validate_safe_path(file_path: &str) -> Result<PathBuf, anyhow::Error> {
    let path = Path::new(file_path);

    // 拒绝包含 .. 的路径（防止目录遍历）
    if file_path.contains("..") {
        return Err(anyhow::anyhow!("路径不允许包含 '..'"));
    }

    // 拒绝绝对路径
    if path.is_absolute() {
        return Err(anyhow::anyhow!("不允许使用绝对路径，请使用相对路径"));
    }

    // 将相对路径解析到应用数据目录下
    let app_dir = get_app_data_dir()?;
    let full_path = app_dir.join(path);

    // 如果路径已存在，使用 canonicalize 验证不会逃逸
    if full_path.exists() {
        let canonical = full_path.canonicalize()
            .map_err(|e| anyhow::anyhow!("路径解析失败: {}", e))?;
        let canonical_app_dir = app_dir.canonicalize()
            .map_err(|e| anyhow::anyhow!("应用目录解析失败: {}", e))?;

        if !canonical.starts_with(&canonical_app_dir) {
            return Err(anyhow::anyhow!("文件路径超出应用数据目录范围"));
        }
        Ok(canonical)
    } else {
        // 路径不存在时，验证父目录
        if let Some(parent) = full_path.parent() {
            if parent.exists() {
                let canonical_parent = parent.canonicalize()
                    .map_err(|e| anyhow::anyhow!("父目录解析失败: {}", e))?;
                let canonical_app_dir = app_dir.canonicalize()
                    .map_err(|e| anyhow::anyhow!("应用目录解析失败: {}", e))?;

                if !canonical_parent.starts_with(&canonical_app_dir) {
                    return Err(anyhow::anyhow!("文件路径超出应用数据目录范围"));
                }
            }
        }
        Ok(full_path)
    }
}

/// 验证读取路径（额外允许系统 hosts 文件的只读访问）
fn validate_read_path(file_path: &str) -> Result<PathBuf, anyhow::Error> {
    let path = Path::new(file_path);

    // 特殊处理：允许只读访问系统 hosts 文件
    if path.is_absolute() && is_system_hosts_path(path) {
        return Ok(path.to_path_buf());
    }

    // 其他路径走标准安全验证
    validate_safe_path(file_path)
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
///
/// 支持读取应用数据目录内的文件，以及系统 hosts 文件（只读）
#[tauri::command]
pub async fn read_text_file(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<String>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!((|| -> anyhow::Result<String> {
        let safe_path = validate_read_path(&file_path)?;
        std::fs::read_to_string(&safe_path)
            .map_err(|e| anyhow::anyhow!("读取文件失败: {}", e))
    })());
    Ok(result)
}

/// 写入文本文件
///
/// 仅允许写入应用数据目录内的文件
#[tauri::command]
pub async fn write_text_file(
    _state: State<'_, AppState>,
    file_path: String,
    content: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!((|| -> anyhow::Result<()> {
        let safe_path = validate_safe_path(&file_path)?;
        // 确保父目录存在
        if let Some(parent) = safe_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("创建父目录失败: {}", e))?;
        }
        std::fs::write(&safe_path, content)
            .map_err(|e| anyhow::anyhow!("写入文件失败: {}", e))
    })());
    Ok(result)
}

/// 检查文件是否存在
#[tauri::command]
pub async fn file_exists(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<bool>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!((|| -> anyhow::Result<bool> {
        let safe_path = validate_read_path(&file_path)?;
        Ok(safe_path.exists())
    })());
    Ok(result)
}

/// 创建目录
///
/// 仅允许在应用数据目录内创建子目录
#[tauri::command]
pub async fn create_directory(
    _state: State<'_, AppState>,
    dir_path: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!((|| -> anyhow::Result<()> {
        let safe_path = validate_safe_path(&dir_path)?;
        std::fs::create_dir_all(&safe_path)
            .map_err(|e| anyhow::anyhow!("创建目录失败: {}", e))
    })());
    Ok(result)
}

/// 删除文件
///
/// 仅允许删除应用数据目录内的文件
#[tauri::command]
pub async fn delete_file(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<()>, ()> {
    let _auth = require_auth!();

    let result = safe_execute!((|| -> anyhow::Result<()> {
        let safe_path = validate_safe_path(&file_path)?;
        std::fs::remove_file(&safe_path)
            .map_err(|e| anyhow::anyhow!("删除文件失败: {}", e))
    })());
    Ok(result)
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_info(
    _state: State<'_, AppState>,
    file_path: String,
) -> Result<ApiResult<Value>, ()> {
    let result = safe_execute!((|| -> anyhow::Result<Value> {
        let safe_path = validate_read_path(&file_path)?;
        let metadata = std::fs::metadata(&safe_path)
            .map_err(|e| anyhow::anyhow!("获取文件信息失败: {}", e))?;
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
        Ok(info)
    })());
    Ok(result)
}

/// 列出目录内容
///
/// 仅允许列出应用数据目录内的内容
#[tauri::command]
pub async fn list_directory(
    _state: State<'_, AppState>,
    dir_path: String,
) -> Result<ApiResult<Vec<Value>>, ()> {
    let result = safe_execute!((|| -> anyhow::Result<Vec<Value>> {
        let safe_path = validate_safe_path(&dir_path)?;
        let entries =
            std::fs::read_dir(&safe_path).map_err(|e| anyhow::anyhow!("读取目录失败: {}", e))?;

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
    })());
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

    #[test]
    fn test_validate_safe_path_rejects_traversal() {
        // 包含 .. 的路径应被拒绝
        assert!(validate_safe_path("../etc/passwd").is_err());
        assert!(validate_safe_path("foo/../../bar").is_err());
    }

    #[test]
    fn test_validate_safe_path_rejects_absolute() {
        // 绝对路径应被拒绝
        assert!(validate_safe_path("/etc/passwd").is_err());
        assert!(validate_safe_path("/tmp/test.txt").is_err());
    }

    #[test]
    fn test_validate_safe_path_accepts_relative() {
        // 合法的相对路径应被接受
        let result = validate_safe_path("data/hosts.txt");
        assert!(result.is_ok());
        let path = result.unwrap();
        // 验证路径在应用数据目录下
        let app_dir = get_app_data_dir().unwrap();
        assert!(path.starts_with(&app_dir));
    }

    #[test]
    fn test_validate_read_path_allows_hosts_file() {
        // 系统 hosts 文件路径应允许读取
        let hosts_path = get_system_hosts_path();
        let result = validate_read_path(hosts_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_read_path_rejects_other_system_files() {
        // 其他系统文件的绝对路径应被拒绝
        #[cfg(not(target_os = "windows"))]
        {
            assert!(validate_read_path("/etc/passwd").is_err());
            assert!(validate_read_path("/etc/shadow").is_err());
        }
    }
}
