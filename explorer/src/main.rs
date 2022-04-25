mod service;
mod utils;

use crate::service::address::GetAddressResponse;
use crate::service::asset::GetAssetResponse;
use crate::service::block::{GetBlockResponse, GetBlocksResponse};
use crate::service::chain::{ChainStatisticsResponse, StakingResponse};
use crate::service::tx::{GetTxResponse, GetTxsResponse};
use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::{OpenApi, OpenApiService, Tags};
use scanner::rpc::TendermintRPC;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::sync::Mutex;

pub struct Api {
    rpc: TendermintRPC,
    storage: Mutex<Pool<Postgres>>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/tx/:tx_id", method = "get", tag = "ApiTags::Transaction")]
    async fn get_tx(
        &self,
        /// transaction hash, like 'c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0'.
        tx_id: Path<String>,
    ) -> poem::Result<GetTxResponse> {
        service::tx::get_tx(self, tx_id)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/txs", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs(
        &self,
        /// block hash, like '4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0'.
        block_id: Query<Option<String>>,
        /// from address, like 'fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v'.
        from: Query<Option<String>>,
        /// to address.
        to: Query<Option<String>>,
        /// transaction type. 0 is for Findora tx, 1 is for evm tx.
        ty: Query<Option<i64>>,
        /// time of transaction starts in seconds.
        start_time: Query<Option<i64>>,
        /// time of transaction ends in seconds.
        end_time: Query<Option<i64>>,
        /// page number, staring at 1, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<GetTxsResponse> {
        service::tx::get_txs(
            self, block_id, from, to, ty, start_time, end_time, page, page_size,
        )
        .await
        .map_err(utils::handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/txs/triple_masking",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_triple_masking_txs(
        &self,
        /// block hash, like '4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0'.
        block_id: Query<Option<String>>,
        /// output public key, like 'b2fdE7jKfQg_XL2CT7jdw84XkTdpX3uiRgpgW-h6k6o='.
        pub_key: Query<Option<String>>,
        /// 0: both, default.
        /// 1: AbarToBar.
        /// 2: BarToAbar.
        bar: Query<Option<i64>>,
        /// time of transaction starts in seconds.
        start_time: Query<Option<i64>>,
        /// time of transaction ends in seconds.
        end_time: Query<Option<i64>>,
        /// page number, staring at 1, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<GetTxsResponse> {
        service::tx::get_triple_masking_txs(
            self, block_id, pub_key, bar, start_time, end_time, page, page_size,
        )
        .await
        .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/block/height/:height", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_height(
        &self,
        /// block height.
        height: Path<i64>,
    ) -> poem::Result<GetBlockResponse> {
        service::block::get_block_by_height(self, height)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/block/hash/:hash", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_hash(
        &self,
        /// block hash.
        hash: Path<String>,
    ) -> poem::Result<GetBlockResponse> {
        service::block::get_block_by_hash(self, hash)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/blocks", method = "get", tag = "ApiTags::Block")]
    async fn get_blocks(
        &self,
        /// height of block start.
        start_height: Query<Option<i64>>,
        /// height of block end.
        end_height: Query<Option<i64>>,
        /// time of block starts in seconds.
        start_time: Query<Option<i64>>,
        /// time of block ends in seconds.
        end_time: Query<Option<i64>>,
        /// page number, starting at 1, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
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
    async fn get_address(
        &self,
        /// account address, like 'fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v'.
        address: Path<String>,
    ) -> poem::Result<GetAddressResponse> {
        service::address::get_address(self, address)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/asset/:code", method = "get", tag = "ApiTags::Asset")]
    async fn get_asset(
        &self,
        /// an asset address, like 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='.
        code: Path<String>,
    ) -> poem::Result<GetAssetResponse> {
        service::asset::get_asset(self, code)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/statistics",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn statistics(
        &self,
        /// ty: tx type, 0 - Findora tx, 1 - EVM tx.
        ty: Query<Option<i64>>,
    ) -> poem::Result<ChainStatisticsResponse> {
        service::chain::statistics(self, ty)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/chain/staking", method = "get", tag = "ApiTags::BlockChain")]
    async fn staking(
        &self,
        /// block height
        height: Query<Option<i64>>,
    ) -> poem::Result<StakingResponse> {
        service::chain::staking_info(self, height)
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
    /// Operations about Chain
    BlockChain,
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

    // tendermint rpc
    let rpc_client = TendermintRPC::new(
        Duration::from_secs(10),
        config.tendermint.rpc.to_string().parse().unwrap(),
    );

    let api = Api {
        rpc: rpc_client,
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
