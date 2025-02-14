use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};
use bytes::Bytes;
use http_body_util::{Full, BodyExt};
use std::error::Error;
use hyper::{http, Method, Request, Response, StatusCode};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use hyper::body::Body;
use consts::*;
use configs::*;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

static NOTFOUND: &[u8] = b"Not Found";
static MISSING: &[u8] = b"Missing field";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn response_text(msg: &'static str) -> Response<BoxBody> {
    Response::builder().header(http::header::CONTENT_TYPE,"text/plain; charset=UTF-8").status(StatusCode::UNPROCESSABLE_ENTITY).body(full(msg)).unwrap()
}

#[derive(Debug,Default, Deserialize, Serialize)]
struct LoginRequest {
    open_id: String, // 账号
    token: String, // token
    platform: PlatformType,//平台
    channel: i64, // 渠道
}

#[derive(Debug,Default, Deserialize, Serialize)]
struct LoginResponse {
    msg: String,
    platform: PlatformType,
}

async fn api_login(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody>, Box<dyn Error + Send + Sync>> {
    let query = if let Some(q) = req.uri().query() {
        q
    } else {
        return Ok(Response::builder()
            .status(StatusCode::UNPROCESSABLE_ENTITY)
            .body(full(MISSING))
            .unwrap());
    };
    println!("{:?}", LANG.MISSING_OPEN_ID_FIELD());
    let mut req = LoginRequest::default();
    let params = form_urlencoded::parse(query.as_ref()).into_owned().collect::<HashMap<String, String>>();
    req.open_id = if let Some(n) = params.get("open_id") {
        n.clone()
    } else {
        return Ok(response_text(LANG.MISSING_OPEN_ID_FIELD()));
    };
    let platform = if let Some(p) = params.get("platform") {
        p.clone()
    } else {
        return Ok(response_text(LANG.MISSING_PLATFORM_FIELD()));
    };
    req.platform = match PlatformType::try_from(platform.as_str()) {
        Ok(p) => p,
        Err(e) => {
            return Ok(Response::builder().status(StatusCode::UNPROCESSABLE_ENTITY).body(full(e)).unwrap());
        }
    };
    let channel = if let Some(p) = params.get("channel") {
        p.clone()
    } else {
        return Ok(response_text(LANG.MISSING_CHANNEL_FIELD()));
    };
    req.channel = match channel.parse::<i64>()  {
        Ok(p) => p,
        Err(e) => {
            return Ok(Response::builder().status(StatusCode::UNPROCESSABLE_ENTITY).body(full(e.to_string())).unwrap());
        }
    };

    let mut resp = LoginResponse::default();
    resp.msg = req.open_id.clone();
    resp.platform = req.platform;
    let json = serde_json::to_string(&resp)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(full(json))
        .unwrap())
}

async fn handler(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody>, Box<dyn Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/login") => {
            api_login(req).await
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(NOTFOUND))
                .unwrap())
        }
    }
    // let headers = req.headers();
    // headers.iter().for_each(|(k, v)| {
    //     println!("{:?}:{:?}", k.as_str(), v.to_str().unwrap());
    // });
    // Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = configs::get_server_config().bind.parse().unwrap();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, service_fn(handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}