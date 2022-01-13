use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxRef {
    pub txid: String,
    pub from: String,
    pub to: String,
    pub height: i64,
    pub timestamp: i64,
}
