use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockId {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockHeader {
    pub height: String,
    pub time: String,
    pub app_hash: String,
    pub proposer_address: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Signature {
    pub validator_address: Option<String>,
    pub timestamp: Option<String>,
    pub signature: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LastCommit {
    pub signatures: Option<Vec<Signature>>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub txs: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub data: Data,
    pub last_commit: LastCommit,
}

#[derive(Deserialize, Debug)]
pub struct BlockRPC {
    pub block_id: BlockId,
    pub block: Block,
}

#[derive(Deserialize, Debug)]
pub struct BlockSizeRPC {
    pub last_height: String,
    pub block_metas: Option<Vec<BlockMeta>>,
}

#[derive(Deserialize, Debug)]
pub struct BlockMeta {
    pub block_id: BlockId,
    pub block_size: String,
    pub header: BlockHeader,
    pub num_txs: String,
}
