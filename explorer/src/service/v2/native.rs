use crate::service::api::Api;
use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;

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
    pub code: i32,
    pub message: String,
    pub data: Option<V2NativeTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub height: i64,
    pub timestamp: i64,
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
    let content: Value = row.try_get("content")?;

    let res = V2NativeTx {
        tx_hash: tx,
        block_hash: block,
        height,
        timestamp,
        value: content,
    };

    Ok(V2NativeTxResponse::Ok(Json(V2NativeTxResult {
        code: 200,
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum V2NativeTxsResponse {
    #[oai(status = 200)]
    Ok(Json<V2NativeTxsResult>),
    #[oai(status = 400)]
    BadRequest(Json<V2NativeTxsResult>),
    #[oai(status = 404)]
    NotFound(Json<V2NativeTxsResult>),
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeTxsResult {
    pub code: i32,
    pub message: String,
    pub data: Option<NativeTxsData>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct NativeTxsData {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub txs: Vec<V2NativeTx>,
}

pub async fn v2_get_native_txs(
    api: &Api,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2NativeTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut sql_query = String::from("SELECT * FROM native_txs ");
    let mut sql_total = String::from("SELECT count(*) as cnt FROM native_txs ");
    let mut params: Vec<String> = vec![];

    let mut txs: Vec<V2NativeTx> = vec![];

    if let Some(sender) = from.0 {
        let pk = public_key_from_bech32(sender.as_str());
        if pk.is_err() {
            return Ok(V2NativeTxsResponse::BadRequest(Json(V2NativeTxsResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        let base64_pk = public_key_to_base64(&pk);

        params.push(format!(
            "(content @? '$.TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{}\")') ",
            base64_pk
        ));
    }
    if let Some(receiver) = to.0 {
        let pk = public_key_from_bech32(receiver.as_str());
        if pk.is_err() {
            return Ok(V2NativeTxsResponse::BadRequest(Json(V2NativeTxsResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        let base64_pk = public_key_to_base64(&pk);
        params.push(format!(
            "(content @? '$.TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{}\")') ",
            base64_pk
        ));
    }

    if !params.is_empty() {
        sql_query = sql_query.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    }
    sql_query = sql_query.add(
        format!(
            "ORDER BY height DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let row_total = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_total.try_get("cnt")?;
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let tx_hash: String = row.try_get("tx")?;
        let block_hash: String = row.try_get("block")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let content: Value = row.try_get("content")?;
        txs.push(V2NativeTx {
            tx_hash,
            block_hash,
            height,
            timestamp,
            value: content,
        });
    }

    Ok(V2NativeTxsResponse::Ok(Json(V2NativeTxsResult {
        code: 200,
        message: "".to_string(),
        data: Some(NativeTxsData {
            total,
            page,
            page_size,
            txs,
        }),
    })))
}
