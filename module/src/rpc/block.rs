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
    pub signatures: Vec<Signature>,
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
