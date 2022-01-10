use clap::Parser;
use tokio_postgres::NoTls;

use crate::{block, db, Result};

/// Scanner tool for findora.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Server to tendermint.
    #[clap(short, long)]
    server: String,

    /// Begin block height.
    #[clap(short, long, default_value_t = 0)]
    begin: i64,

    /// Target block height.
    #[clap(short, long)]
    height: i64,

    /// Postgres.
    ///
    /// ie. "host=localhost user=postgres"
    #[clap(short, long)]
    postgres: String,
}

impl Args {
    pub async fn execute(&self) -> Result<()> {
        let target = self.height + 1;
        let mut begin = self.begin;

        if begin == 0 {
            begin = target - 1;
        }

        let (client, connection) = tokio_postgres::connect(&self.postgres, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        for idx in begin..target {
            println!("Got header {}", idx);
            let r = block::Block::load_height(self.server.clone(), idx).await?;
            db::save(r, &client).await?;
        }

        Ok(())
    }
}
