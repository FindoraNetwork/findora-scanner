use crate::service::api::Api;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2EvmToNativeTxsResponse {
    #[oai(status = 200)]
    Ok(Json<V2EvmToNativeTxsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmToNativeTxsResult {
    pub code: u16,
    pub message: String,
    pub data: V2EvmToNativeTxsData,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmToNativeTxsData {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub txs: Vec<V2EvmToNativeTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2EvmToNativeTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub asset: String,
    pub amount: String,
    pub decimal: i32,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}
#[allow(dead_code)]
pub async fn v2_get_e2n_txs(
    api: &Api,
    page: Query<Option<i32>>,
    page_size: Query<Option<i32>>,
) -> anyhow::Result<V2EvmToNativeTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let sql_total = "SELECT count(*) FROM e2n";
    let row = sqlx::query(sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("count")?;

    let sql_query = format!("SELECT tx_hash,block_hash,sender,receiver,asset,amount,decimal,height,timestamp,value FROM e2n ORDER BY timestamp DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size);
    let mut res: Vec<V2EvmToNativeTx> = vec![];
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let from: String = row.try_get("sender")?;
        let to: String = row.try_get("receiver")?;
        let asset: String = row.try_get("asset")?;
        let decimal: i32 = row.try_get("decimal")?;
        let amount: String = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("value")?;
        res.push(V2EvmToNativeTx {
            tx_hash,
            block_hash,
            from,
            to,
            asset,
            amount,
            decimal,
            height,
            timestamp,
            value,
        })
    }

    Ok(V2EvmToNativeTxsResponse::Ok(Json(V2EvmToNativeTxsResult {
        code: 200,
        message: "".to_string(),
        data: V2EvmToNativeTxsData {
            total,
            page,
            page_size,
            txs: res,
        },
    })))
}
