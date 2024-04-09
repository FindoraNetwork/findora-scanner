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
pub struct GetDelegationByHashParams {
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct DelegationResponse {
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

pub async fn get_delegation_by_tx_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetDelegationByHashParams>,
) -> Result<Json<DelegationResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;
    let sql_query = r#"SELECT tx,block,amount,sender,validator,new_validator,height,timestamp,content
        FROM delegations WHERE tx=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let tx_hash: String = row.try_get("tx").map_err(internal_error)?;
    let block_hash: String = row.try_get("block").map_err(internal_error)?;
    let amount: i64 = row.try_get("amount").map_err(internal_error)?;
    let from: String = row.try_get("sender").map_err(internal_error)?;
    let validator: String = row.try_get("validator").map_err(internal_error)?;
    let new_validator: String = row.try_get("new_validator").map_err(internal_error)?;
    let height: i64 = row.try_get("height").map_err(internal_error)?;
    let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
    let value: Value = row.try_get("content").map_err(internal_error)?;

    let delegation = DelegationResponse {
        tx_hash,
        block_hash,
        from,
        amount,
        validator,
        new_validator,
        height,
        timestamp,
        value,
    };

    Ok(Json(delegation))
}

#[derive(Serialize, Deserialize)]
pub struct GetDelegationsParams {
    pub from: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[allow(dead_code)]
pub async fn get_delegations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetDelegationsParams>,
) -> Result<Json<QueryResult<Vec<DelegationResponse>>>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_count, sql_query) = if let Some(addr) = params.from {
        (
            format!(
                "SELECT count(*) FROM delegations WHERE sender='{}'",
                addr.to_lowercase()
            ),
            format!(
                "SELECT tx,block,sender,amount,validator,new_validator,timestamp,height,content \
                FROM delegations WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
                addr.to_lowercase(),
                page_size,
                (page - 1) * page_size
            ),
        )
    } else {
        (
            "SELECT count(*) FROM delegations".to_string(),
            format!(
                "SELECT tx,block,sender,amount,validator,new_validator,timestamp,height,content \
                FROM delegations ORDER BY timestamp DESC LIMIT {} OFFSET {}",
                page_size,
                (page - 1) * page_size
            ),
        )
    };

    let row_cnt = sqlx::query(&sql_count)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let total: i64 = row_cnt.try_get("count").map_err(internal_error)?;

    let mut delegations: Vec<DelegationResponse> = vec![];
    let rows = sqlx::query(&sql_query)
        .fetch_all(&mut *conn)
        .await
        .map_err(internal_error)?;

    for row in rows {
        let tx_hash: String = row.try_get("tx").map_err(internal_error)?;
        let block_hash: String = row.try_get("block").map_err(internal_error)?;
        let amount: i64 = row.try_get("amount").map_err(internal_error)?;
        let from: String = row.try_get("sender").map_err(internal_error)?;
        let validator: String = row.try_get("validator").map_err(internal_error)?;
        let new_validator: String = row.try_get("new_validator").map_err(internal_error)?;
        let height: i64 = row.try_get("height").map_err(internal_error)?;
        let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
        let value: Value = row.try_get("content").map_err(internal_error)?;

        delegations.push(DelegationResponse {
            tx_hash,
            block_hash,
            from,
            amount,
            validator,
            new_validator,
            height,
            timestamp,
            value,
        });
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegations,
    }))
}
