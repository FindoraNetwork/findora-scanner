use crate::service::error::Result;
use crate::service::QueryResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct GetAssetsParams {
    pub address: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetResponse {
    pub asset: String,
    pub tx: String,
    pub block: String,
    pub issuer: String,
    pub height: i64,
    pub timestamp: i64,
    pub ty: i32,
    pub value: Value,
}

pub async fn get_assets(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetAssetsParams>,
) -> Result<Json<QueryResult<Vec<AssetResponse>>>> {
    let mut conn = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut sql_total = "SELECT count(*) FROM assets ".to_string();
    let mut sql_query =
        "SELECT asset,tx,block,issuer,height,timestamp,ty,content FROM assets ".to_string();
    let mut query_params: Vec<String> = vec![];
    if let Some(addr) = params.address {
        query_params.push(format!("asset='{}' ", addr));
    }
    if !query_params.is_empty() {
        sql_query = sql_query
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
        sql_total = sql_total
            .add("WHERE ")
            .add(query_params.join("AND ").as_str());
    }
    sql_query = sql_query.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let row = sqlx::query(&sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("count")?;

    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;

    let mut assets: Vec<AssetResponse> = vec![];
    for row in rows {
        let asset: String = row.try_get("asset")?;
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let issuer: String = row.try_get("issuer")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("content")?;
        assets.push(AssetResponse {
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

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: assets,
    }))
}
