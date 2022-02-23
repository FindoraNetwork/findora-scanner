use std::time::Duration;

use crate::{Error, Result};

use serde::de::DeserializeOwned;

use module::rpc::{
    block::BlockRPC as ModuleBlockRPC, tx::Transaction as ModuleTx,
    validator::ValidatorsRPC as ModuleValidatorsRPC, RPCPesponse,
};

use reqwest::{Client, ClientBuilder, Url};

pub struct TendermintRPC {
    rpc: Url,
    client: Client,
}

impl TendermintRPC {
    pub fn new(timeout: Duration, rpc: Url) -> Self {
        let client = ClientBuilder::new().timeout(timeout).build().unwrap();
        TendermintRPC { client, rpc }
    }

    pub async fn load_block(&self, height: i64) -> Result<ModuleBlockRPC> {
        let mut url = self.rpc.join("block").unwrap();
        url.set_query(Some(&format!("height={}", height)));
        debug!("{}", url.as_str());
        let r: ModuleBlockRPC = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_validators(&self, height: i64) -> Result<ModuleValidatorsRPC> {
        let mut url = self.rpc.join("validators").unwrap();
        url.set_query(Some(&format!("height={}", height)));

        let r: ModuleValidatorsRPC = self.client_get(url).await?;
        Ok(r)
    }

    pub async fn load_transaction(&self, hash: &str) -> Result<ModuleTx> {
        let mut url = self.rpc.join("tx").unwrap();
        url.set_query(Some(&format!("hash=0x{}", hash)));

        let r: ModuleTx = self.client_get(url).await?;
        Ok(r)
    }

    async fn client_get<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let resp = self.client.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(resp.text().await?.into());
        }
        if let Ok(r) = resp.json::<RPCPesponse<T>>().await {
            Ok(r.result)
        } else {
            Err(Error::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc() -> Result<()> {
        let rpc = TendermintRPC::new(
            Duration::from_secs(10),
            "https://prod-mainnet.prod.findora.org:26657"
                .to_string()
                .parse()
                .unwrap(),
        );
        let _ = rpc.load_block(1550667).await?;
        let _ = rpc.load_validators(1550667).await?;
        let _ = rpc
            .load_transaction("c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0")
            .await?;
        Ok(())
    }
}
