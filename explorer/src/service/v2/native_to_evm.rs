use crate::service::api::Api;
use crate::service::v1::transaction::{
    ConvertAccount, V2PrismItem, V2PrismRecord, V2PrismRecordResponse, V2PrismRecordResult,
};
use anyhow::Result;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum V2NativeToEvmTxsResponse {
    #[oai(status = 200)]
    Ok(Json<V2NativeToEvmTxsResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTxsResult {
    pub code: u16,
    pub message: String,
    pub data: V2NativeToEvmTxsData,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTxsData {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub txs: Vec<V2NativeToEvmTx>,
}

pub async fn v2_get_n2e_txs(
    api: &Api,
    page: Query<Option<i32>>,
    page_size: Query<Option<i32>>,
) -> Result<V2NativeToEvmTxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let sql_total = "SELECT count(*) FROM n2e";
    let row = sqlx::query(sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("count")?;

    let sql_query = format!("SELECT tx,block,sender,receiver,asset,amount,height,timestamp,content FROM n2e ORDER BY timestamp DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size);
    let mut res: Vec<V2NativeToEvmTx> = vec![];
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let tx: String = row.try_get("tx")?;
        let block: String = row.try_get("block")?;
        let sender: String = row.try_get("sender")?;
        let receiver: String = row.try_get("receiver")?;
        let asset: String = row.try_get("asset")?;
        let amount: String = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let value: Value = row.try_get("content")?;
        res.push(V2NativeToEvmTx {
            tx_hash: tx,
            block_hash: block,
            from: sender,
            to: receiver,
            asset,
            amount,
            height,
            timestamp,
            value,
        })
    }

    Ok(V2NativeToEvmTxsResponse::Ok(Json(V2NativeToEvmTxsResult {
        code: 200,
        message: "".to_string(),
        data: V2NativeToEvmTxsData {
            total,
            page,
            page_size,
            txs: res,
        },
    })))
}

#[derive(ApiResponse)]
pub enum V2NativeToEvmTxResponse {
    #[oai(status = 200)]
    Ok(Json<V2NativeToEvmTxResult>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTxResult {
    pub code: u16,
    pub message: String,
    pub data: Option<V2NativeToEvmTx>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct V2NativeToEvmTx {
    pub tx_hash: String,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub asset: String,
    pub amount: String,
    pub height: i64,
    pub timestamp: i64,
    pub value: Value,
}

pub async fn v2_get_n2e_tx(api: &Api, tx_hash: Path<String>) -> Result<V2NativeToEvmTxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = format!("SELECT tx,block,sender,receiver,asset,amount,height,timestamp,content FROM n2e WHERE tx='{}'", tx_hash.0.to_lowercase());

    let row = sqlx::query(sql_query.as_str())
        .fetch_one(&mut *conn)
        .await?;

    let tx: String = row.try_get("tx")?;
    let block: String = row.try_get("block")?;
    let sender: String = row.try_get("sender")?;
    let receiver: String = row.try_get("receiver")?;
    let asset: String = row.try_get("asset")?;
    let amount: String = row.try_get("amount")?;
    let height: i64 = row.try_get("height")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let value: Value = row.try_get("content")?;

    let res = V2NativeToEvmTx {
        tx_hash: tx,
        block_hash: block,
        from: sender,
        to: receiver,
        asset,
        amount,
        height,
        timestamp,
        value,
    };

    Ok(V2NativeToEvmTxResponse::Ok(Json(V2NativeToEvmTxResult {
        code: StatusCode::OK.as_u16(),
        message: "".to_string(),
        data: Some(res),
    })))
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConvertAccountReceiver {
    #[serde(rename = "Ethereum")]
    pub ethereum: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct V2ConvertAccount {
    pub nonce: Value,
    pub asset_type: Option<Vec<u8>>,
    pub receiver: ConvertAccountReceiver,
    pub signer: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConvertAccountOperation {
    #[serde(rename = "ConvertAccount")]
    pub convert_account: ConvertAccount,
}

pub async fn v2_get_prism_records_send(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2PrismRecordResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut items: Vec<V2PrismItem> = vec![];

    let sql_total = format!(
        "select count(*) as cnt from n2e where sender='{}'",
        address.0
    );
    let row_total = sqlx::query(sql_total.as_str())
        .fetch_one(&mut *conn)
        .await?;
    let total: i64 = row_total.try_get("cnt")?;

    let sql_query = format!(
        "select tx,block,sender,receiver,asset,amount,height,timestamp,content from n2e where sender='{}' order by timestamp desc limit {} offset {}",
        address.0,
        page_size,
        (page - 1) * page_size
    );
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;

    for row in rows {
        let tx_hash: String = row.try_get("tx")?;
        let block_hash: String = row.try_get("block")?;
        let sender: String = row.try_get("sender")?;
        let receiver: String = row.try_get("receiver")?;
        let asset: String = row.try_get("asset")?;
        let amount: String = row.try_get("amount")?;
        let height: i64 = row.try_get("height")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let content: Value = row.try_get("content")?;
        let ca: ConvertAccountOperation = serde_json::from_value(content)?;
        let ca_bin = serde_json::to_vec(&ca.convert_account)?;
        items.push(V2PrismItem {
            block_hash,
            tx_hash,
            from: sender,
            to: receiver,
            asset,
            amount,
            decimal: 6,
            height,
            timestamp,
            data: base64::encode(&ca_bin),
        });
    }

    Ok(V2PrismRecordResponse::Ok(Json(V2PrismRecordResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2PrismRecord {
            total,
            page,
            page_size,
            items,
        }),
    })))
}
