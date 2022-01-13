use crate::Api;
use anyhow::Result;
use module::db::block_meta::BlockMeta;
use poem_openapi::Object;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum BlockChainResponse {
    #[oai(status = 200)]
    Ok(Json<BlockChainRes>),
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct BlockChainRes {
    pub last_height: i64,
    pub block_metas: Vec<BlockMeta>,
}

pub async fn blockchain(
    api: &Api,
    min_height: Path<i64>,
    max_height: Path<i64>,
) -> Result<BlockChainResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!(
        "select * from block_meta where height >= {} AND height <= {}",
        min_height.0, max_height.0
    );
    let row = sqlx::query(str.as_str()).fetch_all(&mut conn).await?;

    let mut block_metas: Vec<BlockMeta> = vec![];
    for v in row.iter() {
        let block_id: Value = v.try_get("block_id")?;
        let header: Value = v.try_get("header")?;
        let block_size: i64 = v.try_get("block_size")?;
        let num_txs: i64 = v.try_get("num_txs")?;
        let block_meta = BlockMeta {
            block_id,
            block_size,
            header,
            num_txs,
        };
        block_metas.push(block_meta);
    }

    let res = BlockChainRes {
        last_height: 0, // todo: get the latest height.
        block_metas,
    };

    Ok(BlockChainResponse::Ok(Json(res)))
}
