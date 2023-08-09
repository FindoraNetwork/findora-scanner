use crate::service::api::Api;
use crate::service::v2::TransactionType;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;

#[derive(ApiResponse)]
pub enum V2EvmTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2EvmTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2EvmTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmTx {
    pub tx_hash: String,
    pub evm_tx_hash: String,
    pub block: String,
    pub from: String,
    pub to: String,
    pub ty: i8,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_evm_tx(api: &Api, tx_hash: Path<String>) -> Result<V2EvmTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM evm_txs WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let evm_tx: String = row.try_get("evm_tx")?;
    let sender: String = row.try_get("sender")?;
    let receiver: String = row.try_get("receiver")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2EvmTx {
        tx_hash: tx,
        evm_tx_hash: evm_tx,
        block,
        from: sender,
        to: receiver,
        ty: TransactionType::Evm as i8,
        height,
        timestamp,
        value,
    };

    Ok(V2EvmTxResponse::Ok(Json(V2EvmTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum V2EvmTxsResponse {
    #[oai(status = 200)]
    Ok(Json<V2EvmTxsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmTxsResult {
    pub code: u16,
    pub message: String,
    pub data: V2EvmTxsData,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmTxsData {
    pub txs: Vec<V2EvmTx>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}

pub async fn v2_get_evm_txs(
    api: &Api,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2EvmTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let mut sql_query = String::from("SELECT * FROM evm_txs ");
    let mut sql_total = String::from("SELECT count(*) AS total FROM evm_txs ");

    let mut params: Vec<String> = vec![];
    if let Some(sender) = from.0 {
        params.push(format!("sender='{sender}' "));
    }
    if let Some(receiver) = to.0 {
        params.push(format!("receiver='{receiver}' "));
    }

    if !params.is_empty() {
        sql_query = sql_query.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    }

    sql_query = sql_query.add(
        format!(
            "ORDER BY height DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row.try_get("total")?;

    let mut txs: Vec<V2EvmTx> = vec![];
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let evm_tx: String = row.try_get("evm_tx")?;
        let sender: String = row.try_get("sender")?;
        let receiver: String = row.try_get("receiver")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;

        txs.push(V2EvmTx {
            tx_hash: tx,
            evm_tx_hash: evm_tx,
            block,
            from: sender,
            to: receiver,
            ty: TransactionType::Evm as i8,
            height,
            timestamp,
            value,
        });
    }

    Ok(V2EvmTxsResponse::Ok(Json(V2EvmTxsResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: V2EvmTxsData {
            txs,
            page,
            page_size,
            total,
        },
    })))
}
