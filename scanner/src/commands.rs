use clap::Parser;
use module::schema::LastHeight;
use range_scanner::RangeScanner;
use reqwest::Url;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Parser)]
pub enum Scanner {
    Scan(BatchScan),
    Load(Load),
}
use crate::{range_scanner, Error, Result};

const DEFAULT_TIMEOUT_SECS: u64 = 32;
const DEFAULT_RETIES: usize = 3;
const DEFAULT_BATCH_SIZE: u64 = 64;

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

///batch scan for findora.
#[derive(Parser)]
#[clap(about, version, author)]
pub struct BatchScan {
    /// Server to tendermint.
    #[clap(short, long)]
    server: String,
    ///Start height
    #[clap(long)]
    start: u64,
    ///End height, included.
    #[clap(long)]
    end: u64,
    ///Rpc timeout with seconds, default is 10.
    #[clap(long)]
    timeout: Option<u64>,
    ///Times to retry to pull a block, default is 3.
    #[clap(long)]
    retries: Option<usize>,
    ///How many concurrency would be used to call rpc, default is 64.
    #[clap(long)]
    batch_size: Option<u64>,
}

impl Load {
    pub async fn execute(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let mut target = 1;
        if let Some(h) = self.height {
            if h <= 0 {
                return Err(format!("Invalid height: {}.", h).into());
            }
            target = h;
        } else if let Ok(lh) = sqlx::query_as!(LastHeight, "SELECT * FROM last_height")
            .fetch_one(&pool)
            .await
        {
            target += lh.height;
        }

        info!("Got header {}", target);

        let range_scanner = RangeScanner::new(timeout, rpc, retries, pool);

        let _ = range_scanner.range_scan(target, target + 1).await?;
        info!("Load block at height {} succeed.", target);
        Ok(())
    }
}

impl BatchScan {
    pub async fn execute(&self) -> Result<()> {
        let (rpc, pool) = prepare(&self.server).await?;
        let timeout = Duration::from_secs(self.timeout.unwrap_or(DEFAULT_TIMEOUT_SECS));
        let retries = self.retries.unwrap_or(DEFAULT_RETIES);

        let range_scanner = RangeScanner::new(timeout, rpc, retries, pool);

        let batch_size = self.batch_size.unwrap_or(DEFAULT_BATCH_SIZE);

        if self.start < 1 {
            return Err("`start` must >= 1.".into());
        }

        if self.end < self.start {
            return Err("`end` must large than `start`.".into());
        }

        let mut cursor = self.start;
        let mut end_flag = false;

        loop {
            if end_flag {
                break;
            }
            let end = {
                let e = cursor + batch_size;
                if e > self.end {
                    end_flag = true;
                    self.end + 1
                } else {
                    e
                }
            };

            let res = range_scanner.range_scan(cursor as i64, end as i64).await?;
            info!(
                "Scanned blocks of height [{},{}), {}/{} succeed.",
                cursor,
                end,
                res.len(),
                end - cursor
            );

            cursor += batch_size;
        }

        Ok(())
    }
}

async fn prepare(rpc: &str) -> Result<(Url, PgPool)> {
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var `DATABASE_URL` is required for the findora scanner.");

    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let rpc: Url = rpc.parse().map_err(|e| Error::from(format!("{}", e)))?;

    Ok((rpc, pool))
}
