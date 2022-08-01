use crate::Api;
use anyhow::Result;
use module::schema::DelegationInfo;
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::Local;
use sqlx::Row;
use std::collections::{HashMap, HashSet};

#[derive(ApiResponse)]
pub enum ChainStatisticsResponse {
    #[oai(status = 200)]
    Ok(Json<ChainStatisticsRes>),
    #[oai(status = 500)]
    InternalError(Json<ChainStatisticsRes>),
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
    pub total_txs: i64,
    pub daily_txs: i64,
}

#[derive(ApiResponse)]
pub enum StakingResponse {
    #[oai(status = 200)]
    Ok(Json<StakingRes>),
    #[oai(status = 200)]
    InternalError(Json<StakingRes>),
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
    pub stake_ratio: f64,
    pub apy: f64,
    pub active_validators: Vec<String>,
    pub nonactive_validators: Vec<String>,
}

pub async fn statistics(api: &Api, ty: Query<Option<i64>>) -> Result<ChainStatisticsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    // total txs
    let sql_str = if let Some(ty) = ty.0 {
        format!("SELECT COUNT(*) as cnt FROM transaction WHERE ty={}", ty)
    } else {
        "SELECT COUNT(*) as cnt FROM transaction".to_string()
    };
    let total_txs_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    let row = match total_txs_res {
        Ok(row) => row,
        _ => {
            return Ok(ChainStatisticsResponse::InternalError(Json(
                ChainStatisticsRes {
                    code: 500,
                    message: "internal error while querying total txs.".to_string(),
                    data: None,
                },
            )));
        }
    };
    let total_txs = row.try_get("cnt")?;

    // total address
    let sql_str = if let Some(ty) = ty.0 {
        format!("SELECT jsonb_path_query(value,'$.body.operations[*].TransferAsset.body.transfer.*.public_key') \
        as addr FROM transaction WHERE ty={}", ty)
    } else {
        "SELECT jsonb_path_query(value,'$.body.operations[*].TransferAsset.body.transfer.*.public_key') \
        as addr FROM transaction".to_string()
    };
    let active_addresses_res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let rows = match active_addresses_res {
        Ok(rows) => rows,
        _ => {
            return Ok(ChainStatisticsResponse::InternalError(Json(
                ChainStatisticsRes {
                    code: 500,
                    message: "internal error while querying total addresses.".to_string(),
                    data: None,
                },
            )));
        }
    };

    let mut hs: HashSet<String> = HashSet::new();
    for row in rows {
        let value: Value = row.try_get("addr")?;
        let addr: String = serde_json::from_value(value).unwrap();
        hs.insert(addr);
    }
    let active_addresses = hs.len() as i64;

    // daily txs
    let now = Local::now().date().and_hms(0, 0, 0);
    let sql_str = if let Some(ty) = ty.0 {
        format!(
            "SELECT COUNT(*) as cnt FROM transaction WHERE ty={} AND timestamp>={}",
            ty,
            now.timestamp()
        )
    } else {
        format!(
            "SELECT COUNT(*) as cnt FROM transaction where timestamp>={}",
            now.timestamp()
        )
    };
    let daily_txs_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    let row = match daily_txs_res {
        Ok(row) => row,
        _ => {
            return Ok(ChainStatisticsResponse::InternalError(Json(
                ChainStatisticsRes {
                    code: 500,
                    message: "internal error while querying daily txs.".to_string(),
                    data: None,
                },
            )));
        }
    };
    let daily_txs = row.try_get("cnt")?;

    let res_data = StatisticsData {
        active_addresses,
        total_txs,
        daily_txs,
    };

    Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsRes {
        code: 200,
        message: "".to_string(),
        data: Some(res_data),
    })))
}

