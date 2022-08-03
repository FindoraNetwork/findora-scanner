use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
    pub rpc: TendermintConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostgresConfig {
    pub account: String,
    pub password: String,
    pub addr: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TendermintConfig {
    pub platform: String,
    pub tendermint: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut str = String::new();
        file.read_to_string(&mut str)?;

        let config: Config = toml::from_str(&str)?;
        Ok(config)
    }
}
