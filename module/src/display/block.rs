use crate::rpc::block::{Block, BlockHeader, BlockId};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct DisplayBlock {
    pub block_hash: String,
    pub app_hash: String,
    pub proposer: String,
    pub num_txs: i64,
    pub block_size: i64,
    pub block_id: BlockId,
    pub block_header: BlockHeader,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct DisplayFullBlock {
    pub block_id: BlockId,
    pub block: Block,
}
