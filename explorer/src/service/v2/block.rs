use crate::service::error::Result;
use crate::service::QueryResult;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use module::rpc::block::{Block, BlockHeader, BlockId, BlockRPC};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockResponse {
    pub block_hash: String,
    pub block_num: i64,
    pub app_hash: String,
    pub proposer: String,
    pub num_txs: i64,
    pub block_size: i64,
    pub block_id: BlockId,
    pub block_header: BlockHeader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullBlockResponse {
    pub block_id: BlockId,
    pub block: Block,
}

pub async fn get_full_block_by_height(
    State(state): State<Arc<AppState>>,
    Path(num): Path<i64>,
) -> Result<Json<FullBlockResponse>> {
    let mut conn = state.pool.acquire().await?;

    let sql_query = r#"SELECT block_data FROM block WHERE height=$1"#;
    let row = sqlx::query(sql_query)
        .bind(num)
        .fetch_one(&mut *conn)
        .await?;
    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;
    let full_block = FullBlockResponse {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(Json(full_block))
}

pub async fn get_simple_block_by_height(
    State(state): State<Arc<AppState>>,
    Path(num): Path<i64>,
) -> Result<Json<BlockResponse>> {
    let mut conn = state.pool.acquire().await?;

    let sql_query =
        "SELECT block_hash,height,size,tx_count,time,app_hash,proposer,block_data FROM block WHERE height=$1";
    let row = sqlx::query(sql_query)
        .bind(num)
        .fetch_one(&mut *conn)
        .await?;

    let block_hash: String = row.try_get("block_hash")?;
    let block_num: i64 = row.try_get("height")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;

    let block = BlockResponse {
        block_hash,
        num_txs,
        block_size,
        app_hash,
        proposer,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
        block_num,
    };

    Ok(Json(block))
}

pub async fn get_full_block_by_hash(
    State(state): State<Arc<AppState>>,
    Path(hash): Path<String>,
) -> Result<Json<FullBlockResponse>> {
    let mut conn = state.pool.acquire().await?;

    let sql_query = "SELECT block_data FROM block WHERE block_hash=$1";
    let row = sqlx::query(sql_query)
        .bind(hash)
        .fetch_one(&mut *conn)
        .await?;
    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;
    let full_block = FullBlockResponse {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(Json(full_block))
}

pub async fn get_simple_block_by_hash(
    State(state): State<Arc<AppState>>,
    Path(hash): Path<String>,
) -> Result<Json<BlockResponse>> {
    let mut conn = state.pool.acquire().await?;

    let sql_query = r#"SELECT block_hash,height,size,tx_count,time,app_hash,proposer,block_data FROM block WHERE block_hash=$1"#;
    let row = sqlx::query(sql_query)
        .bind(hash)
        .fetch_one(&mut *conn)
        .await?;
    let block_hash: String = row.try_get("block_hash")?;
    let block_num: i64 = row.try_get("height")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;
    let block = BlockResponse {
        block_hash,
        num_txs,
        block_size,
        app_hash,
        proposer,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
        block_num,
    };

    Ok(Json(block))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockByHeightParams {
    pub num: i64,
}

#[allow(dead_code)]
pub async fn get_block_by_num(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetBlockByHeightParams>,
) -> Result<Json<BlockResponse>> {
    let mut pool = state.pool.acquire().await?;

    let sql_query = r#"SELECT block_hash,height,size,tx_count,time,app_hash,proposer,block_data FROM block WHERE height=$1"#;
    let row = sqlx::query(sql_query)
        .bind(params.num)
        .fetch_one(&mut *pool)
        .await?;

    let block_hash: String = row.try_get("block_hash")?;
    let block_num: i64 = row.try_get("height")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;

    Ok(Json(BlockResponse {
        block_hash,
        block_num,
        app_hash,
        proposer,
        num_txs,
        block_size,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
    }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockByHashParams {
    pub hash: String,
}

pub async fn get_block_by_hash(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetBlockByHashParams>,
) -> Result<Json<BlockResponse>> {
    let mut pool = state.pool.acquire().await?;

    let sql_query = r#"SELECT block_hash,height,size,tx_count,time,app_hash,proposer,block_data
        FROM block WHERE block_hash=$1"#;

    let row = sqlx::query(sql_query)
        .bind(params.hash.to_uppercase())
        .fetch_one(&mut *pool)
        .await?;

    let block_hash: String = row.try_get("block_hash")?;
    let block_num: i64 = row.try_get("height")?;
    let app_hash: String = row.try_get("app_hash")?;
    let proposer: String = row.try_get("proposer")?;
    let block_size: i64 = row.try_get("size")?;
    let num_txs: i64 = row.try_get("tx_count")?;
    let block_data: Value = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data)?;

    Ok(Json(BlockResponse {
        block_hash,
        block_num,
        app_hash,
        proposer,
        num_txs,
        block_size,
        block_id: block_rpc.block_id,
        block_header: block_rpc.block.header,
    }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_blocks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetBlocksParams>,
) -> Result<Json<QueryResult<Vec<BlockResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let sql_total = "SELECT max(height) FROM block";
    let row = sqlx::query(sql_total).fetch_one(&mut *pool).await?;
    let total = row.try_get("max")?;

    let sql_query = r#"SELECT block_hash,height,size,tx_count,time,app_hash,proposer,block_data
        FROM block ORDER BY height DESC LIMIT $1 OFFSET $2"#;

    let rows = sqlx::query(sql_query)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&mut *pool)
        .await?;

    let mut blocks: Vec<BlockResponse> = vec![];
    for row in rows {
        let block_hash: String = row.try_get("block_hash")?;
        let block_num: i64 = row.try_get("height")?;
        let app_hash: String = row.try_get("app_hash")?;
        let proposer: String = row.try_get("proposer")?;
        let block_size: i64 = row.try_get("size")?;
        let num_txs: i64 = row.try_get("tx_count")?;
        let block_data: Value = row.try_get("block_data")?;
        let block_rpc: BlockRPC = serde_json::from_value(block_data)?;

        blocks.push(BlockResponse {
            block_hash,
            block_num,
            app_hash,
            proposer,
            num_txs,
            block_size,
            block_id: block_rpc.block_id,
            block_header: block_rpc.block.header,
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: blocks,
    }))
}
