use crate::service::error::{internal_error, Result};
use crate::service::v2::QueryResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetClaimByTxHash {
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimResponse {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub amount: u64,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

#[allow(dead_code)]
pub async fn get_claim_by_tx_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetClaimByTxHash>,
) -> Result<Json<ClaimResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let sql_query =
        r#"SELECT tx,block,sender,amount,height,timestamp,content FROM claims WHERE tx=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let tx_hash: String = row.try_get("tx").map_err(internal_error)?;
    let block_hash: String = row.try_get("block").map_err(internal_error)?;
    let from: String = row.try_get("sender").map_err(internal_error)?;
    let amount: i64 = row.try_get("amount").map_err(internal_error)?;
    let height: i64 = row.try_get("height").map_err(internal_error)?;
    let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
    let value: Value = row.try_get("content").map_err(internal_error)?;

    let claim = ClaimResponse {
        tx_hash,
        block_hash,
        from,
        amount: amount as u64,
        height,
        timestamp,
        value,
    };

    Ok(Json(claim))
}

#[derive(Serialize, Deserialize)]
pub struct GetClaimsParams {
    pub from: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[allow(dead_code)]
pub async fn get_claims(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetClaimsParams>,
) -> Result<Json<QueryResult<Vec<ClaimResponse>>>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_count, sql_query) = if let Some(from) = params.from {
        (format!(
            "SELECT count(*) FROM claims WHERE sender='{}'",
            from.to_lowercase()
        ),format!(
            "SELECT tx,block,sender,amount,height,timestamp,content FROM claims WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            from.to_lowercase(), page_size, (page-1)*page_size
        ))
    } else {
        ("SELECT count(*) FROM claims".to_string(),
         format!(
             "SELECT tx,block,sender,amount,height,timestamp,content FROM claims ORDER BY timestamp DESC LIMIT {} OFFSET {}",
             page_size, (page-1)*page_size
         ))
    };

    let row_cnt = sqlx::query(&sql_count)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let total: i64 = row_cnt.try_get("count").map_err(internal_error)?;

    let mut claims: Vec<ClaimResponse> = vec![];
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await
        .map_err(internal_error)?;
    for row in rows {
        let tx_hash: String = row.try_get("tx").map_err(internal_error)?;
        let block_hash: String = row.try_get("block").map_err(internal_error)?;
        let from: String = row.try_get("sender").map_err(internal_error)?;
        let amount: i64 = row.try_get("amount").map_err(internal_error)?;
        let height: i64 = row.try_get("height").map_err(internal_error)?;
        let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
        let value: Value = row.try_get("content").map_err(internal_error)?;
        claims.push(ClaimResponse {
            tx_hash,
            block_hash,
            from,
            amount: amount as u64,
            height,
            timestamp,
            value,
        });
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: claims,
    }))
}
