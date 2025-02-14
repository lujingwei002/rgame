use serde_derive::{Deserialize, Serialize};

use consts::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConf {
    pub id: i64,
    pub platform: PlatformType,
}

