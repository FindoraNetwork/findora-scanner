use crate::{Api, GetBlocksParam};
use anyhow::Result;
use module::display::block::DisplayBlock;
use poem::web::Query;
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

pub async fn get_block_by_height(api: &Api, height: Path<i64>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    //let txs: Value = row.try_get("txs")?;
    let size: i64 = row.try_get("size")?;
    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count: 0, //txs.as_array().unwrap().len(),
        size,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(block)))
}

pub async fn get_block_by_hash(api: &Api, hash: Path<String>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE block_id = '{}'", hash.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    //let txs: Value = row.try_get("txs")?;
    let size: i64 = row.try_get("size")?;
    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count: 0, //txs.as_array().unwrap().len(),
        size,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(block)))
}

pub async fn get_blocks(api: &Api, param: Query<GetBlocksParam>) -> Result<GetBlocksResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM block ");
    let mut params: Vec<String> = vec![];

    if let Some(begin_height) = param.0.begin_height {
        params.push(format!(" height >= {} ", begin_height));
    }
    if let Some(end_height) = param.0.end_height {
        params.push(format!(" height <= {} ", end_height));
    }
    if let Some(begin_time) = param.0.begin_time {
        params.push(format!(" time >= '{}' ", NaiveDateTime::from_timestamp(begin_time, 0)));
    }
    if let Some(end_time) = param.0.end_time {
        params.push(format!(" time <= '{}' ", NaiveDateTime::from_timestamp(end_time, 0)));
    }
    let page = param.0.page.unwrap_or(1);
    let page_size = param.0.page_size.unwrap_or(10);
    if !params.is_empty() {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ")
    }
    sql_str += &format!(
        " ORDER BY time DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );
    println!("{}", sql_str);
    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;

    let mut blocks: Vec<DisplayBlock> = vec![];
    for row in rows.iter() {
        let block_id: String = row.try_get("block_id")?;
        let height: i64 = row.try_get("height")?;
        let time: NaiveDateTime = row.try_get("time")?;
        let app_hash: String = row.try_get("app_hash")?;
        let proposer: String = row.try_get("proposer")?;
        //let txs: Value = r.try_get("txs")?;
        let size: i64 = row.try_get("size")?;
        let block = DisplayBlock {
            block_id,
            height,
            time: time.timestamp(),
            tx_count: 0, //txs.as_array().unwrap().len(),
            size,
            app_hash,
            proposer,
        };
        blocks.push(block);
    }

    Ok(GetBlocksResponse::Ok(Json(blocks)))
}
