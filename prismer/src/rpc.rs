use std::time::Duration;

use crate::{Error, Result};
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sqlx::PgPool;

use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx, JsonRpcResponse, TdRpcResult,
};

use crate::{db, tx};

use module::schema::{DelegationInfo, TxResult};

use reqwest::{Client, ClientBuilder, Url};
use serde_json::Value;

pub struct TendermintRPC {
    pub rpc: Url,
    pub client: Client,
}

impl TendermintRPC {
    pub fn new(timeout: Duration, rpc: Url) -> Self {
        let client = ClientBuilder::new().timeout(timeout).build().unwrap();
        TendermintRPC { client, rpc }
    }

    pub async fn load_block(&self, height: i64) -> Result<ModuleBlockRPC> {
        let mut url = self.rpc.join("block").unwrap();
        url.set_query(Some(&format!("height={height}")));
        debug!("{}", url.as_str());
        let r: ModuleBlockRPC = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_transaction(&self, hash: &str) -> Result<ModuleTx> {
        let mut url = self.rpc.join("tx").unwrap();
        url.set_query(Some(&format!("hash=0x{hash}")));

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

#[derive(Serialize, Deserialize)]
pub struct TxResultData {
    #[serde(rename = "Call")]
    pub call: Value,
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

    pub async fn load_height(&self, height: i64) -> Result<Vec<TxResult>> {
        let block = self.rpc.load_block(height).await?;
        let block_hash = block.block_id.hash;
        let height = block.block.header.height.parse::<i64>()?;
        let timestamp =
            NaiveDateTime::parse_from_str(&block.block.header.time, "%Y-%m-%dT%H:%M:%S%.fZ")?;

        let mut res: Vec<TxResult> = vec![];

        for tx in block.block.data.txs.unwrap_or_default() {
            let bytes = base64::decode(&tx)?;
            let hasher = sha2::Sha256::digest(&bytes);
            let txid = hex::encode(hasher);
            let tx = self.rpc.load_transaction(&txid).await?;
            let result_data = tx.tx_result.data.clone();
            if let Value::String(s) = result_data {
                let bin_data = base64::decode(s)?;
                let result_data: TxResultData = serde_json::from_slice(&bin_data)?;
                let result: Value = serde_json::to_value(&result_data)?;
                match tx::try_tx_catalog(&bytes) {
                    tx::TxCatalog::EvmTx => {
                        res.push(TxResult {
                            tx_hash: txid,
                            block_hash: block_hash.clone(),
                            height,
                            timestamp: timestamp.timestamp(),
                            code: tx.tx_result.code,
                            ty: 1,
                            value: result,
                        });
                    }
                    tx::TxCatalog::FindoraTx => {
                        res.push(TxResult {
                            tx_hash: txid,
                            block_hash: block_hash.clone(),
                            height,
                            timestamp: timestamp.timestamp(),
                            code: tx.tx_result.code,
                            ty: 0,
                            value: result,
                        });
                    }
                    tx::TxCatalog::Unknown => {}
                }
            }
        }

        Ok(res)
    }

    pub async fn load_height_retried(&self, height: i64) -> Result<Vec<TxResult>> {
        for i in 0..self.retries + 1 {
            match self.load_height(height).await {
                Ok(r) => return Ok(r),
                Err(Error::NotFound) => {
                    return Err(Error::NotFound);
                }
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
}
