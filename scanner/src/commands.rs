use crate::db::{
    save_claim_tx, save_define_asset_tx, save_delegation_tx, save_evm_tx, save_issue_asset_tx,
    save_n2e_tx, save_native_tx, save_tx_type, save_undelegation_tx,
};
use crate::types::{
    ClaimOpt, ConvertAccountOperation, DefineAssetOpt, DelegationOpt, FindoraEVMTx, FindoraTxType,
    IssueAssetOpt, TransferAssetOpt, TxValue, UnDelegationOpt,
};
use crate::util::pubkey_to_fra_address;
use crate::{db, rpc::RPCCaller, scanner::RangeScanner};
use crate::{Error, Result};
use base64::URL_SAFE;
use clap::Parser;
use ethereum::TransactionAction;
use ethereum_types::H256;
use futures::TryStreamExt;
use module::utils::crypto::recover_signer;
use reqwest::Url;
use serde_json::Value;
use sha3::{Digest, Keccak256};
use sqlx::{PgPool, Row};
use std::env;
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 32;
const DEFAULT_RETIES: usize = 3;
const DEFAULT_CONCURRENCY: usize = 8;
//const DEFAULT_INTERVAL: Duration = Duration::from_secs(15);

const FRA_ASSET: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

#[derive(Parser)]
pub enum ScannerCmd {
    Scan(RangeScan),
    Load(Load),
    Subscribe(Subscribe),
    Migrate(Migrate),
}

/// load block at specific height.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Load {
    /// Server to tendermint.
    #[clap(short, long)]
    server: String,
    /// Target block height.
    #[clap(long)]
    height: Option<i64>,
    ///Rpc timeout with seconds.
    #[clap(long)]
    timeout: Option<u64>,
    ///Times to retry to pull a block.
    #[clap(long)]
    retries: Option<usize>,
    ///whether to load staking.
    #[clap(long, parse(from_flag))]
    staking: bool,
}

impl Load {
    pub async fn execute(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let target = if let Some(h) = self.height {
            if h <= 0 {
                return Err(format!("Invalid height: {h}.").into());
            }
            h
        } else if let Ok(h) = db::load_last_height(&pool).await {
            h + 1
        } else {
            1
        };

        info!("Got header {}", target);
        let caller = RPCCaller::new(retries, 1, timeout, rpc, pool);
        caller.load_and_save_block(target).await?;

        info!("Load block at height {} succeed.", target);
        Ok(())
    }
}

///batch scan for findora.
#[derive(Parser)]
#[clap(about, version, author)]
pub struct RangeScan {
    /// Server to tendermint.
    #[clap(short, long)]
    server: String,
    ///Start height
    #[clap(long)]
    start: u64,
    ///End height, included.
    #[clap(long)]
    end: u64,
    ///Rpc timeout with seconds, default is 32 seconds.
    #[clap(long)]
    timeout: Option<u64>,
    ///Times to retry to pull a block, default is 3.
    #[clap(long)]
    retries: Option<usize>,
    ///How many concurrency would be used to call rpc, default is 8.
    #[clap(long)]
    concurrency: Option<usize>,
}

impl RangeScan {
    pub async fn execute(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);
        let concurrency = self.concurrency.unwrap_or(DEFAULT_CONCURRENCY);

        let range_scanner = RangeScanner::new(timeout, rpc, retries, concurrency, pool);

        if self.start < 1 {
            return Err("`start` must >= 1.".into());
        }

        if self.end < self.start {
            return Err("`end` must large than `start`.".into());
        }

        let _ = range_scanner
            .range_scan(self.start as i64, self.end as i64 + 1)
            .await?;
        Ok(())
    }
}

/// Pull a block periodically.
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Subscribe {
    /// Server to tendermint.
    #[clap(short, long)]
    server: String,
    ///Start height
    #[clap(long)]
    start: Option<i64>,
    ///Rpc timeout with seconds, default is 10.
    #[clap(long)]
    timeout: Option<u64>,
    ///Times to retry to pull a block, default is 3.
    #[clap(long)]
    retries: Option<usize>,
    #[clap(long)]
    ///block generation interval, with seconds.
    interval: Option<u64>,
    ///How many concurrency would be used when scanning, default is 8.
    #[clap(long)]
    concurrency: Option<usize>,
    ///Load staking while subscribing.
    #[clap(long, parse(from_flag))]
    staking: bool,
}

