use crate::service::api::Api;
use crate::service::v2::TransactionType;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2EvmTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2EvmTxResult>),
    #[oai(status = 404)]
    NotFound(Json<V2EvmTxResult>),
    #[oai(status = 500)]
    InternalError(Json<V2EvmTxResult>),
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmTxResult {
    pub code: u16,
    pub message: String,
    pub data: Vec<V2EvmTx>,
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

    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<V2EvmTx> = vec![];
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
        })
    }

    Ok(V2EvmTxResponse::Ok(Json(V2EvmTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: txs,
    })))
}
