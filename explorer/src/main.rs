mod service;
mod utils;

use crate::service::address::GetAddressResponse;
use crate::service::asset::GetAssetResponse;
use crate::service::block::{GetBlockResponse, GetBlocksResponse};
use crate::service::tx::{GetTxResponse, GetTxsResponse};
use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::{OpenApi, OpenApiService, Tags};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub struct Api {
    storage: Mutex<Pool<Postgres>>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/tx/:tx_id", method = "get", tag = "ApiTags::Transaction")]
    async fn get_tx(&self, tx_id: Path<String>) -> poem::Result<GetTxResponse> {
        service::tx::get_tx(self, tx_id)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/txs", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs(
        &self,
        block_id: Query<Option<String>>,
        from: Query<Option<String>>,
        to: Query<Option<String>>,
        ty: Query<Option<i64>>,
        start_time: Query<Option<i64>>,
        end_time: Query<Option<i64>>,
        page: Query<Option<i64>>,
        page_size: Query<Option<i64>>,
    ) -> poem::Result<GetTxsResponse> {
        service::tx::get_txs(
            self, block_id, from, to, ty, start_time, end_time, page, page_size,
        )
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
    async fn get_blocks(
        &self,
        start_height: Query<Option<i64>>,
        end_height: Query<Option<i64>>,
        start_time: Query<Option<i64>>,
        end_time: Query<Option<i64>>,
        page: Query<Option<i64>>,
        page_size: Query<Option<i64>>,
    ) -> poem::Result<GetBlocksResponse> {
        service::block::get_blocks(
            self,
            start_height,
            end_height,
            start_time,
            end_time,
            page,
            page_size,
        )
        .await
        .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/address/:address", method = "get", tag = "ApiTags::Address")]
    async fn get_address(&self, address: Path<String>) -> poem::Result<GetAddressResponse> {
        service::address::get_address(self, address)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/asset/:code", method = "get", tag = "ApiTags::Asset")]
    async fn get_asset(&self, code: Path<String>) -> poem::Result<GetAssetResponse> {
        service::asset::get_asset(self, code)
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
    let config_path = std::env::var("CONFIG_FILE_PATH").unwrap();
    let config = module::config::explorer_config::Config::new(&config_path)?;
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
