use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::{ClaimOpt, DelegationOpt, TdValidator, UnDelegationOpt};
use poem_openapi::param::{Path, Query};
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;

#[derive(ApiResponse)]
pub enum DelegationResponse {
    #[oai(status = 200)]
    Ok(Json<DelegationResult>),
    #[oai(status = 400)]
    BadRequest(Json<DelegationResult>),
    #[oai(status = 404)]
    NotFound(Json<DelegationResult>),
    #[oai(status = 500)]
    InternalError(Json<DelegationResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationResult {
    pub code: i32,
    pub message: String,
    pub data: Option<DelegationData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationData {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<DelegationItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationItem {
    pub tx_hash: String,
    pub node_address: String,
    pub node_name: String,
    pub node_logo: String,
    pub amount: i64,
    pub timestamp: i64,
}

pub async fn get_delegation_tx(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<DelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let pubkey = match public_key_from_bech32(address.0.as_str()) {
        Ok(_) => public_key_from_bech32(address.0.as_str()).unwrap(),
        Err(_) => {
            return Ok(DelegationResponse::BadRequest(Json(DelegationResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            })));
        }
    };
    let base64_address = public_key_to_base64(&pubkey);

    let sql_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].Delegation.pubkey ? (@==\"{}\")'", base64_address);
    let sql_query = format!("SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].Delegation') AS delegation FROM transaction WHERE value @? '$.body.operations[*].Delegation.pubkey ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", base64_address, page_size, (page-1)*page_size);

    let mut items: Vec<DelegationItem> = vec![];
    let row_cnt = sqlx::query(sql_count.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let timestamp: i64 = row.try_get("timestamp")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let val: Value = row.try_get("delegation")?;
        let opt: DelegationOpt = serde_json::from_value(val).unwrap();

        let validator_detail_url = api
            .platform
            .rpc
            .join(format!("validator_detail/{}", opt.body.validator).as_str())
            .unwrap();
        let res = reqwest::get(validator_detail_url).await?.json().await?;
        let validator: TdValidator = serde_json::from_value(res).unwrap();

        items.push(DelegationItem {
            tx_hash,
            node_address: validator.addr,
            node_name: validator.memo.name,
            node_logo: validator.memo.logo,
            amount: opt.body.amount,
            timestamp,
        });
    }

    Ok(DelegationResponse::Ok(Json(DelegationResult {
        code: 200,
        message: "".to_string(),
        data: Some(DelegationData {
            total,
            page,
            page_size,
            items,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum UnDelegationResponse {
    #[oai(status = 200)]
    Ok(Json<UnDelegationResult>),
    #[oai(status = 400)]
    BadRequest(Json<UnDelegationResult>),
    #[oai(status = 404)]
    NotFound(Json<UnDelegationResult>),
    #[oai(status = 500)]
    InternalError(Json<UnDelegationResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UnDelegationResult {
    pub code: i32,
    pub message: String,
    pub data: Option<UnDelegationData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UnDelegationData {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<UnDelegationItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UnDelegationItem {
    pub tx_hash: String,
    pub node_address: String,
    pub node_name: String,
    pub node_logo: String,
    pub amount: i64,
    pub timestamp: i64,
    pub expected_arrival_time: i64,
}

pub async fn get_undelegation(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<UnDelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let pubkey = match public_key_from_bech32(address.0.as_str()) {
        Ok(_) => public_key_from_bech32(address.0.as_str()).unwrap(),
        Err(_) => {
            return Ok(UnDelegationResponse::BadRequest(Json(UnDelegationResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            })));
        }
    };
    let base64_address = public_key_to_base64(&pubkey);

    let sql_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].UnDelegation.pubkey ? (@==\"{}\")'", base64_address);
    let sql_query = format!("SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].UnDelegation') AS undelegation FROM transaction WHERE value @? '$.body.operations[*].UnDelegation.pubkey ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", base64_address, page_size, (page-1)*page_size);

    let mut items: Vec<UnDelegationItem> = vec![];
    let row_cnt = sqlx::query(sql_count.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let timestamp: i64 = row.try_get("timestamp")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let val: Value = row.try_get("undelegation")?;
        let opt: UnDelegationOpt = serde_json::from_value(val).unwrap();
        if opt.body.pu.is_none() {
            continue;
        }
        let validator_address =
            hex::encode(opt.body.pu.as_ref().unwrap().target_validator).to_uppercase();
        let validator_detail_url = api
            .platform
            .rpc
            .join(format!("validator_detail/{}", validator_address).as_str())
            .unwrap();
        let res = reqwest::get(validator_detail_url).await?.json().await?;
        let validator: TdValidator = serde_json::from_value(res).unwrap();

        items.push(UnDelegationItem {
            tx_hash,
            node_address: validator.addr,
            node_name: validator.memo.name,
            node_logo: validator.memo.logo,
            amount: opt.body.pu.unwrap().am,
            timestamp,
            expected_arrival_time: timestamp + 21 * 24 * 60 * 60,
        });
    }

    Ok(UnDelegationResponse::Ok(Json(UnDelegationResult {
        code: 200,
        message: "".to_string(),
        data: Some(UnDelegationData {
            total,
            page,
            page_size,
            items,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum ClaimResponse {
    #[oai(status = 200)]
    Ok(Json<ClaimResult>),
    #[oai(status = 400)]
    BadRequest(Json<ClaimResult>),
    #[oai(status = 404)]
    NotFound(Json<ClaimResult>),
    #[oai(status = 500)]
    InternalError(Json<ClaimResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ClaimResult {
    pub code: i32,
    pub message: String,
    pub data: Option<ClaimData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ClaimData {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<ClaimItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ClaimItem {
    pub tx_hash: String,
    pub node_address: String,
    pub node_name: String,
    pub node_logo: String,
    pub amount: u64,
    pub timestamp: i64,
}

pub async fn get_claim(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<ClaimResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    let pubkey = match public_key_from_bech32(address.0.as_str()) {
        Ok(_) => public_key_from_bech32(address.0.as_str()).unwrap(),
        Err(_) => {
            return Ok(ClaimResponse::BadRequest(Json(ClaimResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            })));
        }
    };
    let base64_address = public_key_to_base64(&pubkey);

    let sql_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].Claim.pubkey ? (@==\"{}\")'", base64_address);
    let sql_query =format!("SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].Claim') AS claim FROM transaction WHERE value @? '$.body.operations[*].Claim.pubkey ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", base64_address, page_size, (page-1)*page_size);

    let mut items: Vec<ClaimItem> = vec![];
    let row_cnt = sqlx::query(sql_count.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let timestamp: i64 = row.try_get("timestamp")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let val: Value = row.try_get("claim")?;
        let opt: ClaimOpt = serde_json::from_value(val).unwrap();

        items.push(ClaimItem {
            tx_hash,
            node_address: "".to_string(),
            node_name: "".to_string(),
            node_logo: "".to_string(),
            amount: opt.body.amount as u64,
            timestamp,
        });
    }

    Ok(ClaimResponse::Ok(Json(ClaimResult {
        code: 200,
        message: "".to_string(),
        data: Some(ClaimData {
            total,
            page,
            page_size,
            items,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum DelegationInfoResponse {
    #[oai(status = 200)]
    Ok(Json<DelegationInfoResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationInfoResult {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

pub async fn delegation(api: &Api, pubkey: Path<String>) -> Result<DelegationInfoResponse> {
    let delegation_info_url = api
        .platform
        .rpc
        .join(format!("delegation_info/{}", pubkey.0).as_str())
        .unwrap();

    debug!("delegation_info_url: {}", delegation_info_url);

    let res = api
        .platform
        .client
        .get(delegation_info_url)
        .send()
        .await?
        .json()
        .await?;

    Ok(DelegationInfoResponse::Ok(Json(DelegationInfoResult {
        code: 200,
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum UndelegationResponse {
    #[oai(status = 200)]
    Ok(Json<UndelegationResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UndelegationResult {
    pub code: i32,
    pub message: String,
    pub data: UndelegationResultData,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UndelegationResultData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub undelegations: Vec<UndelegationInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct UndelegationInfo {
    pub tx_hash: String,
    pub timestamp: i64,
    pub pubkey: String,
    pub amount: u64,
    pub validator: String,
}

pub async fn get_undelegation_info(
    api: &Api,
    pubkey: Query<Option<String>>,
    start: Query<Option<i64>>,
    end: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<UndelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let mut params: Vec<String> = vec![];
    if let Some(start) = start.0 {
        params.push(format!(" timestamp>={} ", start));
    }
    if let Some(end) = end.0 {
        params.push(format!(" timestamp<={} ", end));
    }
    if let Some(pk) = pubkey.0 {
        params.push(format!(
            "(value @? '$.body.operations[*].UnDelegation.pubkey ? (@==\"{}\")')",
            pk
        ));
    }

    let mut query_sql = "SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].UnDelegation') AS ud FROM transaction".to_string();
    if !params.is_empty() {
        query_sql = query_sql.add(" WHERE ").add(params.join(" AND ").as_str());
    }
    query_sql =
        query_sql.add(format!(" LIMIT {} OFFSET {}", page_size, (page - 1) * page_size).as_str());

    let rows = sqlx::query(query_sql.as_str()).fetch_all(&mut conn).await?;
    let l = rows.len();
    let mut undelegations: Vec<UndelegationInfo> = vec![];
    for r in rows {
        let tx_hash: String = r.try_get("tx_hash")?;
        let timestamp: i64 = r.try_get("timestamp")?;
        let val: Value = r.try_get("ud")?;
        let opt: UnDelegationOpt = serde_json::from_value(val)?;
        if opt.body.pu.is_none() {
            continue;
        }
        let vaddr = hex::encode(opt.body.pu.as_ref().unwrap().target_validator).to_uppercase();

        undelegations.push(UndelegationInfo {
            tx_hash,
            timestamp,
            pubkey: opt.pubkey,
            amount: opt.body.pu.unwrap().am as u64,
            validator: vaddr,
        })
    }

    let res = UndelegationResultData {
        page,
        page_size,
        total: l as i64,
        undelegations,
    };

    Ok(UndelegationResponse::Ok(Json(UndelegationResult {
        code: 200,
        message: "".to_string(),
        data: res,
    })))
}

#[derive(ApiResponse)]
pub enum SimpleDelegationResponse {
    #[oai(status = 200)]
    Ok(Json<SimpleDelegationResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct SimpleDelegationResult {
    pub code: i32,
    pub message: String,
    pub data: DelegationResultData,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationResultData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub delegations: Vec<DelegationInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationInfo {
    pub tx_hash: String,
    pub timestamp: i64,
    pub pubkey: String,
    pub amount: u64,
    pub validator: String,
}

pub async fn get_delegation_info(
    api: &Api,
    pubkey: Query<Option<String>>,
    start: Query<Option<i64>>,
    end: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<SimpleDelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let mut params: Vec<String> = vec![];
    if let Some(start) = start.0 {
        params.push(format!(" timestamp>={} ", start));
    }
    if let Some(end) = end.0 {
        params.push(format!(" timestamp<={} ", end));
    }
    if let Some(pk) = pubkey.0 {
        params.push(format!(
            "(value @? '$.body.operations[*].Delegation.pubkey ? (@==\"{}\")')",
            pk
        ));
    }

    let mut query_sql = "SELECT tx_hash,timestamp,jsonb_path_query(value,'$.body.operations[*].Delegation') AS d FROM transaction".to_string();
    if !params.is_empty() {
        query_sql = query_sql.add(" WHERE ").add(params.join(" AND ").as_str());
    }
    query_sql =
        query_sql.add(format!(" LIMIT {} OFFSET {}", page_size, (page - 1) * page_size).as_str());

    let rows = sqlx::query(query_sql.as_str()).fetch_all(&mut conn).await?;
    let l = rows.len();
    let mut delegations: Vec<DelegationInfo> = vec![];

    for r in rows {
        let tx_hash: String = r.try_get("tx_hash")?;
        let timestamp: i64 = r.try_get("timestamp")?;
        let val: Value = r.try_get("d")?;
        let opt: DelegationOpt = serde_json::from_value(val)?;

        delegations.push(DelegationInfo {
            tx_hash,
            timestamp,
            pubkey: opt.pubkey,
            amount: opt.body.amount as u64,
            validator: opt.body.validator,
        })
    }

    let res = DelegationResultData {
        page,
        page_size,
        total: l as i64,
        delegations,
    };

    Ok(SimpleDelegationResponse::Ok(Json(SimpleDelegationResult {
        code: 200,
        message: "".to_string(),
        data: res,
    })))
}

#[derive(ApiResponse)]
pub enum DelegationAmountResponse {
    #[oai(status = 200)]
    Ok(Json<DelegationAmountResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationAmountResult {
    pub code: i32,
    pub message: String,
    pub data: DelegationAmountData,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DelegationAmountData {
    pub amount: u64,
}

pub async fn get_delegation_amount(
    api: &Api,
    pubkey: Query<Option<String>>,
    start: Query<Option<i64>>,
    end: Query<Option<i64>>,
) -> Result<DelegationAmountResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let mut params: Vec<String> = vec![];
    if let Some(start) = start.0 {
        params.push(format!(" timestamp>={} ", start));
    }
    if let Some(end) = end.0 {
        params.push(format!(" timestamp<={} ", end));
    }
    if let Some(pk) = pubkey.0 {
        params.push(format!(
            "(value @? '$.body.operations[*].Delegation.pubkey ? (@==\"{}\")')",
            pk
        ));
    }

    let mut query_sql =
        "SELECT jsonb_path_query(value,'$.body.operations[*].Delegation') AS d FROM transaction"
            .to_string();
    if !params.is_empty() {
        query_sql = query_sql.add(" WHERE ").add(params.join(" AND ").as_str());
    }

    let rows = sqlx::query(query_sql.as_str()).fetch_all(&mut conn).await?;
    let mut amount: u64 = 0;
    for r in rows {
        let val: Value = r.try_get("d")?;
        let opt: DelegationOpt = serde_json::from_value(val)?;
        amount += opt.body.amount as u64;
    }

    Ok(DelegationAmountResponse::Ok(Json(DelegationAmountResult {
        code: 200,
        message: "".to_string(),
        data: DelegationAmountData { amount },
    })))
}
