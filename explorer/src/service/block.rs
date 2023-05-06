use crate::Api;
use anyhow::Result;
use module::display::block::{DisplayBlock, DisplayFullBlock};
use module::rpc::block::BlockRPC;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::ops::Add;

#[derive(ApiResponse)]
pub enum SimpleBlockResponse {
    #[oai(status = 200)]
    Ok(Json<SimpleBlock>),
    #[oai(status = 400)]
    BadRequest(Json<SimpleBlock>),
    #[oai(status = 404)]
    NotFound(Json<SimpleBlock>),
    #[oai(status = 500)]
    InternalError(Json<SimpleBlock>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct SimpleBlock {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayBlock>,
}

#[derive(ApiResponse)]
pub enum FullBlockResponse {
    #[oai(status = 200)]
    Ok(Json<FullBlock>),
    #[oai(status = 400)]
    BadRequest(Json<FullBlock>),
    #[oai(status = 404)]
    NotFound(Json<FullBlock>),
    #[oai(status = 500)]
    InternalError(Json<FullBlock>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct FullBlock {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayFullBlock>,
}

#[derive(ApiResponse)]
pub enum BlocksResponse {
    #[oai(status = 200)]
    Ok(Json<BlocksResult>),
    #[oai(status = 400)]
    BadRequest(Json<BlocksResult>),
    #[oai(status = 404)]
    NotFound(Json<BlocksResult>),
    #[oai(status = 500)]
    InternalError(Json<BlocksResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct BlocksResult {
    pub code: i32,
    pub message: String,
    pub data: Option<BlocksData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct BlocksData {
    page: i64,
    page_size: i64,
    total: i64,
    blocks: Vec<DisplayBlock>,
}

/// return full block by given height.
pub async fn get_full_block_by_height(api: &Api, height: Path<i64>) -> Result<FullBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
    let full_block = DisplayFullBlock {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(FullBlockResponse::Ok(Json(FullBlock {
        code: 200,
        message: "".to_string(),
        data: Some(full_block),
    })))
}

/// return simple block by given height.
pub async fn get_block_by_height(api: &Api, height: Path<i64>) -> Result<SimpleBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_hash: String = row.try_get("block_hash")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();

    let block = DisplayBlock {
        block_hash,
        num_txs,
        block_size,
        app_hash,
        proposer,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
    };

    Ok(SimpleBlockResponse::Ok(Json(SimpleBlock {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

/// return full block by given block hash.
pub async fn get_full_block_by_hash(api: &Api, hash: Path<String>) -> Result<FullBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM block WHERE block_hash = '{}'", hash.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
    let full_block = DisplayFullBlock {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(FullBlockResponse::Ok(Json(FullBlock {
        code: 200,
        message: "".to_string(),
        data: Some(full_block),
    })))
}

/// return simple block by given block hash.
pub async fn get_block_by_hash(api: &Api, hash: Path<String>) -> Result<SimpleBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM block WHERE block_hash = '{}'", hash.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let block_hash: String = row.try_get("block_hash")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
    let block = DisplayBlock {
        block_hash,
        num_txs,
        block_size,
        app_hash,
        proposer,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
    };

    Ok(SimpleBlockResponse::Ok(Json(SimpleBlock {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

/// return simple blocks.
pub async fn get_blocks(
    api: &Api,
    start_height: Query<Option<i64>>,
    end_height: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<BlocksResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM block ");
    let mut sql_total = String::from("SELECT count(*) as total FROM block ");
    let mut params: Vec<String> = vec![];

    if let Some(start_height) = start_height.0 {
        params.push(format!(" height >= {start_height} "));
    }
    if let Some(end_height) = end_height.0 {
        params.push(format!(" height <= {end_height} "));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(
            " time >= '{}' ",
            NaiveDateTime::from_timestamp_opt(start_time, 0).unwrap()
        ));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(
            " time <= '{}' ",
            NaiveDateTime::from_timestamp_opt(end_time, 0).unwrap()
        ));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    if !params.is_empty() {
        sql_str = sql_str.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    }
    sql_str = sql_str.add(
        format!(
            " ORDER BY time DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;

    let mut blocks: Vec<DisplayBlock> = vec![];
    for row in rows {
        let block_hash: String = row.try_get("block_hash")?;
        let app_hash: String = row.try_get("app_hash")?;
        let proposer: String = row.try_get("proposer")?;
        let block_size: i64 = row.try_get("size")?;
        let num_txs: i64 = row.try_get("tx_count")?;
        let block_data: Value = row.try_get("block_data")?;
        let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
        let block = DisplayBlock {
            block_hash,
            num_txs,
            block_size,
            app_hash,
            proposer,
            block_id: block_rpc.block_id,
            block_header: block_rpc.block.header,
        };
        blocks.push(block);
    }

    // total items
    let total: i64;
    if blocks.is_empty() {
        let res = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await;
        total = res.unwrap().try_get("total")?;
    } else {
        total = blocks[0].block_header.height.parse()?;
    }

    Ok(BlocksResponse::Ok(Json(BlocksResult {
        code: 200,
        message: "".to_string(),
        data: Some(BlocksData {
            page,
            page_size,
            total,
            blocks,
        }),
    })))
}
