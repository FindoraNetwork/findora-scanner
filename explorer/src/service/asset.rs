use crate::Api;
use anyhow::Result;
use module::db::asset::Asset;
use poem_openapi::{param::Path, payload::Json, ApiResponse};
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetAssetResponse {
    #[oai(status = 200)]
    Ok(Json<Asset>),
}

pub async fn get_asset(api: &Api, address: Path<String>) -> Result<GetAssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let str = format!("select * from asset where address = {}", address.0);
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let name: String = row.try_get("name")?;
    let address: String = row.try_get("address")?;
    let publisher: String = row.try_get("publisher")?;
    let memo: String = row.try_get("memo")?;
    let transferable: i8 = row.try_get("transferable")?;
    let amount: i64 = row.try_get("amount")?;
    let decimals: i8 = row.try_get("decimals")?;

    let asset = Asset {
        name,
        address,
        publisher,
        memo,
        transferable,
        amount,
        decimals,
    };

    Ok(GetAssetResponse::Ok(Json(asset)))
}
