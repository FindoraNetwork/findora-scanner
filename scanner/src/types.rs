use ethereum::LegacyTransaction;
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
pub enum FindoraTxType {
    Native,
    Evm,
    HideAssetType,
    HideAssetAmount,
    HideAssetTypeAndAmount,
    AbarToBar,
    AbarToAbar,
    BarToAbar,
    NativeToEVM,
    EVMToNative,
    Staking,
    UnStaking,
    Claim,
    DefineAsset,
    IssueAsset,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// evm tx
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct FindoraEVMTx {
    pub function: Ethereum,
}

impl Encodable for FindoraEVMTx {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.function.ethereum.transact.rlp_append(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ethereum {
    #[serde(rename = "Ethereum")]
    pub ethereum: Transact,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transact {
    #[serde(rename = "Transact")]
    pub transact: LegacyTransaction,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ConvertAccount
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConvertAccountReceiver {
    #[serde(rename = "Ethereum")]
    pub ethereum: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConvertAccount {
    pub nonce: Value,
    pub asset_type: Option<Vec<u8>>,
    pub receiver: ConvertAccountReceiver,
    pub signer: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConvertAccountOperation {
    #[serde(rename = "ConvertAccount")]
    pub convert_account: ConvertAccount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TransferAsset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TxValue {
    pub body: TxBody,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TxBody {
    pub operations: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAssetOpt {
    #[serde(rename = "TransferAsset")]
    pub transfer_asset: TransferAsset,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAsset {
    pub body: TransferBody,
    pub body_signatures: Vec<BodySignature>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BodySignature {
    pub address: SignatureKey,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignatureKey {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferBody {
    pub transfer: TransferOutputs,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferOutputs {
    pub outputs: Vec<TransferOutput>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferOutput {
    pub amount: TransferAmount,
    pub asset_type: TransferAssetType,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAmount {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransferAssetType {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: Vec<u8>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// delegation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DelegationOpt {
    #[serde(rename = "Delegation")]
    pub delegation: Delegation,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Delegation {
    pub body: DelegationBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DelegationBody {
    pub amount: i64,
    pub new_validator: Option<String>,
    pub validator: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// undelegation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UnDelegationOpt {
    #[serde(rename = "UnDelegation")]
    pub undelegation: UnDelegation,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UnDelegation {
    pub body: UnDelegationBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UnDelegationBody {
    pub pu: Option<Pu>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Pu {
    pub am: i64,
    pub new_delegator_id: String,
    pub target_validator: [u8; 20],
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// claim
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClaimOpt {
    #[serde(rename = "Claim")]
    pub claim: Claim,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Claim {
    pub body: ClaimOptBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClaimOptBody {
    pub amount: i64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// define asset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DefineAssetOpt {
    #[serde(rename = "DefineAsset")]
    pub define_asset: DefineAsset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DefineAsset {
    pub pubkey: Key,
    pub body: DefineAssetBody,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Key {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssetCode {
    pub val: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssetRules {
    pub decimals: i64,
    pub max_units: String,
    pub transfer_multisig_rules: Option<Value>,
    pub transferable: bool,
    pub updatable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DefineAssetBody {
    pub asset: Asset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Asset {
    pub asset_rules: AssetRules,
    pub code: AssetCode,
    pub issuer: Key,
    pub memo: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// issue asset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IssueAssetOpt {
    #[serde(rename = "IssueAsset")]
    pub issue_asset: IssueAsset,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IssueAsset {
    pub body: IssueAssetBody,
    pub pubkey: Key,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IssueAssetBody {
    pub code: AssetCode,
}
