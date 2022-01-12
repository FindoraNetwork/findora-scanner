use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Transaction {
    pub txid: String,
    pub value: serde_json::Value,
    pub code: i64,
    pub log: String,
    pub events: Vec<serde_json::Value>,
}
