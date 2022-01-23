use crate::Api;
use anyhow::Result;
use module::display::address::DisplayAddress;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use sqlx::Row;
use module::db::tx::TransactionRef;

#[derive(ApiResponse)]
pub enum GetAddressResponse {
    #[oai(status = 200)]
    Ok(Json<DisplayAddress>),
}

pub async fn get_address(api: &Api, address: Path<String>) -> Result<GetAddressResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!(
        "select * from block where transaction_ref where from = {} or to = {} ",
        address.0, address.0
    );
    let rows = sqlx::query(str.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<TransactionRef> = vec![];
    for r in rows {
        let txid: String = r.try_get("txid")?;
        let block_id: String = r.try_get("block_id")?;
        let height: i64 = r.try_get("height")?;
        let from: String = r.try_get("from")?;
        let to: String = r.try_get("to")?;
        let asset: String = r.try_get("asset")?;
        let value: i64 = r.try_get("value")?;
        let op: String = r.try_get("op")?;
        let status:String = r.try_get("status")?;
        let tx = TransactionRef{
            txid,
            block_id,
            height,
            from,
            to,
            asset,
            value,
            op,
            status,
        };
        txs.push(tx)
    }

    Ok(GetAddressResponse::Ok(Json(DisplayAddress { txs })))
}
