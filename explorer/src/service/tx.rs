use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use module::db::tx::Transaction;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetTxResponse {
    #[oai(status = 200)]
    Ok(Json<Transaction>),
}

#[derive(ApiResponse)]
pub enum GetTxsResponse {
    #[oai(status = 200)]
    Ok(Json<TxsResult>),
    #[oai(status = 400)]
    Err(Json<String>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxsResult {
    counts: usize,
    txs: Vec<Transaction>,
}

pub async fn get_tx(api: &Api, tx_id: Path<String>) -> Result<GetTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM transaction WHERE txid = '{}'", tx_id.0);
    let res = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match res {
        Ok(row) => row,
        _ => {
            return Ok(GetTxResponse::Ok(Json(Transaction::default())));
        }
    };
    let tx_id: String = row.try_get("txid")?;
    let block_id: String = row.try_get("block_id")?;
    let ty: i32 = row.try_get("ty")?;
    let value: Value = row.try_get("value")?;
    let code: i64 = row.try_get("code")?;
    let log: String = row.try_get("log")?;

    let tx = Transaction {
        txid: tx_id,
        block_id,
        ty,
        value,
        code,
        log,
        events: vec![],
    };

    Ok(GetTxResponse::Ok(Json(tx)))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_txs(
    api: &Api,
    block_id: Query<Option<String>>,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    ty: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<GetTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from(
        "SELECT * FROM transaction AS t LEFT JOIN block AS b ON t.block_id=b.block_id ",
    );

    let mut params: Vec<String> = vec![];
    if let Some(block_id) = block_id.0 {
        params.push(format!(" block_id='{}' ", block_id));
    }
    if let Some(from_address) = from.0 {
        let pk = public_key_from_bech32(from_address.as_str());
        if pk.is_err() {
            return Ok(GetTxsResponse::Err(Json(String::from("invalid address"))));
        }
        params.push(format!(
            " (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\" )') ",
            public_key_to_base64(&pk.unwrap())));
    }
    if let Some(to_address) = to.0 {
        let pk = public_key_from_bech32(to_address.as_str());
        if pk.is_err() {
            return Ok(GetTxsResponse::Err(Json(String::from("invalid address"))));
        }
        params.push(format!(
            " (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') ",
            public_key_to_base64(&pk.unwrap())));
    }
    if let Some(ty) = ty.0 {
        params.push(format!(" ty={} ", ty));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(
            " time>='{}' ",
            NaiveDateTime::from_timestamp(start_time, 0)
        ));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(
            " time<='{}' ",
            NaiveDateTime::from_timestamp(end_time, 0)
        ));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ");
    }
    sql_str += &format!(" ORDER BY time DESC LIMIT {} OFFSET {}", page_size, (page - 1) * page_size);

    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let mut txs: Vec<Transaction> = vec![];
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(GetTxsResponse::Ok(Json(TxsResult::default())));
        }
    };

    for row in rows.iter() {
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("value")?;
        let code: i64 = row.try_get("code")?;
        let log: String = row.try_get("log")?;

        let tx = Transaction {
            txid: tx_id,
            block_id,
            ty,
            value,
            code,
            log,
            events: vec![],
        };

        txs.push(tx);
    }

    Ok(GetTxsResponse::Ok(Json(TxsResult {
        counts: txs.len(),
        txs,
    })))
}
