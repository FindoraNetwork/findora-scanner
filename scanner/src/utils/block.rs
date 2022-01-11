use crate::Result;

use super::RPCResponse;

use module::rpc::block::BlockRPC as module_block_rpc;

pub struct BlockRPC {}

impl BlockRPC {
    pub async fn load_height(url: String, height: i64) -> Result<module_block_rpc> {
        let url = format!("{}/block?height={}", url, height);

        let r = reqwest::get(url)
            .await?
            .json::<RPCResponse<module_block_rpc>>()
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
