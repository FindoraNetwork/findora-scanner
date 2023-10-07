use crate::service::v1::transaction::{TxsData, TxsRes, TxsResponse};
use crate::Api;
use anyhow::Result;
use log::debug;
use module::schema::TransactionResponse;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;

#[allow(clippy::too_many_arguments)]
pub async fn v2_get_txs(
    api: &Api,
    block_hash: Query<Option<String>>,
    block_height: Query<Option<i64>>,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    ty: Query<Option<i32>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT tx_hash,block_hash,ty_sub,timestamp,height,code,log,origin,result,value FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as cnt FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_hash) = block_hash.0 {
        params.push(format!("block_hash='{block_hash}' "));
    }
    if let Some(height) = block_height.0 {
        params.push(format!("height={height} "));
    }
    if let Some(from_address) = from.0 {
        params.push(format!("sender='{}' ", from_address));
    }
    if let Some(to_address) = to.0 {
        params.push(format!(
            "(receiver @? '$.addrs[*] ? (@==\"{}\")') ",
            to_address
        ));
    }
    if let Some(ty) = ty.0 {
        params.push(format!("ty={ty} "));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!("timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!("timestamp<={end_time} "));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add("WHERE ").add(params.join("AND ").as_str());
        sql_total = sql_total.add("WHERE ").add(params.join("AND ").as_str());
    }

    sql_str = sql_str.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );
    debug!("{}", sql_str);
    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<TransactionResponse> = vec![];

    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty_sub")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log: String = "".to_string();
        let origin: String = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };

        txs.push(tx);
    }

    // total items
    let row_cnt = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_cnt.try_get("cnt")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}
