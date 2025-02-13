
use lazy_static::lazy_static;
use tokio::io::{self, AsyncBufReadExt};
use tokio::io::BufReader;

struct Server {

}

impl Server {
    fn new() -> Self {
        Server {}
    }
}

lazy_static! {
    static ref SERVER: Server = Server::new();
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    hall_service::main().await;


    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    println!("请输入一些文字：");

    // 使用 next_line 方法异步读取一行输入
    if let Some(line) = reader.next_line().await.expect("读取失败") {
        println!("你输入了: {}", line);
    } else {
        println!("没有输入或读取错误");
    }
    Ok(())
}
