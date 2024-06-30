use crate::service::error::Result;
use crate::service::QueryResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct E2NTxResponse {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub asset: String,
    pub amount: String,
    pub decimal: i32,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct GetE2NByTxHashParams {
    pub hash: String,
}

pub async fn get_e2n_by_tx_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetE2NByTxHashParams>,
) -> Result<Json<E2NTxResponse>> {
    let mut conn = state.pool.acquire().await?;
    let sql_query = r#"SELECT tx_hash,block_hash,sender,receiver,asset,amount,decimal,height,timestamp,value FROM e2n WHERE tx_hash=$1"#;
    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await?;

    let tx_hash: String = row.try_get("tx_hash")?;
    let block_hash: String = row.try_get("block_hash")?;
    let from: String = row.try_get("sender")?;
    let to: String = row.try_get("receiver")?;
    let asset: String = row.try_get("asset")?;
    let decimal: i32 = row.try_get("decimal")?;
    let amount: String = row.try_get("amount")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("value")?;
    let tx = E2NTxResponse {
        tx_hash,
        block_hash,
        from,
        to,
        asset,
        amount,
        decimal,
        height,
        timestamp,
        value,
    };

    Ok(Json(tx))
}

#[derive(Serialize, Deserialize)]
pub struct GetE2NTxsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_e2n_txs(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetE2NTxsParams>,
) -> Result<Json<QueryResult<Vec<E2NTxResponse>>>> {
    let mut conn = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_total = "SELECT count(height) FROM e2n ".to_string();
    let mut sql_query = "SELECT tx_hash,block_hash,sender,receiver,asset,amount,decimal,height,timestamp,value FROM e2n ".to_string();

    let mut query_params: Vec<String> = vec![];
    if let Some(from) = params.from {
        query_params.push(format!("sender='{}'", from));
    }
    if let Some(to) = params.to {
        query_params.push(format!("receiver='{}'", to));
    }
    if !query_params.is_empty() {
        sql_total = sql_total
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
        sql_query = sql_query
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
    }
    sql_query = sql_query.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let row = sqlx::query(&sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("count")?;

    let mut txs: Vec<E2NTxResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *conn).await?;
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let from: String = row.try_get("sender")?;
        let to: String = row.try_get("receiver")?;
        let asset: String = row.try_get("asset")?;
        let decimal: i32 = row.try_get("decimal")?;
        let amount: String = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("value")?;
        txs.push(E2NTxResponse {
            tx_hash,
            block_hash,
            from,
            to,
            asset,
            amount,
            decimal,
            height,
            timestamp,
            value,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: txs,
    }))
}
