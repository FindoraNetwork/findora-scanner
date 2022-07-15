use crate::Api;
use anyhow::Result;
use module::utils::crypto::bech32enc;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use ruc::{d, RucResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use zei::serialization::ZeiFromToBytes;
use zei::xfr::sig::XfrPublicKey;

#[derive(ApiResponse)]
pub enum AssetResponse {
    #[oai(status = 200)]
    Ok(Json<AssetRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetRes {
    pub code: i32,
    pub message: String,
    pub data: Option<AssetDisplay>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetRules {
    pub decimals: i64,
    pub max_units: String,
    pub transfer_multisig_rules: Option<String>,
    pub transferable: bool,
    pub updatable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Code {
    pub val: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PubKey {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Asset {
    pub asset_rules: AssetRules,
    pub code: Code,
    pub issuer: PubKey,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetDisplay {
    pub asset_rules: AssetRules,
    pub code: Code,
    pub issuer: String,
    pub memo: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Body {
    pub asset: Asset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DefineAsset {
    pub body: Body,
    pub pubkey: PubKey,
    pub signature: String,
}

pub async fn get_asset(api: &Api, address: Path<String>) -> Result<AssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let code_res = base64::decode_config(&address.0, base64::URL_SAFE);
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

    let mut asset = AssetDisplay::default();
    for row in rows {
        let v: Value = row.try_get("asset").unwrap();
        let a: Asset = serde_json::from_value(v).unwrap();

        if a.code.val.eq(&code) {
            let pk = base64::decode_config(&a.issuer.key, base64::URL_SAFE)
                .c(d!())
                .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
                .unwrap();
            let issuer_addr = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));

            asset.memo = a.memo;
            asset.issuer = issuer_addr;
            asset.code = a.code;
            asset.asset_rules = a.asset_rules;
        }
    }

    Ok(AssetResponse::Ok(Json(AssetRes {
        code: 200,
        message: "".to_string(),
        data: Some(asset),
    })))
}
