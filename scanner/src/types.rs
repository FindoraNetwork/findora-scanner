use ethereum::{LegacyTransaction, TransactionAction, TransactionSignature};
use ethereum_types::U256;
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
pub enum FindoraTxType {
    Native,             //0
    Evm,                //1
    TypeHideAmountShow, //2
    TypeShowAmountHide, //3
    TypeHideAmountHide, //4
    AbarToBar,
    AbarToAbar,
    BarToAbar,
    NativeToEVM,  //8
    EVMToNative,  //9
    Delegation,   //10
    Undelegation, //11
    Claim,        //12
    DefineAsset,  //13
    IssueAsset,   //14
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// XHub: evm to native
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct XHubOpt {
    pub function: XHub,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XHub {
    #[serde(rename = "XHub")]
    pub xhub: NonConfidentialTransfer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonConfidentialTransfer {
    #[serde(rename = "NonConfidentialTransfer")]
    pub nonconfidential_transfer: XHubTransfer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XHubTransfer {
    pub input_value: i64,
    pub outputs: Vec<XHubOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XHubOutput {
    pub amount: i64,
    pub asset: [u8; 32],
    pub target: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// evm tx
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct FindoraEVMTx {
    pub function: Ethereum,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FindoraEVMTxWrap {
    pub function: EthereumWrap,
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
pub struct EthereumWrap {
    #[serde(rename = "Ethereum")]
    pub ethereum: TransactWrap,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transact {
    #[serde(rename = "Transact")]
    pub transact: LegacyTransaction,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactWrap {
    #[serde(rename = "Transact")]
    pub transact: TransactWrapData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactWrapData {
    pub from: String,
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
    pub signature: TransactionSignature,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ConvertAccount
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertAccountReceiver {
    #[serde(rename = "Ethereum")]
    pub ethereum: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertAccount {
    pub nonce: Value,
    pub asset_type: Option<Vec<u8>>,
    pub receiver: ConvertAccountReceiver,
    pub signer: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertAccountOperation {
    #[serde(rename = "ConvertAccount")]
    pub convert_account: ConvertAccount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TransferAsset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct TxValue {
    pub body: TxBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxBody {
    pub operations: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAssetOpt {
    #[serde(rename = "TransferAsset")]
    pub transfer_asset: TransferAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAsset {
    pub body: TransferBody,
    pub body_signatures: Vec<BodySignature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BodySignature {
    pub address: SignatureKey,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureKey {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferBody {
    pub inputs: Value,
    pub outputs: Value,
    pub policies: Value,
    pub transfer: Transfer,
    pub transfer_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub asset_tracing_memos: Value,
    pub inputs: Value,
    pub outputs: Vec<Value>,
    pub owners_memos: Value,
    pub proofs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferInput {
    pub amount: Value,
    pub asset_type: Value,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputTypeShowAmountShow {
    pub amount: TransferAmountShow,
    pub asset_type: TransferAssetTypeShow,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAmountShow {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAssetTypeShow {
    #[serde(rename = "NonConfidential")]
    pub non_confidential: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputTypeShowAmountHide {
    pub amount: TransferAmountHide,
    pub asset_type: TransferAssetTypeShow,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputTypeHideAmountShow {
    pub amount: TransferAmountShow,
    pub asset_type: TransferAssetTypeHide,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputTypeHideAmountHide {
    pub amount: TransferAmountHide,
    pub asset_type: TransferAssetTypeHide,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAmountHide {
    #[serde(rename = "Confidential")]
    pub confidential: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAssetTypeHide {
    #[serde(rename = "Confidential")]
    pub confidential: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// delegation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegationOpt {
    #[serde(rename = "Delegation")]
    pub delegation: Delegation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delegation {
    pub body: DelegationBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegationBody {
    pub amount: i64,
    pub new_validator: Option<String>,
    pub validator: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// undelegation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnDelegationOpt {
    #[serde(rename = "UnDelegation")]
    pub undelegation: UnDelegation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnDelegation {
    pub body: UnDelegationBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnDelegationBody {
    pub pu: Option<Pu>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pu {
    pub am: i64,
    pub new_delegator_id: String,
    pub target_validator: [u8; 20],
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// claim
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimOpt {
    #[serde(rename = "Claim")]
    pub claim: Claim,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    pub body: ClaimOptBody,
    pub pubkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimOptBody {
    pub amount: i64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// define asset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct DefineAssetOpt {
    #[serde(rename = "DefineAsset")]
    pub define_asset: DefineAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefineAsset {
    pub pubkey: Key,
    pub body: DefineAssetBody,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetCode {
    pub val: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetRules {
    pub decimals: i32,
    pub max_units: String,
    pub transfer_multisig_rules: Option<Value>,
    pub transferable: bool,
    pub updatable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefineAssetBody {
    pub asset: Asset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub asset_rules: AssetRules,
    pub code: AssetCode,
    pub issuer: Key,
    pub memo: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// issue asset
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssetOpt {
    #[serde(rename = "IssueAsset")]
    pub issue_asset: IssueAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAsset {
    pub body: IssueAssetBody,
    pub pubkey: Key,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueAssetBody {
    pub code: AssetCode,
    pub num_outputs: i64,
    pub records: Value,
    pub seq_num: i64,
}