pub async fn staking_info(api: &Api, height: Query<Option<i64>>) -> Result<StakingResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let sql_str = if let Some(height) = height.0 {
        format!(
            "SELECT height,info FROM delegations WHERE height={}",
            height
        )
    } else {
        "SELECT height,info FROM delegations ORDER BY height DESC LIMIT 1".to_string()
    };
    let delegation_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    let row = match delegation_res {
        Ok(row) => row,
        _ => {
            return Ok(StakingResponse::InternalError(Json(StakingRes {
                code: 500,
                message: "internal error.".to_string(),
                data: None,
            })));
        }
    };

    let height: i64 = row.try_get("height")?;
    let info_value: Value = row.try_get("info")?;
    let delegation_info: DelegationInfo = serde_json::from_value(info_value).unwrap();

    let mut reward: u64 = 0;
    let mut total_stake: u64 = 0;
    for (_, dl) in delegation_info.global_delegation_records_map {
        reward += dl.rwd_amount;
        for (_, amount) in dl.delegations {
            total_stake += amount
        }
    }

    let validators_res = api.rpc.load_validators(height).await;
    let validators = validators_res.unwrap();
    let mut mp = HashMap::new();
    for v in &validators.validators {
        mp.insert(v.address.clone(), v);
    }

    let mut active_validators: Vec<String> = vec![];
    let mut nonactive_validators: Vec<String> = vec![];
    for (addr, _) in delegation_info.validator_addr_map {
        if !mp.contains_key(addr.as_str()) {
            nonactive_validators.push(addr);
        } else {
            active_validators.push(addr);
        }
    }

    let data = StakingData {
        block_reward: reward,
        apy: delegation_info.return_rate.value,
        stake_ratio: total_stake as f64 / 21_420_000_000_000_000.0,
        active_validators,
        nonactive_validators,
    };

    Ok(StakingResponse::Ok(Json(StakingRes {
        code: 200,
        message: "".to_string(),
        data: Some(data),
    })))
}

#[derive(ApiResponse)]
pub enum DistributeResponse {
    #[oai(status = 200)]
    Ok(Json<DistributeResult>),
    #[oai(status = 500)]
    InternalError(Json<DistributeResult>),
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct DistributeResult {
    pub code: i32,
    pub message: String,
    pub data: Option<TxsDistribute>,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct TxsDistribute {
    pub transparent: i64,
    pub privacy: i64,
    pub prism: i64,
    pub evm_compatible: i64,
}

pub async fn distribute(api: &Api) -> Result<DistributeResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    // xhub
    let xhub: i64 =
        sqlx::query("SELECT count(*) as cnt FROM transaction WHERE value @? '$.function.XHub'")
            .fetch_one(&mut conn)
            .await?
            .try_get("cnt")?;

    // evm
    let evm: i64 = sqlx::query("SELECT count(*) as cnt FROM transaction WHERE ty=1")
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;

    let convert_account_sql = "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount'";
    let bar_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].AbarToBar') OR (value @? '$.body.operations[*].BarToAbar') OR (value @? '$.body.operations[*].TransferAnonAsset')";
    let hide_amount_sql = "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential'";
    let hide_type_sql = "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential'";
    let hide_amount_and_type_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential') AND (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential')";

    let convert_account: i64 = sqlx::query(convert_account_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let bar: i64 = sqlx::query(bar_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide_amount: i64 = sqlx::query(hide_amount_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide_type: i64 = sqlx::query(hide_type_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide_amount_and_type: i64 = sqlx::query(hide_amount_and_type_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide = hide_amount + hide_type - hide_amount_and_type;

    let non_evm: i64 = sqlx::query("SELECT count(*) as cnt FROM transaction where ty=0")
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;

    Ok(DistributeResponse::Ok(Json(DistributeResult {
        code: 0,
        message: "".to_string(),
        data: Some(TxsDistribute {
            transparent: non_evm - convert_account - bar - hide,
            privacy: bar + hide,
            prism: xhub + convert_account,
            evm_compatible: evm - xhub,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum AddressCountResponse {
    #[oai(status = 200)]
    Ok(Json<AddressCountResult>),
    #[oai(status = 200)]
    InternalError(Json<AddressCountResult>),
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct AddressCountResult {
    pub code: i32,
    pub message: String,
    pub data: Option<AddressCount>,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct AddressCount {
    pub address_count: i64,
}

pub async fn address_count(
    api: &Api,
    start_time: Query<i64>,
    end_time: Query<i64>,
) -> Result<AddressCountResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let address_count_sql = format!("SELECT jsonb_path_query(value,'$.body.operations[*].TransferAsset.body.transfer.*.public_key') \
        as addr FROM transaction WHERE timestamp >= {} AND timestamp <= {}", start_time.0, end_time.0);
    let res = sqlx::query(address_count_sql.as_str())
        .fetch_all(&mut conn)
        .await;
    if res.is_err() {
        return Ok(AddressCountResponse::InternalError(Json(
            AddressCountResult {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            },
        )));
    }

    let mut addrs: Vec<String> = vec![];
    for row in res.unwrap() {
        let v: Value = row.try_get("addr").unwrap();
        let addr = serde_json::from_value(v).unwrap();
        addrs.push(addr);
    }
    addrs.dedup();

    Ok(AddressCountResponse::Ok(Json(AddressCountResult {
        code: 200,
        message: "".to_string(),
        data: Some(AddressCount {
            address_count: addrs.len() as i64,
        }),
    })))
}
