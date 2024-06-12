use crate::sqlite_db::schema::hosts_data;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct HostData {
    pub id: Option<i32>,
    pub name: String,
    pub hosts_type: i32,
    pub hosts_path: Option<String>,
    pub content: String,
    pub status: i32,
    pub is_del: i32,
    pub is_readonly: i32,
    pub created_by: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "hosts_data"]
pub struct AddHostData {
    pub id: i32,
    pub name: String,
    pub hosts_type: i32,
    pub hosts_path: String,
    pub content: String,
    pub status: i32,
    pub is_del: i32,
    pub is_readonly: i32,
    pub created_by: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "hosts_data"]
pub struct EditHostData {
    pub name: String,
    pub hosts_type: i32,
    pub hosts_path: String,
    pub content: String,
}
