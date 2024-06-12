use crate::sqlite_db::schema::logs;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Log {
    pub id: Option<i32>,
    pub log_type: i32,
    pub content: String,
    pub created_at: String,
    pub created_by: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "logs"]
pub struct AddLog {
    pub log_type: i32,
    pub content: String,
    pub created_at: String,
    pub created_by: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "logs"]
pub struct AddLogData {
    pub log_type: i32,
    pub content: String,
}

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct LogsList {
    pub id: Option<i32>,         // 对应 Nullable<Integer>
    pub log_type: i32,           // 对应 Integer
    pub content: String,         // 对应 Text
    pub created_at: String,      // 对应 Text
    pub created_by: Option<i32>, // 对应 Nullable<Integer>
    pub user_name: String,       // 对应 Text
    pub user_email: String,      // 对应 Text
    pub user_avatar_url: String, // 对应 Text
}
