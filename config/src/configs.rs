use serde_derive::{Deserialize, Serialize};
use mysql_common::prelude::FromValue;


#[derive(Debug, PartialEq, Eq, Clone,Default, Serialize, Deserialize,FromValue)]
#[mysql(rename_all = "snake_case", crate_name = "mysql_common")]
#[repr(u8)]
pub enum PlatformType {
    #[default]
    Test = 1,
    Quick,
}

impl TryFrom<&str> for PlatformType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "test" => Ok(PlatformType::Test),
            "quick" => Ok(PlatformType::Quick),
            _ => Err("Unknown platform type"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConf {
    pub id: i64,
    pub platform: PlatformType,
}

