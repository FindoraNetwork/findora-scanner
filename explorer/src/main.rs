mod service;
use crate::service::api::Api;
use crate::service::v2::asset::get_assets;
use crate::service::v2::block::{get_block_by_hash, get_block_by_num, get_blocks};
use crate::service::v2::transaction::{get_tx_by_hash, get_txs};
use anyhow::Result;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use log::info;
use sqlx::pool::PoolOptions;
use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    pub pool: PgPool,
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

    let pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&postgres_config)
        .await
        .expect("can't connect to database");

    info!("Connecting DB...ok");

    let app_state = Arc::new(AppState { pool });
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);
    let addr = format!("{}:{}", config.server.addr, config.server.port);
    let app = Router::new()
        .route("/api/v2/number/block", get(get_block_by_num))
        .route("/api/v2/hash/block", get(get_block_by_hash))
        .route("/api/v2/blocks", get(get_blocks))
        .route("/api/v2/hash/tx", get(get_tx_by_hash))
        .route("/api/v2/txs", get(get_txs))
        .route("/api/v2/assets", get(get_assets))
        .layer(cors)
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening at: {}", addr);
    info!("Starting server...ok");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
