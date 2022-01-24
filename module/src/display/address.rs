use crate::db::tx::TransactionRef;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct DisplayAddress {
    pub txs: Vec<TransactionRef>,
    pub total: usize,
}
