use crate::service::api::Api;
use anyhow::Result;
use module::utils::crypto::bech32enc;
use poem_openapi::{param::Path, param::Query, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use zei::serialization::ZeiFromToBytes;
use zei::xfr::sig::XfrPublicKey;

#[derive(ApiResponse)]
pub enum AssetResponse {
    #[oai(status = 200)]
    Ok(Json<AssetResult>),
    #[oai(status = 400)]
    BadRequest(Json<AssetResult>),
    #[oai(status = 404)]
    NotFound(Json<AssetResult>),
    #[oai(status = 500)]
    InternalError(Json<AssetResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetResult {
    pub code: i32,
    pub message: String,
    pub data: Option<Vec<AssetDisplay>>,
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
    pub issuer: String,
    pub issued_at_block: String,
    pub issued_at_tx: String,
    pub issued_at_height: i64,
    pub memo: String,
    pub asset_rules: AssetRules,
    pub asset_code: String,
    pub code: Code,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Properties {
    pub code: Code,
    pub issuer: PubKey,
    pub asset_rules: AssetRules,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetRPCResult {
    pub properties: Properties,
    pub digest: [u8; 32],
    pub units: u32,
    pub confidential_units: [u8; 32],
}

pub async fn get_asset(api: &Api, address: Path<String>) -> Result<AssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let code_res = base64::decode_config(&address.0, base64::URL_SAFE);
    let code = match code_res {
        Ok(code) => code,
        _ => {
            return Ok(AssetResponse::BadRequest(Json(AssetResult {
                code: 400,
                message: "invalid base64 asset code".to_string(),
                data: None,
            })));
        }
    };

    let sql_query = "SELECT jsonb_path_query(value,'$.body.operations[*].DefineAsset.body.asset') AS asset,tx_hash,block_hash,height FROM transaction ORDER BY height DESC".to_string();
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;

    let mut assets: Vec<AssetDisplay> = vec![];
    for row in rows {
        let v: Value = row.try_get("asset").unwrap();
        let a: Asset = serde_json::from_value(v).unwrap();

        if a.code.val.eq(&code) {
            let height: i64 = row.try_get("height")?;
            let block: String = row.try_get("block_hash")?;
            let tx: String = row.try_get("tx_hash")?;

            let pk_vec = base64::decode_config(&a.issuer.key, base64::URL_SAFE)?;
            let pk = XfrPublicKey::zei_from_bytes(&pk_vec).unwrap_or_default();
            let issuer_addr = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
            let asset_code = base64::encode_config(&a.code.val, base64::URL_SAFE);

            assets.push(AssetDisplay {
                issuer: issuer_addr,
                issued_at_block: block,
                issued_at_tx: tx,
                issued_at_height: height,
                memo: a.memo,
                asset_rules: a.asset_rules,
                asset_code,
                code: a.code,
            });
        }
    }

    if assets.is_empty() {
        // get asset from node RPC.
        let asset_url = api
            .platform_server
            .rpc
            .join(format!("asset_token/{}", address.0).as_str())?;

        let resp = api
            .platform_server
            .client
            .get(asset_url)
            .send()
            .await?
            .json::<AssetRPCResult>()
            .await?;

        let pk_vec = base64::decode_config(&resp.properties.issuer.key, base64::URL_SAFE)?;
        let pk = XfrPublicKey::zei_from_bytes(&pk_vec).unwrap_or_default();
        let issuer_addr = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
        let asset_code = base64::encode_config(&resp.properties.code.val, base64::URL_SAFE);

        assets.push(AssetDisplay {
            issuer: issuer_addr,
            issued_at_block: "".to_string(),
            issued_at_tx: "".to_string(),
            issued_at_height: 0,
            memo: "".to_string(),
            asset_rules: resp.properties.asset_rules,
            asset_code,
            code: resp.properties.code,
        })
    }

    Ok(AssetResponse::Ok(Json(AssetResult {
        code: 200,
        message: "".to_string(),
        data: Some(assets),
    })))
}

#[derive(ApiResponse)]
pub enum AssetListResponse {
    #[oai(status = 200)]
    Ok(Json<AssetListResult>),
    #[oai(status = 400)]
    BadRequest(Json<AssetListResult>),
    #[oai(status = 404)]
    NotFound(Json<AssetListResult>),
    #[oai(status = 500)]
    InternalError(Json<AssetListResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<AssetListData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct AssetListData {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub assets: Vec<AssetDisplay>,
}

pub async fn get_asset_list(
    api: &Api,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<AssetListResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    if page_size > 100 {
        return Ok(AssetListResponse::BadRequest(Json(AssetListResult {
            code: 400,
            message: "invalid page size, the maximum page size is 100".to_string(),
            data: None,
        })));
    }

    let sql_total =
        "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].DefineAsset'";
    let row = sqlx::query(sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("cnt")?;

    let sql_query = format!("SELECT jsonb_path_query(value,'$.body.operations[*].DefineAsset.body.asset') AS asset,tx_hash,block_hash,height FROM transaction ORDER BY height DESC LIMIT {} OFFSET {}", page_size, page_size*(page-1));
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;

    let mut assets: Vec<AssetDisplay> = vec![];
    for row in rows {
        let height: i64 = row.try_get("height")?;
        let block: String = row.try_get("block_hash")?;
        let tx: String = row.try_get("tx_hash")?;

        let v: Value = row.try_get("asset").unwrap();
        let a: Asset = serde_json::from_value(v).unwrap();
        let pk_vec = base64::decode_config(&a.issuer.key, base64::URL_SAFE)?;
        let pk = XfrPublicKey::zei_from_bytes(&pk_vec).unwrap_or_default();
        let issuer_addr = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
        let asset_code = base64::encode_config(&a.code.val, base64::URL_SAFE);

        assets.push(AssetDisplay {
            issuer: issuer_addr,
            issued_at_block: block,
            issued_at_tx: tx,
            issued_at_height: height,
            memo: a.memo,
            asset_rules: a.asset_rules,
            asset_code,
            code: a.code,
        });
    }

    Ok(AssetListResponse::Ok(Json(AssetListResult {
        code: 200,
        message: "".to_string(),
        data: Some(AssetListData {
            page,
            page_size,
            total,
            assets,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum IssueAssetResponse {
    #[oai(status = 200)]
    Ok(Json<IssueAssetResult>),
    #[oai(status = 400)]
    BadRequest(Json<IssueAssetResult>),
    #[oai(status = 404)]
    NotFound(Json<IssueAssetResult>),
    #[oai(status = 500)]
    InternalError(Json<IssueAssetResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct IssueAssetResult {
    pub code: i32,
    pub message: String,
    pub data: Option<IssueAssetDisplay>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct IssueAssetDisplay {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub assets: Vec<IssueAssetData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct IssueAssetData {
    pub issuer: String,
    pub issued_at_block: String,
    pub issued_at_tx: String,
    pub issued_at_height: i64,
    pub asset_code: String,
    pub asset: IssueAsset,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct IssueAsset {
    pub body: IssueAssetBody,
    pub pubkey: PubKey,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct IssueAssetBody {
    pub code: Code,
    pub num_outputs: u64,
    pub seq_num: u64,
    pub records: Value,
}

pub async fn get_issued_asset_list(
    api: &Api,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<IssueAssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    if page_size > 100 {
        return Ok(IssueAssetResponse::BadRequest(Json(IssueAssetResult {
            code: 400,
            message: "invalid page size, the maximum page size is 100".to_string(),
            data: None,
        })));
    }

    let sql_total =
        "SELECT count(*) as cnt FROM transaction WHERE value @? '$.body.operations[*].IssueAsset'";
    let row = sqlx::query(sql_total).fetch_one(&mut *conn).await?;
    let total: i64 = row.try_get("cnt")?;

    let sql_query = format!("SELECT jsonb_path_query(value,'$.body.operations[*].IssueAsset') AS asset,block_hash,tx_hash,height FROM transaction ORDER BY height DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size);
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    let mut assets: Vec<IssueAssetData> = vec![];
    for row in rows {
        let block: String = row.try_get("block_hash")?;
        let tx: String = row.try_get("tx_hash")?;
        let height: i64 = row.try_get("height")?;

        let v: Value = row.try_get("asset").unwrap();
        let a: IssueAsset = serde_json::from_value(v).unwrap();
        let pk_vec = base64::decode_config(&a.pubkey.key, base64::URL_SAFE)?;
        let pk = XfrPublicKey::zei_from_bytes(&pk_vec).unwrap_or_default();
        let issuer = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
        let asset_code = base64::encode_config(&a.body.code.val, base64::URL_SAFE);

        assets.push(IssueAssetData {
            issuer,
            issued_at_block: block,
            issued_at_tx: tx,
            issued_at_height: height,
            asset_code,
            asset: a,
        });
    }

    Ok(IssueAssetResponse::Ok(Json(IssueAssetResult {
        code: 200,
        message: "".to_string(),
        data: Some(IssueAssetDisplay {
            page,
            page_size,
            total,
            assets,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum SingleIssueAssetResponse {
    #[oai(status = 200)]
    Ok(Json<SingleIssueAssetResult>),
    #[oai(status = 400)]
    BadRequest(Json<SingleIssueAssetResult>),
    #[oai(status = 404)]
    NotFound(Json<SingleIssueAssetResult>),
    #[oai(status = 500)]
    InternalError(Json<SingleIssueAssetResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct SingleIssueAssetResult {
    pub code: i32,
    pub message: String,
    pub data: Option<IssueAssetData>,
}

pub async fn get_issued_asset(
    api: &Api,
    address: Path<String>,
) -> Result<SingleIssueAssetResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let code_res = base64::decode_config(&address.0, base64::URL_SAFE);
    let code = match code_res {
        Ok(code) => code,
        _ => {
            return Ok(SingleIssueAssetResponse::BadRequest(Json(
                SingleIssueAssetResult {
                    code: 400,
                    message: "invalid base64 asset code".to_string(),
                    data: None,
                },
            )));
        }
    };

    let sql_query = "SELECT jsonb_path_query(value,'$.body.operations[*].IssueAsset') AS asset,block_hash,tx_hash,height FROM transaction ORDER BY height DESC".to_string();
    let rows = sqlx::query(sql_query.as_str())
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let v: Value = row.try_get("asset").unwrap();
        let a: IssueAsset = serde_json::from_value(v).unwrap();

        if a.body.code.val.eq(&code) {
            let block: String = row.try_get("block_hash")?;
            let tx: String = row.try_get("tx_hash")?;
            let height: i64 = row.try_get("height")?;
            let pk_vec = base64::decode_config(&a.pubkey.key, base64::URL_SAFE)?;
            let pk = XfrPublicKey::zei_from_bytes(&pk_vec).unwrap_or_default();
            let issuer = bech32enc(&XfrPublicKey::zei_to_bytes(&pk));
            let asset_code = base64::encode_config(&a.body.code.val, base64::URL_SAFE);

            return Ok(SingleIssueAssetResponse::Ok(Json(SingleIssueAssetResult {
                code: 200,
                message: "".to_string(),
                data: Some(IssueAssetData {
                    issuer,
                    issued_at_block: block,
                    issued_at_tx: tx,
                    issued_at_height: height,
                    asset_code,
                    asset: a,
                }),
            })));
        }
    }

    Ok(SingleIssueAssetResponse::NotFound(Json(
        SingleIssueAssetResult {
            code: 404,
            message: "asset not found".to_string(),
            data: None,
        },
    )))
}
