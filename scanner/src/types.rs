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
pub struct TransferAsset {
    pub body: TransferBody,
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
