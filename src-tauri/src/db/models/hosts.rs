use crate::db::schema::hosts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = hosts)]
pub struct Host {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub is_active: i32,
    pub is_system: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = hosts)]
pub struct NewHost {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub is_active: i32,
    pub is_system: i32,
    pub created_at: i32,
    pub updated_at: i32,
}
