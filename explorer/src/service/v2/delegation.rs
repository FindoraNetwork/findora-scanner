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
    let mut conn = state.pool.acquire().await?;
    let sql_query = r#"SELECT tx,block,amount,sender,validator,new_validator,height,timestamp,content
        FROM delegations WHERE tx=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await?;

    let tx_hash: String = row.try_get("tx")?;
    let block_hash: String = row.try_get("block")?;
    let amount: i64 = row.try_get("amount")?;
    let from: String = row.try_get("sender")?;
    let validator: String = row.try_get("validator")?;
    let new_validator: String = row.try_get("new_validator")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

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
    let mut conn = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_count, sql_query) = if let Some(addr) = params.from {
        (
            format!(
                "SELECT count(height) FROM delegations WHERE sender='{}'",
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
            "SELECT count(height) FROM delegations".to_string(),
            format!(
                "SELECT tx,block,sender,amount,validator,new_validator,timestamp,height,content \
                FROM delegations ORDER BY timestamp DESC LIMIT {} OFFSET {}",
                page_size,
                (page - 1) * page_size
            ),
        )
    };

    let row_cnt = sqlx::query(&sql_count).fetch_one(&mut *conn).await?;
    let total: i64 = row_cnt.try_get("count")?;

    let mut delegations: Vec<DelegationResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *conn).await?;

    for row in rows {
        let tx_hash: String = row.try_get("tx")?;
        let block_hash: String = row.try_get("block")?;
        let amount: i64 = row.try_get("amount")?;
        let from: String = row.try_get("sender")?;
        let validator: String = row.try_get("validator")?;
        let new_validator: String = row.try_get("new_validator")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;

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
