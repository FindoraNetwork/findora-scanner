mod service;
mod utils;

use crate::service::address::GetAddressResponse;
use crate::service::asset::GetAssetResponse;
use crate::service::block::{GetBlockResponse, GetBlocksResponse};
use crate::service::tx::{GetTxResponse, GetTxsResponse};
use anyhow::Result;
use poem::web::Query;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::param::Path;
use poem_openapi::{OpenApi, OpenApiService, Tags};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub struct Api {
    storage: Mutex<Pool<Postgres>>,
}

#[derive(Deserialize)]
pub struct GetTxsParam {
    block_id: Option<String>,
    from_address: Option<String>,
    to_address: Option<String>,
    ty: Option<i64>,
    begin_time: Option<i64>,
    end_time: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct GetBlocksParam {
    begin_height: Option<i64>,
    end_height: Option<i64>,
    begin_time: Option<i64>,
    end_time: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/tx/:tx_id", method = "get", tag = "ApiTags::Transaction")]
    async fn get_tx(&self, tx_id: Path<String>) -> poem::Result<GetTxResponse> {
        service::tx::get_tx(self, tx_id)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/txs", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs(&self, param: Query<GetTxsParam>) -> poem::Result<GetTxsResponse> {
        service::tx::get_txs(self, param)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/block/height/:height", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_height(&self, height: Path<i64>) -> poem::Result<GetBlockResponse> {
        service::block::get_block_by_height(self, height)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/block/hash/:hash", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_hash(&self, hash: Path<String>) -> poem::Result<GetBlockResponse> {
        service::block::get_block_by_hash(self, hash)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/blocks", method = "get", tag = "ApiTags::Block")]
    async fn get_blocks(&self, param: Query<GetBlocksParam>) -> poem::Result<GetBlocksResponse> {
        service::block::get_blocks(self, param)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/address/:address", method = "get", tag = "ApiTags::Address")]
    async fn get_address(&self, address: Path<String>) -> poem::Result<GetAddressResponse> {
        service::address::get_address(self, address)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/asset/:address", method = "get", tag = "ApiTags::Asset")]
    async fn get_asset(&self, address: Path<String>) -> poem::Result<GetAssetResponse> {
        service::asset::get_asset(self, address)
            .await
            .map_err(utils::handle_fetch_one_err)
    }
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about Transaction
    Transaction,
    /// Operations about Block
    Block,
    /// Operations about Address
    Address,
    /// Operations about Asset
    Asset,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let current_path = std::env::current_exe()?;
    let config_path = current_path.parent().unwrap().join("config.toml");
    let config = module::config::explorer_config::Config::new(config_path.to_str().unwrap())?;
    let postgres_config = format!(
        "postgres://{}:{}@{}/{}",
        config.postgres.account,
        config.postgres.password,
        config.postgres.addr,
        config.postgres.database
    );
    //let postgres_config=format!("host={} user={} password={}",config.postgres.addr, config.postgres.account, config.postgres.password);
    // std::env::set_var("DATABASE_URL", postgres_config);
    let pool = sqlx::PgPool::connect(&postgres_config).await.unwrap();

    let api = Api {
        storage: Mutex::new(pool),
    };

    let server_config = format!("http://{}:{}/api", config.server.addr, config.server.port);

    let api_service = OpenApiService::new(api, "explorer", "1.0").server(server_config);
    let ui = api_service.swagger_ui();

    let server_addr = format!("{}:{}", config.server.addr, config.server.port);
    Server::new(TcpListener::bind(server_addr))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await?;

    Ok(())
}
