use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MyRequest {
    #[serde(rename = "equipmentSn")]
    pub equipment_sn: String,

    pub protocol: Option<Protocol>,
}

#[derive(Debug, Deserialize)]
pub struct Protocol {
    // pub name: String,
    // pub version: String,
}

#[derive(Debug, Serialize)]
pub struct MyResponse {
    pub status: i32,
    pub rows: i32,
    pub msg: String,
    pub timestamp: i64,
}

impl MyResponse {
    pub fn new(status: i32, msg: &str) -> Self {
        MyResponse {
            status,
            rows: 0,
            msg: msg.to_string(),
            timestamp: Utc::now().timestamp(),
        }
    }
}
