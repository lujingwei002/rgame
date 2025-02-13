use std::net::SocketAddr;
use std::ops::Deref;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};
use bytes::Bytes;
use http_body_util::Full;
use std::convert::Infallible;
use hyper::{Request, Response};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

async fn hello(req: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let headers = req.headers();
    headers.iter().for_each(|(k, v)| {
       println!("{:?}:{:?}", k.as_str(), v.to_str().unwrap());
    });
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = config::get_server_config().bind.parse().unwrap();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}