use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2DelegationResponse {
    #[oai(status = 200)]
    Ok(Json<V2DelegationResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2DelegationResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2Delegation>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2Delegation {
    pub tx_hash: String,
    pub block_hash: String,
    pub amount: i64,
    pub from: String,
    pub validator: String,
    pub new_validator: String,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_delegation(api: &Api, tx_hash: Path<String>) -> Result<V2DelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM delegations WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let amount: i64 = row.try_get("amount")?;
    let sender: String = row.try_get("sender")?;
    let validator: String = row.try_get("validator")?;
    let new_validator: String = row.try_get("new_validator")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2Delegation {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        amount,
        validator,
        new_validator,
        height,
        timestamp,
        value,
    };

    Ok(V2DelegationResponse::Ok(Json(V2DelegationResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum V2DelegationsResponse {
    #[oai(status = 200)]
    Ok(Json<V2DelegationsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2DelegationsResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2DelegationTxsData>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2DelegationTxsData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub items: Vec<V2Delegation>,
}

pub async fn v2_get_delegations(
    api: &Api,
    address: Query<Option<String>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2DelegationsResponse> {
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut conn = api.storage.lock().await.acquire().await?;

    let (sql_count, sql_query) = if let Some(addr) = address.0 {
        (
            format!(
                "SELECT count(*) AS cnt FROM delegations WHERE sender='{}'",
                addr.to_lowercase()
            ),
            format!(
            "SELECT tx,block,sender,amount,validator,new_validator,timestamp,height,content FROM delegations WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            addr.to_lowercase(),
            page_size,
            (page - 1) * page_size
        ),
        )
    } else {
        (
            "SELECT count(*) AS cnt FROM delegations".to_string(),
            format!(
            "SELECT tx,block,sender,amount,validator,new_validator,timestamp,height,content FROM delegations ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        ),
        )
    };

    let row_cnt = sqlx::query(sql_count.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;

    let mut res: Vec<V2Delegation> = vec![];
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let amount: i64 = row.try_get("amount")?;
        let sender: String = row.try_get("sender")?;
        let validator: String = row.try_get("validator")?;
        let new_validator: String = row.try_get("new_validator")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;
        res.push(V2Delegation {
            tx_hash: tx,
            block_hash: block,
            from: sender,
            amount,
            validator,
            new_validator,
            height,
            timestamp,
            value,
        });
    }

    Ok(V2DelegationsResponse::Ok(Json(V2DelegationsResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2DelegationTxsData {
            page,
            page_size,
            total,
            items: res,
        }),
    })))
}
