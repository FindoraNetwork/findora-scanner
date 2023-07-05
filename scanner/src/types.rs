use ethereum::LegacyTransaction;
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};

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
