use crate::service::error::{internal_error, Result};
use crate::service::QueryResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use module::schema::TransactionResponse;
use scanner::types::FindoraEVMTxWrap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetTxsParams {
    pub from: Option<String>,
    pub to: Option<String>,
    pub ty: Option<i32>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_txs(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetTxsParams>,
) -> Result<Json<QueryResult<Vec<TransactionResponse>>>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_query = String::from("SELECT tx_hash,block_hash,ty,timestamp,height,code,log,origin,result,value FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) FROM transaction ");
    let mut query_params: Vec<String> = vec![];
    if let Some(from) = params.from {
        query_params.push(format!("sender='{}' ", from))
    }
    if let Some(to) = params.to {
        query_params.push(format!("(receiver @? '$.addrs[*] ? (@==\"{}\")') ", to));
    }
    if let Some(ty) = params.ty {
        query_params.push(format!("ty={} ", ty));
    }
    if !query_params.is_empty() {
        sql_query = sql_query
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
        sql_total = sql_total
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

    let row = sqlx::query(&sql_total)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let total: i64 = row.try_get("count").map_err(internal_error)?;

    let rows = sqlx::query(&sql_query)
        .fetch_all(&mut *conn)
        .await
        .map_err(internal_error)?;
    let mut txs: Vec<TransactionResponse> = vec![];
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash").map_err(internal_error)?;
        let block_hash: String = row.try_get("block_hash").map_err(internal_error)?;
        let ty: i32 = row.try_get("ty").map_err(internal_error)?;
        let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
        let height: i64 = row.try_get("height").map_err(internal_error)?;
        let code: i64 = row.try_get("code").map_err(internal_error)?;
        let log: String = row.try_get("log").map_err(internal_error)?;
        let origin: String = row.try_get("origin").map_err(internal_error)?;
        let result: Value = row.try_get("result").map_err(internal_error)?;
        let value: Value = row.try_get("value").map_err(internal_error)?;

        let evm_tx_hash = if ty == 1 {
            let evm_tx: FindoraEVMTxWrap = serde_json::from_value(value.clone()).unwrap();
            let hash = evm_tx.hash();
            format!("{hash:?}")
        } else {
            "".to_string()
        };

        txs.push(TransactionResponse {
            tx_hash,
            evm_tx_hash,
            block_hash,
            height,
            timestamp,
            ty,
            code,
            log,
            origin,
            result,
            value,
        });
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: txs,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct GetTxByHashParams {
    pub hash: String,
}

pub async fn get_tx_by_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetTxByHashParams>,
) -> Result<Json<TransactionResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let sql_query = r#"SELECT tx_hash,block_hash,height,timestamp,ty,code,log,origin,result,value
        FROM transaction WHERE tx_hash=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let tx_hash: String = row.try_get("tx_hash").map_err(internal_error)?;
    let block_hash: String = row.try_get("block_hash").map_err(internal_error)?;
    let ty: i32 = row.try_get("ty").map_err(internal_error)?;
    let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
    let height: i64 = row.try_get("height").map_err(internal_error)?;
    let code: i64 = row.try_get("code").map_err(internal_error)?;
    let log: String = row.try_get("log").map_err(internal_error)?;
    let origin: String = row.try_get("origin").map_err(internal_error)?;
    let result: Value = row.try_get("result").map_err(internal_error)?;
    let value: Value = row.try_get("value").map_err(internal_error)?;

    let evm_tx_hash = if ty == 1 {
        let evm_tx: FindoraEVMTxWrap = serde_json::from_value(value.clone()).unwrap();
        let hash = evm_tx.hash();
        format!("{hash:?}")
    } else {
        "".to_string()
    };

    let tx = TransactionResponse {
        tx_hash,
        evm_tx_hash,
        block_hash,
        height,
        timestamp,
        ty,
        code,
        log,
        origin,
        result,
        value,
    };

    Ok(Json(tx))
}
