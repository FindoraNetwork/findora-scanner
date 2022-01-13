use crate::Api;
use anyhow::Result;
use module::db::tx_ref::TxRef;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum TxSearchResponse {
    #[oai(status = 200)]
    Ok(Json<TxRef>),
}

pub async fn tx_search(api: &Api, tx_id: Path<String>) -> Result<TxSearchResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from tx_ref where txid = '{}'", tx_id.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;
    let txid: String = row.try_get("txid")?;
    let from: String = row.try_get("from")?;
    let to: String = row.try_get("to")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;

    let tx = TxRef {
        txid,
        from,
        to,
        height,
        timestamp,
    };

    Ok(TxSearchResponse::Ok(Json(tx)))
}