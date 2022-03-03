use std::time::Duration;

use crate::{Error, Result};
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use sha2::Digest;
use sqlx::PgPool;

use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx,
    validator::ValidatorsRPC as ModuleValidatorsRPC, JsonRpcResponse, TdRpcResult,
};

use crate::{db, tx};

use module::schema::{Block as ModuleBlock, DelegationInfo, Transaction, Validator};

use reqwest::{Client, ClientBuilder, Url};

pub struct TendermintRPC {
    rpc: Url,
    client: Client,
}

impl TendermintRPC {
    pub fn new(timeout: Duration, rpc: Url) -> Self {
        let client = ClientBuilder::new().timeout(timeout).build().unwrap();
        TendermintRPC { client, rpc }
    }

    pub async fn load_block(&self, height: i64) -> Result<ModuleBlockRPC> {
        let mut url = self.rpc.join("block").unwrap();
        url.set_query(Some(&format!("height={}", height)));
        debug!("{}", url.as_str());
        let r: ModuleBlockRPC = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_validators(&self, height: i64) -> Result<ModuleValidatorsRPC> {
        let mut url = self.rpc.join("validators").unwrap();
        url.set_query(Some(&format!("height={}", height)));

        let r: ModuleValidatorsRPC = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_transaction(&self, hash: &str) -> Result<ModuleTx> {
        let mut url = self.rpc.join("tx").unwrap();
        url.set_query(Some(&format!("hash=0x{}", hash)));

        let r: ModuleTx = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_delegations(&self) -> Result<(i64, DelegationInfo)> {
        let mut url = self.rpc.join("abci_query").unwrap();
        let mut queries = url.query_pairs_mut();
        queries.append_pair("path", "\"/delegations\"");
        queries.append_pair("data", "");
        drop(queries);

        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(resp.text().await?.into());
        }

        let result: JsonRpcResponse<TdRpcResult> = resp.json().await?;

        let response = result.result.response;

        if response.code != 0 {
            return Err(response.info.into());
        }

        let h = response.height.parse()?;

        //let data = response.info.replace("\\\"", "\"");

        let staking: DelegationInfo = serde_json::from_str(&response.info)?;

        Ok((h, staking))
    }

    async fn client_get<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(resp.text().await?.into());
        }
        let bytes = resp.bytes().await?;
        if let Ok(r) = serde_json::from_slice::<'_, JsonRpcResponse<T>>(&bytes) {
            Ok(r.result)
        } else {
            debug!("{}", String::from_utf8_lossy(&bytes));
            Err(Error::NotFound)
        }
    }
}
pub struct RPCCaller {
    pub(crate) retries: usize,
    pub(crate) concurrency: usize,
    pub(crate) rpc: TendermintRPC,
    pub(crate) pool: PgPool,
}

impl RPCCaller {
    pub fn new(
        retries: usize,
        concurrency: usize,
        timeout: Duration,
        tendermint_rpc: Url,
        pool: PgPool,
    ) -> Self {
        let rpc = TendermintRPC::new(timeout, tendermint_rpc);
        RPCCaller {
            retries,
            concurrency,
            rpc,
            pool,
        }
    }

    pub async fn load_height(&self, height: i64) -> Result<ModuleBlock> {
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

    pub async fn load_height_retried(&self, height: i64) -> Result<ModuleBlock> {
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

    pub async fn load_and_save_block(&self, target: i64) -> Result<()> {
        let block = self.load_height_retried(target).await?;
        db::save(block, &self.pool).await?;
        db::save_last_height(target, &self.pool).await?;
        Ok(())
    }

    pub async fn load_and_save_staking(&self) -> Result<i64> {
        let (h, info) = self.rpc.load_delegations().await?;
        db::save_delegations(h, &info, &self.pool).await?;
        Ok(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc() -> Result<()> {
        let rpc = TendermintRPC::new(
            Duration::from_secs(10),
            "https://prod-mainnet.prod.findora.org:26657"
                .to_string()
                .parse()
                .unwrap(),
        );
        let _ = rpc.load_block(1550667).await?;
        let _ = rpc.load_validators(1550667).await?;
        let _ = rpc
            .load_transaction("c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0")
            .await?;
        Ok(())
    }
}
