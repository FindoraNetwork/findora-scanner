use crate::rpc::validator::PubKey;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Validator {
    pub address: String,
    pub power: u64,
    pub pub_key: PubKey,
    pub priority: i64,
    pub signature: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
}
