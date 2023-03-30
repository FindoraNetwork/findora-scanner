use poem_openapi::param::{Path, Query};
use poem_openapi::types::Type;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub data: Value,
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
    pub data: Value,
}

#[allow(clippy::let_unit_value)]
pub async fn simple_price(
    ids: Query<String>,
    vs_currencies: Query<String>,
) -> poem::Result<SimplePriceResponse> {
    if ids.is_empty() || vs_currencies.is_empty() {
        return Ok(SimplePriceResponse::BadRequest);
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        ids.0, vs_currencies.0
    );
    let resp1 = reqwest::get(url).await;
    if let Err(e) = resp1 {
        return Ok(SimplePriceResponse::InternalError(Json(
            SimplePriceResult {
                code: 500,
                message: e.to_string(),
                data: Default::default(),
            },
        )));
    }
    let resp2 = resp1.unwrap().json::<Value>().await.unwrap();

    Ok(SimplePriceResponse::Ok(Json(SimplePriceResult {
        code: 200,
        message: "".to_string(),
        data: resp2,
    })))
}

#[allow(clippy::let_unit_value)]
pub async fn market_chart(
    id: Path<String>,
    vs_currency: Query<String>,
    interval: Query<Option<String>>,
    days: Query<i32>,
) -> poem::Result<MarketChartResponse> {
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
                data: Default::default(),
            },
        )));
    }
    let resp2 = resp1.unwrap().json::<Value>().await.unwrap();

    Ok(MarketChartResponse::Ok(Json(MarketChartResult {
        code: 200,
        message: "".to_string(),
        data: resp2,
    })))
}
