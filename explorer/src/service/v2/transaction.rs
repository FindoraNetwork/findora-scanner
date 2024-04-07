use crate::service::v2::error::internal_error;
use crate::service::v2::error::Result;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use module::schema::TransactionResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetTxByHashParams {
    pub hash: String,
}

pub async fn get_tx_by_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetTxByHashParams>,
) -> Result<Json<TransactionResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let sql_query = r#"SELECT tx_hash,block_hash,height,timestamp,ty_sub,code,log,origin,result,value
        FROM transaction WHERE tx_hash=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let tx_hash: String = row.try_get("tx_hash").map_err(internal_error)?;
    let block_hash: String = row.try_get("block_hash").map_err(internal_error)?;
    let ty: i32 = row.try_get("ty_sub").map_err(internal_error)?;
    let timestamp: i64 = row.try_get("timestamp").map_err(internal_error)?;
    let height: i64 = row.try_get("height").map_err(internal_error)?;
    let code: i64 = row.try_get("code").map_err(internal_error)?;
    let log: String = row.try_get("log").map_err(internal_error)?;
    let origin: String = row.try_get("origin").map_err(internal_error)?;
    let result: Value = row.try_get("result").map_err(internal_error)?;
    let value: Value = row.try_get("value").map_err(internal_error)?;

    let tx = TransactionResponse {
        tx_hash,
        evm_tx_hash: "".to_string(),
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
