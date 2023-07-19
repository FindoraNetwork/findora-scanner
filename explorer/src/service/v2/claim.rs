use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2ClaimTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2ClaimTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2ClaimTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2ClaimTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2ClaimTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub amount: u64,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_claim_tx(api: &Api, tx_hash: Path<String>) -> Result<V2ClaimTxResponse> {
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

    let res = V2ClaimTx {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        amount: amount as u64,
        height,
        timestamp,
        value,
    };

    Ok(V2ClaimTxResponse::Ok(Json(V2ClaimTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
