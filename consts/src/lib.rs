pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use serde_derive::{Deserialize, Serialize};
use mysql_common::prelude::FromValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


#[derive(Debug, Copy, PartialEq, Eq, Clone,Default, Serialize, Deserialize,FromValue)]
#[mysql(rename_all = "snake_case", crate_name = "mysql_common")]
#[repr(u8)]
pub enum PlatformType {
    #[default]
    #[mysql(explicit_invalid)]
    None = 0,
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