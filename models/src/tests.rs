

#[cfg(test)]
mod tests {
    use mysql::*;
    use crate::*;

    #[test]
    fn it_works() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let url = configs::get_server_config().gamedb.to_url();
        let pool = Pool::new(url.as_str())?;
        let mut conn = pool.get_conn()?;
        let account = Account::get(&mut conn, "111", PlatformType::Test, 2);
        println!("accounts {:?}", account);
        Ok(())
    }

    #[test]
    fn create_account() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut conn = GAMEDB.get_conn()?;
        let result = Account::create(&mut conn, "11", PlatformType::Test, 1000);
        if result.is_err() {
            if is_dup(result.as_ref().unwrap_err()) {
                println!("dup");
                Account::delete(&mut conn, "11", PlatformType::Test, 1000).unwrap();
            } else {
                result.unwrap();
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}