use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::sqlite_db::schema::users;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub status: i32,
    pub user_level: i32,
    pub password: String,
    pub is_del: i32,
    pub is_third: i32,
    pub third_account_uid: Option<String>, // 注意这是一个 Option 类型
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub status: i32,
    pub user_level: i32,
    pub is_del: i32,
    pub is_third: i32,
    pub third_account_uid: Option<String>, // 注意这是一个 Option 类型
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct UserLog {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct AddUser {
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub status: i32,
    pub user_level: i32,
    pub password: String,
    pub is_del: i32,
    pub is_third: i32,
    pub third_account_uid: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct EditUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<i32>,
    pub user_level: Option<i32>,
    pub password: Option<String>,
    pub is_del: Option<i32>,
}
