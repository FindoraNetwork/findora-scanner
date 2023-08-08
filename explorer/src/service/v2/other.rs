use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2ChainStatisticsResponse {
    #[oai(status = 200)]
    Ok(Json<V2ChainStatisticsResult>),
    #[oai(status = 404)]
    NotFound(Json<V2ChainStatisticsResult>),
    #[oai(status = 500)]
    InternalError(Json<V2ChainStatisticsResult>),
}

#[derive(Serialize, Deserialize, Object)]
pub struct V2ChainStatisticsResult {
    pub code: i32,
    pub message: String,
    pub data: Option<V2StatisticsData>,
}

#[derive(Serialize, Deserialize, Object)]
pub struct V2StatisticsData {
    pub active_addrs: i64,
    pub total_txs: i64,
    pub daily_txs: i64,
}

pub async fn v2_statistics(api: &Api) -> Result<V2ChainStatisticsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    // total txs
    let sql_txs_count = "select count(*) as cnt from transaction".to_string();
    let row = sqlx::query(sql_txs_count.as_str())
        .fetch_one(&mut conn)
        .await?;
    let total_txs = row.try_get("cnt")?;

    // total addrs
    let sql_addr_count = "select count(distinct address) as cnt from native_txs".to_string();
    let row = sqlx::query(sql_addr_count.as_str())
        .fetch_one(&mut conn)
        .await?;
    let active_addrs = row.try_get("cnt")?;

    // daily txs
    let start_time = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
    let sql_daily_txs = format!(
        "select count(*) as cnt from transaction where timestamp>={}",
        start_time.timestamp()
    );
    let row = sqlx::query(sql_daily_txs.as_str())
        .fetch_one(&mut conn)
        .await?;
    let daily_txs = row.try_get("cnt")?;

    Ok(V2ChainStatisticsResponse::Ok(Json(
        V2ChainStatisticsResult {
            code: 200,
            message: "".to_string(),
            data: Some(V2StatisticsData {
                active_addrs,
                total_txs,
                daily_txs,
            }),
        },
    )))
}

#[derive(ApiResponse)]
pub enum V2DistributeResponse {
    #[oai(status = 200)]
    Ok(Json<V2DistributeResult>),
    #[oai(status = 500)]
    InternalError(Json<V2DistributeResult>),
}

#[derive(Serialize, Deserialize, Default, Object)]
pub struct V2DistributeResult {
    pub code: i32,
    pub message: String,
    pub data: Option<V2TxsDistribute>,
}

#[derive(Serialize, Deserialize, Default, Object)]
pub struct V2TxsDistribute {
    pub transparent: i64,
    pub privacy: i64,
    pub prism: i64,
    pub evm_compatible: i64,
}
