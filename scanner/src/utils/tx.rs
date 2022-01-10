use serde::Deserialize;

use crate::{Error, Result};

use super::RPCResponse;

#[derive(Deserialize, Debug)]
pub struct TxResult {
    pub code: i64,
    pub log: String,
    pub events: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub tx_result: TxResult,
}

impl Transaction {
    pub async fn load_height(url: &str, hash: &str) -> Result<Self> {
        let url = format!("{}/tx?hash=0x{}", url, hash);

        let r = reqwest::get(url)
            .await?
            .json::<RPCResponse<Transaction>>()
            .await?;
        Ok(r.result)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse() -> Result<()> {
        let _ = Transaction::load_height(
            "https://prod-mainnet.prod.findora.org:26657",
            "c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0",
        )
        .await;
        Ok(())
    }
}
