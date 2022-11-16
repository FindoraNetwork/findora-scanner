use crate::rpc::validator::PubKey;
use crate::utils::crypto::recover_signer;
use anyhow::Result;
use chrono::NaiveDateTime;
use ethereum::{LegacyTransaction, TransactionAction, TransactionSignature};
use ethereum_types::U256;
use poem_openapi::Object;
use rlp::{Encodable, RlpStream};
use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use zei::xfr::sig::XfrPublicKey;

pub const NATIVE_TRANSFER: i32 = 0;
pub const EVM_TRANSFER: i32 = 1;
pub const HIDE_ASSET_TYPE: i32 = 2;
pub const HIDE_ASSET_AMOUNT: i32 = 3;
pub const HIDE_ASSET_TYPE_AND_AMOUNT: i32 = 4;
pub const ABAR_TO_BAR: i32 = 5;
pub const ABAR_TO_ABAR: i32 = 6;
pub const BAR_TO_ABAR: i32 = 7;
pub const PRISM_NATIVE_TO_EVM: i32 = 8;
pub const PRISM_EVM_TO_NATIVE: i32 = 9;
pub const STAKING: i32 = 10;
pub const UNSTAKING: i32 = 11;
pub const CLAIM: i32 = 12;
pub const DEFINE_OR_ISSUE_ASSET: i32 = 13;

#[derive(Serialize, Deserialize)]
pub struct EvmTx {
    pub function: Ethereum,
}

impl Encodable for EvmTx {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.function.ethereum.transact.rlp_append(s)
    }
}

impl EvmTx {
    pub fn to_evm_tx_response(&self) -> Result<EvmTxResponse> {
        let signer = recover_signer(&self.function.ethereum.transact).unwrap();

        let res = EvmTxResponse {
            function: EthereumResponse {
                ethereum: TransactResponse {
                    transact: LegacyTransactionResponse {
                        from: format!("{:?}", signer),
                        nonce: self.function.ethereum.transact.nonce,
                        gas_price: self.function.ethereum.transact.gas_price,
                        gas_limit: self.function.ethereum.transact.gas_limit,
                        action: self.function.ethereum.transact.action,
                        value: self.function.ethereum.transact.value,
                        input: self.function.ethereum.transact.input.clone(),
                        signature: self.function.ethereum.transact.signature.clone(),
                    },
                },
            },
        };

        Ok(res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Ethereum {
    #[serde(rename = "Ethereum")]
    pub ethereum: Transact,
}

#[derive(Serialize, Deserialize)]
pub struct Transact {
    #[serde(rename = "Transact")]
    pub transact: LegacyTransaction,
}

#[derive(Serialize, Deserialize)]
pub struct EvmTxResponse {
    pub function: EthereumResponse,
}

#[derive(Serialize, Deserialize)]
pub struct EthereumResponse {
    #[serde(rename = "Ethereum")]
    pub ethereum: TransactResponse,
}

#[derive(Serialize, Deserialize)]
pub struct TransactResponse {
    #[serde(rename = "Transact")]
    pub transact: LegacyTransactionResponse,
}

#[derive(Serialize, Deserialize)]
pub struct LegacyTransactionResponse {
    pub from: String,
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
    pub signature: TransactionSignature,
}

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
pub struct TransactionResponse {
    pub tx_hash: String,
    pub evm_tx_hash: String,
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
pub struct TdValidator {
    pub addr: String,
    pub memo: Memo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClaimOpt {
    pub body: ClaimBody,
    pub pubkey: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClaimBody {
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DelegationOpt {
    pub body: DelegationOptBody,
    pub pubkey: String,
    pub signature: String,
    pub v_signature: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
pub struct UnDelegationValue {
    pub body: UndelegationBody,
}

impl UnDelegationValue {
    pub fn wrap(&self) -> UnDelegationValueWrap {
        let vaddr = hex::encode(
            self.body
                .operations
                .0
                .as_ref()
                .unwrap()
                .undelegation
                .body
                .pu
                .as_ref()
                .unwrap()
                .target_validator,
        )
        .to_uppercase();

        let wpu = PuWrap {
            am: self
                .body
                .operations
                .0
                .as_ref()
                .unwrap()
                .undelegation
                .body
                .pu
                .as_ref()
                .unwrap()
                .am,
            new_delegator_id: self
                .body
                .operations
                .0
                .as_ref()
                .unwrap()
                .undelegation
                .body
                .pu
                .as_ref()
                .unwrap()
                .new_delegator_id
                .clone(),
            target_validator: vaddr,
        };
        let ud = UnDelegationOptWrap {
            body: UnDelegationOptBodyWrap {
                nonce: self
                    .body
                    .operations
                    .0
                    .as_ref()
                    .unwrap()
                    .undelegation
                    .body
                    .nonce
                    .clone(),
                pu: Some(wpu),
            },
            pubkey: self
                .body
                .operations
                .0
                .as_ref()
                .unwrap()
                .undelegation
                .pubkey
                .clone(),
            signature: self
                .body
                .operations
                .0
                .as_ref()
                .unwrap()
                .undelegation
                .signature
                .clone(),
        };

        UnDelegationValueWrap {
            body: UndelegationBodyWrap {
                no_replay_token: self.body.no_replay_token.clone(),
                operations: (
                    UnDelegationWrap { undelegation: ud },
                    self.body.operations.1.clone(),
                ),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnDelegationValueWrap {
    pub body: UndelegationBodyWrap,
}

#[derive(Serialize, Deserialize)]
pub struct UndelegationBody {
    pub no_replay_token: Value,
    pub operations: (Option<UnDelegation>, Value),
}

#[derive(Serialize, Deserialize)]
pub struct UndelegationBodyWrap {
    pub no_replay_token: Value,
    pub operations: (UnDelegationWrap, Value),
}

#[derive(Serialize, Deserialize)]
pub struct UnDelegation {
    #[serde(rename = "UnDelegation")]
    pub undelegation: UnDelegationOpt,
}

#[derive(Serialize, Deserialize)]
pub struct UnDelegationWrap {
    #[serde(rename = "UnDelegation")]
    pub undelegation: UnDelegationOptWrap,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnDelegationOpt {
    pub body: UnDelegationOptBody,
    pub pubkey: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnDelegationOptWrap {
    pub body: UnDelegationOptBodyWrap,
    pub pubkey: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnDelegationOptBody {
    pub nonce: Value,
    pub pu: Option<Pu>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnDelegationOptBodyWrap {
    pub nonce: Value,
    pub pu: Option<PuWrap>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Pu {
    pub am: i64,
    pub new_delegator_id: String,
    pub target_validator: [u8; 20],
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PuWrap {
    pub am: i64,
    pub new_delegator_id: String,
    pub target_validator: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DelegationOptBody {
    pub validator: String,
    pub new_validator: Option<NewValidator>,
    pub amount: i64,
    //pub nonce: Value,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewValidator {
    pub id: String,
    pub td_pubkey: Vec<i64>,
    pub td_addr: Vec<i64>,
    pub td_power: i64,
    pub commission_rate: Vec<i64>,
    pub memo: Memo,
    pub kind: String,
    pub signed_last_block: bool,
    pub signed_cnt: i64,
    pub delegators: Value,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct Memo {
    pub name: String,
    pub desc: String,
    pub website: String,
    pub logo: String,
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
