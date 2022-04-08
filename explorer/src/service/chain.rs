use crate::Api;
use anyhow::Result;
use module::schema::DelegationInfo;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(ApiResponse)]
pub enum StakingResponse {
    #[oai(status = 200)]
    Ok(Json<StakingRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct StakingRes {
    pub code: i32,
    pub message: String,
    pub data: Option<StakingData>,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct StakingData {
    pub block_reward: u64,
    pub pledge_rate: f64,
    pub annual_interest_rate: f64,
    pub active_validators: Vec<String>,
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
    let daily_txs_res = sqlx::query("SELECT COUNT(*) as cnt FROM transaction where timestamp>=$1")
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

pub async fn staking_info(api: &Api) -> Result<StakingResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let delegation_res = sqlx::query("SELECT info FROM delegations ORDER BY height DESC LIMIT 1")
        .fetch_one(&mut conn)
        .await;
    if let Err(ref err) = delegation_res {
        match err {
            RowNotFound => {
                return Ok(StakingResponse::Ok(Json(StakingRes {
                    code: 200,
                    message: "".to_string(),
                    data: Some(StakingData::default()),
                })));
            }
            _ => {
                return Ok(StakingResponse::Ok(Json(StakingRes {
                    code: 500,
                    message: "internal error, query delegations.".to_string(),
                    data: None,
                })));
            }
        }
    }
    let info_value: Value = delegation_res.unwrap().try_get("info")?;
    let delegation_info: DelegationInfo = serde_json::from_value(info_value).unwrap();

    let mut active_validators: Vec<String> = vec![];
    for (id, _) in delegation_info.validator_addr_map {
        active_validators.push(id);
    }
    let mut reward: u64 = 0;
    let mut total_pledge: u64 = 0;
    for (_, dl) in delegation_info.global_delegation_records_map {
        reward += dl.delegation_rwd_cnt;
        for (_, amount) in dl.delegations {
            total_pledge += amount
        }
    }

    let data = StakingData {
        block_reward: reward,
        pledge_rate: delegation_info.return_rate.value,
        annual_interest_rate: total_pledge as f64 / 21_420_000_000_000_000.0,
        active_validators,
    };

    Ok(StakingResponse::Ok(Json(StakingRes {
        code: 200,
        message: "".to_string(),
        data: Some(data),
    })))
}
