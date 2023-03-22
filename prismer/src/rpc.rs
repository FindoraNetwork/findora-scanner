use crate::utils::bech32_encode;
use crate::{db, tx};
use crate::{Error, Result};
use chrono::NaiveDateTime;
use ethabi::{Event as EthEvent, EventParam, Hash, ParamType, RawLog};
use ethereum::LegacyTransaction;
use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx, JsonRpcResponse, TdRpcResult,
};
use module::schema::{DelegationInfo, TxResult};
use module::utils::crypto::recover_signer;
use reqwest::{Client, ClientBuilder, Url};
use rlp::{Encodable, RlpStream};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest;
use sqlx::PgPool;
use std::str::FromStr;
use std::string::ToString;
use std::time::Duration;

const DEPOSIT_ASSET: &str = "DepositAsset";
// DepositAsset(bytes32,bytes,uint256,uint8,uint256);
const DEPOSIT_ASSET_EVENT_HASH: &str =
    "0xaae31ca36c1ef3c9daa9d5efff8c47306109c0f7cf997e61d766ba15d27e071e";

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
pub struct TxCallLog {
    pub data: Vec<u8>,
    pub topics: Vec<String>,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct TxCall {
    pub logs: Vec<TxCallLog>,
    pub value: Value,
    pub used_gas: String,
    pub exit_reason: Value,
}

#[derive(Serialize, Deserialize)]
pub struct TxResultData {
    #[serde(rename = "Call")]
    pub call: Option<TxCall>,
}

#[derive(Serialize, Deserialize)]
pub struct Ethereum {
    #[serde(rename = "Ethereum")]
    pub ethereum: Transact,
}

#[derive(Serialize, Deserialize)]
pub struct Transact {
    #[serde(rename = "Transact")]
    pub transact: LegacyTransaction,
}

#[derive(Serialize, Deserialize)]
pub struct EvmTx {
    pub function: Ethereum,
}

impl Encodable for EvmTx {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.function.ethereum.transact.rlp_append(s)
    }
}

impl EvmTx {
    pub fn recover_signer(&self) -> Result<String> {
        let signer = recover_signer(&self.function.ethereum.transact).unwrap();
        Ok(format!("{:?}", signer))
    }
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
                if let tx::TxCatalog::EvmTx = tx::try_tx_catalog(&bytes) {
                    if let Some(call) = result_data.call {
                        let tx_value: Value = serde_json::from_slice(tx::unwrap(&bytes)?)?;
                        for log in call.logs {
                            for topic in log.topics {
                                if topic.eq(DEPOSIT_ASSET_EVENT_HASH) {
                                    let evm_tx: EvmTx =
                                        serde_json::from_value(tx_value.clone()).unwrap();
                                    let signer = evm_tx.recover_signer().unwrap();

                                    let params: Vec<EventParam> = vec![
                                        EventParam {
                                            name: "asset".to_string(),
                                            kind: ParamType::FixedBytes(32),
                                            indexed: false,
                                        },
                                        EventParam {
                                            name: "receiver".to_string(),
                                            kind: ParamType::Bytes,
                                            indexed: false,
                                        },
                                        EventParam {
                                            name: "amount".to_string(),
                                            kind: ParamType::Uint(256),
                                            indexed: false,
                                        },
                                        EventParam {
                                            name: "decimal".to_string(),
                                            kind: ParamType::Uint(8),
                                            indexed: false,
                                        },
                                        EventParam {
                                            name: "max_supply".to_string(),
                                            kind: ParamType::Uint(256),
                                            indexed: false,
                                        },
                                    ];

                                    let e = EthEvent {
                                        name: DEPOSIT_ASSET.to_string(),
                                        inputs: params,
                                        anonymous: false,
                                    };

                                    let h = Hash::from_str(DEPOSIT_ASSET_EVENT_HASH).unwrap();
                                    let raw_log = RawLog {
                                        topics: vec![h],
                                        data: log.data.clone(),
                                    };
                                    let log_res = e.parse_log(raw_log).unwrap();
                                    // asset code
                                    let asset_bytes =
                                        log_res.params[0].value.clone().into_fixed_bytes().unwrap();
                                    let asset =
                                        base64::encode_config(&asset_bytes, base64::URL_SAFE);

                                    // receiver address
                                    let receiver_bytes =
                                        log_res.params[1].value.clone().into_bytes().unwrap();
                                    let receiver = bech32_encode(&receiver_bytes);

                                    // amount
                                    let amount = log_res.params[2]
                                        .value
                                        .clone()
                                        .into_uint()
                                        .unwrap()
                                        .as_u128();

                                    res.push(TxResult {
                                        tx_hash: txid.clone(),
                                        block_hash: block_hash.clone(),
                                        sender: signer,
                                        receiver,
                                        asset,
                                        amount: amount.to_string(),
                                        height,
                                        timestamp: timestamp.timestamp(),
                                        value: result.clone(),
                                    });

                                    break;
                                }
                            }
                        }
                    }
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
