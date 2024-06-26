use crate::service::error::{internal_error, Result};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;
use sqlx::Row;
use std::ops::Add;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct StatisticsResponse {
    pub active_addrs: i64,
    pub total_txs: i64,
    pub daily_txs: i64,
}

#[derive(Serialize, Deserialize)]
pub struct StatisticsParams {
    pub ty: Option<i32>,
}

#[allow(dead_code)]
pub async fn get_statistics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<StatisticsParams>,
) -> Result<Json<StatisticsResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let mut stat = StatisticsResponse {
        active_addrs: 0,
        total_txs: 0,
        daily_txs: 0,
    };

    let start_time = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();

    if let Some(tx_type) = params.ty {
        let sql_txs_count = format!("SELECT count(*) FROM transaction WHERE ty={}", tx_type);
        let row_txs_count = sqlx::query(sql_txs_count.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let txs_count = row_txs_count.try_get("count").map_err(internal_error)?;

        let sql_addrs_count: String;
        let sql_daily_txs: String;
        match tx_type {
            0 => {
                sql_addrs_count = "SELECT count(distinct address) FROM native_addrs".to_string();
                sql_daily_txs = format!(
                    "SELECT count(*) FROM transaction WHERE ty=0 AND timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
            _ => {
                sql_addrs_count = "SELECT count(distinct address) FROM evm_addrs".to_string();
                sql_daily_txs = format!(
                    "SELECT count(*) FROM transaction WHERE ty=1 AND timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
        }

        let row_addr_count = sqlx::query(sql_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let addr_count: i64 = row_addr_count.try_get("count").map_err(internal_error)?;

        let row_daily = sqlx::query(sql_daily_txs.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let daily_txs = row_daily.try_get("count").map_err(internal_error)?;

        stat.active_addrs = addr_count;
        stat.total_txs = txs_count;
        stat.daily_txs = daily_txs
    } else {
        let sql_txs_count = "SELECT count(*) FROM transaction".to_string();
        let row_txs_count = sqlx::query(sql_txs_count.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let txs_count = row_txs_count.try_get("count").map_err(internal_error)?;

        let sql_evm_addrs_count = "SELECT count(distinct address) FROM evm_addrs".to_string();
        let row_evm_addr = sqlx::query(sql_evm_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let evm_addrs: i64 = row_evm_addr.try_get("count").map_err(internal_error)?;

        let sql_native_addrs_count = "SELECT count(distinct address) FROM native_addrs".to_string();
        let row_native_addr = sqlx::query(sql_native_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let native_addrs: i64 = row_native_addr.try_get("count").map_err(internal_error)?;

        let sql_daily_txs = format!(
            "SELECT count(*) FROM transaction WHERE timestamp>={}",
            start_time.and_utc().timestamp()
        );
        let row_daily = sqlx::query(sql_daily_txs.as_str())
            .fetch_one(&mut *conn)
            .await
            .map_err(internal_error)?;
        let daily_txs = row_daily.try_get("count").map_err(internal_error)?;

        stat.active_addrs = native_addrs + evm_addrs;
        stat.total_txs = txs_count;
        stat.daily_txs = daily_txs
    }

    Ok(Json(stat))
}

#[derive(Serialize, Deserialize)]
pub struct TxsDistributeResponse {
    pub transparent: i64,
    pub privacy: i64,
    pub prism: i64,
    pub evm_compatible: i64,
}

#[allow(dead_code)]
pub async fn get_tx_distribute(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TxsDistributeResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let sql_native = "SELECT count(*) FROM transaction WHERE ty=0";
    let row_native = sqlx::query(sql_native)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let native_count: i64 = row_native.try_get("count").map_err(internal_error)?;

    let sql_privacy = "SELECT count(*) FROM transaction WHERE ty_sub=2 or ty_sub=3 or ty_sub=4";
    let row_privacy = sqlx::query(sql_privacy)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let privacy: i64 = row_privacy.try_get("count").map_err(internal_error)?;

    let sql_evm = "SELECT count(*) FROM transaction WHERE ty=1";
    let row_evm = sqlx::query(sql_evm)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let evm_count: i64 = row_evm.try_get("count").map_err(internal_error)?;

    let sql_prism_n2e = "SELECT count(*) FROM n2e";
    let row_n2e = sqlx::query(sql_prism_n2e)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let n2e_count: i64 = row_n2e.try_get("count").map_err(internal_error)?;

    let sql_prism_e2n = "SELECT count(*) FROM e2n";
    let row_e2n = sqlx::query(sql_prism_e2n)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let e2n_count: i64 = row_e2n.try_get("count").map_err(internal_error)?;

    Ok(Json(TxsDistributeResponse {
        transparent: native_count - privacy,
        privacy,
        prism: n2e_count + e2n_count,
        evm_compatible: evm_count,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct AddressCountParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct AddressCountResponse {
    count: i64,
}

pub async fn get_address_count(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AddressCountParams>,
) -> Result<Json<AddressCountResponse>> {
    let mut conn = state.pool.acquire().await.map_err(internal_error)?;

    let mut query_params: Vec<String> = vec![];
    if let Some(start_time) = params.start_time {
        query_params.push(format!("timestamp > {start_time} "));
    }
    if let Some(end_time) = params.end_time {
        query_params.push(format!("timestamp < {end_time} "));
    }

    let mut sql_native = "SELECT count(distinct address) FROM native_addrs ".to_string();
    let mut sql_evm = "SELECT count(distinct address) FROM evm_addrs ".to_string();

    if !query_params.is_empty() {
        sql_native = sql_native
            .add("WHERE ")
            .add(query_params.join(" AND ").as_str());
        sql_evm = sql_evm
            .add("WHERE ")
            .add(query_params.join(" AND ").as_str());
    }

    let row_native = sqlx::query(sql_native.as_str())
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let native_count: i64 = row_native.try_get("count").map_err(internal_error)?;

    let row_evm = sqlx::query(sql_evm.as_str())
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;
    let evm_count: i64 = row_evm.try_get("count").map_err(internal_error)?;

    Ok(Json(AddressCountResponse {
        count: native_count + evm_count,
    }))
}
