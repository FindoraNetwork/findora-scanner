pub mod range_scanner;

#[macro_use]
extern crate log;

mod error;
pub use error::*;

pub mod commands;
pub mod db;
pub mod rpc;
pub mod tx;

use clap::Parser;

use commands::Scanner;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    match commands::Scanner::parse() {
        Scanner::Load(load) => load.execute().await,
        Scanner::Scan(batch_scan) => batch_scan.execute().await,
        Scanner::Subscribe(subscribe) => {
            if let Err(e) = subscribe.run().await {
                error!("{:?}", e);
                println!("Existing...");
                // EX_SOFTWARE (70) : An internal software error has been detected. This
                // should be limited to	non-operating system related
                // errors as possible.
                std::process::exit(70);
            }
            //for type check, meaningless.
            Ok(())
        }
    }
}
