use crate::Api;
use anyhow::Result;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
#[derive(ApiResponse)]
pub enum AssetResponse {
    #[oai(status = 200)]
    Ok(Json<AssetRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetRes {
    pub code: i32,
    pub message: String,
    pub data: Option<DisplayAsset>,
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
    pub chain_name: String,
    pub max_units: i64,
    pub decimals: i64,
    pub price: f64,
    pub market_value: f64,
    pub transferable: bool,
    pub updatable: bool,
}

pub async fn get_asset(api: &Api, address: Path<String>) -> Result<AssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let code_res = base64::decode(&address.0);
    let code = match code_res {
        Ok(code) => code,
        _ => {
            return Ok(AssetResponse::Ok(Json(AssetRes {
                code: 400,
                message: "invalid asset code".to_string(),
                data: None,
            })));
        }
    };

    let str = "SELECT jsonb_path_query(value,'$.body.operations[*].DefineAsset.body.asset') as asset FROM transaction".to_string();
    let res = sqlx::query(str.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(AssetResponse::Ok(Json(AssetRes {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
    };

    let mut da = DisplayAsset::default();
    for row in rows {
        let v: Value = row.try_get("asset").unwrap();
        let asset: Asset = serde_json::from_value(v).unwrap();
        if asset.code.val.eq(&code) {
            da.code = base64::encode(&asset.code.val);
            da.memo = asset.memo;
            da.issuer = asset.issuer.key;
            da.chain_name = String::from("Findora");
            da.max_units = asset.asset_rules.max_units.parse().unwrap();
            da.decimals = asset.asset_rules.decimals;
            da.price = 0.0;
            da.market_value = 0.0;
            da.transferable = asset.asset_rules.transferable;
            da.updatable = asset.asset_rules.updatable;
        }
    }

    Ok(AssetResponse::Ok(Json(AssetRes {
        code: 200,
        message: "".to_string(),
        data: Some(da),
    })))
}
