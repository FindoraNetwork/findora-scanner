use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TxResult {
    pub code: i64,
    pub log: String,
    pub events: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
    pub tx_result: TxResult,
}
