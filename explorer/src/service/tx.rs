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

#[derive(ApiResponse)]
pub enum GetTxsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Transaction>>),
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

#[allow(clippy::too_many_arguments)]
pub async fn get_txs(
    api: &Api,
    block: Path<String>,
    typ: Path<String>,
    from_address: Path<String>,
    to_address: Path<String>,
    begin_time: Path<i64>,
    end_time: Path<i64>,
    page: Path<i64>,
    page_size: Path<i64>,
) -> Result<GetTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let pg_size = if page_size.0 <= 0 { 10 } else { page_size.0 };
    let pg = if page.0 <= 0 { 1 } else { page.0 };

    let mut sql_str = String::from("SELECT * FROM transaction_ref ");
    let mut params: Vec<String> = vec![];
    if !block.is_empty() {
        params.push(format!(" block_id={} ", block.0));
    }
    if !typ.is_empty() {
        params.push(format!(" typ={} ", typ.0));
    }
    if !from_address.is_empty() {
        params.push(format!(" from_address={} ", from_address.0));
    }
    if !to_address.is_empty() {
        params.push(format!(" to_address={} ", to_address.0));
    }
    if begin_time.is_positive() {
        params.push(format!(" time>={} ", begin_time.0));
    }
    if end_time.is_positive() {
        params.push(format!(" time<={} ", end_time.0));
    }
    if !params.is_empty() {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ");
    }
    sql_str += &format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        pg_size,
        (pg - 1) * pg_size
    );

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<Transaction> = vec![];
    for r in rows.iter() {
        let tx_id: String = r.try_get("txid")?;
        let _ty: i32 = r.try_get("ty")?;
        let value: Value = r.try_get("value")?;
        let code: i64 = r.try_get("code")?;
        let log: String = r.try_get("log")?;

        let tx = Transaction {
            txid: tx_id,
            value,
            code,
            log,
            events: vec![],
        };

        txs.push(tx);
    }

    Ok(GetTxsResponse::Ok(Json(txs)))
}
