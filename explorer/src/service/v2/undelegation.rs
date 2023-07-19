use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2UndelegationTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2UndelegationTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2UndelegationTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2UndelegationTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2UndelegationTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub new_delegator: String,
    pub target_validator: String,
    pub amount: u64,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_undelegation_tx(
    api: &Api,
    tx_hash: Path<String>,
) -> Result<V2UndelegationTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT * FROM undelegations WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let sender: String = row.try_get("sender")?;
    let amount: i64 = row.try_get("amount")?;
    let target_validator: String = row.try_get("target_validator")?;
    let new_delegator: String = row.try_get("new_delegator")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2UndelegationTx {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        new_delegator,
        target_validator,
        amount: amount as u64,
        height,
        timestamp,
        value,
    };

    Ok(V2UndelegationTxResponse::Ok(Json(V2UndelegationTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
