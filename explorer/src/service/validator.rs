use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::Memo;
use poem_openapi::param::{Path, Query};
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

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

#[derive(ApiResponse)]
pub enum ValidatorDelegationResponse {
    #[oai(status = 200)]
    Ok(Json<ValidatorDelegationResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorDelegationResult {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegatorListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
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
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorDetailResult {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<WrapValidatorListData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct WrapValidatorListData {
    pub threshold: [i64; 2],
    pub validator_cnt: i64,
    pub cur_height: i64,
    pub validators: Vec<WrapValidator>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct WrapValidator {
    pub addr: String,
    pub power: i64,
    pub commission_rate: [i64; 2],
    pub accept_delegation: bool,
    pub online: bool,
    pub rank: i64,
    pub extra: Memo,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorListData {
    pub threshold: [i64; 2],
    pub validator_cnt: i64,
    pub cur_height: i64,
    pub validators: Vec<Validator>,
}

impl ValidatorListData {
    fn wrap(self) -> WrapValidatorListData {
        let mut wrap_validators: Vec<WrapValidator> = vec![];
        for v in self.validators {
            wrap_validators.push(v.wrap());
        }

        WrapValidatorListData {
            threshold: self.threshold,
            validator_cnt: self.validator_cnt,
            cur_height: self.cur_height,
            validators: wrap_validators,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Validator {
    pub addr: String,
    pub power: i64,
    pub commission_rate: [i64; 2],
    pub accept_delegation: bool,
    pub rank: i64,
    pub extra: Memo,
}

impl Validator {
    fn wrap(self) -> WrapValidator {
        WrapValidator {
            addr: self.addr,
            power: self.power,
            commission_rate: self.commission_rate,
            accept_delegation: self.accept_delegation,
            online: false,
            rank: self.rank,
            extra: self.extra,
        }
    }
}

pub async fn validator_delegation(
    api: &Api,
    address: Query<String>,
) -> Result<ValidatorDelegationResponse> {
    let mut validator_delegation_url = api.platform.rpc.join("validator_delegation").unwrap();
    validator_delegation_url.set_query(Some(&format!("address={}", address.0)));

    debug!("validator_delegation_url: {}", validator_delegation_url);

    let res = api
        .platform
        .client
        .get(validator_delegation_url)
        .send()
        .await?
        .json()
        .await?;

    Ok(ValidatorDelegationResponse::Ok(Json(
        ValidatorDelegationResult {
            code: 200,
            message: "".to_string(),
            data: Some(res),
        },
    )))
}

pub async fn validator_list(api: &Api) -> Result<ValidatorListResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let validator_list_url = api.platform.rpc.join("validator_list").unwrap();
    debug!("validator_list_url: {}", validator_list_url);

    let res = api
        .platform
        .client
        .get(validator_list_url)
        .send()
        .await?
        .json()
        .await?;

    let validator_data: ValidatorListData = serde_json::from_value(res).unwrap();

    let sql = format!(
        "SELECT address FROM block_generation WHERE height={}",
        validator_data.cur_height.clone()
    );

    let mut wrap_validator_data = validator_data.wrap();

    let rows = sqlx::query(sql.as_str()).fetch_all(&mut conn).await?;
    let mut signers: Vec<String> = vec![];
    for r in rows {
        let addr: String = r.try_get("address")?;
        signers.push(addr);
    }

    for v in &mut wrap_validator_data.validators {
        if signers.contains(&v.addr) {
            v.online = true
        }
    }

    Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
        code: 200,
        message: "".to_string(),
        data: Some(wrap_validator_data),
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
        .json()
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
        .json()
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
        .json()
        .await?;

    Ok(CirculatingSupplyResponse::Ok(Json(
        CirculatingSupplyResult {
            code: 200,
            message: "".to_string(),
            data: Some(res),
        },
    )))
}
