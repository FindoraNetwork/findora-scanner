use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct DisplayBlock {
    pub block_id: String,
    pub height: i64,
    pub time: i64,
    pub app_hash: String,
    pub proposer: String,
}