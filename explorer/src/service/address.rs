use crate::Api;
use anyhow::Result;
use module::db::tx::TransactionRef;
use module::display::address::DisplayAddress;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetAddressResponse {
    #[oai(status = 200)]
    Ok(Json<DisplayAddress>),
}

pub async fn get_address(api: &Api, address: Path<String>) -> Result<GetAddressResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!(
        "SELECT * FROM transaction WHERE \
        (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@ == \"{}\")') \
        or (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@ == \"{}\")')",
        address.0, address.0
    );
    let res = sqlx::query(str.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(GetAddressResponse::Ok(Json(DisplayAddress::default())));
        }
    };

    let mut txs: Vec<TransactionRef> = vec![];
    for r in rows {
        let txid: String = r.try_get("txid")?;
        let block_id: String = r.try_get("block_id")?;
        let height: i64 = r.try_get("height")?;
        let from_address: String = r.try_get("from_address")?;
        let to_address: String = r.try_get("to_address")?;
        let asset: String = r.try_get("asset")?;
        let value: i64 = r.try_get("value")?;
        let typ: String = r.try_get("op")?;
        let status: String = r.try_get("status")?;
        let timestamp = r.try_get("timestamp")?;

        let tx = TransactionRef {
            txid,
            block_id,
            height,
            from_address,
            to_address,
            asset,
            value,
            typ,
            status,
            timestamp,
        };

        txs.push(tx)
    }

    Ok(GetAddressResponse::Ok(Json(DisplayAddress {
        total: txs.len(),
        txs,
    })))
}
