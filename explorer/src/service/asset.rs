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
    let str = format!("SELECT * FROM transaction WHERE address='{}' ", address.0);
    let res = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match res {
        Ok(row) => row,
        _ => {
            return Ok(GetAssetResponse::Ok(Json(Asset::default())));
        }
    };

    let code: String = row.try_get("code")?;
    let memo: String = row.try_get("memo")?;
    let issuer: String = row.try_get("issuer")?;
    let max_uints: i64 = row.try_get("max_ints")?;
    let transferable: bool = row.try_get("transferable")?;
    let updatable: bool = row.try_get("updatable")?;

    let asset = Asset {
        code,
        memo,
        issuer,
        max_uints,
        transferable,
        updatable,
    };

    Ok(GetAssetResponse::Ok(Json(asset)))
}
