use crate::db::tx::Transaction;
use crate::db::validator::Validator;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_id: String,
    pub height: i64,
    pub timestamp: NaiveDateTime,
    pub app_hash: String,
    pub proposer: String,
    pub txs: Vec<Transaction>,
    pub evm_txs: Vec<Transaction>,
    pub validators: Vec<Validator>,
}
