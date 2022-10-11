use crate::service::util::{public_key_from_base64, public_key_to_bech32};
use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::{DelegationOpt, Memo, UnDelegationOpt};
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
        "SELECT address FROM block_generation WHERE height={} AND signature is not null",
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

const VALIDATOR_DELEGATION: i8 = 0;
const VALIDATOR_UN_DELEGATION: i8 = 2;

#[derive(ApiResponse)]
pub enum ValidatorHistoryResponse {
    #[oai(status = 200)]
    Ok(Json<ValidatorHistoryResult>),
    #[oai(status = 400)]
    BadRequest(Json<ValidatorHistoryResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorHistoryResult {
    pub code: i32,
    pub message: String,
    pub data: Option<ValidatorHistoryData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorHistoryData {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<ValidatorHistoryItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct ValidatorHistoryItem {
    pub tx_hash: String,
    pub account: String,
    pub operation: i8,
    pub amount: i64,
    pub timestamp: i64,
}

pub async fn validator_history(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<ValidatorHistoryResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let addr = address.0.to_uppercase();
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    let sql_delegation = format!("SELECT tx_hash,timestamp,jsonb_path_query(value, '$.body.operations[*].Delegation') AS delegation FROM transaction WHERE value @? '$.body.operations[*].Delegation.body.validator ? (@==\"{}\")'", addr);
    let sql_undelegation = "SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].UnDelegation') AS undelegation FROM transaction";

    let mut tmp_items: Vec<ValidatorHistoryItem> = vec![];

    let delegation_rows = sqlx::query(sql_delegation.as_str())
        .fetch_all(&mut conn)
        .await?;
    for row in delegation_rows {
        let timestamp: i64 = row.try_get("timestamp")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let val: Value = row.try_get("delegation")?;
        let opt: DelegationOpt = serde_json::from_value(val).unwrap();

        let pubkey = public_key_from_base64(&opt.pubkey).unwrap();
        tmp_items.push(ValidatorHistoryItem {
            tx_hash,
            account: public_key_to_bech32(&pubkey),
            operation: VALIDATOR_DELEGATION,
            amount: opt.body.amount,
            timestamp,
        })
    }

    let undelegation_rows = sqlx::query(sql_undelegation).fetch_all(&mut conn).await?;
    for row in undelegation_rows {
        let timestamp: i64 = row.try_get("timestamp")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let val: Value = row.try_get("undelegation")?;
        let opt: UnDelegationOpt = serde_json::from_value(val).unwrap();
        if opt.body.pu.is_none() {
            continue;
        }
        let vaddr = hex::encode(opt.body.pu.as_ref().unwrap().target_validator).to_uppercase();
        if addr.eq(&vaddr) {
            let pubkey = public_key_from_base64(&opt.pubkey).unwrap();
            tmp_items.push(ValidatorHistoryItem {
                tx_hash,
                account: public_key_to_bech32(&pubkey),
                operation: VALIDATOR_UN_DELEGATION,
                amount: opt.body.pu.unwrap().am,
                timestamp,
            })
        }
    }

    tmp_items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let items;
    let total = tmp_items.len() as i64;
    let offset = page_size * (page - 1);
    if offset >= total {
        items = vec![];
    } else {
        let start = offset as usize;
        if offset + page_size <= total {
            let end = (offset + page_size) as usize;
            items = Vec::from(&tmp_items[start..end]);
        } else {
            items = Vec::from(&tmp_items[start..]);
        }
    }

    Ok(ValidatorHistoryResponse::Ok(Json(ValidatorHistoryResult {
        code: 200,
        message: "".to_string(),
        data: Some(ValidatorHistoryData {
            total,
            page,
            page_size,
            items,
        }),
    })))
}
