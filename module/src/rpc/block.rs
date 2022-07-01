use poem_openapi::Object;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct Parts {
    pub total: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct BlockId {
    pub hash: String,
    pub parts: Parts,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct Version {
    pub block: String,
    pub app: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct BlockHeader {
    pub version: Version,
    pub chain_id: String,
    pub height: String,
    pub time: String,
    pub last_block_id: BlockId,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_results_hash: String,
    pub evidence_hash: String,
    pub proposer_address: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct Signature {
    pub validator_address: Option<String>,
    pub timestamp: Option<String>,
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Object)]
pub struct LastCommit {
    pub height: String,
    pub round: String,
    pub block_id: BlockId,
    pub signatures: Option<Vec<Signature>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Object)]
pub struct Data {
    pub txs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Object)]
pub struct Block {
    pub header: BlockHeader,
    pub data: Data,
    pub last_commit: LastCommit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Object)]
pub struct BlockRPC {
    pub block_id: BlockId,
    pub block: Block,
}

#[derive(Deserialize, Debug, Object)]
pub struct BlockSizeRPC {
    pub last_height: String,
    pub block_metas: Option<Vec<BlockMeta>>,
}

#[derive(Deserialize, Debug, Object)]
pub struct BlockMeta {
    pub block_id: BlockId,
    pub block_size: String,
    pub header: BlockHeader,
    pub num_txs: String,
}
#[derive(Deserialize, Debug, Object)]
pub struct PubKey {
    pub value: String,
}
