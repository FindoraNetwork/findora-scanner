use crate::Result;

use super::RPCResponse;

use module::rpc::validator::ValidatorsRPC as module_validators_rpc;

pub struct ValidatorsRPC {}

impl ValidatorsRPC {
    pub async fn load_height(url: String, height: i64) -> Result<module_validators_rpc> {
        let url = format!("{}/validators?height={}", url, height);

        let r = reqwest::get(url)
            .await?
            .json::<RPCResponse<module_validators_rpc>>()
            .await?;
        Ok(r.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<()> {
        let _ = ValidatorsRPC::load_height(
            String::from("https://prod-mainnet.prod.findora.org:26657"),
            1550667,
        )
        .await;
        Ok(())
    }
}
