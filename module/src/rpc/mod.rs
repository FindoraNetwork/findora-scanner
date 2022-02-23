pub mod block;
pub mod tx;
pub mod validator;

use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RPCPesponse<T: DeserializeOwned> {
    pub jsonrpc: String,
    pub id: i64,
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub result: T,
}
