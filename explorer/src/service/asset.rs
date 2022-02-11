use crate::Api;
use anyhow::Result;
use poem_openapi::types::ToJSON;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use sqlx::Row;

#[derive(ApiResponse)]
pub enum GetAssetResponse {
    #[oai(status = 200)]
    Ok(Json<DisplayAsset>),

    #[oai(status = 400)]
    Err(Json<String>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetRules {
    pub decimals: i64,
    pub max_units: String,
    pub transfer_multisig_rules: Option<String>,
    pub transferable: bool,
    pub updatable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Code {
    pub val: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PubKey {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Asset {
    pub asset_rules: AssetRules,
    pub code: Code,
    pub issuer: PubKey,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Body {
    pub asset: Asset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct DefineAsset {
    pub body: Body,
    pub pubkey: PubKey,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct DisplayAsset {
    pub code: String,
    pub memo: String,
    pub issuer: String,
    pub max_units: i64,
    pub decimals: i64,
    pub transferable: bool,
    pub updatable: bool,
}

pub async fn get_asset(api: &Api, address: Path<String>) -> Result<GetAssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let code_res = base64::decode(&address.0);
    let code = match code_res {
        Ok(code) => code,
        _ => {
            return Ok(GetAssetResponse::Err(Json(String::from(
                "invalid base64 asset code",
            ))));
        }
    };

    let str = "select value from transaction where value @? '$.body.operations[*].DefineAsset.body.asset'".to_string();
    let res = sqlx::query(str.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(GetAssetResponse::Ok(Json(DisplayAsset::default())));
        }
    };

    let mut asset = DisplayAsset::default();
    for row in rows {
        let value: Value = row.try_get("value")?;
        let v = value
            .get("body")
            .unwrap()
            .get("operations")
            .unwrap()
            .get(0)
            .unwrap()
            .get("DefineAsset")
            .unwrap();

        let da: DefineAsset = from_value(v.to_json()).unwrap();
        if da.body.asset.code.val.eq(code.as_slice()) {
            asset.code = base64::encode(&da.body.asset.code.val);
            asset.memo = da.body.asset.memo;
            asset.issuer = da.body.asset.issuer.key;
            asset.decimals = da.body.asset.asset_rules.decimals;
            asset.max_units = da.body.asset.asset_rules.max_units.parse().unwrap();
            asset.transferable = da.body.asset.asset_rules.transferable;
            asset.updatable = da.body.asset.asset_rules.updatable;
        }
    }

    Ok(GetAssetResponse::Ok(Json(asset)))
}
