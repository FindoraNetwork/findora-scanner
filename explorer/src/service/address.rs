use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use module::schema::Transaction;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum AddressResponse {
    #[oai(status = 200)]
    Ok(Json<AddressResult>),
    #[oai(status = 400)]
    BadRequest(Json<AddressResult>),
    #[oai(status = 404)]
    NotFound(Json<AddressResult>),
    #[oai(status = 500)]
    InternalError(Json<AddressResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AddressResult {
    pub code: i32,
    pub message: String,
    pub data: Option<AddressData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AddressData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub txs: Vec<Transaction>,
}

pub async fn get_address(
    api: &Api,
    address: Path<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<AddressResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let pk = public_key_from_bech32(address.0.as_str());
    if pk.is_err() {
        return Ok(AddressResponse::BadRequest(Json(AddressResult {
            code: 400,
            message: "invalid bech32 address".to_string(),
            data: None,
        })));
    }

    let pk_b64 = public_key_to_base64(&pk.unwrap());
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let sql_str = format!(
        "SELECT * FROM transaction WHERE \
        (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') \
        or (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\")') \
        ORDER BY timestamp DESC LIMIT {} OFFSET {}", pk_b64, pk_b64, page_size, (page - 1) * page_size);

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<Transaction> = vec![];
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log: String = row.try_get("log")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;
        let tx = Transaction {
            tx_hash,
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            result,
            value,
        };
        txs.push(tx)
    }

    // total items
    let sql_total = format!(
        "SELECT count(*) as total FROM transaction WHERE \
        (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') \
        or (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\")')", pk_b64, pk_b64);
    let res = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await;
    let total: i64 = res.unwrap().try_get("total")?;

    Ok(AddressResponse::Ok(Json(AddressResult {
        code: 200,
        message: "".to_string(),
        data: Some(AddressData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}
