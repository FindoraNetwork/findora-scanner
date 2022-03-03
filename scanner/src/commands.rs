use crate::{db, range_scanner::RPCCaller};
use clap::Parser;
use range_scanner::RangeScanner;
use reqwest::Url;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Parser)]
pub enum Scanner {
    Scan(RangeScan),
    Load(Load),
    Subscribe(Subscribe),
}
use crate::{range_scanner, Error, Result};

const DEFAULT_TIMEOUT_SECS: u64 = 32;
const DEFAULT_RETIES: usize = 3;
const DEFAULT_CONCURRENCY: usize = 8;
const DEFAULT_INTERVER: Duration = Duration::from_secs(15);

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
}

impl Load {
    pub async fn execute(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let target = if let Some(h) = self.height {
            if h <= 0 {
                return Err(format!("Invalid height: {}.", h).into());
            }
            h
        } else if let Ok(h) = db::load_last_height(&pool).await {
            h + 1
        } else {
            1
        };

        info!("Got header {}", target);

        let caller = RPCCaller::new(retries, 1, timeout, rpc, pool);

        let staking = caller.load_staking().await?;

        println!("{:?}", &staking);

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
}

impl Subscribe {
    pub async fn run(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let interval = self
            .interval
            .map(Duration::from_secs)
            .unwrap_or(DEFAULT_INTERVER);
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let mut cursor = if let Some(h) = self.start {
            if h <= 0 {
                return Err(format!("Invalid height: {}.", h).into());
            }
            h
        } else if let Ok(h) = db::load_last_height(&pool).await {
            h + 1
        } else {
            1
        };

        info!("Subscribing start from {}", cursor);

        let caller = RPCCaller::new(retries, 1, timeout, rpc, pool);
        loop {
            if caller.load_and_save_block(cursor).await.is_ok() {
                info!("Load block at height {} succeed.", cursor);
                cursor += 1;
            };
            tokio::time::sleep(interval).await;
        }
        //may handle signal here.
    }
}

async fn prepare(rpc: &str) -> Result<(Url, PgPool)> {
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var `DATABASE_URL` is required for the findora scanner.");

    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let rpc: Url = rpc.parse().map_err(|e| Error::from(format!("{}", e)))?;

    Ok((rpc, pool))
}
