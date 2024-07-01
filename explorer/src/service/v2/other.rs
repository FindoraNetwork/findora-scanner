use crate::service::error::Result;
use crate::service::v1::price::{FraMarketChart, FraPrice, SimplePrice};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::pool::PoolConnection;
use sqlx::types::chrono::Local;
use sqlx::{Postgres, Row};
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
    let mut conn = state.pool.acquire().await?;

    let mut stat = StatisticsResponse {
        active_addrs: 0,
        total_txs: 0,
        daily_txs: 0,
    };

    let start_time = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();

    if let Some(tx_type) = params.ty {
        let sql_txs_count = format!("SELECT count(height) FROM transaction WHERE ty={}", tx_type);
        let sql_addrs_count: String;
        let sql_daily_txs: String;
        match tx_type {
            0 => {
                sql_addrs_count = "SELECT count(distinct address) FROM native_addrs".to_string();
                sql_daily_txs = format!(
                    "SELECT count(height) FROM transaction WHERE ty=0 AND timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
            _ => {
                sql_addrs_count = "SELECT count(distinct address) FROM evm_addrs".to_string();
                sql_daily_txs = format!(
                    "SELECT count(height) FROM transaction WHERE ty=1 AND timestamp>={}",
                    start_time.and_utc().timestamp()
                );
            }
        }

        let sql_query = format!(
            "select ({}) as txs_count, ({}) as addrs_count, ({}) as daily_count",
            sql_txs_count, sql_addrs_count, sql_daily_txs
        );
        let row = sqlx::query(sql_query.as_str())
            .fetch_one(&mut *conn)
            .await?;

        let txs_count: i64 = row.try_get("txs_count")?;
        let addr_count: i64 = row.try_get("addrs_count")?;
        let daily_txs: i64 = row.try_get("daily_count")?;

        stat.active_addrs = addr_count;
        stat.total_txs = txs_count;
        stat.daily_txs = daily_txs
    } else {
        let sql_txs_count = "SELECT count(height) FROM transaction".to_string();
        let sql_evm_addrs_count = "SELECT count(distinct address) FROM evm_addrs".to_string();
        let sql_native_addrs_count = "SELECT count(distinct address) FROM native_addrs".to_string();
        let sql_daily_txs = format!(
            "SELECT count(height) FROM transaction WHERE timestamp>={}",
            start_time.and_utc().timestamp()
        );

        let sql_query = format!("select ({}) as txs_count, ({}) as evm_addrs_count, ({}) as native_addrs_count, ({}) as daily_count", sql_txs_count, sql_evm_addrs_count, sql_native_addrs_count, sql_daily_txs);
        let row = sqlx::query(sql_query.as_str())
            .fetch_one(&mut *conn)
            .await?;

        let txs_count: i64 = row.try_get("txs_count")?;
        let evm_addrs: i64 = row.try_get("evm_addrs_count")?;
        let native_addrs: i64 = row.try_get("native_addrs_count")?;
        let daily_txs: i64 = row.try_get("daily_count")?;

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
    let mut conn = state.pool.acquire().await?;

    let sql_query = "select (SELECT count(height) FROM transaction WHERE ty=0) as native_count,\
        (SELECT count(height) FROM transaction WHERE ty_sub in (2,3,4)) as privacy_count,\
        (SELECT count(height) FROM transaction WHERE ty=1) as evm_count,\
        (SELECT count(height) FROM n2e) as n2e_count,\
        (SELECT count(height) FROM e2n) as e2n_count";

    let row = sqlx::query(sql_query).fetch_one(&mut *conn).await?;

    let native_count: i64 = row.try_get("native_count")?;
    let privacy: i64 = row.try_get("privacy_count")?;
    let evm_count: i64 = row.try_get("evm_count")?;
    let n2e_count: i64 = row.try_get("n2e_count")?;
    let e2n_count: i64 = row.try_get("e2n_count")?;

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
    let mut conn = state.pool.acquire().await?;

    let mut query_params: Vec<String> = vec![];
    if let Some(start_time) = params.start_time {
        query_params.push(format!("timestamp >= {start_time} "));
    }
    if let Some(end_time) = params.end_time {
        query_params.push(format!("timestamp <= {end_time} "));
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

    let sql_query = format!(
        "select ({}) as native_count, ({}) as evm_count",
        sql_native, sql_evm
    );
    let row = sqlx::query(sql_query.as_str())
        .fetch_one(&mut *conn)
        .await?;
    let native_count: i64 = row.try_get("native_count")?;
    let evm_count: i64 = row.try_get("evm_count")?;

    Ok(Json(AddressCountResponse {
        count: native_count + evm_count,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct MarketParams {
    pub vs_currency: Option<String>,
    pub days: Option<i32>,
    pub interval: Option<String>,
}

async fn get_market_data(mut conn: PoolConnection<Postgres>) -> Result<FraMarketChart> {
    let row = sqlx::query("SELECT val FROM market")
        .fetch_one(&mut *conn)
        .await?;
    let val: Value = row.try_get("val")?;
    let fmc: FraMarketChart = serde_json::from_value(val).unwrap();
    Ok(fmc)
}

async fn upsert_market_data(mut conn: PoolConnection<Postgres>, val: Value) -> Result<()> {
    sqlx::query("INSERT INTO market VALUES($1,$2) ON CONFLICT(name) DO UPDATE SET val=$2")
        .bind("fra")
        .bind(val)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub async fn get_market(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MarketParams>,
) -> Result<Json<FraMarketChart>> {
    let conn = state.pool.acquire().await?;

    let vs_currency = params.vs_currency.unwrap_or("usd".to_string());
    let days = params.days.unwrap_or(7);
    let interval = params.interval.unwrap_or("daily".to_string());

    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}&interval={}",
        id, vs_currency, days, interval
    );

    let resp1 = reqwest::get(url).await;
    if resp1.is_err() {
        let fmc = get_market_data(conn).await?;
        return Ok(Json(fmc));
    }
    let resp2 = resp1?.json::<FraMarketChart>().await;
    if resp2.is_err() {
        let fmc = get_market_data(conn).await?;
        return Ok(Json(fmc));
    }

    let fmc = resp2?;
    let v = serde_json::to_value(&fmc)?;
    upsert_market_data(conn, v).await?;

    Ok(Json(fmc))
}

#[derive(Serialize, Deserialize)]
pub struct PriceParams {
    pub ids: Option<String>,
    pub vs_currencies: Option<String>,
}

async fn get_price_data(mut conn: PoolConnection<Postgres>) -> Result<FraPrice> {
    let row = sqlx::query("SELECT price FROM prices")
        .fetch_one(&mut *conn)
        .await?;
    let p: String = row.try_get("price")?;
    let fra_price = FraPrice { usd: p.parse()? };
    Ok(fra_price)
}

async fn upsert_price_data(mut conn: PoolConnection<Postgres>, price: &str) -> Result<()> {
    sqlx::query("INSERT INTO prices VALUES($1,$2) ON CONFLICT(name) DO UPDATE SET price=$2")
        .bind("fra")
        .bind(price)
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn get_price(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PriceParams>,
) -> Result<Json<SimplePrice>> {
    let conn = state.pool.acquire().await?;
    let ids = params.ids.unwrap_or("findora".to_string());
    let vs_currencies = params.vs_currencies.unwrap_or("usd".to_string());

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        ids, vs_currencies
    );
    let resp1 = reqwest::get(url).await;
    if resp1.is_err() {
        let fra_price = get_price_data(conn).await?;
        return Ok(Json(SimplePrice { findora: fra_price }));
    }
    let resp2 = resp1?.json::<SimplePrice>().await;
    if resp2.is_err() {
        let fra_price = get_price_data(conn).await?;
        return Ok(Json(SimplePrice { findora: fra_price }));
    }

    let fra_price = resp2?.findora;
    upsert_price_data(conn, fra_price.usd.to_string().as_str()).await?;

    Ok(Json(SimplePrice { findora: fra_price }))
}
