use crate::{Api, GetTxsParam};
use anyhow::Result;
use module::db::tx::Transaction;
use poem::web::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
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
    Ok(Json<Vec<Transaction>>),
}

pub async fn get_tx(api: &Api, tx_id: Path<String>) -> Result<GetTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM transaction WHERE txid = '{}'", tx_id.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

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

pub async fn get_txs(api: &Api, param: Query<GetTxsParam>) -> Result<GetTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from(
        "SELECT * FROM transaction AS t LEFT JOIN block AS b ON t.block_id=b.block_id ",
    );

    let mut params: Vec<String> = vec![];
    if let Some(block_id) = param.0.block_id {
        params.push(format!(" block_id='{}' ", block_id));
    }
    if let Some(from_address) = param.0.from_address {
        params.push(format!(" (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\" )') ", from_address));
    }
    if let Some(to_address) = param.0.to_address {
        params.push(format!(" (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') ", to_address));
    }
    if let Some(ty) = param.0.ty {
        params.push(format!(" ty={} ", ty));
    }
    if let Some(begin_time) = param.0.begin_time {
        params.push(format!(
            " time>='{}' ",
            NaiveDateTime::from_timestamp(begin_time, 0)
        ));
    }
    if let Some(end_time) = param.0.end_time {
        params.push(format!(
            " time<='{}' ",
            NaiveDateTime::from_timestamp(end_time, 0)
        ));
    }
    let page = param.0.page.unwrap_or(1);
    let page_size = param.0.page_size.unwrap_or(10);

    if !params.is_empty() {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ");
    }
    sql_str += &format!(" LIMIT {} OFFSET {}", page_size, (page - 1) * page_size);

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<Transaction> = vec![];
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

    Ok(GetTxsResponse::Ok(Json(txs)))
}
