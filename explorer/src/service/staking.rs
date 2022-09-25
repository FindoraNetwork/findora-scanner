use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use module::schema::{ClaimOpt, DelegationOpt, TdValidator, UnDelegationOpt};
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

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

pub async fn get_delegation(
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
//fra18fnyetvs2kc035xz78kyfcygmej8pk5h2kwczy03w6uewdphzfxsk74dym
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

        let validator_address = hex::encode(opt.body.pu.target_validator).to_uppercase();
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
            amount: opt.body.pu.am,
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
