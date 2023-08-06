use crate::service::api::Api;
use anyhow::Result;
use module::display::block::{DisplayBlock, DisplayFullBlock};
use module::rpc::block::BlockRPC;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, Row};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2SimpleBlock {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayBlock>,
}

#[derive(ApiResponse)]
pub enum V2SimpleBlockResponse {
    #[oai(status = 200)]
    Ok(Json<V2SimpleBlock>),
    #[oai(status = 400)]
    BadRequest(Json<V2SimpleBlock>),
    #[oai(status = 404)]
    NotFound(Json<V2SimpleBlock>),
    #[oai(status = 500)]
    InternalError(Json<V2SimpleBlock>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2FullBlock {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayFullBlock>,
}

#[derive(ApiResponse)]
pub enum V2FullBlockResponse {
    #[oai(status = 200)]
    Ok(Json<V2FullBlock>),
    #[oai(status = 400)]
    BadRequest(Json<V2FullBlock>),
    #[oai(status = 404)]
    NotFound(Json<V2FullBlock>),
    #[oai(status = 500)]
    InternalError(Json<V2FullBlock>),
}

#[derive(ApiResponse)]
pub enum V2BlocksResponse {
    #[oai(status = 200)]
    Ok(Json<V2BlocksResult>),
    #[oai(status = 400)]
    BadRequest(Json<V2BlocksResult>),
    #[oai(status = 404)]
    NotFound(Json<V2BlocksResult>),
    #[oai(status = 500)]
    InternalError(Json<V2BlocksResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2BlocksResult {
    pub code: i32,
    pub message: String,
    pub data: Option<V2BlocksData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2BlocksData {
    page: i64,
    page_size: i64,
    total: i64,
    blocks: Vec<DisplayBlock>,
}

/// return full block by given height.
pub async fn v2_get_full_block_by_height(
    api: &Api,
    height: Path<i64>,
) -> Result<V2FullBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let row_result = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match row_result {
        Ok(row) => row,
        Err(e) => {
            return match e {
                Error::RowNotFound => Ok(V2FullBlockResponse::NotFound(Json(V2FullBlock {
                    code: 404,
                    message: "block not found".to_string(),
                    data: None,
                }))),
                _ => Ok(V2FullBlockResponse::InternalError(Json(V2FullBlock {
                    code: 500,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
    let full_block = DisplayFullBlock {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(V2FullBlockResponse::Ok(Json(V2FullBlock {
        code: 200,
        message: "".to_string(),
        data: Some(full_block),
    })))
}

/// return simple block by given height.
pub async fn v2_get_simple_block_by_height(
    api: &Api,
    height: Path<i64>,
) -> Result<V2SimpleBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("SELECT * FROM block WHERE height = {}", height.0);
    let row_result = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match row_result {
        Ok(row) => row,
        Err(e) => {
            return match e {
                Error::RowNotFound => Ok(V2SimpleBlockResponse::NotFound(Json(V2SimpleBlock {
                    code: 404,
                    message: "block not found".to_string(),
                    data: None,
                }))),
                _ => Ok(V2SimpleBlockResponse::InternalError(Json(V2SimpleBlock {
                    code: 500,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

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

    Ok(V2SimpleBlockResponse::Ok(Json(V2SimpleBlock {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

/// return full block by given block hash.
pub async fn v2_get_full_block_by_hash(
    api: &Api,
    hash: Path<String>,
) -> Result<V2FullBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!(
        "SELECT * FROM block WHERE block_hash = '{}'",
        hash.0.to_uppercase()
    );
    let row_result = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match row_result {
        Ok(row) => row,
        Err(e) => {
            return match e {
                Error::RowNotFound => Ok(V2FullBlockResponse::NotFound(Json(V2FullBlock {
                    code: 404,
                    message: "block not found".to_string(),
                    data: None,
                }))),
                _ => Ok(V2FullBlockResponse::NotFound(Json(V2FullBlock {
                    code: 500,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

    let block_data = row.try_get("block_data")?;
    let block_rpc: BlockRPC = serde_json::from_value(block_data).unwrap();
    let full_block = DisplayFullBlock {
        block_id: block_rpc.block_id,
        block: block_rpc.block,
    };

    Ok(V2FullBlockResponse::Ok(Json(V2FullBlock {
        code: 200,
        message: "".to_string(),
        data: Some(full_block),
    })))
}

/// return simple block by given block hash.
pub async fn v2_get_simple_block_by_hash(
    api: &Api,
    hash: Path<String>,
) -> Result<V2SimpleBlockResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!(
        "SELECT * FROM block WHERE block_hash = '{}'",
        hash.0.to_uppercase()
    );

    let row_result = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match row_result {
        Ok(row) => row,
        Err(e) => {
            return match e {
                Error::RowNotFound => Ok(V2SimpleBlockResponse::NotFound(Json(V2SimpleBlock {
                    code: 404,
                    message: "block not found".to_string(),
                    data: None,
                }))),
                _ => Ok(V2SimpleBlockResponse::InternalError(Json(V2SimpleBlock {
                    code: 500,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

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

    Ok(V2SimpleBlockResponse::Ok(Json(V2SimpleBlock {
        code: 200,
        message: "".to_string(),
        data: Some(block),
    })))
}

/// return simple blocks.
pub async fn v2_get_blocks(
    api: &Api,
    start_height: Query<Option<i64>>,
    end_height: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2BlocksResponse> {
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
    let res = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total = res.try_get("total")?;

    Ok(V2BlocksResponse::Ok(Json(V2BlocksResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2BlocksData {
            page,
            page_size,
            total,
            blocks,
        }),
    })))
}
