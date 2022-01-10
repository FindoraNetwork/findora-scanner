use serde::Deserialize;

use crate::Result;

use super::RPCResponse;

#[derive(Deserialize, Debug)]
pub struct PubKey {
    pub r#type: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Validator {
    pub address: String,
    pub pub_key: PubKey,
    pub voting_power: String,
    pub proposer_priority: String,
}

#[derive(Deserialize, Debug)]
pub struct ValidatorsRPC {
    pub validators: Vec<Validator>,
}

impl ValidatorsRPC {
    pub async fn load_height(url: &str, height: i64) -> Result<Self> {
        let url = format!("{}/validators?height={}", url, height);

        let r = reqwest::get(url)
            .await?
            .json::<RPCResponse<ValidatorsRPC>>()
            .await?;
        Ok(r.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<()> {
        let _ = ValidatorsRPC::load_height("https://prod-mainnet.prod.findora.org:26657", 1550668);
        Ok(())
    }
}
