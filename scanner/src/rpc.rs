use crate::commands::FRA_ASSET;
use crate::types::{
    ClaimOpt, ConvertAccountOperation, DefineAssetOpt, DelegationOpt, FindoraEVMTx, IssueAssetOpt,
    TransferAssetOpt, TxValue, UnDelegationOpt, XHubOpt,
};
use crate::util::pubkey_to_fra_address;
use crate::{db, tx};
use crate::{Error, Result};
use base64::URL_SAFE;
use chrono::NaiveDateTime;
use ethereum::TransactionAction;
use ethereum_types::H256;
use module::rpc::block::BlockSizeRPC;
use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx,
    validator::ValidatorsRPC as ModuleValidatorsRPC, JsonRpcResponse, TdRpcResult,
};
use module::schema::{
    Block as ModuleBlock, DelegationInfo, Transaction, V2ClaimTx, V2ConvertAccountTx,
    V2DefineAssetTx, V2DelegationTx, V2EvmTx, V2IssueAssetTx, V2NativeTransfer, V2UndelegationTx,
    Validator, XHubTx,
};
use module::utils::crypto::recover_signer;
use reqwest::{Client, ClientBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::Digest;
use sha3::Keccak256;
use sqlx::PgPool;
use std::time::Duration;

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

    pub async fn get_block_size(&self, height: i64) -> Result<BlockSizeRPC> {
        let mut url = self.rpc.join("blockchain").unwrap();
        url.set_query(Some(&format!("minHeight={height}&maxHeight={height}")));
        debug!("{}", url.as_str());
        let r: BlockSizeRPC = self.client_get(url).await?;
        Ok(r)
    }

    fn validator_url(&self, height: i64, page: i32, per_page: i32) -> Result<Url> {
        let mut url = self.rpc.join("validators").unwrap();
        url.set_query(Some(&format!(
            "height={height}&per_page={per_page}&page={page}"
        )));
        Ok(url)
    }

    pub async fn load_validators(&self, height: i64) -> Result<ModuleValidatorsRPC> {
        let mut page = 1;
        let per_page = 100;

        let url = self.validator_url(height, page, per_page).unwrap();
        let mut r: ModuleValidatorsRPC = self.client_get(url).await?;
        let mut count = r.count.clone().parse::<i32>().unwrap();
        let mut total = r.total.clone().parse::<i32>().unwrap();

        while total > count {
            total -= count;
            page += 1;
            let tmp_url = self.validator_url(height, page, per_page).unwrap();
            let mut tmp_res: ModuleValidatorsRPC = self.client_get(tmp_url).await?;
            count = tmp_res.count.clone().parse::<i32>().unwrap();
            r.validators.append(&mut tmp_res.validators)
        }

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
        let block_data = serde_json::to_value(block.clone()).unwrap();
        let block_size_rpc = self.rpc.get_block_size(height).await?;
        let block_size = block_size_rpc.block_metas.unwrap().as_slice()[0]
            .block_size
            .parse::<i64>()
            .unwrap();
        let validator_info = self.rpc.load_validators(height).await?;

        let block_hash = block.block_id.hash;
        let height = block.block.header.height.parse::<i64>()?;
        let timestamp =
            NaiveDateTime::parse_from_str(&block.block.header.time, "%Y-%m-%dT%H:%M:%S%.fZ")?;
        let app_hash = block.block.header.app_hash;
        let proposer = block.block.header.proposer_address;
        let mut txs = Vec::new();
        let mut evm_txs = Vec::new();
        let mut validators = Vec::new();
        let mut xhub_txs = Vec::new();
        let mut v2_evm_txs = Vec::new();
        let mut v2_convert_account_txs = Vec::new();
        let mut v2_undelegation_txs = Vec::new();
        let mut v2_delegation_txs = Vec::new();
        let mut v2_claim_txs = Vec::new();
        let mut v2_define_asset_txs = Vec::new();
        let mut v2_issue_asset_txs = Vec::new();
        let mut v2_native_transfer_txs = Vec::new();

        for tx in block.block.data.txs.unwrap_or_default() {
            let bytes = base64::decode(&tx)?;
            let origin = tx;
            let hasher = sha2::Sha256::digest(&bytes);
            let txid = hex::encode(hasher);
            let tx = self.rpc.load_transaction(&txid).await?;
            let result = serde_json::to_value(tx.tx_result.clone()).unwrap();
            match tx::try_tx_catalog(&bytes) {
                tx::TxCatalog::EvmTx => {
                    let value: Value = serde_json::from_slice(tx::unwrap(&bytes)?)?;
                    let v: Value = value.clone();
                    evm_txs.push(Transaction {
                        tx_hash: txid.clone(),
                        block_hash: block_hash.clone(),
                        height,
                        timestamp: timestamp.timestamp(),
                        code: tx.tx_result.code,
                        ty: 1,
                        log: tx.tx_result.log,
                        origin,
                        result,
                        value,
                    });
                    ////////////////////////////////////////////////////////////////////////////////
                    // v2: parse evm txs
                    ////////////////////////////////////////////////////////////////////////////////
                    let evm_tx_str = serde_json::to_string(&v).unwrap();
                    if evm_tx_str.contains("XHub") {
                        // XHub
                        let xhub_opt: XHubOpt = serde_json::from_value(v).unwrap();
                        for o in &xhub_opt.function.xhub.nonconfidential_transfer.outputs {
                            let asset = base64::encode_config(o.asset, base64::URL_SAFE);
                            let receiver = pubkey_to_fra_address(&o.target).unwrap();
                            let content = serde_json::from_str(&evm_tx_str).unwrap();
                            xhub_txs.push(XHubTx {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                sender: "".to_string(),
                                receiver,
                                asset,
                                amount: o.amount,
                                decimal: 6,
                                height,
                                timestamp: timestamp.timestamp(),
                                content,
                            })
                        }
                    } else {
                        // Ethereum
                        let evm_tx: FindoraEVMTx = serde_json::from_value(v).unwrap();
                        let evm_tx_hash =
                            H256::from_slice(Keccak256::digest(&rlp::encode(&evm_tx)).as_slice());
                        let signer = recover_signer(&evm_tx.function.ethereum.transact).unwrap();
                        let receiver = match evm_tx.function.ethereum.transact.action {
                            TransactionAction::Call(to) => {
                                format!("{to:?}")
                            }
                            _ => "".to_string(),
                        };
                        let v: Value = serde_json::to_value(&evm_tx).unwrap();
                        let evm_tx_hash = format!("{evm_tx_hash:?}");
                        let sender = format!("{signer:?}");
                        let amount = evm_tx.function.ethereum.transact.value.to_string();
                        v2_evm_txs.push(V2EvmTx {
                            tx_hash: txid,
                            block_hash: block_hash.clone(),
                            evm_tx_hash,
                            sender,
                            receiver,
                            amount,
                            height,
                            timestamp: timestamp.timestamp(),
                            content: v,
                        })
                    }
                }
                tx::TxCatalog::FindoraTx => {
                    let value: Value = serde_json::from_slice(&bytes)?;
                    let v: Value = value.clone();
                    txs.push(Transaction {
                        tx_hash: txid.clone(),
                        block_hash: block_hash.clone(),
                        height,
                        timestamp: timestamp.timestamp(),
                        code: tx.tx_result.code,
                        ty: 0,
                        log: tx.tx_result.log,
                        origin,
                        result,
                        value,
                    });

                    ////////////////////////////////////////////////////////////////////////////////
                    // v2: parse findora tx
                    ////////////////////////////////////////////////////////////////////////////////
                    let tx_val: TxValue = serde_json::from_value(v).unwrap();
                    for op in tx_val.body.operations {
                        let op_str = serde_json::to_string(&op).unwrap();
                        if op_str.contains("ConvertAccount") {
                            // convert account
                            let op_copy = op.clone();
                            let opt: ConvertAccountOperation = serde_json::from_value(op).unwrap();
                            let asset: String;
                            if let Some(asset_bin) = &opt.convert_account.asset_type {
                                asset = base64::encode_config(asset_bin, base64::URL_SAFE);
                            } else {
                                asset = FRA_ASSET.to_string();
                            }
                            let signer =
                                pubkey_to_fra_address(&opt.convert_account.signer).unwrap();
                            v2_convert_account_txs.push(V2ConvertAccountTx {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                signer,
                                receiver: opt.convert_account.receiver.ethereum,
                                asset,
                                amount: opt.convert_account.value,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("UnDelegation") {
                            let op_copy = op.clone();
                            let opt: UnDelegationOpt = serde_json::from_value(op).unwrap();
                            let sender = pubkey_to_fra_address(&opt.undelegation.pubkey).unwrap();
                            let (amount, new_delegator, target_validator) =
                                match opt.undelegation.body.pu {
                                    Some(pu) => {
                                        let target_validator_addr =
                                            hex::encode(pu.target_validator);
                                        (
                                            pu.am,
                                            pu.new_delegator_id,
                                            target_validator_addr.to_uppercase(),
                                        )
                                    }
                                    _ => (0, "".to_string(), "".to_string()),
                                };

                            v2_undelegation_txs.push(V2UndelegationTx {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                sender,
                                amount,
                                target_validator,
                                new_delegator,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("Delegation") {
                            let op_copy = op.clone();
                            let opt: DelegationOpt = serde_json::from_value(op).unwrap();
                            let sender = pubkey_to_fra_address(&opt.delegation.pubkey).unwrap();
                            let new_validator =
                                opt.delegation.body.new_validator.unwrap_or("".to_string());
                            v2_delegation_txs.push(V2DelegationTx {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                sender,
                                amount: opt.delegation.body.amount,
                                validator: opt.delegation.body.validator,
                                new_validator,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("Claim") {
                            let op_copy = op.clone();
                            let opt: ClaimOpt = serde_json::from_value(op).unwrap();
                            let sender = pubkey_to_fra_address(&opt.claim.pubkey).unwrap();
                            v2_claim_txs.push(V2ClaimTx {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                sender,
                                amount: opt.claim.body.amount,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("DefineAsset") {
                            let op_copy = op.clone();
                            let opt: DefineAssetOpt = serde_json::from_value(op).unwrap();
                            let issuer =
                                pubkey_to_fra_address(&opt.define_asset.pubkey.key).unwrap();
                            let asset = base64::encode_config(
                                opt.define_asset.body.asset.code.val,
                                URL_SAFE,
                            );
                            v2_define_asset_txs.push(V2DefineAssetTx {
                                asset,
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                issuer,
                                max_units: opt.define_asset.body.asset.asset_rules.max_units,
                                decimals: opt.define_asset.body.asset.asset_rules.decimals,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("IssueAsset") {
                            let op_copy = op.clone();
                            let opt: IssueAssetOpt = serde_json::from_value(op).unwrap();
                            let issuer =
                                pubkey_to_fra_address(&opt.issue_asset.pubkey.key).unwrap();
                            let asset =
                                base64::encode_config(opt.issue_asset.body.code.val, URL_SAFE);
                            v2_issue_asset_txs.push(V2IssueAssetTx {
                                asset,
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                issuer,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("TransferAsset") {
                            let op_copy = op.clone();
                            let opt: TransferAssetOpt = serde_json::from_value(op).unwrap();
                            let pk = &opt.transfer_asset.body_signatures[0].address.key;
                            let address = pubkey_to_fra_address(pk).unwrap();
                            v2_native_transfer_txs.push(V2NativeTransfer {
                                tx_hash: txid.clone(),
                                block_hash: block_hash.clone(),
                                address,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        }
                    }
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
            block_hash,
            height,
            size: block_size,
            tx_count: (evm_txs.len() + txs.len()) as i64,
            timestamp,
            app_hash,
            proposer,
            txs,
            evm_txs,
            validators,
            xhub_txs,
            v2_evm_txs,
            v2_convert_account_txs,
            v2_undelegation_txs,
            v2_delegation_txs,
            v2_claim_txs,
            v2_define_asset_txs,
            v2_issue_asset_txs,
            v2_native_transfer_txs,
            block_data,
        })
    }

    pub async fn load_height_retried(&self, height: i64) -> Result<ModuleBlock> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc() -> Result<()> {
        // let rpc = TendermintRPC::new(
        //     Duration::from_secs(10),
        //     "https://prod-mainnet.prod.findora.org:26657"
        //         .to_string()
        //         .parse()
        //         .unwrap(),
        // );
        // let _ = rpc.load_block(1550667).await?;
        // let _ = rpc.load_validators(1550667).await?;
        // let _ = rpc
        //     .load_transaction("c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0")
        //     .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_load_validators() -> Result<()> {
        // let rpc = TendermintRPC::new(
        //     Duration::from_secs(10),
        //     "https://prod-mainnet.prod.findora.org:26657"
        //         .to_string()
        //         .parse()
        //         .unwrap(),
        // );
        // let _ = rpc.load_validators(2360073).await?;
        Ok(())
    }
}
