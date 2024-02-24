use crate::service::api::Api;
use anyhow::Result;
use log::error;
use poem_openapi::param::{Path, Query};
use poem_openapi::types::Type;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum SimplePriceResponse {
    #[oai(status = 200)]
    Ok(Json<SimplePriceResult>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError(Json<SimplePriceResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct SimplePriceResult {
    pub code: i32,
    pub message: String,
    pub data: Option<SimplePrice>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct SimplePrice {
    pub findora: FraPrice,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct FraPrice {
    pub usd: f64,
}

#[derive(ApiResponse)]
pub enum MarketChartResponse {
    #[oai(status = 200)]
    Ok(Json<MarketChartResult>),
    #[oai(status = 400)]
    BadRequest,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError(Json<MarketChartResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct MarketChartResult {
    pub code: i32,
    pub message: String,
    pub data: Option<FraMarketChart>,
}

pub async fn get_fra_price(api: &Api) -> Result<FraPrice> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let row = sqlx::query("SELECT price FROM prices")
        .fetch_one(&mut conn)
        .await?;
    let p: String = row.try_get("price")?;
    let fra_price = FraPrice { usd: p.parse()? };
    Ok(fra_price)
}

pub async fn upsert_fra_price(api: &Api, price: &str) -> Result<()> {
    let mut conn = api.storage.lock().await.acquire().await?;
    sqlx::query("INSERT INTO prices VALUES($1,$2) ON CONFLICT(name) DO UPDATE SET price=$2")
        .bind("fra")
        .bind(price)
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[allow(clippy::let_unit_value)]
pub async fn simple_price(
    api: &Api,
    ids: Query<String>,
    vs_currencies: Query<String>,
) -> Result<SimplePriceResponse> {
    if ids.is_empty() || vs_currencies.is_empty() {
        return Ok(SimplePriceResponse::BadRequest);
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        ids.0, vs_currencies.0
    );
    let resp1 = reqwest::get(url).await;
    if let Err(e) = resp1 {
        error!("Get FRA price: {}", e.to_string());
        let fra_price = get_fra_price(api).await?;
        return Ok(SimplePriceResponse::Ok(Json(SimplePriceResult {
            code: 200,
            message: "".to_string(),
            data: Some(SimplePrice { findora: fra_price }),
        })));
    }

    let resp2 = resp1.unwrap().json::<SimplePrice>().await;
    if let Err(e) = resp2 {
        error!("Parse FRA price: {}", e.to_string());
        let fra_price = get_fra_price(api).await?;
        return Ok(SimplePriceResponse::Ok(Json(SimplePriceResult {
            code: 200,
            message: "".to_string(),
            data: Some(SimplePrice { findora: fra_price }),
        })));
    }

    let fra_price = resp2.unwrap().findora;
    upsert_fra_price(api, fra_price.usd.to_string().as_str()).await?;

    Ok(SimplePriceResponse::Ok(Json(SimplePriceResult {
        code: 200,
        message: "".to_string(),
        data: Some(SimplePrice { findora: fra_price }),
    })))
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct FraMarketChart {
    pub market_caps: Value,
    pub prices: Value,
    pub total_volumes: Value,
}

#[allow(clippy::let_unit_value)]
pub async fn market_chart(
    id: Path<String>,
    vs_currency: Query<String>,
    interval: Query<Option<String>>,
    days: Query<i32>,
) -> Result<MarketChartResponse> {
    if id.is_empty() || vs_currency.is_empty() || days.is_empty() {
        return Ok(MarketChartResponse::BadRequest);
    }

    let mut url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}",
        id.0, vs_currency.0, days.0
    );
    if let Some(itv) = interval.0 {
        url = format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency={}&days={}&interval={}", id.0, vs_currency.0, days.0, itv);
    }
    let resp1 = reqwest::get(url).await;
    if let Err(e) = resp1 {
        return Ok(MarketChartResponse::InternalError(Json(
            MarketChartResult {
                code: 500,
                message: e.to_string(),
                data: None,
            },
        )));
    }
    let resp2 = resp1.unwrap().text().await;
    if let Err(e) = resp2 {
        return Ok(MarketChartResponse::InternalError(Json(
            MarketChartResult {
                code: 500,
                message: e.to_string(),
                data: None,
            },
        )));
    }

    let r = resp2.unwrap();
    let fmc: FraMarketChart = serde_json::from_str(r.as_str()).unwrap();

    Ok(MarketChartResponse::Ok(Json(MarketChartResult {
        code: 200,
        message: "".to_string(),
        data: Some(fmc),
    })))
}
