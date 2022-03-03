use crate::{db, rpc::RPCCaller, Error};

use crossbeam::channel::bounded;
use reqwest::Url;

use sqlx::PgPool;
use std::sync::atomic::{AtomicI64, Ordering};
use std::{sync::Arc, time::Duration};

pub struct RangeScanner {
    caller: Arc<RPCCaller>,
}

impl RangeScanner {
    pub fn new(
        timeout: Duration,
        tendermint_rpc: Url,
        retries: usize,
        concurrency: usize,
        pool: PgPool,
    ) -> Self {
        RangeScanner {
            caller: Arc::new(RPCCaller::new(
                retries,
                concurrency,
                timeout,
                tendermint_rpc,
                pool,
            )),
        }
    }

    ///scan block in [start..end].
    pub async fn range_scan(&self, start: i64, end: i64) -> Result<i64, Error> {
        info!("Scanning [{},{}) ...", start, end);
        let concurrency = self.caller.concurrency; //how many spawned.

        let (sender, rev) = bounded(concurrency);

        //Store the max height.
        let last_height = Arc::new(AtomicI64::new(0));
        //counter of successful tasks.
        let succeed_cnt = Arc::new(AtomicI64::new(0));

        let inner_p = self.caller.clone();
        let last_height_p = last_height.clone();

        //start producer.
        let succeed_cnt_cloned = succeed_cnt.clone();
        let handle_producer = tokio::task::spawn_blocking(move || {
            for h in start..end {
                let fut = task(
                    inner_p.clone(),
                    h,
                    last_height_p.clone(),
                    succeed_cnt_cloned.clone(),
                );
                //build a future that have not been executed.
                sender.send(Some(fut)).unwrap();
            }

            //make them exit.
            for _ in 0..concurrency {
                sender.send(None).unwrap();
            }
        });

        //spawn consumers.
        let spawned_handles: Vec<_> = (0..concurrency)
            .map(move |_| {
                let rev_cloned = rev.clone();
                tokio::spawn(async move {
                    while let Ok(Some(fut)) = rev_cloned.recv() {
                        //handle error within.
                        let _: () = fut.await;
                    }
                })
            })
            .collect();

        for h in spawned_handles {
            h.await?;
        }
        handle_producer.await?;
        info!("Scanning [{},{}) complete.", start, end);
        Ok(succeed_cnt.load(Ordering::Acquire))
    }

    pub fn caller(&self) -> &Arc<RPCCaller> {
        &self.caller
    }
}

async fn task(
    caller: Arc<RPCCaller>,
    h: i64,
    last_height: Arc<AtomicI64>,
    succeed_cnt: Arc<AtomicI64>,
) {
    match caller.load_height_retried(h).await {
        Ok(block) => match db::save(block, &caller.pool).await {
            Ok(_) => {
                let h_old = last_height.load(Ordering::Acquire);
                if h > h_old {
                    last_height.store(h, Ordering::Release);
                    //write the last height to database.
                    if let Err(e) = db::save_last_height(h, &caller.pool).await {
                        error!("Database error: {:?}", e);
                    }
                }
                succeed_cnt.fetch_add(1, Ordering::Release);
                debug!("Height at {} succeed.", h);
            }
            Err(e) => error!("Database error: {:?}", e),
        },
        Err(e) => {
            if let Error::NotFound = e {
                info!("Block not found at height {}.", h);
            } else {
                error!("Load height error: {:?}", e);
            }
        }
    };
}
