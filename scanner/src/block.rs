use chrono::NaiveDateTime;

use crate::{utils, Result};

#[derive(Debug)]
pub struct Validator {
    pub address: String,
    pub power: u64,
    pub pub_key: utils::validator::PubKey,
    pub priority: i64,
    pub signature: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct Block {
    pub block_id: String,
    pub height: i64,
    pub timestamp: NaiveDateTime,
    pub app_hash: String,
    pub proposer: String,
    pub txs: Vec<serde_json::Value>,
    pub validators: Vec<Validator>,
}

impl Block {
    pub async fn load_height(url: &str, height: i64) -> Result<Self> {
        let block = utils::block::BlockRPC::load_height(url, height).await?;
        let validator_info = utils::validator::ValidatorsRPC::load_height(url, height).await?;

        let block_id = block.block_id.hash;
        let height = i64::from_str_radix(&block.block.header.height, 10)?;
        let timestamp =
            NaiveDateTime::parse_from_str(&block.block.header.time, "%Y-%m-%dT%H:%M:%S%.fZ")?;
        let app_hash = block.block.header.app_hash;
        let proposer = block.block.header.proposer_address;
        let mut txs = Vec::new();
        let mut validators = Vec::new();

        for tx in block.block.data.txs {
            let bytes = base64::decode(&tx)?;
        }

        for vv in validator_info.validators {
            let address = vv.address;
            let power = u64::from_str_radix(&vv.voting_power, 10)?;
            let pub_key = vv.pub_key;
            let priority = i64::from_str_radix(&vv.proposer_priority, 10)?;
            let sign_info = block
                .block
                .last_commit
                .signatures
                .iter()
                .find(|v| Some(&address) == v.validator_address.as_ref());

            let (signature, timestamp) = if let Some(s) = sign_info {
                let signature = s.signature.clone();
                let timestamp = if let Some(s) = &s.timestamp {
                    Some(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")?)
                } else {
                    None
                };

                (signature, timestamp)
            } else {
                (None, None)
            };

            let validator = Validator {
                address, power, pub_key, priority, signature, timestamp
            };

            validators.push(validator);
        }

        Ok(Self {
            block_id,
            height,
            timestamp,
            app_hash,
            proposer,
            txs,
            validators,
        })
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
