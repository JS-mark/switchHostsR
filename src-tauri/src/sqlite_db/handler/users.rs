use super::super::models::users::{EditUser, UserData};
use crate::packages::public_type::{List, ResultData};
use crate::sqlite_db::schema::users::dsl::*;
use crate::sqlite_db::DbPool;
use diesel::result::Error;
use diesel::{prelude::*, update};
pub struct Database {
    pub conn: DbPool,
    pub user_id: i32,
}

impl Database {
    pub fn new(conn: DbPool, user_id: i32) -> Result<Database, Error> {
        Ok(Database { conn, user_id })
    }

    pub fn update_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }

    pub fn verify_user(
        &mut self,
        user_email: &str,
        user_password: &str,
    ) -> Result<ResultData<UserData>, String> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let res = users
            .filter(email.eq(user_email))
            .filter(password.eq(user_password))
            .select((
                id,
                name,
                email,
                avatar_url,
                status,
                user_level,
                is_del,
                is_third,
                third_account_uid,
                created_at,
                updated_at,
            ))
            .first::<UserData>(conn);

        match res {
            Ok(result) => Ok(ResultData {
                code: 10000,
                data: Some(result), // 受影响的行数
                msg: "success".to_string(),
            }),
            Err(_) => Err("verify user error".to_string()), // 简化了错误处理
        }
    }

    pub fn edit_user(&mut self, user_data: EditUser) -> Result<ResultData<usize>, String> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let res = update(users)
            .filter(id.eq(self.user_id))
            .set(user_data)
            .execute(conn);

        match res {
            Ok(result) => Ok(ResultData {
                code: 10000,
                data: Some(result), // 受影响的行数
                msg: "success".to_string(),
            }),
            Err(_) => Err("Failed to update user".to_string()), // 简化了错误处理
        }
    }

    pub fn get_all_users(
        &mut self,
        page: i64,
        page_size: i64,
    ) -> QueryResult<ResultData<List<UserData>>> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let total = users.count().get_result::<i64>(conn)?;
        let res = users
            .select((
                id,
                name,
                email,
                avatar_url,
                status,
                user_level,
                is_del,
                is_third,
                third_account_uid,
                created_at,
                updated_at,
            ))
            .order(created_at.desc())
            .limit(page_size)
            .offset((page - 1) * page_size)
            .load::<UserData>(conn);

        match res {
            Ok(result) => Ok(ResultData {
                code: 10000,
                data: Some(List {
                    total: total as i32,
                    list: result,
                }),
                msg: "success".to_string(),
            }),
            Err(err) => Err(err), // 直接传递 Diesel 的错误
        }
    }
}
