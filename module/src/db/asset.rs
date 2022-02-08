use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Asset {
    pub address: String,
    pub name: String,
    pub publisher: String,
    pub memo: String,
    pub transferable: bool,
    pub amount: i64,
    pub decimals: i8,
}
