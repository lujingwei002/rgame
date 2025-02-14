use lazy_static::lazy_static;
use consts::*;
use mysql_common::prelude::*;
use mysql::prelude::*;
use mysql_common::chrono::{NaiveDateTime};
mod role;
mod tests;

pub use role::Role;

pub fn is_dup(err: &mysql::Error)-> bool {
    match err {
        mysql::Error::MySqlError(e) if e.code == mysql::ServerError::ER_DUP_ENTRY as u16=>{
            return true
        }
        _=>{}
    }
    false
}

trait RoleTable: Sized {
    fn load(conn:&mut mysql::PooledConn, user_id: i64) -> Result<Option<Self>, Box<dyn std::error::Error>>;
}

lazy_static!(
    static ref GAMEDB: mysql::Pool = {
        let url = configs::get_server_config().gamedb.to_url();
        let pool = mysql::Pool::new(url.as_str()).unwrap();
        pool
    };
);

// impl RoleTable for Account {
//     fn load(user_id: i64) -> Self {
//         Account::default()
//     }
// }

///
/// role表的加载，保存操作
///
impl Role {
    pub fn load() -> Role {
        Role::default()
    }
}

#[derive(Debug,Default, PartialEq, Eq, FromRow)]
#[mysql(table_name = "account")]
struct Account {
    id: u64,
    open_id: String,
    platform: PlatformType,
    channel: u64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Account {

    /// 从数据库读取数据
    fn get(conn:&mut mysql::PooledConn, open_id: &str, platform_type: PlatformType, channel: i64) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let r:Option<Account> = conn.exec_first(format!("select * from {} where open_id=? and platform=? and channel=?", Account::TABLE_NAME), (open_id, platform_type, channel))?;
        Ok(r)
    }

    /// 插入数据
    fn create(conn:&mut mysql::PooledConn, open_id: &str, platform_type: PlatformType, channel: i64) -> Result<u64, mysql::Error> {
        let now = chrono::Utc::now();
        let created_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
        conn.exec_drop(format!("insert into {} (open_id, platform, channel, created_at, updated_at) values (?, ?, ?, ?, ?)", Account::TABLE_NAME),
                       (open_id, platform_type, channel, created_at.as_str(), created_at.as_str()))?;
        Ok(conn.last_insert_id())
    }

    /// 删除数据
    fn delete(conn:&mut mysql::PooledConn, open_id: &str, platform_type: PlatformType, channel: i64) -> Result<u64, mysql::Error> {
        let now = chrono::Utc::now();
        conn.exec_drop(format!("delete from {} where open_id=? and platform=? and channel=?", Account::TABLE_NAME),
                       (open_id, platform_type, channel))?;
        Ok(conn.last_insert_id())
    }

}


