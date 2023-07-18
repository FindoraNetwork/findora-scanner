mod service;

use crate::service::v1::api::Api;
use anyhow::Result;
use poem::middleware::Cors;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use scanner::rpc::TendermintRPC;
use sqlx::pool::PoolOptions;
use std::time::Duration;
use tokio::sync::Mutex;

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

    let pool = PoolOptions::new()
        .max_connections(100)
        .connect(&postgres_config)
        .await
        .unwrap();

    // tendermint rpc
    let tendermint_rpc_client = TendermintRPC::new(
        Duration::from_secs(60),
        config.rpc.tendermint.to_string().parse().unwrap(),
    );
    let platform_rpc_client = TendermintRPC::new(
        Duration::from_secs(60),
        config.rpc.platform.to_string().parse().unwrap(),
    );
    let platform_server_rpc_client = TendermintRPC::new(
        Duration::from_secs(60),
        config.rpc.platform_server.to_string().parse().unwrap(),
    );

    let api = Api {
        platform: platform_rpc_client,
        platform_server: platform_server_rpc_client,
        tendermint: tendermint_rpc_client,
        storage: Mutex::new(pool),
    };

    let server_config = format!("http://{}:{}/api", config.server.addr, config.server.port);

    let api_service = OpenApiService::new(api, "explorer", "1.0").server(server_config);
    let ui = api_service.swagger_ui();

    let server_addr = format!("{}:{}", config.server.addr, config.server.port);
    let cors = Cors::new();

    Server::new(TcpListener::bind(server_addr))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .with(cors),
        )
        .await?;

    Ok(())
}
