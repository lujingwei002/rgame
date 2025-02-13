mod configs;

use std::env;
use std::env::VarError;
use std::error::Error;
use lazy_static::lazy_static;
use std::fs::{File};
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::format;

///
/// # 环境变量配置
/// |变量名|说明|
/// |-|-|
/// |CONFIG|配置文件路径|
/// |BIND|绑定地址|
///
///
///
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        println!("Hello, world {:?}", env::current_dir().unwrap().deref());
        println!("Env Config {:?}", Env.deref());
        println!("Server Config {:?}", Config.deref());
    }
}

enum EnvName {
    CONFIG,
    BIND,
}

impl EnvName {
    fn as_str(&self) -> &'static str {
        match self {
            EnvName::CONFIG => "CONFIG",
            EnvName::BIND => "BIND",
        }
    }
}

lazy_static!(
    pub static ref Env: EnvConf = EnvConf::init();
    static ref Config: ServerConfig = ServerConfig::init();
);


/// 初始化
pub fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}

///
/// 环境变量配置
///
#[derive(Debug)]
pub struct EnvConf {
    pub config: String,
    pub bind: String
}

impl EnvConf {
    fn init()-> Self {
        println!("current dir: {:?}", env::current_dir().unwrap().deref());
        let mut c = EnvConf::default();
        match env::var(EnvName::CONFIG.as_str()) {
            Ok(s) => { c.config = s }
            _=>{}
        }
        c
    }
}

impl Default for EnvConf {
    fn default() -> Self {
        Self {
            config: "server.toml".to_string(),
            bind: "127.0.0.1:9999".to_string()
        }
    }
}

///
/// 服务器配置
///
#[derive(Default, Debug, Deserialize)]
pub struct ServerConfig {
    pub config: PathBuf,
    pub host: String,
    pub bind: String,
    pub game_db: DbConfig
}

pub fn get_server_config() -> &'static ServerConfig {
    return Config.deref();
}

#[derive(Default, Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
}

impl ServerConfig {
    fn init() -> Self {
        let mut f = File::open(Env.config.as_str()).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        let mut c:ServerConfig = toml::from_str(&content).unwrap();
        if !Env.bind.is_empty() {
            c.bind = Env.deref().bind.clone()
        }
        c
    }
}


#[macro_export]
macro_rules! config_impl {
     ($($SelfT:ident, $Path:literal, $CollectionT:ident, $KeyT:ty, $Key:ident),*) => {
         pub fn load<P: AsRef<Path>>(dir: P) {

         }

         pub fn preload() {
         $(
            format!("preload {}", $CollectionT.len());
         )*
         }

         $(
            impl $SelfT {
                fn load() -> HashMap<$KeyT, $SelfT> {
                    let path: PathBuf = Config.config.join($Path);
                    let mut file = File::open(path.as_path()).expect(format!("Error opening config file: {:?}", path.as_path()).as_str());
                    let mut content = String::new();
                    file.read_to_string(&mut content).expect(format!("Error reading file: {:?}", path.as_path()).as_str());
                    let arr: Vec<$SelfT> = serde_json::from_str(content.as_str()).expect(format!("Error deserializing file: {:?}", path.as_path()).as_str());
                    let mut map = HashMap::new();
                    arr.into_iter().for_each(|v| {
                        map.insert(v.$Key, v);
                    });
                    println!("load {} from {:?}, len:{}", stringify!($SelfT), path.as_path(), map.len());
                    map
                }

                pub fn get() -> &'static HashMap<$KeyT, $SelfT> {
                    return $CollectionT.deref();
                }


            }
         )*
          $(
            lazy_static! {
                static ref $CollectionT: HashMap<$KeyT, $SelfT> = $SelfT::load();
            }
         )*
    };
}

pub use configs::*;

config_impl!(
    ChannelConf, "channel.json", _ChannelConf, i64, id
);
