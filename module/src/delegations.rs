use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct StakingLite {
    pub delegation_info: DelegationInfo,
}

#[derive(Serialize, Deserialize)]
pub struct DelegationInfo {
    global_delegation_records_map: HashMap<String, DelegationLite>,
    end_height_map: HashMap<u64, HashSet<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct DelegationLite {
    pub delegations: HashMap<String, u64>,
    pub id: String, // delegation rewards will be paid to this pk by default
    pub start_height: u64,
    pub end_height: u64,
    pub state: DelegationState,
    pub rwd_amount: u64,
    pub proposer_rwd_cnt: u64,   // how many times you get proposer rewards
    pub delegation_rwd_cnt: u64, // how many times you get delegation rewards
    pub receiver_pk: Option<String>,
    pub tmp_delegators: HashMap<String, u64>, // Temporary partial undelegations of current id
}

#[derive(Serialize, Deserialize)]
pub enum DelegationState {
    Bond, // during delegation, include extra 21 days.
    Free, // it's time to pay principals and rewards.
    Paid, //principals and rewards have been paid successfully.
}
