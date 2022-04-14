use crate::Api;
use anyhow::Result;
use module::display::block::DisplayBlock;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Error::RowNotFound;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetBlockResponse {
    #[oai(status = 200)]
    Ok(Json<BlockRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct BlockRes {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayBlock>,
}

#[derive(ApiResponse)]
pub enum GetBlocksResponse {
    #[oai(status = 200)]
    Ok(Json<BlocksRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct BlocksRes {
    pub code: i32,
    pub message: String,
    pub data: Option<BlocksData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct BlocksData {
    counts: usize,
    blocks: Vec<DisplayBlock>,
}

pub async fn get_block_by_height(api: &Api, height: Path<i64>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let res = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match res {
        Ok(row) => row,
        Err(e) => {
            return match e {
                RowNotFound => Ok(GetBlockResponse::Ok(Json(BlockRes {
                    code: 200,
                    message: "".to_string(),
                    data: Some(DisplayBlock::default()),
                }))),
                _ => Ok(GetBlockResponse::Ok(Json(BlockRes {
                    code: 50001,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let size: i64 = row.try_get("size")?;

    let str = format!(
        "SELECT count(1) as tx_count FROM transaction where block_id='{}'",
        block_id
    );
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;
    let tx_count: i64 = row.try_get("tx_count")?;
    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count,
        size,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(BlockRes {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

pub async fn get_block_by_hash(api: &Api, hash: Path<String>) -> Result<GetBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE block_id = '{}'", hash.0);
    let res = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match res {
        Ok(row) => row,
        Err(e) => {
            return match e {
                RowNotFound => Ok(GetBlockResponse::Ok(Json(BlockRes {
                    code: 200,
                    message: "".to_string(),
                    data: Some(DisplayBlock::default()),
                }))),
                _ => Ok(GetBlockResponse::Ok(Json(BlockRes {
                    code: 50001,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

    let block_id: String = row.try_get("block_id")?;
    let height: i64 = row.try_get("height")?;
    let time: NaiveDateTime = row.try_get("time")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let size: i64 = row.try_get("size")?;

    let str = format!(
        "SELECT count(1) as tx_count FROM transaction where block_id='{}'",
        block_id
    );
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;
    let tx_count: i64 = row.try_get("tx_count")?;

    let block = DisplayBlock {
        block_id,
        height,
        time: time.timestamp(),
        tx_count,
        size,
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(BlockRes {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

pub async fn get_blocks(
    api: &Api,
    start_height: Query<Option<i64>>,
    end_height: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<GetBlocksResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM block ");
    let mut params: Vec<String> = vec![];

    if let Some(start_height) = start_height.0 {
        params.push(format!(" height >= {} ", start_height));
    }
    if let Some(end_height) = end_height.0 {
        params.push(format!(" height <= {} ", end_height));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(
            " time >= '{}' ",
            NaiveDateTime::from_timestamp(start_time, 0)
        ));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(
            " time <= '{}' ",
            NaiveDateTime::from_timestamp(end_time, 0)
        ));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    if !params.is_empty() {
        sql_str += &String::from(" WHERE ");
        sql_str += &params.join(" AND ")
    }
    sql_str += &format!(
        " ORDER BY time DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );
    let mut blocks: Vec<DisplayBlock> = vec![];
    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(GetBlocksResponse::Ok(Json(BlocksRes {
                code: 200,
                message: "".to_string(),
                data: Some(BlocksData::default()),
            })));
        }
    };

    for row in rows.iter() {
        let block_id: String = row.try_get("block_id")?;
        let height: i64 = row.try_get("height")?;
        let time: NaiveDateTime = row.try_get("time")?;
        let app_hash: String = row.try_get("app_hash")?;
        let proposer: String = row.try_get("proposer")?;
        let size: i64 = row.try_get("size")?;

        let str = format!(
            "SELECT count(1) as tx_count FROM transaction where block_id='{}'",
            block_id
        );
        let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;
        let tx_count: i64 = row.try_get("tx_count")?;

        let block = DisplayBlock {
            block_id,
            height,
            time: time.timestamp(),
            tx_count,
            size,
            app_hash,
            proposer,
        };
        blocks.push(block);
    }
    let l = blocks.len();
    Ok(GetBlocksResponse::Ok(Json(BlocksRes {
        code: 200,
        message: "".to_string(),
        data: Some(BlocksData { counts: l, blocks }),
    })))
}
