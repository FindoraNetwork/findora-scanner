use crate::rpc::validator::PubKey;
use chrono::NaiveDateTime;
use poem_openapi::Object;
use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use zei::xfr::sig::XfrPublicKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_hash: String,
    pub height: i64,
    pub size: i64,
    pub tx_count: i64,
    pub timestamp: NaiveDateTime,
    pub app_hash: String,
    pub proposer: String,
    pub txs: Vec<Transaction>,
    pub evm_txs: Vec<Transaction>,
    pub validators: Vec<Validator>,
    pub block_data: Value,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Transaction {
    pub tx_hash: String,
    pub block_hash: String,
    pub height: i64,
    pub timestamp: i64,
    pub ty: i32,
    pub code: i64,
    pub log: String,
    pub result: Value, // result.tx_result
    pub value: Value,  // result.tx
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct PrismTransaction {
    pub tx_hash: String,
    pub block_hash: String,
    pub ty: i32,
    pub fnuc_name: String,
    pub value: Value,
    pub code: i64,
    pub timestamp: i64,
    pub log: String,
    pub events: Vec<Value>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Validator {
    pub address: String,
    pub power: u64,
    pub pub_key: PubKey,
    pub priority: i64,
    pub signature: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
}

pub struct LastHeight {
    pub tip: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegationInfo {
    pub global_delegation_records_map: HashMap<XfrPublicKey, DelegationLite>,
    pub validator_addr_map: HashMap<String, XfrPublicKey>,
    pub return_rate: Rate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegationLite {
    #[serde(rename = "entries")]
    pub delegations: HashMap<XfrPublicKey, u64>,
    pub id: XfrPublicKey, // delegation rewards will be paid to this pk by default
    pub start_height: u64,
    pub end_height: u64,
    pub state: DelegationState,
    pub rwd_amount: u64,
    pub proposer_rwd_cnt: u64,   // how many times you get proposer rewards
    pub delegation_rwd_cnt: u64, // how many times you get delegation rewards
    pub receiver_pk: Option<XfrPublicKey>,
    pub tmp_delegators: HashMap<XfrPublicKey, u64>, // Temporary partial undelegations of current id
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DelegationState {
    Bond, // during delegation, include extra 21 days.
    Free, // it's time to pay principals and rewards.
    Paid, //principals and rewards have been paid successfully.
}

#[derive(Debug, Serialize)]
pub struct Rate {
    pub value: f64,
}

impl<'de> Deserialize<'de> for Rate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RateVisitor;

        enum Field {
            Value,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`Value`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "value" => Ok(Field::Value),
                            _ => Err(de::Error::unknown_field(value, &["value"])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        impl<'de> Visitor<'de> for RateVisitor {
            type Value = Rate;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("str like [<numerator>, <denominator>] or {value: f64}.")
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let n = seq
                    .next_element()?
                    .map(|x: u128| x as f64)
                    .ok_or_else(|| de::Error::custom("Missing numerator."))?;

                let d = seq
                    .next_element()?
                    .map(|x: u128| x as f64)
                    .ok_or_else(|| de::Error::custom("Missing denominator."))?;

                if d == 0.0 {
                    return Err(de::Error::custom("Divide by zero."));
                }

                if seq.next_element::<u128>()?.is_some() {
                    return Err(de::Error::custom("A rate must have only two element."));
                }

                Ok(Rate { value: n / d })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }
                let value: f64 = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(Rate { value })
            }
        }

        deserializer.deserialize_struct("Rate", &["value"], RateVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::Rate;
    #[test]
    fn test_rate_deser() {
        let data = format!("[{},{}]", u128::MAX, u128::MAX);
        let rate: Rate = serde_json::from_str(&data).unwrap();
        assert!((rate.value - 1.0).abs() < f64::EPSILON);

        let data = format!("[{},{}, 1]", u128::MAX, u128::MAX);
        let rate: Result<Rate, _> = serde_json::from_str(&data);
        assert!(rate.is_err());

        let data = format!("[{},{}]", u128::MAX, 0);
        let rate: Result<Rate, _> = serde_json::from_str(&data);
        assert!(rate.is_err());

        let data = "{\"value\": 1.0}".to_string();
        let _rate: Rate = serde_json::from_str(&data).unwrap();
    }
}
