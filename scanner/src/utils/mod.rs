use serde::Deserialize;

pub mod block;

pub mod validator;

pub mod tx;

#[derive(Deserialize, Debug)]
pub struct RPCResponse<T> {
    pub result: T,
}
