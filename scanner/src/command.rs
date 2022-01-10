use clap::Parser;

use crate::{block, Result};

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
}

impl Args {
    pub async fn execute(&self) -> Result<()> {
        let target = self.height + 1;
        let mut begin = self.begin;

        if begin == 0 {
            begin = target - 1;
        }

        for idx in begin..target {
            println!("Got header {}", idx);
            let _r = tokio::spawn(block::Block::load_height(self.server.clone(), idx)).await??;
        }

        Ok(())
    }
}
