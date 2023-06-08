use crate::{db, rpc::RPCCaller, scanner::RangeScanner};
use clap::Parser;
use reqwest::Url;
use sqlx::PgPool;
use std::env;
use std::time::Duration;

#[derive(Parser)]
pub enum ScannerCmd {
    Scan(RangeScan),
    Load(Load),
    Subscribe(Subscribe),
}
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