impl Subscribe {
    pub async fn run(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));

        let itv = env::var("INTERVAL")
            .ok()
            .unwrap_or(String::from("15"))
            .parse::<u64>()?;
        let interval = Duration::from_secs(itv);
        info!("interval={:?}", interval);

        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let mut cursor = if let Some(h) = self.start {
            if h <= 0 {
                return Err(format!("Invalid height: {h}.").into());
            }
            h
        } else if let Ok(h) = db::load_last_height(&pool).await {
            h + 1
        } else {
            1
        };

        let concurrency = self.concurrency.unwrap_or(DEFAULT_CONCURRENCY);
        assert!(concurrency >= 1);
        let range_scanner = RangeScanner::new(timeout, rpc, retries, concurrency, pool.clone());
        let batch_size = 4 * concurrency as i64;

        info!("Subscribing start from {}, try fast sync ...", cursor);
        loop {
            let succeed_cnt = range_scanner
                .range_scan(cursor, cursor + batch_size)
                .await?;
            if succeed_cnt == batch_size {
                cursor += batch_size;
            } else {
                break;
            }
        }
        info!("Fast sync complete.");
        let caller = range_scanner.caller().clone();
        loop {
            if let Ok(h) = db::load_last_height(&pool).await {
                cursor = h + 1;
            }
            match caller.load_and_save_block(cursor).await {
                Ok(_) => {
                    info!("Block at {} loaded.", cursor);
                }
                Err(Error::NotFound) => (),
                Err(e) => return Err(e),
            };
            tokio::time::sleep(interval).await;
        }
        //may handle signal here.
    }
}

async fn prepare(rpc: &str) -> Result<(Url, PgPool)> {
    let pool = db::connect().await?;
    let rpc: Url = rpc.parse().map_err(|e| Error::from(format!("{e}")))?;

    Ok((rpc, pool))
}

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Migrate {}

