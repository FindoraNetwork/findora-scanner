use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2NativeTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2NativeTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2NativeTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub height: i64,
    pub timestamp: i64,
    pub inputs: Value,
    pub outputs: Value,
    pub value: Value,
}

pub async fn v2_get_native_tx(api: &Api, tx_hash: Path<String>) -> Result<V2NativeTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM native_txs WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let inputs: Value = row.try_get("inputs")?;
    let outputs: Value = row.try_get("outputs")?;
    let value: Value = row.try_get("content")?;

    let res = V2NativeTx {
        tx_hash: tx,
        block_hash: block,
        height,
        timestamp,
        inputs,
        outputs,
        value,
    };

    Ok(V2NativeTxResponse::Ok(Json(V2NativeTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
