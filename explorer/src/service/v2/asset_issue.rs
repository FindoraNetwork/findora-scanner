use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2IssueAssetTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2IssueAssetTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2IssueAssetTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2IssueAssetTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2IssueAssetTx {
    pub asset: String,
    pub tx: String,
    pub block: String,
    pub issuer: String,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_issue_asset(api: &Api, asset: Path<String>) -> Result<V2IssueAssetTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!("SELECT * from issued_assets WHERE asset='{}'", asset.0);

    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let asset: String = row.try_get("asset")?;
    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let issuer: String = row.try_get("issuer")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2IssueAssetTx {
        asset,
        tx,
        block,
        issuer,
        height,
        timestamp,
        value,
    };

    Ok(V2IssueAssetTxResponse::Ok(Json(V2IssueAssetTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}
