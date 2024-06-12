use diesel::{Insertable, Queryable};

use crate::sqlite_db::schema::third_account;

#[derive(Queryable, Debug)]
pub struct ThirdAccount {
    pub id: i32,
    pub uid: String,
    pub user_id: Option<i32>, // 注意这是一个 Option 类型
    pub account: String,
    pub email: String,
    pub avatar_url: String,
    pub nickname: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[table_name = "third_account"]
pub struct AddThirdAccount {
    pub uid:  String,
    pub user_id: Option<i32>, // 注意这是一个 Option 类型
    pub account: String,
    pub email: String,
    pub avatar_url: String,
    pub nickname: String,
    pub created_at: String,
    pub updated_at: String,
}
