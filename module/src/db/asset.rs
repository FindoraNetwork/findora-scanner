use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct Asset {
    pub address: String,
    pub name: String,
    pub publisher: String,
    pub memo: String,
    pub transferable: bool,
    pub amount: i64,
    pub decimals: i8,
}
#[derive(Serialize, Deserialize, Debug, Object)]
pub struct Student {
    pub name: String,
    pub age: i64,
}
