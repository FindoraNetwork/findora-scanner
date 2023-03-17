use crate::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, ApiResponse, Object};
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
    let mut conn = api.storage.lock().await.acquire().await?;

    let convert_account_sql = "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount'";
    let bar_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].AbarToBar') OR (value @? '$.body.operations[*].BarToAbar') OR (value @? '$.body.operations[*].TransferAnonAsset')";
    let hide_amount_or_type_sql =  "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential') OR (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential')";
    let hide_amount_and_type_sql = "SELECT count(*) as cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].asset_type.Confidential') AND (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].amount.Confidential')";

    let mut res_data = TxsDistribute::default();

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

    res_data.transparent = not_evm - convert_account - bar - hide;
    res_data.privacy = bar + hide;
    res_data.prism = xhub + convert_account;
    res_data.evm_compatible = evm - xhub;

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

    let rows = sqlx::query(address_count_sql.as_str())
        .fetch_all(&mut conn)
        .await?;

    let mut addrs: Vec<String> = vec![];
    for row in rows {
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
    let mut conn = api.storage.lock().await.acquire().await?;

    let mut res_data = StatisticsData::default();

    // total txs
    let sql_str = if let Some(ty) = ty.0 {
        format!("SELECT COUNT(*) as cnt FROM transaction WHERE ty={ty}")
    } else {
        "SELECT COUNT(*) as cnt FROM transaction".to_string()
    };
    let row = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await?;
    let total_txs = row.try_get("cnt")?;

    // total address
    let evm_addr_cnt_sql =
        "SELECT count(*) AS cnt FROM transaction WHERE (value @? '$.function.Ethereum') AND ty=1"
            .to_string();
    let native_addr_cnt_sql = "SELECT count(*) AS cnt FROM transaction WHERE (value @? '$.body.operations[*].TransferAsset.body.transfer.*.public_key') AND ty=0".to_string();
    let addr_counts;
    match ty.0 {
        Some(0) => {
            let native_counts_row = sqlx::query(native_addr_cnt_sql.as_str())
                .fetch_one(&mut conn)
                .await?;
            addr_counts = native_counts_row.try_get("cnt")?;
        }
        Some(1) => {
            let evm_counts_row = sqlx::query(evm_addr_cnt_sql.as_str())
                .fetch_one(&mut conn)
                .await?;
            addr_counts = evm_counts_row.try_get("cnt")?;
        }
        _ => {
            let evm_counts_row = sqlx::query(evm_addr_cnt_sql.as_str())
                .fetch_one(&mut conn)
                .await?;
            let native_counts_row = sqlx::query(native_addr_cnt_sql.as_str())
                .fetch_one(&mut conn)
                .await?;
            let evm_counts: i64 = evm_counts_row.try_get("cnt")?;
            let native_counts: i64 = native_counts_row.try_get("cnt")?;
            addr_counts = evm_counts + native_counts;
        }
    }

    // daily txs
    let start_time = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
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
    let row = sqlx::query(sql_str.as_str()).fetch_one(&mut conn).await?;
    let daily_txs = row.try_get("cnt")?;

    res_data.active_addresses = addr_counts / 10;
    res_data.total_txs = total_txs;
    res_data.daily_txs = daily_txs;

    Ok(ChainStatisticsResponse::Ok(Json(ChainStatisticsResult {
        code: 200,
        message: "".to_string(),
        data: Some(res_data),
    })))
}

#[derive(ApiResponse)]
pub enum PrismSyncResponse {
    #[oai(status = 200)]
    Ok(Json<PrismResult>),
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct PrismResult {
    pub code: i32,
    pub message: String,
    pub data: PrismInfo,
}

#[derive(Serialize, Deserialize, Default, Debug, Object)]
pub struct PrismInfo {
    pub block: String,
    pub height: i64,
    pub timestamp: i64,
}

pub async fn prism_sync_info(api: &Api) -> Result<PrismSyncResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let sql_query =
        "SELECT block_hash,height,timestamp FROM result order by timestamp desc limit 1"
            .to_string();
    let row = sqlx::query(sql_query.as_str()).fetch_one(&mut conn).await?;

    let prism_info = PrismInfo {
        block: row.try_get("block_hash")?,
        height: row.try_get("height")?,
        timestamp: row.try_get("timestamp")?,
    };

    Ok(PrismSyncResponse::Ok(Json(PrismResult {
        code: 200,
        message: "".to_string(),
        data: prism_info,
    })))
}
