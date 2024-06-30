use crate::service::error::Result;
use crate::service::QueryResult;
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
    let mut conn = state.pool.acquire().await?;

    let sql_query =
        r#"SELECT tx,block,sender,amount,height,timestamp,content FROM claims WHERE tx=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await?;

    let tx_hash: String = row.try_get("tx")?;
    let block_hash: String = row.try_get("block")?;
    let from: String = row.try_get("sender")?;
    let amount: i64 = row.try_get("amount")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

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
    let mut conn = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_count, sql_query) = if let Some(from) = params.from {
        (format!(
            "SELECT count(height) FROM claims WHERE sender='{}'",
            from.to_lowercase()
        ),format!(
            "SELECT tx,block,sender,amount,height,timestamp,content FROM claims WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            from.to_lowercase(), page_size, (page-1)*page_size
        ))
    } else {
        ("SELECT count(height) FROM claims".to_string(),
         format!(
             "SELECT tx,block,sender,amount,height,timestamp,content FROM claims ORDER BY timestamp DESC LIMIT {} OFFSET {}",
             page_size, (page-1)*page_size
         ))
    };

    let row_cnt = sqlx::query(&sql_count).fetch_one(&mut *conn).await?;
    let total: i64 = row_cnt.try_get("count")?;

    let mut claims: Vec<ClaimResponse> = vec![];
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let tx_hash: String = row.try_get("tx")?;
        let block_hash: String = row.try_get("block")?;
        let from: String = row.try_get("sender")?;
        let amount: i64 = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;
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
