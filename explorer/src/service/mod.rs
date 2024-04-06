use poem_openapi::Tags;

pub mod api;
pub mod util;
pub mod v1;
pub mod v2;

#[allow(dead_code)]
#[derive(Tags)]
enum ApiTags {
    /// Operations about Transaction
    Transaction,
    /// Operations about Block
    Block,
    /// Operations about Address
    Address,
    /// Operations about Asset
    Asset,
    /// Operations about Chain
    BlockChain,
    /// Operations about Price
    Price,
    /// Operations about staking
    Staking,
}
