use crate::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::chrono::Local;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum ChainStatisticsResponse {
    #[oai(status = 200)]
    Ok(Json<ChainStatisticsResult>),
    #[oai(status = 404)]
    NotFound(Json<ChainStatisticsResult>),
    #[oai(status = 500)]
    InternalError(Json<ChainStatisticsResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ChainStatisticsResult {
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

#[allow(clippy::let_unit_value)]
pub async fn distribute(api: &Api) -> Result<DistributeResponse> {
    let mut rds_conn = api.redis_client.get_connection().unwrap();
    let res = rds_conn.get("tx_distribute");
    if res.is_ok() {
        let distribute_data: String = res.unwrap();
        let v: TxsDistribute = serde_json::from_str(distribute_data.as_str()).unwrap();
        return Ok(DistributeResponse::Ok(Json(DistributeResult {
            code: 200,
            message: "".to_string(),
            data: Some(v),
        })));
    }

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

    // not evm
    let not_evm: i64 = sqlx::query("SELECT count(*) as cnt FROM transaction WHERE ty=0")
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;

    let convert_account_sql = "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount'";
    let bar_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].AbarToBar') OR (value @? '$.body.operations[*].BarToAbar') OR (value @? '$.body.operations[*].TransferAnonAsset')";
    let hide_amount_or_type_sql =  "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential') OR (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential')";
    let hide_amount_and_type_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential') AND (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential')";

    let convert_account: i64 = sqlx::query(convert_account_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let bar: i64 = sqlx::query(bar_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide_type_or_amount: i64 = sqlx::query(hide_amount_or_type_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide_amount_and_type: i64 = sqlx::query(hide_amount_and_type_sql)
        .fetch_one(&mut conn)
        .await?
        .try_get("cnt")?;
    let hide = hide_type_or_amount - hide_amount_and_type;

    let res_data = TxsDistribute {
        transparent: not_evm - convert_account - bar - hide,
        privacy: bar + hide,
        prism: xhub + convert_account,
        evm_compatible: evm - xhub,
    };

    let v = serde_json::to_string(&res_data).unwrap();
    let _: () = rds_conn.set("tx_distribute", v).unwrap();
    let _: () = rds_conn.expire("tx_distribute", 60 * 60 * 24).unwrap();

    Ok(DistributeResponse::Ok(Json(DistributeResult {
        code: 200,
        message: "".to_string(),
        data: Some(res_data),
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

#[allow(clippy::let_unit_value)]
pub async fn statistics(api: &Api, ty: Query<Option<i32>>) -> Result<ChainStatisticsResponse> {
    let key = if let Some(ty) = ty.0 {
        format!("stat{}", ty)
    } else {
        "stat".to_string()
    };
    let mut rds_conn = api.redis_client.get_connection().unwrap();
    let res = rds_conn.get(key.clone());
    if res.is_ok() {
        let stat_data: String = res.unwrap();
        let v: StatisticsData = serde_json::from_str(stat_data.as_str()).unwrap();
        return Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsResult {
            code: 200,
            message: "".to_string(),
            data: Some(v),
        })));
    }

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
                ChainStatisticsResult {
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
                ChainStatisticsResult {
                    code: 500,
                    message: "internal error while querying total addresses.".to_string(),
                    data: None,
                },
            )));
        }
    };

    let mut addrs: Vec<String> = vec![];
    for row in rows {
        let value: Value = row.try_get("addr")?;
        let addr: String = serde_json::from_value(value).unwrap();
        addrs.push(addr);
    }
    addrs.dedup();

    // daily txs
    let start_time = Local::now().date().and_hms(0, 0, 0);
    let sql_str = if let Some(ty) = ty.0 {
        format!(
            "SELECT COUNT(*) as cnt FROM transaction WHERE ty={} AND timestamp>={}",
            ty,
            start_time.timestamp()
        )
    } else {
        format!(
            "SELECT COUNT(*) as cnt FROM transaction WHERE timestamp>={}",
            start_time.timestamp()
        )
    };
    let daily_txs_res = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await;
    let row = match daily_txs_res {
        Ok(row) => row,
        _ => {
            return Ok(ChainStatisticsResponse::InternalError(Json(
                ChainStatisticsResult {
                    code: 500,
                    message: "internal error while querying daily txs.".to_string(),
                    data: None,
                },
            )));
        }
    };
    let daily_txs = row.try_get("cnt")?;

    let res_data = StatisticsData {
        active_addresses: addrs.len() as i64,
        total_txs,
        daily_txs,
    };

    let v = serde_json::to_string(&res_data).unwrap();
    let _: () = rds_conn.set(key.clone(), v).unwrap();
    let _: () = rds_conn.expire(key, 60 * 60 * 24).unwrap();

    Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsResult {
        code: 200,
        message: "".to_string(),
        data: Some(res_data),
    })))
}
