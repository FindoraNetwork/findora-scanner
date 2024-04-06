use crate::service::api::Api;
use crate::service::v1::chain::{AddressCount, AddressCountResponse, AddressCountResult};
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;
use sqlx::Row;
use std::ops::Add;

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

pub async fn v2_statistics(api: &Api, ty: Query<Option<i32>>) -> Result<V2ChainStatisticsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let mut stat = V2StatisticsData {
        active_addrs: 0,
        total_txs: 0,
        daily_txs: 0,
    };

    let start_time = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();

    if let Some(tx_type) = ty.0 {
        let sql_txs_count = format!(
            "select count(*) as cnt from transaction where ty={}",
            tx_type
        );
        let row_txs_count = sqlx::query(sql_txs_count.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let txs_count = row_txs_count.try_get("cnt")?;

        let sql_addrs_count: String;
        let sql_daily_txs: String;
        match tx_type {
            0 => {
                sql_addrs_count =
                    "select count(distinct address) as cnt from native_addrs".to_string();
                sql_daily_txs = format!(
                    "select count(*) as cnt from transaction where ty=0 and timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
            _ => {
                sql_addrs_count =
                    "select count(distinct address) as cnt from evm_addrs".to_string();
                sql_daily_txs = format!(
                    "select count(*) as cnt from transaction where ty=1 and timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
        }

        let row_addr_count = sqlx::query(sql_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let addr_count: i64 = row_addr_count.try_get("cnt")?;

        let row_daily = sqlx::query(sql_daily_txs.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let daily_txs = row_daily.try_get("cnt")?;

        stat.active_addrs = addr_count;
        stat.total_txs = txs_count;
        stat.daily_txs = daily_txs
    } else {
        let sql_txs_count = "select count(*) as cnt from transaction".to_string();
        let row_txs_count = sqlx::query(sql_txs_count.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let txs_count = row_txs_count.try_get("cnt")?;

        let sql_evm_addrs_count =
            "select count(distinct address) as cnt from evm_addrs".to_string();
        let row_evm_addr = sqlx::query(sql_evm_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let evm_addrs: i64 = row_evm_addr.try_get("cnt")?;

        let sql_native_addrs_count =
            "select count(distinct address) as cnt from native_addrs".to_string();
        let row_native_addr = sqlx::query(sql_native_addrs_count.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let native_addrs: i64 = row_native_addr.try_get("cnt")?;

        let sql_daily_txs = format!(
            "select count(*) as cnt from transaction where timestamp>={}",
            start_time.and_utc().timestamp()
        );
        let row_daily = sqlx::query(sql_daily_txs.as_str())
            .fetch_one(&mut *conn)
            .await?;
        let daily_txs = row_daily.try_get("cnt")?;

        stat.active_addrs = native_addrs + evm_addrs;
        stat.total_txs = txs_count;
        stat.daily_txs = daily_txs
    }

    Ok(V2ChainStatisticsResponse::Ok(Json(
        V2ChainStatisticsResult {
            code: 200,
            message: "".to_string(),
            data: Some(stat),
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

pub async fn v2_distribute(api: &Api) -> Result<V2DistributeResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let sql_native = "select count(*) as cnt from transaction where ty=0";
    let row_native = sqlx::query(sql_native).fetch_one(&mut *conn).await?;
    let native_count: i64 = row_native.try_get("cnt")?;

    let sql_privacy =
        "select count(*) as cnt from transaction where ty_sub=2 or ty_sub=3 or ty_sub=4";
    let row_privacy = sqlx::query(sql_privacy).fetch_one(&mut *conn).await?;
    let privacy: i64 = row_privacy.try_get("cnt")?;

    let sql_evm = "SELECT count(*) as cnt FROM transaction where ty=1";
    let row_evm = sqlx::query(sql_evm).fetch_one(&mut *conn).await?;
    let evm_count: i64 = row_evm.try_get("cnt")?;

    let sql_prism_n2e = "select count(*) as cnt from n2e";
    let row_n2e = sqlx::query(sql_prism_n2e).fetch_one(&mut *conn).await?;
    let n2e_count: i64 = row_n2e.try_get("cnt")?;

    let sql_prism_e2n = "select count(*) as cnt from e2n";
    let row_e2n = sqlx::query(sql_prism_e2n).fetch_one(&mut *conn).await?;
    let e2n_count: i64 = row_e2n.try_get("cnt")?;

    Ok(V2DistributeResponse::Ok(Json(V2DistributeResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2TxsDistribute {
            transparent: native_count - privacy,
            privacy,
            prism: n2e_count + e2n_count,
            evm_compatible: evm_count,
        }),
    })))
}

pub async fn v2_address_count(
    api: &Api,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
) -> Result<AddressCountResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut params: Vec<String> = vec![];
    if let Some(start_time) = start_time.0 {
        params.push(format!("timestamp > {start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!("timestamp < {end_time} "));
    }

    let mut sql_native = "select count(distinct address) as cnt from native_addrs ".to_string();
    let mut sql_evm = "select count(distinct address) as cnt from evm_addrs ".to_string();

    if !params.is_empty() {
        sql_native = sql_native.add("WHERE ").add(params.join(" AND ").as_str());
        sql_evm = sql_evm.add("WHERE ").add(params.join(" AND ").as_str());
    }

    let row_native = sqlx::query(sql_native.as_str())
        .fetch_one(&mut *conn)
        .await?;
    let native_count: i64 = row_native.try_get("cnt")?;

    let row_evm = sqlx::query(sql_evm.as_str()).fetch_one(&mut *conn).await?;
    let evm_count: i64 = row_evm.try_get("cnt")?;

    Ok(AddressCountResponse::Ok(Json(AddressCountResult {
        code: 200,
        message: "".to_string(),
        data: Some(AddressCount {
            address_count: native_count + evm_count,
        }),
    })))
}
