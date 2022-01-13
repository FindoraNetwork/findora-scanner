use serde::{Deserialize, Serialize};
use poem_openapi::Object;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct BlockMeta {
    pub block_id: Value,
    pub block_size: i64,
    pub header: Value,
    pub num_txs: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub version: Value,
    pub chain_id: String,
    pub height: i64,
    pub time: i64,
    pub last_block_id: Value,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validator_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_result_hash: String,
    pub evidence_hash: String,
    pub proposer_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub block: i64,
    pub app: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockID {
    pub hash: String,
    pub parts: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parts {
    pub total: i64,
    pub hash: String,
}


