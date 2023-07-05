extern crate scanner;

use scanner::commands::ScannerCmd;
use scanner::Result;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
    match ScannerCmd::parse() {
        ScannerCmd::Load(load) => load.execute().await,
        ScannerCmd::Scan(batch_scan) => batch_scan.execute().await,
        ScannerCmd::Subscribe(subscribe) => subscribe.run().await,
        ScannerCmd::Migrate(migrate) => migrate.execute().await,
    }
}
