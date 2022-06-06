use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PubKey {
    pub r#type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Validator {
    pub address: String,
    pub pub_key: PubKey,
    pub voting_power: String,
    pub proposer_priority: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ValidatorsRPC {
    pub validators: Vec<Validator>,
    pub count: String,
    pub total: String,
}
