extern crate prismer;

use prismer::commands::PrismerCmd;
use prismer::Result;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();
    match PrismerCmd::parse() {
        PrismerCmd::Load(load) => load.execute().await,
        PrismerCmd::Scan(batch_scan) => batch_scan.execute().await,
        PrismerCmd::Subscribe(subscribe) => subscribe.run().await,
    }
}
