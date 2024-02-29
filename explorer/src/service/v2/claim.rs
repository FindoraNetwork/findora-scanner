use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2ClaimResponse {
    #[oai(status = 200)]
    Ok(Json<V2ClaimResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2ClaimResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2Claim>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2Claim {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub amount: u64,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_claim(api: &Api, tx_hash: Path<String>) -> Result<V2ClaimResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM claims WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let sender: String = row.try_get("sender")?;
    let amount: i64 = row.try_get("amount")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2Claim {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        amount: amount as u64,
        height,
        timestamp,
        value,
    };

    Ok(V2ClaimResponse::Ok(Json(V2ClaimResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum V2ClaimsResponse {
    #[oai(status = 200)]
    Ok(Json<V2ClaimsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2ClaimsResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2ClaimsData>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2ClaimsData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub data: Option<Vec<V2Claim>>,
}

pub async fn v2_get_claims(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2ClaimsResponse> {
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_count = format!(
        "SELECT count(*) AS cnt FROM claims WHERE sender='{}'",
        address.0.to_lowercase()
    );
    let row_cnt = sqlx::query(sql_count.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;

    let sql_query = format!(
        "SELECT tx,block,sender,amount,height,timestamp,content FROM claims WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        address.0.to_lowercase(), page_size, (page-1)*page_size
    );

    let mut res: Vec<V2Claim> = vec![];
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let sender: String = row.try_get("sender")?;
        let amount: i64 = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;
        res.push(V2Claim {
            tx_hash: tx,
            block_hash: block,
            from: sender,
            amount: amount as u64,
            height,
            timestamp,
            value,
        });
    }

    Ok(V2ClaimsResponse::Ok(Json(V2ClaimsResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2ClaimsData {
            page,
            page_size,
            total,
            data: Some(res),
        }),
    })))
}
