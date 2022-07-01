use crate::Api;
use anyhow::Result;
use module::schema::{DelegationOpt, Memo};
use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;

#[derive(ApiResponse)]
pub enum ValidatorListResponse {
    #[oai(status = 200)]
    Ok(Json<ValidatorListResult>),
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorListResult {
    pub code: i32,
    pub message: String,
    pub data: Option<ValidatorList>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct ValidatorList {
    pub validator_cnt: i64,
    pub cur_height: i64,
    pub validators: Vec<Validator>,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Validator {
    pub addr: String,
    pub power: i64,
    pub commission_rate: Vec<i64>,
    pub accept_delegation: bool,
    pub rank: i64,
    pub extra: Memo,
}

pub async fn validator_list(api: &Api) -> Result<ValidatorListResponse> {
    let mut conn = api.storage.lock().await.acquire().await?;
    let sql_latest_height = "SELECT height FROM last_height".to_string();
    let row = sqlx::query(sql_latest_height.as_str())
        .fetch_one(&mut conn)
        .await?;
    let height: i64 = row.try_get("height")?;
    let sql = format!("SELECT jsonb_path_query(value->'body', '$.operations[*].Delegation') as d FROM transaction WHERE height={}", height);
    let res = sqlx::query(sql.as_str()).fetch_all(&mut conn).await;
    let rows = match res {
        Ok(rows) => rows,
        _ => {
            return Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
                code: 500,
                message: "".to_string(),
                data: None,
            })));
        }
    };
    let mut validator_list = ValidatorList {
        validator_cnt: rows.len() as i64,
        cur_height: height,
        validators: vec![],
    };

    for r in rows {
        let d: Value = r.try_get("d").unwrap();
        let delegation: DelegationOpt = serde_json::from_value(d).unwrap();
        if let Some(nv) = delegation.body.new_validator {
            validator_list.validators.push(Validator {
                addr: delegation.body.validator,
                power: nv.td_power,
                commission_rate: nv.commission_rate,
                accept_delegation: false,
                rank: 0,
                extra: Memo {
                    name: nv.memo.name,
                    desc: nv.memo.desc,
                    website: nv.memo.website,
                    logo: nv.memo.logo,
                },
            });
        } else {
            validator_list.validators.push(Validator {
                addr: delegation.body.validator,
                power: 0,
                commission_rate: vec![],
                accept_delegation: false,
                rank: 0,
                extra: Default::default(),
            })
        }
    }

    Ok(ValidatorListResponse::Ok(Json(ValidatorListResult {
        code: 200,
        message: "".to_string(),
        data: Some(validator_list),
    })))
}
