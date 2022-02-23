use crate::db;
use crate::{rpc::TendermintRPC, tx, Error};
use chrono::NaiveDateTime;
use crossbeam::sync::WaitGroup;
use module::schema::{Block as ModuleBlock, Transaction, Validator};
use parking_lot::Mutex;
use reqwest::Url;
use sha2::Digest;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};

pub struct RangeScanner {
    inner: Arc<RangeScannerInner>,
    pool: PgPool,
}

struct RangeScannerInner {
    retries: usize,
    rpc: TendermintRPC,
}

impl RangeScanner {
    pub fn new(timeout: Duration, tendermint_rpc: Url, retries: usize, pool: PgPool) -> Self {
        let rpc = TendermintRPC::new(timeout, tendermint_rpc);
        RangeScanner {
            inner: Arc::new(RangeScannerInner { retries, rpc }),
            pool,
        }
    }

    ///scan block in [start..end), end is excluded.
    pub async fn range_scan(&self, start: i64, end: i64) -> Result<Vec<i64>, Error> {
        //Store the pull result of at the height.
        let succeed_height: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(vec![]));

        let wg = WaitGroup::new();
        for h in start..end {
            let _wg = wg.clone();
            let succeed_height = succeed_height.clone();
            let inner = self.inner.clone();
            let h = h as i64;
            let pool = self.pool.clone();
            tokio::spawn(async move {
                match inner.load_height_retried(h).await {
                    Ok(block) => {
                        tokio::spawn(async move {
                            match db::save(block, &pool).await {
                                Ok(_) => {
                                    succeed_height.lock().push(h);
                                }
                                Err(e) => error!("Database error: {:?}", e),
                            }
                            drop(_wg);
                        });
                    }
                    Err(e) => {
                        if let Error::NotFound = e {
                            info!("Block not found at height {}.", h);
                        }
                        drop(_wg);
                    }
                };
            });
        }
        tokio::task::spawn_blocking(move || wg.wait()).await?;
        let mut res = std::mem::take(&mut *succeed_height.lock());
        if res.is_empty() {
            return Err("No block is got.".into());
        }
        res.sort_unstable();
        db::save_last_height(*res.last().unwrap(), &self.pool).await?;
        Ok(res)
    }
}

impl RangeScannerInner {
    async fn load_height(&self, height: i64) -> Result<ModuleBlock, Error> {
        let block = self.rpc.load_block(height).await?;
        let validator_info = self.rpc.load_validators(height).await?;

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
            let tx = self.rpc.load_transaction(&txid).await?;

            match tx::try_tx_catalog(&bytes) {
                tx::TxCatalog::EvmTx => {
                    let value = serde_json::from_slice(tx::unwrap(&bytes)?)?;
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
                tx::TxCatalog::FindoraTx => {
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
                tx::TxCatalog::Unknown => {}
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

    pub async fn load_height_retried(&self, height: i64) -> Result<ModuleBlock, Error> {
        for i in 0..self.retries + 1 {
            match self.load_height(height).await {
                Ok(r) => return Ok(r),
                Err(Error::NotFound) => return Err(Error::NotFound),
                Err(e) => {
                    if i == self.retries {
                        return Err(e);
                    }
                    info!(
                        "Load height {} failed, error: `{:?}`\nRetry {} ...",
                        height,
                        e,
                        i + 1
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            };
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<(), Error> {
        //[TODO]
        // let _ = Block::load_height(
        //     String::from("https://prod-mainnet.prod.findora.org:26657"),
        //     1550667,
        // )
        // .await?;
        Ok(())
    }
}
