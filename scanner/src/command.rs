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

    /// Target block height.
    #[clap(short, long)]
    height: Option<i64>,

    /// Postgres.
    ///
    /// ie. "host=localhost user=postgres"
    #[clap(short, long)]
    postgres: String,
}

impl Args {
    pub async fn execute(&self) -> Result<()> {
        let (client, connection) = tokio_postgres::connect(&self.postgres, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let mut target = 1;
        if self.height.is_some() {
            target = self.height.unwrap();
        } else {
            let res = client.query("SELECT * FROM last_height", &[]).await;
            if let Ok(rows) = res {
                if !rows.is_empty() {
                    let h: i64 = rows[0].get(1);
                    target += h;
                }
            }
        }

        println!("Got header {}", target);
        let r = block::Block::load_height(self.server.clone(), target).await?;
        db::save(r, &client).await?;

        Ok(())
    }
}
