use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use module::schema::Transaction;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Error::RowNotFound;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetAddressResponse {
    #[oai(status = 200)]
    Ok(Json<AddressRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AddressRes {
    pub code: i32,
    pub message: String,
    pub data: Option<AddressData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AddressData {
    pub txs: Vec<Transaction>,
    pub counts: usize,
}

pub async fn get_address(api: &Api, address: Path<String>) -> Result<GetAddressResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let pk = public_key_from_bech32(address.0.as_str());
    if pk.is_err() {
        return Ok(GetAddressResponse::Ok(Json(AddressRes {
            code: 40001,
            message: "invalid address".to_string(),
            data: None,
        })));
    }
    let pk_b64 = public_key_to_base64(&pk.unwrap());

    let str = format!(
        "SELECT * FROM transaction WHERE \
        (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') \
        or (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\")')",
        pk_b64, pk_b64
    );
    let res = sqlx::query(str.as_str()).fetch_all(&mut conn).await;

    let mut txs: Vec<Transaction> = vec![];
    let rows = match res {
        Ok(rows) => rows,
        Err(e) => {
            return match e {
                RowNotFound => Ok(GetAddressResponse::Ok(Json(AddressRes {
                    code: 200,
                    message: "".to_string(),
                    data: Some(AddressData::default()),
                }))),
                _ => Ok(GetAddressResponse::Ok(Json(AddressRes {
                    code: 50001,
                    message: "internal error".to_string(),
                    data: None,
                }))),
            }
        }
    };

    for row in rows {
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("value")?;
        let code: i64 = row.try_get("code")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let log: String = row.try_get("log")?;

        let tx = Transaction {
            txid: tx_id,
            block_id,
            ty,
            value,
            code,
            timestamp,
            log,
            events: vec![],
        };

        txs.push(tx)
    }
    let l = txs.len();
    Ok(GetAddressResponse::Ok(Json(AddressRes {
        code: 200,
        message: "".to_string(),
        data: Some(AddressData { txs, counts: l }),
    })))
}
