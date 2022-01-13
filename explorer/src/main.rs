mod service;
mod utils;

use crate::service::block::GetBlockResponse;
use crate::service::blockchain::BlockChainResponse;
use crate::service::tx::GetTxResponse;
use crate::service::tx_search::TxSearchResponse;
use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::param::Path;
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

    #[oai(path = "/block/:height", method = "get", tag = "ApiTags::Block")]
    async fn get_block(&self, height: Path<i64>) -> poem::Result<GetBlockResponse> {
        service::block::get_block(self, height)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/tx_search", method = "get", tag = "ApiTags::Transaction")]
    async fn tx_search(&self, start: Path<i64>, end: Path<i64>) -> poem::Result<TxSearchResponse> {
        service::tx_search::tx_search(self, start, end)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/blockchain", method = "get", tag = "ApiTags::Block")]
    async fn blockchain(
        &self,
        min_height: Path<i64>,
        max_height: Path<i64>,
    ) -> poem::Result<BlockChainResponse> {
        service::blockchain::blockchain(self, min_height, max_height)
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
