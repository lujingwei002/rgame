use config::PlatformType;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use mysql::*;
    use mysql::prelude::*;
    use super::*;

    #[test]
    fn it_works() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let url = config::get_server_config().gamedb.to_url();
        let pool = Pool::new(url.as_str())?;
        let mut conn = pool.get_conn()?;
        let accounts = conn
            .query_map(
                "SELECT id, open_id,platform, channel from account limit 2",
                |(id, open_id, platform, channel)| {
                    Account { id, open_id, platform, channel }
                },
            )?;
        println!("accounts {:?}", accounts);
        Ok(())
    }
}
mod role;
pub use role::Role;

///
/// role表的加载，保存操作
///
impl Role {
    pub fn load() -> Role {
        Role::default()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Account {
    id: u64,
    open_id: String,
    platform: config::PlatformType,
    channel: u64,
}
