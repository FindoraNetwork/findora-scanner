use crate::commands::FRA_ASSET;
use crate::types::{
    ClaimOpt, ConvertAccountOpt, DefineAssetOpt, DelegationOpt, EthereumWrap, FindoraEVMTx,
    FindoraEVMTxWrap, FindoraTxType, IssueAssetOpt, OutputTypeHideAmountHide,
    OutputTypeHideAmountShow, OutputTypeShowAmountHide, OutputTypeShowAmountShow, TransactWrap,
    TransactWrapData, TransferAssetOpt, TxValue, UnDelegationOpt, XHubOpt,
};
use crate::util::pubkey_to_fra_address;
use crate::{db, tx};
use crate::{Error, Result};
use base64::URL_SAFE;
use chrono::NaiveDateTime;
use ethereum::TransactionAction;
use module::rpc::block::BlockSizeRPC;
use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx,
    validator::ValidatorsRPC as ModuleValidatorsRPC, JsonRpcResponse, TdRpcResult,
};
use module::schema::{
    Address, Block as ModuleBlock, DelegationInfo, Transaction, V2AssetTx, V2ClaimTx,
    V2ConvertAccountTx, V2DelegationTx, V2UndelegationTx, Validator,
};
use module::utils::crypto::recover_signer;
use reqwest::{Client, ClientBuilder, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Receivers {
    pub addrs: Vec<String>,
}

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
        let mut v2_convert_account_txs = Vec::new();
        let mut v2_undelegation_txs = Vec::new();
        let mut v2_delegation_txs = Vec::new();
        let mut v2_claim_txs = Vec::new();
        let mut v2_asset_txs = Vec::new();
        let mut evm_addrs: Vec<Address> = vec![];
        let mut native_addrs: Vec<Address> = vec![];

        for tx_string in block.block.data.txs.unwrap_or_default() {
            let bytes = base64::decode(&tx_string)?;
            let origin = tx_string;
            let hasher = sha2::Sha256::digest(&bytes);
            let tx_hash = hex::encode(hasher);
            let tx = self.rpc.load_transaction(&tx_hash).await?;
            let result = serde_json::to_value(tx.tx_result.clone()).unwrap();

            match tx::try_tx_catalog(&bytes) {
                tx::TxCatalog::EvmTx => {
                    let value: Value = serde_json::from_slice(tx::unwrap(&bytes)?)?;
                    let sender: String;
                    let ty_sub: i32;
                    let mut addrs: Vec<String> = vec![];
                    let mut v: Value = value.clone();
                    let evm_tx_str = serde_json::to_string(&value).unwrap();
                    if evm_tx_str.contains("XHub") {
                        debug!("[EVM] XHub, height: {}, tx: {}", height, tx_hash);
                        let xhub_opt: XHubOpt = serde_json::from_value(value).unwrap();
                        for xo in &xhub_opt.function.xhub.nonconfidential_transfer.outputs {
                            let to = pubkey_to_fra_address(&xo.target).unwrap();
                            native_addrs.push(Address {
                                tx: tx_hash.clone(),
                                address: to,
                                timestamp: timestamp.timestamp(),
                            });
                        }
                        sender = "".to_string();
                        ty_sub = FindoraTxType::EVMToNative as i32;
                    } else {
                        debug!("[EVM] Ethereum, height: {}, tx: {}", height, tx_hash);
                        let evm_tx: FindoraEVMTx = serde_json::from_value(value).unwrap();
                        let signer = recover_signer(&evm_tx.function.ethereum.transact).unwrap();
                        let to = match evm_tx.function.ethereum.transact.action {
                            TransactionAction::Call(to) => {
                                format!("{to:?}")
                            }
                            _ => "".to_string(),
                        };
                        addrs.push(to);
                        sender = format!("{signer:?}");
                        ty_sub = FindoraTxType::Evm as i32;
                        let wrap_evm_tx = FindoraEVMTxWrap {
                            function: EthereumWrap {
                                ethereum: TransactWrap {
                                    transact: TransactWrapData {
                                        from: sender.clone(),
                                        nonce: evm_tx.function.ethereum.transact.nonce,
                                        gas_price: evm_tx.function.ethereum.transact.gas_price,
                                        gas_limit: evm_tx.function.ethereum.transact.gas_limit,
                                        action: evm_tx.function.ethereum.transact.action,
                                        value: evm_tx.function.ethereum.transact.value,
                                        input: evm_tx.function.ethereum.transact.input,
                                        signature: evm_tx.function.ethereum.transact.signature,
                                    },
                                },
                            },
                        };

                        v = serde_json::to_value(&wrap_evm_tx).unwrap();
                    }
                    let r = Receivers {
                        addrs: addrs.clone(),
                    };
                    let receivers_val = serde_json::to_value(&r).unwrap();
                    evm_txs.push(Transaction {
                        tx_hash: tx_hash.clone(),
                        block_hash: block_hash.clone(),
                        height,
                        timestamp: timestamp.timestamp(),
                        code: tx.tx_result.code,
                        ty: FindoraTxType::Evm as i32,
                        ty_sub,
                        sender: sender.clone(),
                        receiver: receivers_val,
                        log: tx.tx_result.log,
                        origin,
                        result,
                        value: v,
                    });

                    addrs.push(sender);
                    addrs.dedup();
                    for a in addrs {
                        if a.is_empty() {
                            continue;
                        }
                        evm_addrs.push(Address {
                            tx: tx_hash.clone(),
                            address: a,
                            timestamp: timestamp.timestamp(),
                        });
                    }
                }

                tx::TxCatalog::FindoraTx => {
                    let value: Value = serde_json::from_slice(&bytes)?;
                    let v: Value = value.clone();
                    let mut sender: String = "".to_string();
                    let mut ty_sub = 0;
                    let mut addrs: Vec<String> = vec![];
                    let tx_val: TxValue = serde_json::from_value(v).unwrap();

                    for op in tx_val.body.operations {
                        let op_str = serde_json::to_string(&op).unwrap();
                        if op_str.contains("ConvertAccount") {
                            debug!("[Native] ConvertAccount, height: {}", height);
                            let op_copy = op.clone();
                            let opt: ConvertAccountOpt = serde_json::from_value(op).unwrap();
                            let asset: String;
                            if let Some(asset_bin) = &opt.convert_account.asset_type {
                                asset = base64::encode_config(asset_bin, base64::URL_SAFE);
                            } else {
                                asset = FRA_ASSET.to_string();
                            }
                            let signer =
                                pubkey_to_fra_address(&opt.convert_account.signer).unwrap();
                            let receiver = opt.convert_account.receiver.ethereum;
                            evm_addrs.push(Address {
                                tx: tx_hash.clone(),
                                address: receiver.clone(),
                                timestamp: timestamp.timestamp(),
                            });
                            sender = signer.clone();
                            ty_sub = FindoraTxType::NativeToEVM as i32;
                            v2_convert_account_txs.push(V2ConvertAccountTx {
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                sender: signer,
                                receiver,
                                asset,
                                amount: opt.convert_account.value,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("UnDelegation") {
                            debug!("[Native] UnDelegation, height: {}, tx: {}", height, tx_hash);
                            let op_copy = op.clone();
                            let opt: UnDelegationOpt = serde_json::from_value(op).unwrap();
                            let signer = pubkey_to_fra_address(&opt.undelegation.pubkey).unwrap();
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

                            sender = signer.clone();
                            ty_sub = FindoraTxType::Undelegation as i32;
                            v2_undelegation_txs.push(V2UndelegationTx {
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                sender: signer,
                                amount,
                                target_validator,
                                new_delegator,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("Delegation") {
                            debug!("[Native] Delegation, height: {}, tx: {}", height, tx_hash);
                            let op_copy = op.clone();
                            let opt: DelegationOpt = serde_json::from_value(op).unwrap();
                            let signer = pubkey_to_fra_address(&opt.delegation.pubkey).unwrap();
                            let new_validator =
                                opt.delegation.body.new_validator.unwrap_or("".to_string());
                            sender = signer.clone();
                            ty_sub = FindoraTxType::Delegation as i32;
                            v2_delegation_txs.push(V2DelegationTx {
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                sender: signer,
                                amount: opt.delegation.body.amount,
                                validator: opt.delegation.body.validator,
                                new_validator,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("Claim") {
                            debug!("[Native] Claim, height: {}, tx: {}", height, tx_hash);
                            let op_copy = op.clone();
                            let opt: ClaimOpt = serde_json::from_value(op).unwrap();
                            let signer = pubkey_to_fra_address(&opt.claim.pubkey).unwrap();
                            sender = signer.clone();
                            ty_sub = FindoraTxType::Claim as i32;
                            v2_claim_txs.push(V2ClaimTx {
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                sender: signer,
                                amount: opt.claim.body.amount,
                                height,
                                timestamp: timestamp.timestamp(),
                                content: op_copy,
                            });
                        } else if op_str.contains("DefineAsset") {
                            debug!("[Native] DefineAsset, height: {}, tx: {}", height, tx_hash);
                            let op_copy = op.clone();
                            let opt: DefineAssetOpt = serde_json::from_value(op).unwrap();
                            let issuer =
                                pubkey_to_fra_address(&opt.define_asset.pubkey.key).unwrap();
                            let asset = base64::encode_config(
                                opt.define_asset.body.asset.code.val,
                                URL_SAFE,
                            );
                            sender = issuer.clone();
                            ty_sub = FindoraTxType::DefineOrIssueAsset as i32;
                            v2_asset_txs.push(V2AssetTx {
                                asset,
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                issuer,
                                height,
                                timestamp: timestamp.timestamp(),
                                issued: 0,
                                content: op_copy,
                            });
                        } else if op_str.contains("IssueAsset") {
                            debug!("[Native] IssueAsset, height: {}, tx: {}", height, tx_hash);
                            let op_copy = op.clone();
                            let opt: IssueAssetOpt = serde_json::from_value(op).unwrap();
                            let issuer =
                                pubkey_to_fra_address(&opt.issue_asset.pubkey.key).unwrap();
                            let asset =
                                base64::encode_config(opt.issue_asset.body.code.val, URL_SAFE);
                            sender = issuer.clone();
                            ty_sub = FindoraTxType::DefineOrIssueAsset as i32;
                            v2_asset_txs.push(V2AssetTx {
                                asset,
                                tx_hash: tx_hash.clone(),
                                block_hash: block_hash.clone(),
                                issuer,
                                height,
                                timestamp: timestamp.timestamp(),
                                issued: 1,
                                content: op_copy,
                            });
                        } else if op_str.contains("TransferAsset") {
                            debug!(
                                "[Native] TransferAsset, height: {}, tx: {}",
                                height, tx_hash
                            );
                            let opt: TransferAssetOpt = serde_json::from_value(op).unwrap();
                            let pk = &opt.transfer_asset.body_signatures[0].address.key;
                            let signer = pubkey_to_fra_address(pk).unwrap();
                            for o in opt.transfer_asset.body.transfer.outputs {
                                let mut receiver: String = "".to_string();
                                let type_show_amount_show: core::result::Result<
                                    OutputTypeShowAmountShow,
                                    _,
                                > = serde_json::from_value(o.clone());

                                if let Ok(tsas) = type_show_amount_show {
                                    // type show, amount show
                                    let pk = tsas.public_key;
                                    if pk.eq(&FRA_ASSET) {
                                        continue;
                                    }
                                    receiver = pubkey_to_fra_address(&pk).unwrap();
                                } else {
                                    // type show, amount hide
                                    let type_show_amount_hide: core::result::Result<
                                        OutputTypeShowAmountHide,
                                        _,
                                    > = serde_json::from_value(o.clone());

                                    if let Ok(tsah) = type_show_amount_hide {
                                        let pk = tsah.public_key;
                                        if pk.eq(&FRA_ASSET) {
                                            continue;
                                        }
                                        ty_sub = FindoraTxType::TypeShowAmountHide as i32;
                                        receiver = pubkey_to_fra_address(&pk).unwrap();
                                    } else {
                                        // type hide, amount show
                                        let type_hide_amount_show: core::result::Result<
                                            OutputTypeHideAmountShow,
                                            _,
                                        > = serde_json::from_value(o.clone());

                                        if let Ok(thas) = type_hide_amount_show {
                                            let pk = thas.public_key;
                                            if pk.eq(&FRA_ASSET) {
                                                continue;
                                            }
                                            ty_sub = FindoraTxType::TypeHideAmountShow as i32;
                                            receiver = pubkey_to_fra_address(&pk).unwrap();
                                        } else {
                                            // type hide, amount hide
                                            let type_hide_amount_hide: core::result::Result<
                                                OutputTypeHideAmountHide,
                                                _,
                                            > = serde_json::from_value(o.clone());

                                            if let Ok(thah) = type_hide_amount_hide {
                                                if thah.public_key.eq(&FRA_ASSET) {
                                                    continue;
                                                }
                                                ty_sub = FindoraTxType::TypeHideAmountHide as i32;
                                                receiver = pubkey_to_fra_address(&thah.public_key)
                                                    .unwrap();
                                            }
                                        }
                                    }
                                }

                                addrs.push(receiver);
                            }
                            sender = signer;
                        } else {
                            debug!("[Native], height: {}, tx: {}", height, tx_hash);
                        }
                    }

                    let r = Receivers {
                        addrs: addrs.clone(),
                    };
                    let receivers_val = serde_json::to_value(&r).unwrap();
                    txs.push(Transaction {
                        tx_hash: tx_hash.clone(),
                        block_hash: block_hash.clone(),
                        height,
                        timestamp: timestamp.timestamp(),
                        code: tx.tx_result.code,
                        ty: FindoraTxType::Native as i32,
                        ty_sub,
                        sender: sender.clone(),
                        receiver: receivers_val,
                        log: tx.tx_result.log,
                        origin,
                        result,
                        value,
                    });

                    addrs.push(sender);
                    addrs.dedup();
                    for a in addrs {
                        if a.is_empty() {
                            continue;
                        }
                        native_addrs.push(Address {
                            tx: tx_hash.clone(),
                            address: a,
                            timestamp: timestamp.timestamp(),
                        });
                    }
                }

                tx::TxCatalog::Unknown => {
                    info!("Unknown tx: {}", tx_hash);
                }
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
            evm_addrs,
            native_addrs,
            txs,
            evm_txs,
            validators,
            v2_convert_account_txs,
            v2_undelegation_txs,
            v2_delegation_txs,
            v2_claim_txs,
            v2_asset_txs,
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
