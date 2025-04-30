use crate::db::schema::logs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = logs)]
pub struct Log {
    pub id: Option<i32>,
    pub user_id: i32,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<i32>,
    pub details: Option<String>,
    pub created_at: i64,
}
