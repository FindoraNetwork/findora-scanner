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
pub struct TxDetail {
    pub hash: String,
    pub timestamp: i64,
    pub height: i64,
    pub index: i64,
    pub tx_result: Value,
    pub tx: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxMeta {
    pub code: i64,
    pub data: String,
    pub log: String,
    pub info: String,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub events: Vec<Value>,
    pub code_space: String,
}
