use crate::Api;
use anyhow::Result;
use module::db::tx::Transaction;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetTxResponse {
    #[oai(status = 200)]
    Ok(Json<Transaction>),
}

pub async fn get_tx(api: &Api, tx_id: Path<String>) -> Result<GetTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from transaction where txid = '{}'", tx_id.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let tx_id: String = row.try_get("txid")?;
    let _ty: i32 = row.try_get("ty")?;
    let value: Value = row.try_get("value")?;
    let code: i64 = row.try_get("code")?;
    let log: String = row.try_get("log")?;

    let tx = Transaction {
        txid: tx_id,
        value,
        code,
        log,
        events: vec![],
    };

    Ok(GetTxResponse::Ok(Json(tx)))
}
