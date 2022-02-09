use chrono::NaiveDateTime;
use sha2::Digest;

use module::db::block::Block as ModuleBlock;
use module::db::tx::Transaction;
use module::db::validator::Validator;

use crate::{utils, Result};

#[derive(Debug)]
pub struct Block {}

impl Block {
    pub async fn load_height(url: String, height: i64) -> Result<ModuleBlock> {
        let block =
            tokio::spawn(utils::block::BlockRPC::load_height(url.clone(), height)).await??;

        let validator_info = tokio::spawn(utils::validator::ValidatorsRPC::load_height(
            url.clone(),
            height,
        ))
        .await??;

        let block_id = block.block_id.hash;
        let height = block.block.header.height.parse::<i64>()?;
        let timestamp =
            NaiveDateTime::parse_from_str(&block.block.header.time, "%Y-%m-%dT%H:%M:%S%.fZ")?;
        let app_hash = block.block.header.app_hash;
        let proposer = block.block.header.proposer_address;
        let mut txs = Vec::new();
        let mut evm_txs = Vec::new();
        let mut validators = Vec::new();

        for tx in block.block.data.txs.unwrap_or_default() {
            let bytes = base64::decode(&tx)?;

            let hasher = sha2::Sha256::digest(&bytes);
            let txid = hex::encode(hasher);
            let tx = utils::tx::Transaction::load_height(&url, &txid).await?;

            match utils::tx::try_tx_catalog(&bytes) {
                utils::tx::TxCatalog::EvmTx => {
                    let value = serde_json::from_slice(utils::tx::unwrap(&bytes)?)?;
                    evm_txs.push(Transaction {
                        txid,
                        block_id: block_id.clone(),
                        ty: 1,
                        value,
                        code: tx.tx_result.code,
                        log: tx.tx_result.log,
                        events: tx.tx_result.events,
                    });
                }
                utils::tx::TxCatalog::FindoraTx => {
                    let value = serde_json::from_slice(&bytes)?;
                    txs.push(Transaction {
                        txid,
                        block_id: block_id.clone(),
                        ty: 0,
                        value,
                        code: tx.tx_result.code,
                        log: tx.tx_result.log,
                        events: tx.tx_result.events,
                    });
                }
                utils::tx::TxCatalog::Unknown => {}
            }
        }

        for vv in validator_info.validators {
            let address = vv.address;
            let power = vv.voting_power.parse::<u64>()?;
            let pub_key = vv.pub_key;
            let priority = vv.proposer_priority.parse::<i64>()?;
            if block.block.last_commit.signatures.is_none() {
                break;
            }
            let sign_info = block
                .block
                .last_commit
                .signatures
                .as_ref()
                .unwrap()
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
                address,
                power,
                pub_key,
                priority,
                signature,
                timestamp,
            };

            validators.push(validator);
        }

        Ok(ModuleBlock {
            block_id,
            height,
            size: 0,
            timestamp,
            app_hash,
            proposer,
            txs,
            evm_txs,
            validators,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<()> {
        let _ = Block::load_height(
            String::from("https://prod-mainnet.prod.findora.org:26657"),
            1550667,
        )
        .await?;
        Ok(())
    }
}
