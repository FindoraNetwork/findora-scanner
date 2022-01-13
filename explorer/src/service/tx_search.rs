use crate::Api;
use anyhow::Result;
use module::db::tx::{TxDetail};
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use serde_json::Value;
use sqlx::Row;
use serde::{Deserialize, Serialize};
use poem_openapi::Object;


#[derive(ApiResponse)]
pub enum TxSearchResponse {
    #[oai(status = 200)]
    Ok(Json<TxSearchRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxSearchRes {
    pub total_count: i64,
    pub txs: Vec<TxDetail>,
}

pub async fn tx_search(api: &Api, query: Path<String>) -> Result<TxSearchResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from tx_detail where {}", query.0);
    let row = sqlx::query(str.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<TxDetail> = vec![];
    for v in row.iter() {
        let hash: String = v.try_get("hash")?;
        let height: i64 = v.try_get("height")?;
        let index: i64 = v.try_get("index")?;
        let tx_result: Value = v.try_get("tx_result")?;
        let tx: String = v.try_get("tx")?;
        let tx_detail = TxDetail {
            hash,
            height,
            index,
            tx_result,
            tx,
        };
        txs.push(tx_detail);
    }

    let res = TxSearchRes{
        total_count: row.len() as i64,
        txs,
    };

    Ok(TxSearchResponse::Ok(Json(res)))
}