impl Migrate {
    pub async fn execute(&self) -> Result<()> {
        let pool = db::connect().await?;
        let mut conn = pool.acquire().await?;

        let mut cursor =
            sqlx::query("SELECT tx_hash,block_hash,height,timestamp,ty,value FROM transaction")
                .fetch(&mut conn);
        while let Some(row) = cursor.try_next().await? {
            let tx: String = row.try_get("tx_hash")?;
            let block: String = row.try_get("block_hash")?;
            let height: i64 = row.try_get("height")?;
            let timestamp: i64 = row.try_get("timestamp")?;
            let ty: i32 = row.try_get("ty")?;
            let v = row.try_get("value")?;
            if ty == 1 {
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
                save_evm_tx(
                    &tx.to_lowercase(),
                    &block.to_lowercase(),
                    &evm_tx_hash.to_lowercase(),
                    &sender.to_lowercase(),
                    &receiver.to_lowercase(),
                    &amount,
                    height,
                    timestamp,
                    v,
                    &pool,
                )
                .await?;
                save_tx_type(&tx, FindoraTxType::Evm as i32, &pool).await?;
            } else {
                let tx_val: TxValue = serde_json::from_value(v).unwrap();
                for op in tx_val.body.operations {
                    let op_str = serde_json::to_string(&op).unwrap();
                    if op_str.contains("ConvertAccount") {
                        // convert account
                        let opt: ConvertAccountOperation = serde_json::from_value(op).unwrap();
                        let asset: String;
                        if let Some(asset_bin) = &opt.convert_account.asset_type {
                            asset = base64::encode_config(asset_bin, base64::URL_SAFE);
                        } else {
                            asset = FRA_ASSET.to_string();
                        }
                        save_n2e_tx(
                            &tx.to_lowercase(),
                            &block.to_lowercase(),
                            &opt.convert_account.signer,
                            &opt.convert_account.receiver.ethereum,
                            &asset,
                            &opt.convert_account.value,
                            height,
                            timestamp,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::NativeToEVM as i32, &pool).await?;
                    } else if op_str.contains("UnDelegation") {
                        // unstaking
                        let op_copy = op.clone();
                        let opt: UnDelegationOpt = serde_json::from_value(op).unwrap();
                        let sender = pubkey_to_fra_address(&opt.undelegation.pubkey).unwrap();
                        let (amount, new_delegator, target_validator) =
                            match opt.undelegation.body.pu {
                                Some(pu) => {
                                    let target_validator_addr = hex::encode(pu.target_validator);
                                    (
                                        pu.am,
                                        pu.new_delegator_id,
                                        target_validator_addr.to_uppercase(),
                                    )
                                }
                                _ => (0, "".to_string(), "".to_string()),
                            };

                        save_undelegation_tx(
                            &tx.to_lowercase(),
                            &block.to_lowercase(),
                            &sender,
                            amount,
                            &target_validator,
                            &new_delegator,
                            height,
                            timestamp,
                            &op_copy,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::UnStaking as i32, &pool).await?;
                    } else if op_str.contains("Delegation") {
                        // staking
                        let op_copy = op.clone();
                        let opt: DelegationOpt = serde_json::from_value(op).unwrap();
                        let sender = pubkey_to_fra_address(&opt.delegation.pubkey).unwrap();
                        let new_validator =
                            opt.delegation.body.new_validator.unwrap_or("".to_string());

                        save_delegation_tx(
                            &tx.to_lowercase(),
                            &block.to_lowercase(),
                            &sender,
                            opt.delegation.body.amount,
                            &opt.delegation.body.validator,
                            &new_validator,
                            height,
                            timestamp,
                            &op_copy,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::Staking as i32, &pool).await?;
                    } else if op_str.contains("Claim") {
                        // rewards
                        let op_copy = op.clone();
                        let opt: ClaimOpt = serde_json::from_value(op).unwrap();
                        let sender = pubkey_to_fra_address(&opt.claim.pubkey).unwrap();
                        save_claim_tx(
                            &tx.to_lowercase(),
                            &block.to_lowercase(),
                            &sender,
                            opt.claim.body.amount,
                            height,
                            timestamp,
                            &op_copy,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::Claim as i32, &pool).await?;
                    } else if op_str.contains("DefineAsset") {
                        // define asset
                        let op_copy = op.clone();
                        let opt: DefineAssetOpt = serde_json::from_value(op).unwrap();
                        let issuer = pubkey_to_fra_address(&opt.define_asset.pubkey.key).unwrap();
                        let asset =
                            base64::encode_config(opt.define_asset.body.asset.code.val, URL_SAFE);
                        save_define_asset_tx(
                            &asset,
                            &tx.to_lowercase(),
                            &block.to_lowercase(),
                            &issuer,
                            &opt.define_asset.body.asset.asset_rules.max_units,
                            opt.define_asset.body.asset.asset_rules.decimals,
                            height,
                            timestamp,
                            &op_copy,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::DefineAsset as i32, &pool).await?;
                    } else if op_str.contains("IssueAsset") {
                        // issue asset
                        let opt_copy = op.clone();
                        let opt: IssueAssetOpt = serde_json::from_value(op).unwrap();
                        let issuer = pubkey_to_fra_address(&opt.issue_asset.pubkey.key).unwrap();
                        let asset = base64::encode_config(opt.issue_asset.body.code.val, URL_SAFE);
                        save_issue_asset_tx(
                            &asset,
                            &tx,
                            &block.to_lowercase(),
                            &issuer,
                            height,
                            timestamp,
                            &opt_copy,
                            &pool,
                        )
                        .await?;
                        save_tx_type(&tx, FindoraTxType::IssueAsset as i32, &pool).await?;
                    } else {
                        // transfer asset
                        let opt: TransferAssetOpt = serde_json::from_value(op).unwrap();
                        let pubkey = opt.transfer_asset.body_signatures[0].address.key.clone();
                        let sender = pubkey_to_fra_address(&pubkey).unwrap();
                        for output in opt.transfer_asset.body.transfer.outputs {
                            if output.public_key.eq(FRA_ASSET) {
                                continue;
                            }
                            let asset = base64::encode_config(
                                &output.asset_type.non_confidential,
                                base64::URL_SAFE,
                            );
                            let receiver = pubkey_to_fra_address(&output.public_key).unwrap();
                            save_native_tx(
                                &tx.to_lowercase(),
                                &block.to_lowercase(),
                                &sender,
                                &receiver,
                                &asset,
                                &output.amount.non_confidential,
                                height,
                                timestamp,
                                &pool,
                            )
                            .await?;
                            save_tx_type(&tx, FindoraTxType::Native as i32, &pool).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
