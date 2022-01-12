use crate::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use module::display::block::DisplayBlock;

#[derive(ApiResponse)]
pub enum GetBlockResponse {
    #[oai(status = 200)]
    Ok(Json<DisplayBlock>),
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

    let block = DisplayBlock {
        block_id,
        height,
        time:time.timestamp(),
        app_hash,
        proposer,
    };

    Ok(GetBlockResponse::Ok(Json(block)))
}
