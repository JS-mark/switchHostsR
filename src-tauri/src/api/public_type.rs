use serde::{Deserialize, Serialize};

/// 统一API响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    /// 成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    /// 成功响应（无数据）
    pub fn success_empty() -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: None,
        }
    }

    /// 错误响应
    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    /// 通用错误响应
    pub fn internal_error(msg: String) -> Self {
        Self {
            code: 500,
            msg,
            data: None,
        }
    }

    /// 认证错误响应
    pub fn auth_error(msg: String) -> Self {
        Self {
            code: 401,
            msg,
            data: None,
        }
    }

    /// 权限错误响应
    pub fn permission_error(msg: String) -> Self {
        Self {
            code: 403,
            msg,
            data: None,
        }
    }

    /// 参数错误响应
    pub fn param_error(msg: String) -> Self {
        Self {
            code: 400,
            msg,
            data: None,
        }
    }

    /// 资源不存在错误响应
    pub fn not_found_error(msg: String) -> Self {
        Self {
            code: 404,
            msg,
            data: None,
        }
    }
}

/// 分页数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData<T> {
    pub list: Vec<T>,
    pub total: i32,
}

/// 分页请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageParams {
    pub page: i32,
    pub page_size: i32,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 10,
        }
    }
}

impl PageParams {
    pub fn new(page: Option<i32>, page_size: Option<i32>) -> Self {
        Self {
            page: page.unwrap_or(1).max(1),
            page_size: page_size.unwrap_or(10).max(1).min(100),
        }
    }

    pub fn offset(&self) -> i32 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> i32 {
        self.page_size
    }
}
