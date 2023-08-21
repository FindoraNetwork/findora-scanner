use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2AssetTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2AssetTxResult>),
    #[oai(status = 404)]
    NotFound(Json<V2AssetTxResult>),
    #[oai(status = 500)]
    InternalError(Json<V2AssetTxResult>),
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2AssetTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2AssetData>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2AssetData {
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub assets: Vec<V2AssetOp>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2AssetOp {
    pub asset: String,
    pub tx: String,
    pub block: String,
    pub issuer: String,
    pub height: i64,
    pub timestamp: i64,
    pub ty: i32,
    pub value: Value,
}

pub async fn v2_get_asset(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i32>>,
    page_size: Query<Option<i32>>,
) -> Result<V2AssetTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut assets: Vec<V2AssetOp> = vec![];
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let sql_total = format!(
        "SELECT count(*) as cnt from assets WHERE asset='{}'",
        address.0
    );
    let row_count = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row_count.try_get("cnt")?;
    let sql_query = format!(
        "SELECT * from assets WHERE asset='{}' order by height desc limit {} offset {}",
        address.0,
        page_size,
        (page - 1) * page_size
    );

    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let asset: String = row.try_get("asset")?;
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let issuer: String = row.try_get("issuer")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("content")?;
        assets.push(V2AssetOp {
            asset,
            tx,
            block,
            issuer,
            height,
            timestamp,
            ty,
            value,
        });
    }

    Ok(V2AssetTxResponse::Ok(Json(V2AssetTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(V2AssetData {
            page,
            page_size,
            total,
            assets,
        }),
    })))
}
