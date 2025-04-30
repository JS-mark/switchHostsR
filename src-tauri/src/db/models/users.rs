use crate::db::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub is_admin: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}
