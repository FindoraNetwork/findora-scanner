use crate::Api;
use anyhow::Result;
use module::display::block::DisplayBlock;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetBlockResponse {
    #[oai(status = 200)]
    Ok(Json<DisplayBlock>),
}

#[derive(ApiResponse)]
pub enum GetBlocksResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<DisplayBlock>>),
}

pub async fn get_block(api: &Api, height: Path<i64>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from block where height = {}", height.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let txs: Value = row.try_get("txs")?;

    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count: txs.as_array().unwrap().len(),
        size: 0,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(block)))
}

pub async fn get_block_by_hash(api: &Api, hash: Path<String>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from block where block_id = {}", hash.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let txs: Value = row.try_get("txs")?;

    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count: txs.as_array().unwrap().len(),
        size: 0,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(block)))
}

pub async fn get_blocks(
    api: &Api,
    begin_time: Path<i64>,
    end_time: Path<i64>,
    page_size: Path<i64>,
    page: Path<i64>,
) -> Result<GetBlocksResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let pg_size = if page_size.0 == 0 { 10 } else { page_size.0 };
    let pg = if page.0 <= 0 { 1 } else { page.0 };

    let mut sql_str = String::from("SELECT * FROM block ");
    let mut params: Vec<String> = vec![];
    if begin_time.is_positive() {
        params.push(format!(" timestamp >= {} ", begin_time.0))
    }
    if end_time.is_positive() {
        params.push(format!(" timestamp <= {} ", end_time.0))
    }
    if params.len() > 0 {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ")
    }
    sql_str += &String::from(format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        pg_size,
        (pg - 1) * pg_size
    ));

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;

    let mut blocks: Vec<DisplayBlock> = vec![];
    for r in rows.iter() {
        let block_id: String = r.try_get("block_id")?;
        let height: i64 = r.try_get("height")?;
        let time: NaiveDateTime = r.try_get("time")?;
        let app_hash: String = r.try_get("app_hash")?;
        let proposer: String = r.try_get("proposer")?;
        let txs: Value = r.try_get("txs")?;

        let block = DisplayBlock {
            block_id,
            height,
            time: time.timestamp(),
            tx_count: txs.as_array().unwrap().len(),
            size: 0,
            app_hash,
            proposer,
        };
        blocks.push(block);
    }

    Ok(GetBlocksResponse::Ok(Json(blocks)))
}
