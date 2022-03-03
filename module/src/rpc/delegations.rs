use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zei::xfr::sig::XfrPublicKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegationInfo {
    global_delegation_records_map: HashMap<String, DelegationLite>,
    validator_addr_map: HashMap<String, XfrPublicKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegationLite {
    #[serde(rename = "entries")]
    pub delegations: HashMap<XfrPublicKey, u64>,
    pub id: String, // delegation rewards will be paid to this pk by default
    pub start_height: u64,
    pub end_height: u64,
    pub state: DelegationState,
    pub rwd_amount: u64,
    pub proposer_rwd_cnt: u64,   // how many times you get proposer rewards
    pub delegation_rwd_cnt: u64, // how many times you get delegation rewards
    pub receiver_pk: Option<XfrPublicKey>,
    pub tmp_delegators: HashMap<XfrPublicKey, u64>, // Temporary partial undelegations of current id
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DelegationState {
    Bond, // during delegation, include extra 21 days.
    Free, // it's time to pay principals and rewards.
    Paid, //principals and rewards have been paid successfully.
}
