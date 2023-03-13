use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use ethereum_types::{H160, H256};
use module::schema::{
    EvmTx, PrismTransaction, TransactionResponse, UnDelegationValue, ABAR_TO_ABAR, ABAR_TO_BAR,
    BAR_TO_ABAR, CLAIM, DEFINE_OR_ISSUE_ASSET, EVM_TRANSFER, HIDE_ASSET_AMOUNT, HIDE_ASSET_TYPE,
    HIDE_ASSET_TYPE_AND_AMOUNT, PRISM_EVM_TO_NATIVE, PRISM_NATIVE_TO_EVM, STAKING, UNSTAKING,
};
use module::utils::crypto::recover_signer;
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha3::{Digest, Keccak256};
use sqlx::Row;
use std::ops::Add;

#[derive(ApiResponse)]
pub enum TxResponse {
    #[oai(status = 200)]
    Ok(Json<TxRes>),
    #[oai(status = 404)]
    NotFound(Json<TxRes>),
    #[oai(status = 500)]
    InternalError(Json<TxRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxRes {
    pub code: i32,
    pub message: String,
    pub data: Option<TransactionResponse>,
}

#[derive(ApiResponse)]
pub enum TxsResponse {
    #[oai(status = 200)]
    Ok(Json<TxsRes>),
    #[oai(status = 500)]
    InternalError(Json<TxRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxsRes {
    pub code: i32,
    pub message: String,
    pub data: Option<TxsData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxsData {
    page: i64,
    page_size: i64,
    total: i64,
    txs: Vec<TransactionResponse>,
}

#[derive(ApiResponse)]
pub enum PmtxsResponse {
    #[oai(status = 200)]
    Ok(Json<PmtxsRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PmtxsRes {
    pub code: i32,
    pub message: String,
    pub data: Option<PmtxsData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PmtxsData {
    page: i64,
    page_size: i64,
    total: i64,
    txs: Vec<PrismTransaction>,
}

#[derive(ApiResponse)]
pub enum PrismRecordResponse {
    #[oai(status = 200)]
    Ok(Json<PrismRecordResult>),
    #[oai(status = 400)]
    BadRequest(Json<PrismRecordResult>),
    #[oai(status = 500)]
    InternalError(Json<PrismRecordResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PrismRecordResult {
    pub code: i32,
    pub message: String,
    pub data: Option<PrismRecord>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PrismRecord {
    pub receive_from: Vec<PrismItem>,
    pub send_to: Vec<PrismItem>,
}

#[derive(ApiResponse)]
pub enum PrismRecordResponseNew {
    #[oai(status = 200)]
    Ok(Json<PrismRecordResultNew>),
    #[oai(status = 400)]
    BadRequest(Json<PrismRecordResultNew>),
    #[oai(status = 500)]
    InternalError(Json<PrismRecordResultNew>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PrismRecordResultNew {
    pub code: i32,
    pub message: String,
    pub data: Option<PrismRecordNew>,
}

#[derive(ApiResponse)]
pub enum V2PrismRecordResponse {
    #[oai(status = 200)]
    Ok(Json<V2PrismRecordResult>),
    #[oai(status = 400)]
    BadRequest(Json<V2PrismRecordResult>),
    #[oai(status = 500)]
    InternalError(Json<V2PrismRecordResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PrismRecordNew {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<PrismItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2PrismRecordResult {
    pub code: i32,
    pub message: String,
    pub data: Option<V2PrismRecord>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct V2PrismRecord {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub items: Vec<V2PrismItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct V2PrismItem {
    pub address: String,
    pub tx_hash: String,
    pub amount: u64,
    pub timestamp: i64,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct PrismItem {
    pub tx_hash: String,
    pub address: String,
    pub amount: u64,
    pub timestamp: i64,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct NonConfidentialItem {
    pub asset: Vec<u8>,
    pub amount: u64,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct CallDataLog {
    pub data: Vec<u8>,
    pub topics: Vec<String>,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct Call {
    pub logs: Value,
    pub value: Value,
    pub used_gas: Value,
    pub exit_reason: Value,
    pub non_confidential_outputs: Vec<NonConfidentialItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct CallData {
    #[serde(rename = "Call")]
    pub call: Call,
}

pub async fn get_prism_received(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<V2PrismRecordResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut items: Vec<V2PrismItem> = vec![];

    let pubkey = public_key_from_bech32(&address.0);
    if pubkey.is_err() {
        return Ok(V2PrismRecordResponse::BadRequest(Json(
            V2PrismRecordResult {
                code: 400,
                message: "invalid bech32 address".to_string(),
                data: None,
            },
        )));
    }

    let pubkey_base64 = public_key_to_base64(&pubkey.unwrap());

    let sql_counts = format!("SELECT count(*) AS cnt FROM result WHERE value @? '$.Call.non_confidential_outputs[*].target ? (@==\"{pubkey_base64}\")'");
    let row_counts = sqlx::query(sql_counts.as_str())
        .fetch_one(&mut conn)
        .await?;
    let total: i64 = row_counts.try_get("cnt")?;

    let sql_query = format!("SELECT tx_hash,timestamp,value AS v FROM result WHERE value @? '$.Call.non_confidential_outputs[*].target ? (@==\"{pubkey_base64}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size);
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let timestamp: i64 = row.try_get("timestamp")?;

        let result_val: Value = row.try_get("v")?;
        let call_data: CallData = serde_json::from_value(result_val.clone())?;
        let result_bin = serde_json::to_vec(&result_val)?;

        let sql_tx = format!("SELECT value AS v FROM transaction WHERE tx_hash=\'{tx_hash}\'");
        let r = sqlx::query(sql_tx.as_str()).fetch_one(&mut conn).await?;
        let tx_val: Value = r.try_get("v")?;
        let evm_tx: EvmTx = serde_json::from_value(tx_val)?;
        let signer = recover_signer(&evm_tx.function.ethereum.transact).unwrap();

        for noc in call_data.call.non_confidential_outputs {
            items.push(V2PrismItem {
                data: base64::encode(&result_bin),
                address: format!("{signer:?}"),
                tx_hash: tx_hash.clone(),
                amount: noc.amount,
                timestamp,
            });
        }
    }

    Ok(V2PrismRecordResponse::Ok(Json(V2PrismRecordResult {
        code: 200,
        message: "".to_string(),
        data: Some(V2PrismRecord {
            total,
            page,
            page_size,
            items,
        }),
    })))
}

// pub async fn get_prism_records_receive_from(
//     api: &Api,
//     address: Query<String>,
//     page: Query<Option<i64>>,
//     page_size: Query<Option<i64>>,
// ) -> Result<PrismRecordResponseNew> {
//     let mut conn = api.storage.lock().await.acquire().await?;
//
//     let page = page.0.unwrap_or(1);
//     let page_size = page_size.0.unwrap_or(10);
//     let mut receive_from: Vec<PrismItem> = vec![];
//     let total;
//
//     match "fra".as_bytes().eq(&address.0.as_bytes()[..3]) {
//         true => {
//             let pk_result = public_key_from_bech32(address.0.as_str());
//             if pk_result.is_err() {
//                 return Ok(PrismRecordResponseNew::BadRequest(Json(
//                     PrismRecordResultNew {
//                         code: 400,
//                         message: "invalid bech32 address".to_string(),
//                         data: None,
//                     },
//                 )));
//             }
//             let pk = pk_result.unwrap();
//             let base64_addr = public_key_to_base64(&pk);
//
//             let sql_receive_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.function.XHub.NonConfidentialTransfer.outputs[*].target ? (@==\"{base64_addr}\")'");
//             let sql_receive = format!("SELECT tx_hash, timestamp, jsonb_path_query(value, '$.function.XHub.NonConfidentialTransfer.outputs[*].amount') AS amount, jsonb_path_query(value, '$.signature[0]') as pk FROM transaction WHERE value @? '$.function.XHub.NonConfidentialTransfer.outputs[*].target ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", base64_addr, page_size, (page-1)*page_size);
//             let row_receive_count = sqlx::query(sql_receive_count.as_str())
//                 .fetch_one(&mut conn)
//                 .await?;
//             total = row_receive_count.try_get("cnt")?;
//
//             let rows_receive = sqlx::query(sql_receive.as_str())
//                 .fetch_all(&mut conn)
//                 .await?;
//             for row in rows_receive {
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let amount_val: Value = row.try_get("amount")?;
//                 let pk_val: Value = row.try_get("pk")?;
//
//                 let amount: u64 = serde_json::from_value(amount_val).unwrap();
//                 let pk: [u8; 32] = serde_json::from_value(pk_val).unwrap();
//                 let from: String = format!("{:?}", H160::from_slice(&pk[4..24]));
//
//                 receive_from.push(PrismItem {
//                     tx_hash,
//                     address: from,
//                     amount,
//                     timestamp,
//                 });
//             }
//         }
//         _ => {
//             let sql_receive_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.receiver.Ethereum ? (@==\"{}\")'", address.0);
//             let sql_receive = format!("SELECT tx_hash, timestamp, jsonb_path_query(value, '$.body.operations[*].ConvertAccount.value') AS amount, jsonb_path_query(value, '$.body.operations[*].ConvertAccount.signer') AS signer FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.receiver.Ethereum ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", address.0, page_size, (page-1)*page_size);
//             let row_receive_count = sqlx::query(sql_receive_count.as_str())
//                 .fetch_one(&mut conn)
//                 .await?;
//             total = row_receive_count.try_get("cnt")?;
//
//             // from
//             let rows_receive = sqlx::query(sql_receive.as_str())
//                 .fetch_all(&mut conn)
//                 .await?;
//             for row in rows_receive {
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let amount_val: Value = row.try_get("amount")?;
//                 let signer_val: Value = row.try_get("signer")?;
//
//                 let amount_str: String = serde_json::from_value(amount_val).unwrap();
//                 let amount: u64 = amount_str.parse::<u64>().unwrap();
//                 let signer: String = serde_json::from_value(signer_val).unwrap();
//                 let pk = public_key_from_base64(&signer).unwrap();
//
//                 receive_from.push(PrismItem {
//                     tx_hash,
//                     address: public_key_to_bech32(&pk),
//                     amount,
//                     timestamp,
//                 });
//             }
//         }
//     }
//
//     Ok(PrismRecordResponseNew::Ok(Json(PrismRecordResultNew {
//         code: 200,
//         message: "".to_string(),
//         data: Some(PrismRecordNew {
//             total,
//             page,
//             page_size,
//             items: receive_from,
//         }),
//     })))
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct ConvertAccountReceiver {
    #[serde(rename = "Ethereum")]
    pub ethereum: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct ConvertAccount {
    pub nonce: Value,
    pub receiver: ConvertAccountReceiver,
    pub signer: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct NonConfidentialTransferOutput {
    pub amount: u64,
    pub asset: Value,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Object)]
pub struct NonConfidentialTransfer {
    pub input_value: u64,
    pub outputs: Vec<NonConfidentialTransferOutput>,
}

pub async fn get_prism_records_send_to(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<PrismRecordResponseNew> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    let mut send_to: Vec<PrismItem> = vec![];
    let total;

    match "fra".as_bytes().eq(&address.0.as_bytes()[..3]) {
        true => {
            let pk_result = public_key_from_bech32(address.0.as_str());
            if pk_result.is_err() {
                return Ok(PrismRecordResponseNew::BadRequest(Json(
                    PrismRecordResultNew {
                        code: 400,
                        message: "invalid bech32 address".to_string(),
                        data: None,
                    },
                )));
            }
            let pk = pk_result.unwrap();
            let base64_addr = public_key_to_base64(&pk);
            let sql_to_count = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.signer ? (@==\"{base64_addr}\")'");
            let sql_to = format!("SELECT tx_hash, timestamp, jsonb_path_query(value,'$.body.operations[*].ConvertAccount') AS ca FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.signer ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", base64_addr, page_size, (page-1)*page_size);
            let row_to_count = sqlx::query(sql_to_count.as_str())
                .fetch_one(&mut conn)
                .await?;
            total = row_to_count.try_get("cnt")?;

            let rows_send_to = sqlx::query(sql_to.as_str()).fetch_all(&mut conn).await?;
            for row in rows_send_to {
                let timestamp: i64 = row.try_get("timestamp")?;
                let tx_hash: String = row.try_get("tx_hash")?;
                let ca_val: Value = row.try_get("ca")?;
                let ca_bin = serde_json::to_vec(&ca_val).unwrap();
                let convert_account: ConvertAccount = serde_json::from_value(ca_val).unwrap();
                let amount: u64 = convert_account.value.parse::<u64>()?;

                send_to.push(PrismItem {
                    data: base64::encode(&ca_bin),
                    tx_hash,
                    address: convert_account.receiver.ethereum,
                    amount,
                    timestamp,
                })
            }
        }
        _ => {
            let sql_to = "SELECT tx_hash, timestamp, jsonb_path_query(value, '$.function.XHub.NonConfidentialTransfer') AS nct, jsonb_path_query(value, '$.signature[0]') AS sig FROM transaction WHERE value @? '$.function.XHub.NonConfidentialTransfer.outputs[*].amount ? (@ > 0)' ORDER BY timestamp DESC";
            let mut send_to_tmp: Vec<PrismItem> = vec![];
            let rows_send_to = sqlx::query(sql_to).fetch_all(&mut conn).await?;
            for row in rows_send_to {
                let sig_val: Value = row.try_get("sig")?;
                let pk: [u8; 32] = serde_json::from_value(sig_val).unwrap();

                let signer = format!("{:?}", H160::from_slice(&pk[4..24]));
                if !signer.eq(&address.0) {
                    continue;
                }

                let address: String = format!("{:?}", H160::from_slice(&pk[4..24]));
                let timestamp: i64 = row.try_get("timestamp")?;
                let tx_hash: String = row.try_get("tx_hash")?;

                let nct_val = row.try_get("nct")?;
                let nct: NonConfidentialTransfer = serde_json::from_value(nct_val).unwrap();
                let nct_bin = serde_json::to_vec(&nct)?;

                send_to_tmp.push(PrismItem {
                    data: base64::encode(&nct_bin),
                    tx_hash,
                    address,
                    amount: nct.outputs[0].amount,
                    timestamp,
                })
            }

            total = send_to_tmp.len() as i64;
            let offset = page_size * (page - 1);

            if offset >= total {
                send_to = vec![];
            } else {
                let start = offset as usize;
                if offset + page_size <= total {
                    let end = (offset + page_size) as usize;
                    send_to = Vec::from(&send_to_tmp[start..end]);
                } else {
                    send_to = Vec::from(&send_to_tmp[start..]);
                }
            }
        }
    }

    Ok(PrismRecordResponseNew::Ok(Json(PrismRecordResultNew {
        code: 200,
        message: "".to_string(),
        data: Some(PrismRecordNew {
            total,
            page,
            page_size,
            items: send_to,
        }),
    })))
}

// pub async fn get_prism_records(api: &Api, address: Path<String>) -> Result<PrismRecordResponse> {
//     let mut conn = api.storage.lock().await.acquire().await?;
//     let addr_bytes = address.0.as_bytes();
//
//     let mut receive_from: Vec<PrismItem> = vec![];
//     let mut send_to: Vec<PrismItem> = vec![];
//
//     match "fra".as_bytes().eq(&addr_bytes[..3]) {
//         true => {
//             // native: fra...
//             let pk_result = public_key_from_bech32(address.0.as_str());
//             if pk_result.is_err() {
//                 return Ok(PrismRecordResponse::BadRequest(Json(PrismRecordResult {
//                     code: 400,
//                     message: "invalid fra address".to_string(),
//                     data: None,
//                 })));
//             }
//             let pk = pk_result.unwrap();
//             let base64_addr = public_key_to_base64(&pk);
//
//             let to_sql = format!("SELECT tx_hash, timestamp, jsonb_path_query(value,'$.body.operations[*].ConvertAccount.receiver.Ethereum') AS to, jsonb_path_query(value, '$.body.operations[*].ConvertAccount.value') AS amount FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.signer ? (@==\"{base64_addr}\")' ORDER BY timestamp DESC");
//             let from_sql = format!("SELECT tx_hash, timestamp, jsonb_path_query(value, '$.function.XHub.NonConfidentialTransfer.outputs[*].amount') AS amount, jsonb_path_query(value, '$.signature[0]') as pk FROM transaction WHERE value @? '$.function.XHub.NonConfidentialTransfer.outputs[*].target ? (@==\"{base64_addr}\")' ORDER BY timestamp DESC");
//
//             let from_acc_result = sqlx::query(from_sql.as_str()).fetch_all(&mut conn).await?;
//             for row in from_acc_result {
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let amount_val: Value = row.try_get("amount")?;
//                 let pk_val: Value = row.try_get("pk")?;
//
//                 let amount: u64 = serde_json::from_value(amount_val).unwrap();
//                 let pk: [u8; 32] = serde_json::from_value(pk_val).unwrap();
//                 let from: String = format!("{:?}", H160::from_slice(&pk[4..24]));
//
//                 receive_from.push(PrismItem {
//                     tx_hash,
//                     address: from,
//                     amount,
//                     timestamp,
//                 });
//             }
//
//             let to_acc_result = sqlx::query(to_sql.as_str()).fetch_all(&mut conn).await?;
//             for row in to_acc_result {
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let to_val: Value = row.try_get("to")?;
//                 let amount_val: Value = row.try_get("amount")?;
//
//                 let amount_str: String = serde_json::from_value(amount_val).unwrap();
//                 let amount: u64 = amount_str.parse::<u64>().unwrap();
//                 let to: String = serde_json::from_value(to_val).unwrap();
//
//                 send_to.push(PrismItem {
//                     tx_hash,
//                     address: to,
//                     amount,
//                     timestamp,
//                 })
//             }
//         }
//         _ => {
//             // evm: 0x...
//             let from_sql = format!("SELECT tx_hash, timestamp, jsonb_path_query(value, '$.body.operations[*].ConvertAccount.value') AS amount, jsonb_path_query(value, '$.body.operations[*].ConvertAccount.signer') AS signer FROM transaction WHERE value @? '$.body.operations[*].ConvertAccount.receiver.Ethereum ? (@==\"{}\")' ORDER BY timestamp DESC", address.0);
//             let to_sql = "SELECT tx_hash, timestamp, jsonb_path_query(value, '$.function.XHub.NonConfidentialTransfer.outputs[*].amount') AS amount, jsonb_path_query(value, '$.signature[0]') AS sig FROM transaction WHERE value @? '$.function.XHub.NonConfidentialTransfer.outputs[*].amount ? (@ > 0)' ORDER BY timestamp DESC";
//
//             let from_acc_result = sqlx::query(from_sql.as_str()).fetch_all(&mut conn).await?;
//             for row in from_acc_result {
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let amount_val: Value = row.try_get("amount")?;
//                 let signer_val: Value = row.try_get("signer")?;
//
//                 let amount_str: String = serde_json::from_value(amount_val).unwrap();
//                 let amount: u64 = amount_str.parse::<u64>().unwrap();
//                 let signer: String = serde_json::from_value(signer_val).unwrap();
//                 let pk = public_key_from_base64(&signer).unwrap();
//
//                 receive_from.push(PrismItem {
//                     tx_hash,
//                     address: public_key_to_bech32(&pk),
//                     amount,
//                     timestamp,
//                 });
//             }
//
//             let to_acc_result = sqlx::query(to_sql).fetch_all(&mut conn).await?;
//             for row in to_acc_result {
//                 let sig_val: Value = row.try_get("sig")?;
//                 let pk: [u8; 32] = serde_json::from_value(sig_val).unwrap();
//
//                 let signer = format!("{:?}", H160::from_slice(&pk[4..24]));
//                 if !signer.eq(&address.0) {
//                     continue;
//                 }
//
//                 let address: String = format!("{:?}", H160::from_slice(&pk[4..24]));
//                 let timestamp: i64 = row.try_get("timestamp")?;
//                 let tx_hash: String = row.try_get("tx_hash")?;
//                 let amount_val: Value = row.try_get("amount")?;
//                 let amount: u64 = serde_json::from_value(amount_val).unwrap();
//
//                 send_to.push(PrismItem {
//                     tx_hash,
//                     address,
//                     amount,
//                     timestamp,
//                 })
//             }
//         }
//     };
//
//     Ok(PrismRecordResponse::Ok(Json(PrismRecordResult {
//         code: 200,
//         message: "".to_string(),
//         data: Some(PrismRecord {
//             receive_from,
//             send_to,
//         }),
//     })))
// }

pub async fn get_evm_tx(api: &Api, tx_hash: Path<String>) -> Result<TxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_query = "SELECT * FROM transaction WHERE value @? '$.function.Ethereum'";

    let rows = sqlx::query(sql_query).fetch_all(&mut conn).await?;
    for row in rows {
        let value: Value = row.try_get("value")?;
        let evm_tx: EvmTx = serde_json::from_value(value.clone()).unwrap();
        let hash = H256::from_slice(Keccak256::digest(&rlp::encode(&evm_tx)).as_slice());

        let evm_tx_hash = format!("{hash:?}");
        if evm_tx_hash.eq(&tx_hash.0.to_lowercase()) {
            let tx_hash: String = row.try_get("tx_hash")?;
            let block_hash: String = row.try_get("block_hash")?;
            let ty: i32 = row.try_get("ty")?;
            let timestamp: i64 = row.try_get("timestamp")?;
            let height: i64 = row.try_get("height")?;
            let code: i64 = row.try_get("code")?;
            let log = "".to_string();
            let origin = row.try_get("origin")?;
            let result: Value = row.try_get("result")?;

            let tx = TransactionResponse {
                tx_hash,
                evm_tx_hash,
                block_hash,
                height,
                timestamp,
                code,
                ty,
                log,
                origin,
                result,
                value,
            };

            return Ok(TxResponse::Ok(Json(TxRes {
                code: 200,
                message: "".to_string(),
                data: Some(tx),
            })));
        }
    }

    Ok(TxResponse::NotFound(Json(TxRes {
        code: 404,
        message: "not found".to_string(),
        data: None,
    })))
}

pub async fn get_tx(api: &Api, tx_hash: Path<String>) -> Result<TxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!(
        "SELECT * FROM transaction WHERE tx_hash = '{}'",
        tx_hash.0.to_lowercase()
    );
    let row = sqlx::query(str.as_str()).fetch_one(&mut conn).await?;

    let tx_hash: String = row.try_get("tx_hash")?;
    let block_hash: String = row.try_get("block_hash")?;
    let ty: i32 = row.try_get("ty")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let height: i64 = row.try_get("height")?;
    let code: i64 = row.try_get("code")?;
    let log = "".to_string();
    let origin = row.try_get("origin")?;
    let result: Value = row.try_get("result")?;
    let value: Value = row.try_get("value")?;

    let mut tx = TransactionResponse {
        tx_hash,
        evm_tx_hash: "".to_string(),
        block_hash,
        height,
        timestamp,
        code,
        ty,
        log,
        origin,
        result,
        value,
    };

    let _ = wrap_evm_tx(&mut tx);

    Ok(TxResponse::Ok(Json(TxRes {
        code: 200,
        message: "".to_string(),
        data: Some(tx),
    })))
}

pub async fn get_txs_receive_from(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let pk_res = public_key_from_bech32(address.0.as_str());
    if pk_res.is_err() {
        return Ok(TxsResponse::Ok(Json(TxsRes {
            code: 400,
            message: "invalid address".to_string(),
            data: None,
        })));
    }
    let pk = public_key_to_base64(&pk_res.unwrap());

    let sql_total = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{pk}\")'");
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total = row.try_get("cnt")?;

    let sql_query = format!("SELECT * FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", pk, page_size, page_size*(page-1));
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<TransactionResponse> = vec![];
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let mut tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };
        let _ = wrap_evm_tx(&mut tx);
        txs.push(tx);
    }

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

pub async fn get_txs_send_to(
    api: &Api,
    address: Query<String>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    let pk_res = public_key_from_bech32(address.0.as_str());
    if pk_res.is_err() {
        return Ok(TxsResponse::Ok(Json(TxsRes {
            code: 400,
            message: "invalid address".to_string(),
            data: None,
        })));
    }
    let pk = public_key_to_base64(&pk_res.unwrap());

    let sql_total = format!("SELECT count(*) AS cnt FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{pk}\")'");
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total = row.try_get("cnt")?;

    let sql_query = format!("SELECT * FROM transaction WHERE value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{}\")' ORDER BY timestamp DESC LIMIT {} OFFSET {}", pk, page_size, page_size*(page-1));
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<TransactionResponse> = vec![];
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let mut tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };
        let _ = wrap_evm_tx(&mut tx);
        txs.push(tx);
    }

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_txs(
    api: &Api,
    block_hash: Query<Option<String>>,
    block_height: Query<Option<i64>>,
    address: Query<Option<String>>,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    ty: Query<Option<i32>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_hash) = block_hash.0 {
        params.push(format!("block_hash='{block_hash}' "));
    }
    if let Some(height) = block_height.0 {
        params.push(format!("height={height} "));
    }

    if let Some(addr) = address.0 {
        let pk = public_key_from_bech32(addr.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid from address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        let native_addr = public_key_to_base64(&pk);
        params.push(format!(
            "((value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{native_addr}\")') OR (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{native_addr}\")')) "
            ));
    }

    if let Some(from_address) = from.0 {
        let pk = public_key_from_bech32(from_address.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid from address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        params.push(format!(
            "(value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{}\")') ",
            public_key_to_base64(&pk)));
    }
    if let Some(to_address) = to.0 {
        let pk = public_key_from_bech32(to_address.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid to address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        params.push(format!(
            "(value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{}\")') ",
            public_key_to_base64(&pk)));
    }
    if let Some(ty) = ty.0 {
        params.push(format!("ty={ty} "));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!("timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!("timestamp<={end_time} "));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add("WHERE ").add(params.join("AND ").as_str());
        sql_total = sql_total.add("WHERE ").add(params.join("AND ").as_str());
    }

    sql_str = sql_str.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<TransactionResponse> = vec![];

    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let mut tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };
        let _ = wrap_evm_tx(&mut tx);
        txs.push(tx);
    }

    // total items
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row.try_get("total")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_txs_raw(
    api: &Api,
    block_hash: Query<Option<String>>,
    block_height: Query<Option<i64>>,
    address: Query<Option<String>>,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    ty: Query<Option<i32>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_hash) = block_hash.0 {
        params.push(format!("block_hash='{block_hash}' "));
    }
    if let Some(height) = block_height.0 {
        params.push(format!("height={height} "));
    }

    if let Some(addr) = address.0 {
        let pk = public_key_from_bech32(addr.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid from address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        let native_addr = public_key_to_base64(&pk);
        params.push(format!(
            "((value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{native_addr}\")') OR (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{native_addr}\")')) "
            ));
    }

    if let Some(from_address) = from.0 {
        let pk = public_key_from_bech32(from_address.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid from address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        params.push(format!(
            "(value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{}\")') ",
            public_key_to_base64(&pk)));
    }
    if let Some(to_address) = to.0 {
        let pk = public_key_from_bech32(to_address.as_str());
        if pk.is_err() {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 400,
                message: "invalid to address".to_string(),
                data: None,
            })));
        }
        let pk = pk.unwrap();
        params.push(format!(
            "(value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{}\")') ",
            public_key_to_base64(&pk)));
    }
    if let Some(ty) = ty.0 {
        params.push(format!("ty={ty} "));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!("timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!("timestamp<={end_time} "));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add("WHERE ").add(params.join("AND ").as_str());
        sql_total = sql_total.add("WHERE ").add(params.join("AND ").as_str());
    }

    sql_str = sql_str.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {} ",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );

    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let mut txs: Vec<TransactionResponse> = vec![];
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 500,
                message: "internal error".to_string(),
                data: Some(TxsData::default()),
            })));
        }
    };

    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };
        txs.push(tx);
    }

    // total items
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row.try_get("total")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_triple_masking_txs(
    api: &Api,
    block_hash: Query<Option<String>>,
    pub_key: Query<Option<String>>,
    bar: Query<Option<i32>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_hash) = block_hash.0 {
        params.push(format!(" block_hash='{block_hash}' "));
    }
    if let Some(bar) = bar.0 {
        if bar == ABAR_TO_BAR {
            if let Some(pk) = pub_key.0 {
                params.push(format!("(value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@==\"{pk}\")') "));
            } else {
                params.push("(value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@!=\"\")') ".to_string());
            }
        } else if bar == BAR_TO_ABAR {
            if let Some(pk) = pub_key.0 {
                params.push(format!("(value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@==\"{pk}\")') "));
            } else {
                params.push("(value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@!=\"\")') ".to_string());
            }
        }
    } else if let Some(pk) = pub_key.0 {
        params.push(format!("(value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@==\"{pk}\")') OR (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@==\"{pk}\")') "));
    }

    if let Some(start_time) = start_time.0 {
        params.push(format!("timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!("timestamp<={end_time} "));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add("WHERE ").add(params.join("AND ").as_str());
        sql_total = sql_total.add("WHERE ").add(params.join("AND ").as_str());
    } else {
        sql_str += &String::from(
            "WHERE (value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@!=\"\")') \
            OR (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@!=\"\")') ",
        );
    }

    sql_str = sql_str.add(
        format!(
            "ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );
    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<TransactionResponse> = vec![];

    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };

        txs.push(tx);
    }

    // total items
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row.try_get("total")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

pub async fn get_claim_txs(
    api: &Api,
    block_hash: Query<Option<String>>,
    pub_key: Query<Option<String>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_hash) = block_hash.0 {
        params.push(format!(" block_hash='{block_hash}' "));
    }
    if let Some(pk) = pub_key.0 {
        params.push(format!(
            "(value @? '$.body.operations[*].Claim.pubkey ? (@==\"{pk}\")')"
        ));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={end_time} "));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    if !params.is_empty() {
        sql_str = sql_str.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    } else {
        sql_str +=
            &String::from(" WHERE (value @? '$.body.operations[*].Claim.pubkey ? (@!=\"\")') ");
    }

    sql_str = sql_str.add(
        format!(
            " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );
    let rows = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await?;
    let mut txs: Vec<TransactionResponse> = vec![];

    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let height: i64 = row.try_get("height")?;
        let code: i64 = row.try_get("code")?;
        let log = "".to_string();
        let origin = row.try_get("origin")?;
        let result: Value = row.try_get("result")?;
        let value: Value = row.try_get("value")?;

        let tx = TransactionResponse {
            tx_hash,
            evm_tx_hash: "".to_string(),
            block_hash,
            height,
            timestamp,
            code,
            ty,
            log,
            origin,
            result,
            value,
        };

        txs.push(tx);
    }

    // total items
    let res = sqlx::query(sql_total.as_str()).fetch_all(&mut conn).await;
    let total: i64 = res.unwrap()[0].try_get("total")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(TxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

#[derive(ApiResponse)]
pub enum ClaimAmountResponse {
    #[oai(status = 200)]
    Ok(Json<ClaimAmountResult>),
    #[oai(status = 200)]
    Err(Json<ClaimAmountResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ClaimAmountResult {
    pub code: i32,
    pub message: String,
    pub data: ClaimAmount,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ClaimAmount {
    pub amount: u64,
}

pub async fn get_claims_amount(api: &Api, address: Path<String>) -> Result<ClaimAmountResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;

    let pubkey_res = public_key_from_bech32(&address.0);
    if pubkey_res.is_err() {
        return Ok(ClaimAmountResponse::Err(Json(ClaimAmountResult {
            code: 400,
            message: "invalid bech32 address".to_string(),
            data: ClaimAmount { amount: 0 },
        })));
    }

    let base64_address = public_key_to_base64(&pubkey_res.unwrap());
    let sql_query = format!("SELECT jsonb_path_query(value, '$.body.operations[*].Claim.body.amount') AS amount FROM transaction WHERE value @? '$.body.operations[*].Claim.pubkey ? (@==\"{base64_address}\")'");
    let rows = sqlx::query(sql_query.as_str()).fetch_all(&mut conn).await?;
    let mut total: u64 = 0;
    for r in rows {
        let amount: Value = r.try_get("amount")?;
        total += amount.to_string().parse::<u64>()?;
    }

    Ok(ClaimAmountResponse::Ok(Json(ClaimAmountResult {
        code: 200,
        message: "".to_string(),
        data: ClaimAmount { amount: total },
    })))
}

pub async fn get_prism_tx(
    api: &Api,
    address: Path<String>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<PmtxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql = String::from("SELECT tx_hash,block_hash,ty,timestamp,CASE WHEN fnuc_code = '248,251,204,194' THEN '_consumeMint' WHEN fnuc_code = '242,38,15,112' THEN '_withdrawFRA' WHEN fnuc_code = '116,64,166,22' THEN '_withdrawFRC20' WHEN fnuc_code = '250,190,177,88' THEN 'adminSetAsset' WHEN fnuc_code = '185,50,44,225' THEN 'adminSetLedger' WHEN fnuc_code = '5,5,220,224' THEN 'asset_contract' WHEN fnuc_code = '82,79,49,152' THEN 'consumeMint' WHEN fnuc_code = '222,147,129,28' THEN 'depositFRA' WHEN fnuc_code = '230,242,112,109' THEN 'depositFRC20' WHEN fnuc_code = '4,78,219,111' THEN 'ledger_contract' WHEN fnuc_code = '253,253,93,76' THEN 'ops' WHEN fnuc_code = '141,165,203,91' THEN 'owner' WHEN fnuc_code = '216,78,128,56' THEN 'proxy_contract' WHEN fnuc_code = '113,80,24,166' THEN 'renounceOwnership' WHEN fnuc_code = '242,253,227,139' THEN 'transferOwnership' WHEN fnuc_code = '24,188,157,230' THEN 'withdrawFRA' WHEN fnuc_code = '82,119,153,176' THEN 'withdrawFRC20' ELSE 'unknown' END AS fnuc_name,value,code,log FROM(SELECT tx_hash,block_hash,ty,timestamp,concat(value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->0, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->1, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->2, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->3) AS fnuc_code,value,code,log FROM transaction WHERE ty = 1");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction WHERE ty = 1");
    let mut params: Vec<String> = vec![];
    params.push(format!(
        " value -> 'function' -> 'Ethereum' -> 'Transact' -> 'action' -> 'Call' = '\"{}\"'",
        address.as_str().to_lowercase()
    ));
    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={start_time} "));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={end_time} "));
    }
    if !params.is_empty() {
        sql = sql.add(" AND ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" AND ").add(params.join(" AND ").as_str());
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    sql = sql.add(
        format!(
            " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            page_size,
            (page - 1) * page_size
        )
        .as_str(),
    );
    sql += ") AS t";

    let rows = sqlx::query(sql.as_str()).fetch_all(&mut conn).await?;

    let mut txs: Vec<PrismTransaction> = vec![];
    for row in rows {
        let tx_hash: String = row.try_get("tx_hash")?;
        let block_hash: String = row.try_get("block_hash")?;
        let ty: i32 = row.try_get("ty")?;
        let fnuc_name: String = row.try_get("fnuc_name")?;
        let value: Value = row.try_get("value")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let code: i64 = row.try_get("code")?;
        //let log: String = row.try_get("log")?;
        let log = "".to_string();
        let tx = PrismTransaction {
            tx_hash,
            block_hash,
            ty,
            fnuc_name,
            value,
            code,
            timestamp,
            log,
            events: vec![],
        };
        txs.push(tx);
    }

    // total items
    let row = sqlx::query(sql_total.as_str()).fetch_one(&mut conn).await?;
    let total: i64 = row.try_get("total")?;

    Ok(PmtxsResponse::Ok(Json(PmtxsRes {
        code: 200,
        message: "".to_string(),
        data: Some(PmtxsData {
            page,
            page_size,
            total,
            txs,
        }),
    })))
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct HideAmount {
    #[serde(rename = "Confidential")]
    pub confidential: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ShowAmount {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ShowAssetType {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct HideAssetType {
    #[serde(rename = "Confidential")]
    pub confidential: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetTypeShowAmountHide {
    pub amount: HideAmount,
    pub asset_type: ShowAssetType,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetTypeHideAmountShow {
    pub amount: ShowAmount,
    pub asset_type: HideAssetType,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetTypeHideAmountHide {
    pub amount: HideAmount,
    pub asset_type: HideAssetType,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TxData {
    pub body: TxBody,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TxBody {
    pub operations: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Opt {
    #[serde(rename = "TransferAsset")]
    pub transfer_asset: TransferAsset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TransferAsset {
    pub body: TransferAssetBody,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TransferAssetBody {
    pub transfer: Transfer,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Transfer {
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Output {
    pub amount: Value,
    pub asset_type: Value,
    pub public_key: Value,
}

fn wrap_evm_tx(tx: &mut TransactionResponse) -> Result<()> {
    let tx_str: String = serde_json::to_string(&tx.value).unwrap();

    if tx.ty == EVM_TRANSFER {
        if tx_str.contains("XHub") {
            tx.ty = PRISM_EVM_TO_NATIVE;
            return Ok(());
        }
        // calc evm tx hash
        let evm_tx: EvmTx = serde_json::from_value(tx.value.clone()).unwrap();
        let hash = H256::from_slice(Keccak256::digest(&rlp::encode(&evm_tx)).as_slice());
        tx.evm_tx_hash = format!("{hash:?}");
        let evm_tx_response = evm_tx.to_evm_tx_response().unwrap();
        tx.value = serde_json::to_value(evm_tx_response).unwrap();
    } else if tx_str.contains("ConvertAccount") {
        tx.ty = PRISM_NATIVE_TO_EVM;
    } else if tx_str.contains("AbarToBar") {
        tx.ty = ABAR_TO_BAR;
    } else if tx_str.contains("BarToAbar") {
        tx.ty = BAR_TO_ABAR;
    } else if tx_str.contains("TransferAnonAsset") {
        tx.ty = ABAR_TO_ABAR;
    } else if tx_str.contains("Claim") {
        tx.ty = CLAIM;
    } else if tx_str.contains("UnDelegation") {
        tx.ty = UNSTAKING;
        let udv: UnDelegationValue = serde_json::from_value(tx.value.clone()).unwrap();
        tx.value = serde_json::to_value(udv.wrap()).unwrap();
    } else if tx_str.contains("Delegation") {
        tx.ty = STAKING;
    } else if tx_str.contains("DefineAsset") || tx_str.contains("IssueAsset") {
        tx.ty = DEFINE_OR_ISSUE_ASSET;
    } else {
        let tx_data: TxData = serde_json::from_value(tx.value.clone()).unwrap();
        for v in tx_data.body.operations {
            let opt_res: Result<Opt, _> = serde_json::from_value(v);
            if let Ok(opt) = opt_res {
                for output in opt.transfer_asset.body.transfer.outputs {
                    if !output
                        .public_key
                        .eq("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
                    {
                        let hide_amount_res: Result<HideAmount, _> =
                            serde_json::from_value(output.amount);
                        let hide_asset_type_res: Result<HideAssetType, _> =
                            serde_json::from_value(output.asset_type);
                        if hide_amount_res.is_ok() {
                            if hide_asset_type_res.is_ok() {
                                tx.ty = HIDE_ASSET_TYPE_AND_AMOUNT;
                            } else {
                                tx.ty = HIDE_ASSET_AMOUNT;
                            }
                        } else if hide_asset_type_res.is_ok() {
                            tx.ty = HIDE_ASSET_TYPE;
                        }

                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_tx_hash() -> Result<()> {
        // eyJzaWduYXR1cmUiOm51bGwsImZ1bmN0aW9uIjp7IkV0aGVyZXVtIjp7IlRyYW5zYWN0Ijp7Im5vbmNlIjoiMHg5IiwiZ2FzX3ByaWNlIjoiMHhlOGQ0YTUxMDAwIiwiZ2FzX2xpbWl0IjoiMHg1MjA4IiwiYWN0aW9uIjp7IkNhbGwiOiIweGE1MjI1Y2JlZTUwNTIxMDBlYzJkMmQ5NGFhNmQyNTg1NTgwNzM3NTcifSwidmFsdWUiOiIweDk4YTdkOWI4MzE0YzAwMDAiLCJpbnB1dCI6W10sInNpZ25hdHVyZSI6eyJ2IjoxMDgyLCJyIjoiMHg4MDBjZjQ5ZTAzMmJhYzY4MjY3MzdhZGJhZDEzN2Y0MTk5OTRjNjgxZWE1ZDUyYjliMGJhZDJmNDAyYjMwMTI0IiwicyI6IjB4Mjk1Mjc3ZWY2NTYzNDAwY2VkNjFiODhkM2ZiNGM3YjMyY2NkNTcwYThiOWJiOGNiYmUyNTkyMTRhYjdkZTI1YSJ9fX19fQ
        let tx_str= "{\"signature\":null,\"function\":{\"Ethereum\":{\"Transact\":{\"nonce\":\"0x9\",\"gas_price\":\"0xe8d4a51000\",\"gas_limit\":\"0x5208\",\"action\":{\"Call\":\"0xa5225cbee5052100ec2d2d94aa6d258558073757\"},\"value\":\"0x98a7d9b8314c0000\",\"input\":[],\"signature\":{\"v\":1082,\"r\":\"0x800cf49e032bac6826737adbad137f419994c681ea5d52b9b0bad2f402b30124\",\"s\":\"0x295277ef6563400ced61b88d3fb4c7b32ccd570a8b9bb8cbbe259214ab7de25a\"}}}}}";
        let evm_tx: EvmTx = serde_json::from_str(tx_str).unwrap();
        let hash = H256::from_slice(Keccak256::digest(&rlp::encode(&evm_tx)).as_slice());
        let tx_hash = format!("{hash:?}");
        assert_eq!(
            tx_hash,
            "0x0eeb0ff455b1b57b821634cf853e7247e584a675610f13097cc49c2022505df3"
        );

        Ok(())
    }
}
