use crate::{Result, Error};

pub const EVM_TX_TAG: [u8; 4] = [0x65, 0x76, 0x6d, 0x3a];

pub fn unwrap(tx: &[u8]) -> Result<&[u8]> {
    let len = EVM_TX_TAG.len();

    if tx.len() <= len || !tx[..len].eq(&EVM_TX_TAG) {
        return Err(Error::EvmTxParseError);
    }

    Ok(&tx[len..])
}

pub enum TxCatalog {
    /// findora tx
    FindoraTx,

    /// evm tx
    EvmTx,

    /// unknown tx
    Unknown,
}

/// Check Tx Catalog
pub fn try_tx_catalog(tx: &[u8]) -> TxCatalog {
    let len = EVM_TX_TAG.len();
    if tx.len() <= len {
        return TxCatalog::Unknown;
    }

    if EVM_TX_TAG.eq(&tx[..len]) {
        return TxCatalog::EvmTx;
    }

    TxCatalog::FindoraTx
}

