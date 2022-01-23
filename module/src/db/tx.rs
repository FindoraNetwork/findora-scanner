use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Transaction {
    pub txid: String,
    pub value: Value,
    pub code: i64,
    pub log: String,
    pub events: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TransactionRef {
    pub txid: String,
    pub block_id: String,
    pub height: i64,
    pub from: String,
    pub to: String,
    pub asset: String,
    pub value: i64,
    pub op: String,
    pub status: String,
}
