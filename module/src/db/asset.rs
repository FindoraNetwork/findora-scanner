use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Asset {
    pub code: String,
    pub memo: String,
    pub issuer: String,
    pub max_uints: i64,
    pub transferable: bool,
    pub updatable: bool,
}
