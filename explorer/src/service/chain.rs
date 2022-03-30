use crate::Api;
use anyhow::Result;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;
use sqlx::Error::RowNotFound;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum ChainStatisticsResponse {
    #[oai(status = 200)]
    Ok(Json<ChainStatisticsRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ChainStatisticsRes {
    pub code: i32,
    pub message: String,
    pub data: Option<StatisticsData>,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct StatisticsData {
    pub active_addresses: i64,
    pub wallet_downloads: i64,
    pub total_txs: i64,
    pub daily_txs: i64,
}

pub async fn statistics(api: &Api) -> Result<ChainStatisticsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let mut res_data = StatisticsData {
        active_addresses: 0,
        wallet_downloads: 0,
        total_txs: 0,
        daily_txs: 0,
    };

    // total txs
    let mut sql_str = String::from("SELECT COUNT(*) as cnt FROM transaction");
    let total_txs_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    if let Err(ref err) = total_txs_res {
        match err {
            RowNotFound => {}
            _ => {
                return Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsRes {
                    code: 500,
                    message: "internal error, query total txs.".to_string(),
                    data: Some(res_data),
                })));
            }
        }
    }
    let total_txs = total_txs_res.unwrap().try_get("cnt")?;

    // total address
    sql_str = String::from("SELECT COUNT(*) as cnt FROM active_address");
    let active_addresses_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    if let Err(ref err) = active_addresses_res {
        match err {
            RowNotFound => {}
            _ => {
                return Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsRes {
                    code: 500,
                    message: "internal error, query active address.".to_string(),
                    data: Some(res_data),
                })));
            }
        }
    }
    let active_addresses = active_addresses_res.unwrap().try_get("cnt")?;

    // daily txs
    let t = Local::now().timestamp() - 3600 * 24;
    let daily_txs_res = sqlx::query("SELECT COUNT(*) as cnt FROM transaction where time>=$1")
        .bind(t)
        .fetch_one(&mut conn)
        .await;
    if let Err(ref err) = daily_txs_res {
        match err {
            RowNotFound => {}
            _ => {
                return Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsRes {
                    code: 500,
                    message: "internal error, query daily txs.".to_string(),
                    data: Some(res_data),
                })));
            }
        }
    }
    let daily_txs = daily_txs_res.unwrap().try_get("cnt")?;

    res_data.daily_txs = daily_txs;
    res_data.total_txs = total_txs;
    res_data.active_addresses = active_addresses;

    Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsRes {
        code: 200,
        message: "".to_string(),
        data: Some(res_data),
    })))
}
