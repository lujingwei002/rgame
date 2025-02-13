use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone,Default, Serialize, Deserialize)]
pub enum PlatformType {
    #[default]
    test,
    quick,
}

impl TryFrom<&str> for PlatformType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "test" => Ok(PlatformType::test),
            "quick" => Ok(PlatformType::quick),
            _ => Err("Unknown platform type"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConf {
    pub id: i64,
    pub platform: PlatformType,
}

