use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserV01 {
    pub version: u8,
    pub username: String,
    pub password: String,
    pub role: String,
    pub unique_identifier: String
}
