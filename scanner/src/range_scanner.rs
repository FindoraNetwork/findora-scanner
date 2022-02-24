use crate::{db, rpc::TendermintRPC, tx, Error};
use chrono::NaiveDateTime;
use crossbeam::channel::bounded;
use module::schema::{Block as ModuleBlock, Transaction, Validator};
use reqwest::Url;
use sha2::Digest;
use sqlx::PgPool;
use std::sync::atomic::{AtomicI64, Ordering};
use std::{sync::Arc, time::Duration};

pub struct RangeScanner {
    caller: Arc<RPCCaller>,
    pool: PgPool,
}

pub struct RPCCaller {
    retries: usize,
    concurrency: usize,
    rpc: TendermintRPC,
}

impl RangeScanner {
    pub fn new(
        timeout: Duration,
        tendermint_rpc: Url,
        retries: usize,
        concurrency: usize,
        pool: PgPool,
    ) -> Self {
        let rpc = TendermintRPC::new(timeout, tendermint_rpc);
        RangeScanner {
            caller: Arc::new(RPCCaller {
                retries,
                concurrency,
                rpc,
            }),
            pool,
        }
    }

    ///scan block in [start..end].
    pub async fn range_scan(&self, start: i64, end: i64) -> Result<(), Error> {
        let concurrency = self.caller.concurrency; //how many spawned.

        let (sender, rev) = bounded(concurrency);

        //Store the max height.
        let last_height = Arc::new(AtomicI64::new(0));

        let inner_p = self.caller.clone();
        let pool_p = self.pool.clone();
        let last_height_p = last_height.clone();

        //start producer.
        let handle_producer = tokio::task::spawn_blocking(move || {
            for h in start..end {
                let fut = task(inner_p.clone(), h, pool_p.clone(), last_height_p.clone());
                //build a future that have not been executed.
                sender.send(Some(fut)).unwrap();
            }

            //make them exit.
            for _ in 0..concurrency {
                sender.send(None).unwrap();
            }
        });

        //spawn consumers.
        let spawned_handles: Vec<_> = (0..concurrency)
            .map(move |_| {
                let rev_cloned = rev.clone();
                tokio::spawn(async move {
                    while let Ok(Some(fut)) = rev_cloned.recv() {
                        //handle error within.
                        let _: () = fut.await;
                    }
                })
            })
            .collect();

        for h in spawned_handles {
            h.await?;
        }
        handle_producer.await?;
        Ok(())
    }
}

async fn task(caller: Arc<RPCCaller>, h: i64, pool: PgPool, last_height: Arc<AtomicI64>) {
    match caller.load_height_retried(h).await {
        Ok(block) => match db::save(block, &pool).await {
            Ok(_) => {
                let h_old = last_height.load(Ordering::Acquire);
                if h > h_old {
                    last_height.store(h, Ordering::Release);
                    //write the last height to database.
                    if let Err(e) = db::save_last_height(h, &pool).await {
                        error!("Database error: {:?}", e);
                    }
                }
                debug!("Height at {} succeed.", h);
            }
            Err(e) => error!("Database error: {:?}", e),
        },
        Err(e) => {
            if let Error::NotFound = e {
                info!("Block not found at height {}.", h);
            } else {
                error!("Load height error: {:?}", e);
            }
        }
    };
}

impl RPCCaller {
    pub fn new(retries: usize, concurrency: usize, timeout: Duration, tendermint_rpc: Url) -> Self {
        let rpc = TendermintRPC::new(timeout, tendermint_rpc);
        RPCCaller {
            retries,
            concurrency,
            rpc,
        }
    }

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
