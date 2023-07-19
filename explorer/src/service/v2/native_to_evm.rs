use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2NativeToEvmTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2NativeToEvmTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2NativeToEvmTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub asset: String,
    pub amount: String,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}
pub async fn v2_get_n2e_tx(api: &Api, tx_hash: Path<String>) -> Result<V2NativeToEvmTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!("SELECT * FROM n2e WHERE tx='{}'", tx_hash.0.to_lowercase());

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let sender: String = row.try_get("sender")?;
    let receiver: String = row.try_get("receiver")?;
    let asset: String = row.try_get("asset")?;
    let amount: String = row.try_get("amount")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2NativeToEvmTx {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        to: receiver,
        asset,
        amount,
        height,
        timestamp,
        value,
    };

    Ok(V2NativeToEvmTxResponse::Ok(Json(V2NativeToEvmTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
