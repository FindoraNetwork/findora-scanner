use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TxResult {
    pub code: i64,
    pub data: Value,
    pub log: String,
    pub info: String,
    #[serde(rename = "gasWanted")]
    pub gas_wanted: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub events: Vec<Value>,
    pub codespace: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transaction {
    pub hash: String,
    pub height: String,
    pub index: i64,
    pub tx_result: TxResult,
    pub tx: String,
}
