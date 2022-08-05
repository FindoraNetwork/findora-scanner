use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::Memo;
use poem_openapi::param::Path;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(ApiResponse)]
pub enum ValidatorListResponse {
    #[oai(status = 200)]
    Ok(Json<ValidatorListResult>),
}

#[derive(ApiResponse)]
pub enum ValidatorDetailResponse {
    #[oai(status = 200)]
    Ok(Json<ValidatorDetailResult>),
}

#[derive(ApiResponse)]
pub enum CirculatingSupplyResponse {
    #[oai(status = 200)]
    Ok(Json<CirculatingSupplyResult>),
}

#[derive(ApiResponse)]
pub enum DelegatorListResponse {
    #[oai(status = 200)]
    Ok(Json<DelegatorListResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegatorListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<DelegatorList>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegatorList {
    pub delegators: Vec<DelegatorItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegatorItem {
    pub addr: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct CirculatingSupplyResult {
    pub code: i32,
    pub message: String,
    pub data: Option<CirculatingSupply>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct CirculatingSupply {
    pub global_circulating_supply: f64,
    pub global_delegation_amount: f64,
    pub global_return_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorDetailResult {
    pub code: i32,
    pub message: String,
    pub data: Option<ValidatorDetail>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorDetail {
    pub addr: String,
    pub kind: String,
    pub is_online: bool,
    pub voting_power: i64,
    pub voting_power_rank: i64,
    pub commission_rate: Vec<i64>,
    pub self_staking: i64,
    pub fra_rewards: i64,
    pub memo: Memo,
    pub start_height: i64,
    pub cur_height: i64,
    pub block_signed_cnt: i64,
    pub block_proposed_cnt: i64,
    pub validator_realtime_apy: Value,
    pub delegator_cnt: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<ValidatorList>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorList {
    pub threshold: Vec<i64>,
    pub validator_cnt: i64,
    pub cur_height: i64,
    pub validators: Vec<Validator>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Validator {
    pub addr: String,
    pub power: i64,
    pub commission_rate: Vec<i64>,
    pub accept_delegation: bool,
    pub rank: i64,
    pub extra: Memo,
}

pub async fn validator_list(api: &Api) -> Result<ValidatorListResponse> {
    let validator_list_url = api.platform.rpc.join("validator_list").unwrap();
    debug!("validator_list_url: {}", validator_list_url);
    let res = api
        .platform
        .client
        .get(validator_list_url)
        .send()
        .await?
        .json::<ValidatorList>()
        .await?;

    Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
        code: 200,
        message: "".to_string(),
        data: Some(res),
    })))
}

pub async fn validator_detail(api: &Api, address: Path<String>) -> Result<ValidatorDetailResponse> {
    let validator_detail_url = api
        .platform
        .rpc
        .join(format!("validator_detail/{}", address.0).as_str())
        .unwrap();
    debug!("validator_detail_url: {}", validator_detail_url);

    let res = api
        .platform
        .client
        .get(validator_detail_url)
        .send()
        .await?
        .json::<ValidatorDetail>()
        .await?;

    Ok(ValidatorDetailResponse::Ok(Json(ValidatorDetailResult {
        code: 200,
        message: "".to_string(),
        data: Some(res),
    })))
}

pub async fn delegator_list(api: &Api, address: Path<String>) -> Result<DelegatorListResponse> {
    let delegator_list_url = api
        .platform
        .rpc
        .join(format!("delegator_list/{}", address.0).as_str())
        .unwrap();
    debug!("delegator_list_url: {}", delegator_list_url);

    let res = api
        .platform
        .client
        .get(delegator_list_url)
        .send()
        .await?
        .json::<DelegatorList>()
        .await?;

    Ok(DelegatorListResponse::Ok(Json(DelegatorListResult {
        code: 200,
        message: "".to_string(),
        data: Some(res),
    })))
}

pub async fn circulating_supply(api: &Api) -> Result<CirculatingSupplyResponse> {
    let circulating_supply_url = api.platform.rpc.join("circulating_supply").unwrap();
    debug!("circulating_supply_url: {}", circulating_supply_url);

    let res = api
        .platform
        .client
        .get(circulating_supply_url)
        .send()
        .await?
        .json::<CirculatingSupply>()
        .await?;

    Ok(CirculatingSupplyResponse::Ok(Json(
        CirculatingSupplyResult {
            code: 200,
            message: "".to_string(),
            data: Some(res),
        },
    )))
}
