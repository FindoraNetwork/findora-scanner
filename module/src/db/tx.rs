use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub txid: String,
    pub value: serde_json::Value,
    pub code: i64,
    pub log: String,
    pub events: Vec<serde_json::Value>,
}
