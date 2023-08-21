use serde::{Deserialize, Serialize};
pub mod asset;
pub mod claim;
pub mod delegation;
pub mod native_to_evm;
pub mod other;
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
