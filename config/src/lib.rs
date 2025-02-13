mod configs;

use std::{env, fs};
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
use mysql::*;
use mysql::prelude::*;
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
        let mut c = EnvConf::default();
        // 读取CONFIG环境变量
        match env::var(EnvName::CONFIG.as_str()) {
            Ok(s) => {
                // TODO 判断绝对路径和相对路径
                let path = Path::new(&s);
                File::open(path).expect(format!("Unable to open config file: {:?}", path).as_str());
                c.config = s
            }
            _=>{
                // 向上查找
                let mut current_dir = env::current_dir();
                let mut current_dir = current_dir.as_ref().unwrap().as_path();
                let mut p = current_dir.join(c.config.as_str());
                while !fs::exists(p.as_path()).unwrap() {
                    current_dir = current_dir.parent().expect(format!("Unable to search config file: {}", c.config.as_str()).as_str());
                    p = current_dir.join(c.config.as_str());
                }
                c.config = p.to_str().unwrap().to_string();
                env::set_current_dir(current_dir).expect(format!("Unable to change config file location: {}", c.config.as_str()).as_str());
            }
        }
        println!("current dir: {:?}", env::current_dir().unwrap().deref());
        println!("config file: {:?}", c.config.as_str());
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
    pub gamedb: DbConfig
}

pub fn get_server_config() -> &'static ServerConfig {
    return Config.deref();
}

#[derive(Default, Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub name: String
}

impl DbConfig {
    pub fn to_url(&self)->String {
        format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.name)
    }
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
