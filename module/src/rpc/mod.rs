pub mod block;
pub mod tx;
pub mod validator;
pub mod delegations;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonRpcResponse<T: DeserializeOwned> {
    pub jsonrpc: String,
    pub id: i64,
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub result: T,
}


#[derive(Deserialize, Serialize)]
pub struct TdRpcResult {
    pub response:TdRpcResponse,
}

#[derive(Deserialize, Serialize)]
pub struct TdRpcResponse {
    pub code:u32,
    pub log:String,
    pub info:String,
}