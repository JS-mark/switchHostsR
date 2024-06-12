use super::super::models::logs::LogsList;
use crate::packages::public_type::{List, ResultData};
use crate::sqlite_db::models::logs::AddLog;
use crate::sqlite_db::schema::{logs, users};
use crate::sqlite_db::DbPool;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use logs::dsl::*;
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

    /**
     * 添加日志
     */
    pub fn add_log(&mut self, log: AddLog) -> Result<ResultData<usize>, String> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let res = insert_into(logs).values(&log).execute(conn);

        match res {
            Ok(result) => Ok(ResultData {
                code: 10000,
                data: Some(result), // 受影响的行数
                msg: "success".to_string(),
            }),
            Err(_) => Err("Failed to insert log".to_string()), // 简化了错误处理
        }
    }

    /**
     * 查询日志
     */
    pub fn get_all_logs(
        &mut self,
        page: i64,
        page_size: i64,
    ) -> QueryResult<ResultData<List<LogsList>>> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let total = logs
            .count()
            .filter(created_by.eq(self.user_id))
            .get_result::<i64>(conn)?;

        let res = logs::table
            .inner_join(users::table.on(logs::created_by.nullable().eq(users::id.nullable())))
            .select((
                logs::id,
                logs::log_type,
                logs::content,
                logs::created_at,
                logs::created_by,
                users::name,
                users::email,
                users::avatar_url,
            ))
            .filter(logs::created_by.nullable().eq(self.user_id))
            .order(logs::created_at.desc())
            .limit(page_size)
            .offset((page - 1) * page_size)
            .load::<LogsList>(conn);

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
