use super::super::models::hosts::{EditHostData, HostData};
use crate::packages::public_type::{List, ResultData};
use crate::packages::utils::now_time;
use crate::sqlite_db::schema::hosts_data::dsl::*;
use crate::sqlite_db::DbPool;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::update;

pub struct Database {
    pub conn: DbPool,
    pub user_id: i32,
}

impl Database {
    pub fn new(conn: DbPool, user_id: i32) -> Result<Database, Error> {
        Ok(Database {
            conn: conn,
            user_id,
        })
    }

    pub fn update_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }

    pub fn update_host(
        &mut self,
        host_id: i32,
        host: EditHostData,
    ) -> Result<ResultData<()>, String> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");

        let _ = update(hosts_data.filter(id.eq(host_id)))
            .set((
                name.eq(host.name),
                content.eq(host.content),
                hosts_type.eq(host.hosts_type),
                hosts_path.eq(host.hosts_path),
                updated_at.eq(now_time().to_string()),
            ))
            .execute(conn);

        Ok(ResultData {
            code: 10000,
            data: Some(()),
            msg: "success".to_string(),
        })
    }

    pub fn get_all_hosts(
        &mut self,
        page: i64,
        page_size: i64,
    ) -> QueryResult<ResultData<List<HostData>>> {
        let conn = &mut self
            .conn
            .get()
            .expect("Failed to get db connection from pool");
        let total = hosts_data
            .count()
            .filter(created_by.eq(self.user_id))
            .get_result::<i64>(conn)?;

        let res = hosts_data
            .filter(created_by.eq(self.user_id))
            .order(created_at.desc())
            .limit(page_size)
            .offset((page - 1) * page_size)
            .load::<HostData>(conn);

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
