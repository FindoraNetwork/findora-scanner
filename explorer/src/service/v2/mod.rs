use serde::{Deserialize, Serialize};
pub mod block;
pub mod claim;
pub mod define_asset;
pub mod delegation;
pub mod issue_asset;
pub mod native;
pub mod native_to_evm;
pub mod transaction_evm;
pub mod undelegation;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
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
