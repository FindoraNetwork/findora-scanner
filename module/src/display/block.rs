use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct DisplayBlock {
    pub block_id: String,
    pub height: i64,
    pub time: i64,
    pub app_hash: String,
    pub proposer: String,
    pub tx_count: i64,
    pub size: i64,
}
