use crate::service::util::{public_key_from_bech32, public_key_to_base64};
use crate::Api;
use anyhow::Result;
use module::schema::{PrismTransaction, Transaction};
use poem_openapi::param::Query;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::ops::Add;

const ABAR_TO_BAR: i64 = 1;
const BAR_TO_ABAR: i64 = 2;

#[derive(ApiResponse)]
pub enum TxResponse {
    #[oai(status = 200)]
    Ok(Json<TxRes>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct TxRes {
    pub code: i32,
    pub message: String,
    pub data: Option<Transaction>,
}

#[derive(ApiResponse)]
pub enum TxsResponse {
    #[oai(status = 200)]
    Ok(Json<TxsRes>),
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
    txs: Vec<Transaction>,
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
pub async fn get_tx(api: &Api, tx_id: Path<String>) -> Result<TxResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let str = format!("SELECT * FROM transaction WHERE txid = '{}'", tx_id.0);
    let res = sqlx::query(str.as_str()).fetch_one(&mut conn).await;
    let row = match res {
        Ok(row) => row,
        _ => {
            return Ok(TxResponse::Ok(Json(TxRes {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
    };
    let tx_id: String = row.try_get("txid")?;
    let block_id: String = row.try_get("block_id")?;
    let ty: i32 = row.try_get("ty")?;
    let value: Value = row.try_get("value")?;
    let timestamp: i64 = row.try_get("timestamp")?;
    let code: i64 = row.try_get("code")?;
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

    Ok(TxResponse::Ok(Json(TxRes {
        code: 200,
        message: "ok".to_string(),
        data: Some(tx),
    })))
}

#[allow(clippy::too_many_arguments)]
pub async fn get_txs(
    api: &Api,
    block_id: Query<Option<String>>,
    from: Query<Option<String>>,
    to: Query<Option<String>>,
    ty: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_id) = block_id.0 {
        params.push(format!(" block_id='{}' ", block_id));
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
            " (value @? '$.body.operations[*].TransferAsset.body.transfer.inputs[*].public_key ? (@==\"{}\")') ",
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
            " (value @? '$.body.operations[*].TransferAsset.body.transfer.outputs[*].public_key ? (@==\"{}\")') ",
            public_key_to_base64(&pk)));
    }
    if let Some(ty) = ty.0 {
        params.push(format!(" ty={} ", ty));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={} ", start_time));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={} ", end_time));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    }
    sql_str += &format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );

    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let mut txs: Vec<Transaction> = vec![];
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
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("value")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let code: i64 = row.try_get("code")?;
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

#[allow(clippy::too_many_arguments)]
pub async fn get_triple_masking_txs(
    api: &Api,
    block_id: Query<Option<String>>,
    pub_key: Query<Option<String>>,
    bar: Query<Option<i64>>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<TxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql_str = String::from("SELECT * FROM transaction ");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction ");
    let mut params: Vec<String> = vec![];
    if let Some(block_id) = block_id.0 {
        params.push(format!(" block_id='{}' ", block_id));
    }
    if let Some(bar) = bar.0 {
        if bar == ABAR_TO_BAR {
            if let Some(pk) = pub_key.0 {
                params.push(format!(" (value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@==\"{}\")') ", pk));
            } else {
                params.push(" (value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@!=\"\")') ".to_string());
            }
        } else if bar == BAR_TO_ABAR {
            if let Some(pk) = pub_key.0 {
                params.push(format!(" (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@==\"{}\")') ", pk));
            } else {
                params.push(" (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@!=\"\")') ".to_string());
            }
        }
    } else if let Some(pk) = pub_key.0 {
        params.push(format!(" (value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@==\"{}\")') OR (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@==\"{}\")')", pk, pk));
    }

    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={} ", start_time));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={} ", end_time));
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);

    if !params.is_empty() {
        sql_str = sql_str.add(" WHERE ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" WHERE ").add(params.join(" AND ").as_str());
    } else {
        sql_str += &String::from(
            " WHERE (value @? '$.body.operations[*].AbarToBar.note.body.output.public_key ? (@!=\"\")') \
            OR (value @? '$.body.operations[*].BarToAbar.note.body.output.commitment ? (@!=\"\")')",
        );
    }
    sql_str += &format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );

    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let mut txs: Vec<Transaction> = vec![];
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
    };

    for row in rows {
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("value")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let code: i64 = row.try_get("code")?;
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

        txs.push(tx);
    }

    // total items
    let res = sqlx::query(sql_total.as_str()).fetch_all(&mut conn).await;
    let total: i64 = res.unwrap()[0].try_get("total")?;

    Ok(TxsResponse::Ok(Json(TxsRes {
        code: 200,
        message: "ok".to_string(),
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
    block_id: Query<Option<String>>,
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
    if let Some(block_id) = block_id.0 {
        params.push(format!(" block_id='{}' ", block_id));
    }
    if let Some(pk) = pub_key.0 {
        params.push(format!(
            "(value @? '$.body.operations[*].Claim.pubkey ? (@==\"{}\")')",
            pk
        ));
    }
    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={} ", start_time));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={} ", end_time));
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
    sql_str += &format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );

    let res = sqlx::query(sql_str.as_str()).fetch_all(&mut conn).await;
    let mut txs: Vec<Transaction> = vec![];
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(TxsResponse::Ok(Json(TxsRes {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
    };

    for row in rows {
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let value: Value = row.try_get("value")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let code: i64 = row.try_get("code")?;
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

pub async fn get_prism_tx(
    api: &Api,
    address: Path<String>,
    start_time: Query<Option<i64>>,
    end_time: Query<Option<i64>>,
    page: Query<Option<i64>>,
    page_size: Query<Option<i64>>,
) -> Result<PmtxsResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let mut sql = String::from("SELECT txid,block_id,ty,timestamp,CASE WHEN fnuc_code = '248,251,204,194' THEN '_consumeMint' WHEN fnuc_code = '242,38,15,112' THEN '_withdrawFRA' WHEN fnuc_code = '116,64,166,22' THEN '_withdrawFRC20' WHEN fnuc_code = '250,190,177,88' THEN 'adminSetAsset' WHEN fnuc_code = '185,50,44,225' THEN 'adminSetLedger' WHEN fnuc_code = '5,5,220,224' THEN 'asset_contract' WHEN fnuc_code = '82,79,49,152' THEN 'consumeMint' WHEN fnuc_code = '222,147,129,28' THEN 'depositFRA' WHEN fnuc_code = '230,242,112,109' THEN 'depositFRC20' WHEN fnuc_code = '4,78,219,111' THEN 'ledger_contract' WHEN fnuc_code = '253,253,93,76' THEN 'ops' WHEN fnuc_code = '141,165,203,91' THEN 'owner' WHEN fnuc_code = '216,78,128,56' THEN 'proxy_contract' WHEN fnuc_code = '113,80,24,166' THEN 'renounceOwnership' WHEN fnuc_code = '242,253,227,139' THEN 'transferOwnership' WHEN fnuc_code = '24,188,157,230' THEN 'withdrawFRA' WHEN fnuc_code = '82,119,153,176' THEN 'withdrawFRC20' ELSE 'unknown' END AS fnuc_name,value,code,log FROM(SELECT txid,block_id,ty,timestamp,concat(value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->0, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->1, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->2, NULL, ',', value -> 'function' -> 'Ethereum' -> 'Transact' -> 'input'->3) AS fnuc_code,value,code,log FROM transaction WHERE ty = 1");
    let mut sql_total = String::from("SELECT count(*) as total FROM transaction WHERE ty = 1");
    let mut params: Vec<String> = vec![];
    params.push(format!(
        " value -> 'function' -> 'Ethereum' -> 'Transact' -> 'action' -> 'Call' = '\"{}\"'",
        address.as_str().to_lowercase()
    ));
    if let Some(start_time) = start_time.0 {
        params.push(format!(" timestamp>={} ", start_time));
    }
    if let Some(end_time) = end_time.0 {
        params.push(format!(" timestamp<={} ", end_time));
    }
    if !params.is_empty() {
        sql = sql.add(" AND ").add(params.join(" AND ").as_str());
        sql_total = sql_total.add(" AND ").add(params.join(" AND ").as_str());
    }
    let page = page.0.unwrap_or(1);
    let page_size = page_size.0.unwrap_or(10);
    sql += &format!(
        " ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        page_size,
        (page - 1) * page_size
    );
    sql += ") AS t";

    let res = sqlx::query(sql.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        Err(_) => {
            return Ok(PmtxsResponse::Ok(Json(PmtxsRes {
                code: 500,
                message: "internal error".to_string(),
                data: None,
            })));
        }
    };

    let mut txs: Vec<PrismTransaction> = vec![];
    for row in rows {
        let tx_id: String = row.try_get("txid")?;
        let block_id: String = row.try_get("block_id")?;
        let ty: i32 = row.try_get("ty")?;
        let fnuc_name: String = row.try_get("fnuc_name")?;
        let value: Value = row.try_get("value")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let code: i64 = row.try_get("code")?;
        let log: String = row.try_get("log")?;

        let tx = PrismTransaction {
            txid: tx_id,
            block_id,
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
    let res = sqlx::query(sql_total.as_str()).fetch_all(&mut conn).await;
    let total: i64 = res.unwrap()[0].try_get("total")?;
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
