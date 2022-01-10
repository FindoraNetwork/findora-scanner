pub mod block;
pub mod utils;

mod error;
use clap::StructOpt;
pub use error::*;

pub mod command;
pub mod db;

#[tokio::main]
async fn main() {
    let args = command::Args::parse();
    args.execute().await.unwrap();
}
