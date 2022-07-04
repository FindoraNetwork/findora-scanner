mod service;
mod utils;

use crate::service::address::AddressResponse;
use crate::service::asset::AssetResponse;
use crate::service::block::{BlockResponse, BlocksResponse, FullBlockResponse};
use crate::service::chain::{ChainStatisticsResponse, StakingResponse};
use crate::service::tx::{TxResponse, TxsResponse};
use crate::service::validator::{
    CirculatingSupplyResponse, ValidatorDetailResponse, ValidatorListResponse,
};
use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::{OpenApi, OpenApiService, Tags};
use scanner::rpc::TendermintRPC;
use service::tx::PmtxsResponse;
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
        /// transaction hash, e.g. 'c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0'.
        tx_id: Path<String>,
    ) -> poem::Result<TxResponse> {
        service::tx::get_tx(self, tx_id)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/txs", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs(
        &self,
        /// block hash, e.g. '4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0'.
        block_id: Query<Option<String>>,
        /// from address, e.g. 'fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v'.
        from: Query<Option<String>>,
        /// to address.
        to: Query<Option<String>>,
        /// transaction type. 0 is for Findora tx, 1 is for evm tx.
        ty: Query<Option<i64>>,
        /// start timestamp.
        start_time: Query<Option<i64>>,
        /// end timestamp.
        end_time: Query<Option<i64>>,
        /// page index, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
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
        /// block hash, e.g. '4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0'.
        block_id: Query<Option<String>>,
        /// output public key, e.g. 'b2fdE7jKfQg_XL2CT7jdw84XkTdpX3uiRgpgW-h6k6o='.
        pub_key: Query<Option<String>>,
        /// 0: both, default.
        /// 1: AbarToBar.
        /// 2: BarToAbar.
        bar: Query<Option<i64>>,
        /// start timestamp.
        start_time: Query<Option<i64>>,
        /// end timestamp.
        end_time: Query<Option<i64>>,
        /// page index, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_triple_masking_txs(
            self, block_id, pub_key, bar, start_time, end_time, page, page_size,
        )
        .await
        .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/txs/claim", method = "get", tag = "ApiTags::Transaction")]
    async fn get_claim_txs(
        &self,
        /// block hash.
        block_id: Query<Option<String>>,
        /// public key.
        pub_key: Query<Option<String>>,
        /// start timestamp.
        start_time: Query<Option<i64>>,
        /// end timestamp.
        end_time: Query<Option<i64>>,
        /// page index, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_claim_txs(
            self, block_id, pub_key, start_time, end_time, page, page_size,
        )
        .await
        .map_err(utils::handle_fetch_one_err)
    }
    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/txs/prism/:address",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_prism_tx(
        &self,
        ///Bridge Contract deploy address, e.g. 0x2B7835AE05C9Cb5EF086e3BFe249e2658b450E8d
        address: Path<String>,
        /// start timestamp.
        start_time: Query<Option<i64>>,
        /// end timestamp.
        end_time: Query<Option<i64>>,
        /// page index, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<PmtxsResponse> {
        service::tx::get_prism_tx(self, address, start_time, end_time, page, page_size)
            .await
            .map_err(utils::handle_fetch_one_err)
    }
    #[oai(path = "/block/height/:height", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_height(
        &self,
        /// block height.
        height: Path<i64>,
    ) -> poem::Result<BlockResponse> {
        service::block::get_block_by_height(self, height)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(
        path = "/block/full/height/:height",
        method = "get",
        tag = "ApiTags::Block"
    )]
    async fn get_full_block_by_height(
        &self,
        /// block height.
        height: Path<i64>,
    ) -> poem::Result<FullBlockResponse> {
        service::block::get_full_block_by_height(self, height)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/block/hash/:hash", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_hash(
        &self,
        /// block hash.
        hash: Path<String>,
    ) -> poem::Result<BlockResponse> {
        service::block::get_block_by_hash(self, hash)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(
        path = "/block/full/hash/:hash",
        method = "get",
        tag = "ApiTags::Block"
    )]
    async fn get_full_block_by_hash(
        &self,
        /// block hash.
        hash: Path<String>,
    ) -> poem::Result<FullBlockResponse> {
        service::block::get_full_block_by_hash(self, hash)
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
        /// page index, starting at 1, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<BlocksResponse> {
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
        /// bech32 account address, e.g. 'fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v'.
        address: Path<String>,
        /// page index, default 1.
        page: Query<Option<i64>>,
        /// page size, default 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<AddressResponse> {
        service::address::get_address(self, address, page, page_size)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(path = "/asset/:code", method = "get", tag = "ApiTags::Asset")]
    async fn get_asset(
        &self,
        /// base64 asset code, e.g. 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='.
        code: Path<String>,
    ) -> poem::Result<AssetResponse> {
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

    #[oai(
        path = "/chain/validator_list",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_list(&self) -> poem::Result<ValidatorListResponse> {
        service::validator::validator_list(self)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator_detail",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_detail(
        &self,
        address: Path<String>,
    ) -> poem::Result<ValidatorDetailResponse> {
        service::validator::validator_detail(self, address)
            .await
            .map_err(utils::handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/circulating_supply",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn circulating_supply(&self) -> poem::Result<CirculatingSupplyResponse> {
        service::validator::circulating_supply(self)
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
