use crate::rpc::validator::PubKey;
use chrono::NaiveDateTime;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_id: String,
    pub height: i64,
    pub size: i64,
    pub timestamp: NaiveDateTime,
    pub app_hash: String,
    pub proposer: String,
    pub txs: Vec<Transaction>,
    pub evm_txs: Vec<Transaction>,
    pub validators: Vec<Validator>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockRef {
    pub block_id: String,
    pub height: i64,
    pub txid: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Transaction {
    pub txid: String,
    pub block_id: String,
    pub ty: i32,
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
    pub from_address: String,
    pub to_address: String,
    pub asset: String,
    pub value: i64,
    pub typ: String,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Validator {
    pub address: String,
    pub power: u64,
    pub pub_key: PubKey,
    pub priority: i64,
    pub signature: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
}

pub struct LastHeight {
    pub tip: String,
    pub height: i64,
}
