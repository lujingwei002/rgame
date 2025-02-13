use mysql::*;
use mysql::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let url = config::get_server_config().gamedb.to_url();
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;
    Ok(())
}