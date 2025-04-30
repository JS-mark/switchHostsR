use crate::db::schema::{host_groups, host_group_relations};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = host_groups)]
pub struct HostGroup {
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = host_group_relations)]
pub struct HostGroupRelation {
    pub id: Option<i32>,
    pub group_id: i32,
    pub host_id: i32,
    pub created_at: i64,
}
