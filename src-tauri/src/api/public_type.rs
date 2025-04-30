use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List<T> {
    pub list: Vec<T>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultData<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}
