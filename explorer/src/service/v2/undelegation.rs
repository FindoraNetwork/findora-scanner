use crate::service::api::Api;
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2UndelegationResponse {
    #[oai(status = 200)]
    Ok(Json<V2UndelegationResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2UndelegationResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2Undelegation>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2Undelegation {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub new_delegator: String,
    pub target_validator: String,
    pub amount: u64,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}
#[allow(dead_code)]
pub async fn v2_get_undelegation(
    api: &Api,
    tx_hash: Path<String>,
) -> Result<V2UndelegationResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!(
        "SELECT tx,block,sender,amount,target_validator,new_delegator,height,timestamp,content FROM undelegations WHERE tx='{}'",
        tx_hash.0.to_lowercase()
    );

    let row = sqlx::query(sql_query.as_str())
        .fetch_one(&mut *conn)
        .await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let sender: String = row.try_get("sender")?;
    let amount: i64 = row.try_get("amount")?;
    let target_validator: String = row.try_get("target_validator")?;
    let new_delegator: String = row.try_get("new_delegator")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2Undelegation {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        new_delegator,
        target_validator,
        amount: amount as u64,
        height,
        timestamp,
        value,
    };

    Ok(V2UndelegationResponse::Ok(Json(V2UndelegationResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(ApiResponse)]
pub enum V2UndelegationsResponse {
    #[oai(status = 200)]
    Ok(Json<V2UndelegationsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2UndelegationsResult {
    pub code: u16,
    pub message: String,
    pub data: V2UndelegationsData,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2UndelegationsData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub items: Option<Vec<V2Undelegation>>,
}
#[allow(dead_code)]
pub async fn v2_get_undelegations(
    api: &Api,
    address: Query<Option<String>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2UndelegationsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let (sql_count, sql_query) = if let Some(addr) = address.0 {
        (
            format!(
                "SELECT count(*) AS cnt FROM undelegations WHERE sender='{}'",
                addr.to_lowercase()
            ),
            format!(
            "SELECT tx,block,sender,amount,target_validator,new_delegator,height,timestamp,content \
            FROM undelegations WHERE sender='{}' ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            addr.to_lowercase(), page_size, (page-1)*page_size),
        )
    } else {
        (
            "SELECT count(*) AS cnt FROM undelegations".to_string(),
            format!("SELECT tx,block,sender,amount,target_validator,new_delegator,height,timestamp,content \
            FROM undelegations ORDER BY timestamp DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size)
        )
    };

    let row_cnt = sqlx::query(sql_count.as_str())
        .fetch_one(&mut *conn)
        .await?;
    let total: i64 = row_cnt.try_get("cnt")?;
    let mut res: Vec<V2Undelegation> = vec![];
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let sender: String = row.try_get("sender")?;
        let amount: i64 = row.try_get("amount")?;
        let target_validator: String = row.try_get("target_validator")?;
        let new_delegator: String = row.try_get("new_delegator")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;
        res.push(V2Undelegation {
            tx_hash: tx,
            block_hash: block,
            from: sender,
            new_delegator,
            target_validator,
            amount: amount as u64,
            height,
            timestamp,
            value,
        });
    }

    Ok(V2UndelegationsResponse::Ok(Json(V2UndelegationsResult {
        code: 200,
        message: "".to_string(),
        data: V2UndelegationsData {
            page,
            page_size,
            total,
            items: Some(res),
        },
    })))
}
