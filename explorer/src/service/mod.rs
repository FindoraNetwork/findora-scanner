use poem_openapi::Tags;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod error;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult<T> {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub data: T,
}
