use serde::Deserialize;

use crate::Result;

use super::RPCResponse;

#[derive(Deserialize, Debug)]
pub struct BlockId {
    pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockHeader {
    pub height: String,
    pub time: String,
    pub app_hash: String,
    pub proposer_address: String,
}

#[derive(Deserialize, Debug)]
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

impl BlockRPC {
    pub async fn load_height(url: String, height: i64) -> Result<Self> {
        let url = format!("{}/block?height={}", url, height);

        let r = reqwest::get(url)
            .await?
            .json::<RPCResponse<BlockRPC>>()
            .await?;
        Ok(r.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<()> {
        let _ = BlockRPC::load_height(
            String::from("https://prod-mainnet.prod.findora.org:26657"),
            1550667,
        )
        .await;
        Ok(())
    }
}
