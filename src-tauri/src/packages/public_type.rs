use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List<T> {
    pub list: T,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultData<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
