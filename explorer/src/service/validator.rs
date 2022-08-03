use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::{DelegationOpt, Memo};
use poem_openapi::param::Path;
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
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_latest_height = "SELECT height FROM last_height".to_string();
    let row = sqlx::query(sql_latest_height.as_str())
        .fetch_one(&mut conn)
        .await?;
    let height: i64 = row.try_get("height")?;
    let sql = format!("SELECT jsonb_path_query(value->'body', '$.operations[*].Delegation') as d FROM transaction WHERE code=0 AND height={}", height);
    let res = sqlx::query(sql.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
                code: 500,
                message: "".to_string(),
                data: None,
            })));
        }
    };
    let mut validator_list = ValidatorList {
        validator_cnt: rows.len() as i64,
        cur_height: height,
        validators: vec![],
    };

    for r in rows {
        let d: Value = r.try_get("d").unwrap();
        let delegation: DelegationOpt = serde_json::from_value(d).unwrap();
        if let Some(nv) = delegation.body.new_validator {
            validator_list.validators.push(Validator {
                addr: delegation.body.validator,
                power: nv.td_power,
                commission_rate: nv.commission_rate,
                accept_delegation: false,
                rank: 0,
                extra: Memo {
                    name: nv.memo.name,
                    desc: nv.memo.desc,
                    website: nv.memo.website,
                    logo: nv.memo.logo,
                },
            });
        } else {
            validator_list.validators.push(Validator {
                addr: delegation.body.validator,
                power: 0,
                commission_rate: vec![],
                accept_delegation: false,
                rank: 0,
                extra: Default::default(),
            })
        }
    }

    Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
        code: 200,
        message: "".to_string(),
        data: Some(validator_list),
    })))
}

pub async fn validator_detail(api: &Api, address: Path<String>) -> Result<ValidatorDetailResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_latest_height = "SELECT height FROM last_height".to_string();
    let row = sqlx::query(sql_latest_height.as_str())
        .fetch_one(&mut conn)
        .await?;
    let height: i64 = row.try_get("height")?;

    let sql_proposed_cnt = format!(
        "SELECT count(*) as cnt from block WHERE proposer=\'{}\'",
        address.0
    );
    let row = sqlx::query(sql_proposed_cnt.as_str())
        .fetch_one(&mut conn)
        .await?;
    let proposed_cnt: i64 = row.try_get("cnt")?;

    let sql_update = format!("SELECT jsonb_path_query(value->'body', '$.operations[*].UpdateStaker ? (@.body.validator==\"{}\")') as d FROM transaction WHERE code=0 AND height={} ORDER BY height LIMIT 1", address.0, height);
    let update_res = sqlx::query(sql_update.as_str()).fetch_one(&mut conn).await;

    if let Ok(r) = update_res {
        let d: Value = r.try_get("d").unwrap();
        let delegation: DelegationOpt = serde_json::from_value(d).unwrap();
        let nv = delegation.body.new_validator.unwrap_or_default();
        let detail = ValidatorDetail {
            addr: delegation.body.validator,
            kind: nv.kind,
            is_online: false,
            voting_power: nv.td_power,
            voting_power_rank: 0,
            commission_rate: nv.commission_rate,
            self_staking: 0,
            fra_rewards: 0,
            memo: nv.memo,
            start_height: 0,
            cur_height: height,
            block_signed_cnt: nv.signed_cnt,
            block_proposed_cnt: proposed_cnt,
            validator_realtime_apy: Default::default(),
            delegator_cnt: 0,
        };

        Ok(ValidatorDetailResponse::Ok(Json(ValidatorDetailResult {
            code: 200,
            message: "".to_string(),
            data: Some(detail),
        })))
    } else {
        let sql_delegation = format!("SELECT jsonb_path_query(value->'body', '$.operations[*].Delegation ? (@.body.validator==\"{}\")') as d FROM transaction WHERE code=0 AND height={} ORDER BY height LIMIT 1", address.0, height);
        let delegate_res = sqlx::query(sql_delegation.as_str())
            .fetch_one(&mut conn)
            .await;
        if delegate_res.is_err() {
            return Ok(ValidatorDetailResponse::Ok(Json(ValidatorDetailResult {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
        let r = delegate_res.unwrap();
        let d: Value = r.try_get("d").unwrap();
        let delegation: DelegationOpt = serde_json::from_value(d).unwrap();
        let nv = delegation.body.new_validator.unwrap_or_default();
        let detail = ValidatorDetail {
            addr: delegation.body.validator,
            kind: nv.kind,
            is_online: false,
            voting_power: nv.td_power,
            voting_power_rank: 0,
            commission_rate: nv.commission_rate,
            self_staking: 0,
            fra_rewards: 0,
            memo: nv.memo,
            start_height: 0,
            cur_height: height,
            block_signed_cnt: nv.signed_cnt,
            block_proposed_cnt: proposed_cnt,
            validator_realtime_apy: Default::default(),
            delegator_cnt: 0,
        };

        Ok(ValidatorDetailResponse::Ok(Json(ValidatorDetailResult {
            code: 200,
            message: "".to_string(),
            data: Some(detail),
        })))
    }
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

pub async fn delegator_list(api: &Api, address: Path<String>) -> Result<DelegatorListResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql = format!("SELECT jsonb_path_query(value->'body'->'operations', '$[*].Delegation?(@.body.validator==\"{}\")') as d from transaction", address.0);
    let list_res = sqlx::query(sql.as_str()).fetch_all(&mut conn).await;

    let rows = match list_res {
        Ok(rows) => rows,
        _ => {
            return Ok(DelegatorListResponse::Ok(Json(DelegatorListResult {
                code: 500,
                message: "".to_string(),
                data: None,
            })))
        }
    };
    let mut lst: Vec<DelegatorItem> = vec![];
    for r in rows {
        let v: Value = r.try_get("d")?;
        let delegation: DelegationOpt = serde_json::from_value(v).unwrap();
        let item = DelegatorItem {
            addr: delegation.body.validator,
            amount: delegation.body.amount,
        };
        lst.push(item);
    }

    Ok(DelegatorListResponse::Ok(Json(DelegatorListResult {
        code: 0,
        message: "".to_string(),
        data: Some(DelegatorList { delegators: lst }),
    })))
}
