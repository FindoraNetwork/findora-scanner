use crate::{db, rpc::RPCCaller, scanner::RangeScanner};
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

#[derive(Parser)]
pub enum ScannerCmd {
    Scan(RangeScan),
    Load(Load),
    Subscribe(Subscribe),
    Migrate(Migrate),
}
use crate::db::{save_evm_tx, save_tx_type};
use crate::types::{FindoraEVMTx, FindoraTxType, TxValue};
use crate::{Error, Result};

const DEFAULT_TIMEOUT_SECS: u64 = 32;
const DEFAULT_RETIES: usize = 3;
const DEFAULT_CONCURRENCY: usize = 8;
//const DEFAULT_INTERVAL: Duration = Duration::from_secs(15);

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
                let evm_tx_hash = format!("{evm_tx_hash:?}").to_lowercase();
                let sender = format!("{signer:?}").to_lowercase();
                let receiver = receiver.to_lowercase();
                save_evm_tx(
                    &tx,
                    &block.to_lowercase(),
                    evm_tx_hash.as_str(),
                    sender.as_str(),
                    receiver.as_str(),
                    height,
                    timestamp,
                    v,
                    &pool,
                )
                .await?;
                save_tx_type(&tx, FindoraTxType::Evm as i32, &pool).await?;
            } else {
                let tx_str = serde_json::to_string(&v).unwrap();

                if tx_str.contains("ConvertAccount") {
                    let tx_val: TxValue = serde_json::from_value(v).unwrap();
                    for v in tx_val.body.operations {
                        let _s = serde_json::to_string(&v).unwrap();
                    }

                    // native to evm
                    // let n2e_tx: ConvertAccount = serde_json::from_value(v).unwrap();
                    // let asset: String;
                    // if let Some(asset_bin) = &n2e_tx.asset_type {
                    //     asset = base64::encode_config(asset_bin, base64::URL_SAFE);
                    // } else {
                    //     asset = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string();
                    // }
                    //
                    // sqlx::query("INSERT INTO n2e VALUES($1,$2,$3,$4,$5,$6,$7)")
                    //     .bind(&tx)
                    //     .bind(block)
                    //     .bind(n2e_tx.receiver.ethereum)
                    //     .bind(asset)
                    //     .bind(n2e_tx.value)
                    //     .bind(height)
                    //     .bind(timestamp)
                    //     .execute(&pool)
                    //     .await?;
                    // sqlx::query("INSERT INTO tx_types VALUES($1,$2)")
                    //     .bind(tx)
                    //     .bind(FindoraTxType::NativeToEVM as i32)
                    //     .execute(&pool)
                    //     .await?;
                }
                // else if tx_str.contains("XHub") { // old: evm to native
                // } else if tx_str.contains("Delegation") { // staking
                // } else if tx_str.contains("UnDelegation") { // unstaking
                // } else if tx_str.contains("Claim") { // rewards
                // } else if tx_str.contains("DefineAsset") {
                // } else if tx_str.contains("IssueAsset") {
                // } else if tx_str.contains("AbarToBar") {
                //     // TODO
                // } else if tx_str.contains("BarToAbar") {
                //     // TODO
                // } else if tx_str.contains("TransferAnonAsset") {
                //     // TODO
                // }
            }
        }
        Ok(())
    }
}
