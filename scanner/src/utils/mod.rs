use serde::Deserialize;

pub mod block;

pub mod validator;

#[derive(Deserialize, Debug)]
pub struct RPCResponse<T> {
    pub result: T,
}
