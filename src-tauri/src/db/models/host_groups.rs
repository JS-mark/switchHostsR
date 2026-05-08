use crate::db::schema::{host_group_relations, host_groups};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = host_groups)]
pub struct HostGroup {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = host_groups)]
pub struct NewHostGroup {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = host_group_relations)]
pub struct HostGroupRelation {
    pub id: i32,
    pub group_id: i32,
    pub host_id: i32,
    pub created_at: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = host_group_relations)]
pub struct NewHostGroupRelation {
    pub group_id: i32,
    pub host_id: i32,
    pub created_at: i32,
}
