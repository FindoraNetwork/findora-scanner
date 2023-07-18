use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2DelegationTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2DelegationTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2DelegationTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2DelegationTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2DelegationTx {
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

pub async fn v2_get_delegation_tx(
    api: &Api,
    tx_hash: Path<String>,
) -> Result<V2DelegationTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM delegations WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let amount: i64 = row.try_get("amount")?;
    let sender: String = row.try_get("sender")?;
    let validator: String = row.try_get("validator")?;
    let new_validator: String = row.try_get("new_validator")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2DelegationTx {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        amount,
        validator,
        new_validator,
        height,
        timestamp,
        value,
    };

    Ok(V2DelegationTxResponse::Ok(Json(V2DelegationTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
