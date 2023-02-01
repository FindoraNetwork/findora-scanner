mod service;
mod utils;

use crate::service::address::AddressResponse;
use crate::service::asset::AssetResponse;
use crate::service::block::{BlocksResponse, FullBlockResponse, SimpleBlockResponse};
use crate::service::chain::{AddressCountResponse, ChainStatisticsResponse, DistributeResponse};
use crate::service::price::{MarketChartResponse, SimplePriceResponse};
use crate::service::staking::{
    ClaimResponse, DelegationAmountResponse, DelegationInfoResponse, DelegationResponse,
    SimpleDelegationResponse, UnDelegationResponse, UndelegationAmountResponse,
    UndelegationResponse,
};
use crate::service::tx::{
    ClaimAmountResponse, PrismRecordResponse, PrismRecordResponseNew, TxResponse, TxsResponse,
};
use crate::service::validator::{
    CirculatingSupplyResponse, DelegatorListResponse, ValidatorDelegationResponse,
    ValidatorDetailResponse, ValidatorHistoryResponse, ValidatorListResponse,
    ValidatorSignedCountResponse,
};
use crate::utils::handle_fetch_one_err;
use anyhow::Result;
use poem::middleware::Cors;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::{OpenApi, OpenApiService, Tags};
use scanner::rpc::TendermintRPC;
use service::tx::PmtxsResponse;
use sqlx::pool::PoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::sync::Mutex;

#[allow(dead_code)]
pub struct Api {
    redis_client: redis::Client,
    platform: TendermintRPC,
    tendermint: TendermintRPC,
    storage: Mutex<Pool<Postgres>>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/tx/:tx_hash", method = "get", tag = "ApiTags::Transaction")]
    async fn get_tx(
        &self,
        /// tx hash, e.g. c19fc22beb61030607367b42d4898a26ede1e6aa6b400330804c95b241f29bd0.
        tx_hash: Path<String>,
    ) -> poem::Result<TxResponse> {
        service::tx::get_tx(self, tx_hash)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/tx/evm/:tx_hash",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_evm_tx(
        &self,
        /// evm tx hash, e.g. 0x697c0492b64b8e786061818c12af46e9b62b9ee20e573d7549e7a82e94ef13cf.
        tx_hash: Path<String>,
    ) -> poem::Result<TxResponse> {
        service::tx::get_evm_tx(self, tx_hash)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/txs/to", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs_send_to(
        &self,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_txs_send_to(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/txs/from", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs_receive_from(
        &self,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_txs_receive_from(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/txs", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs(
        &self,
        /// block hash, e.g. 4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0.
        block_id: Query<Option<String>>,
        /// block height.
        height: Query<Option<i64>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        address: Query<Option<String>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        from: Query<Option<String>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        to: Query<Option<String>>,
        /// transaction type.
        /// 0 - Findora native tx.
        /// 1 - EVM tx.
        ty: Query<Option<i32>>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_txs(
            self, block_id, height, address, from, to, ty, start_time, end_time, page, page_size,
        )
        .await
        .map_err(handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(path = "/txs/raw", method = "get", tag = "ApiTags::Transaction")]
    async fn get_txs_no_wrap(
        &self,
        /// block hash, e.g. 4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0.
        block_id: Query<Option<String>>,
        /// block height.
        height: Query<Option<i64>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        address: Query<Option<String>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        from: Query<Option<String>>,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        to: Query<Option<String>>,
        /// transaction type.
        /// 0 - Findora native tx.
        /// 1 - EVM tx.
        ty: Query<Option<i32>>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_txs_raw(
            self, block_id, height, address, from, to, ty, start_time, end_time, page, page_size,
        )
        .await
        .map_err(handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/txs/triple_masking",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_triple_masking_txs(
        &self,
        /// block hash, e.g. 4B7C22FA8FC6913E091DC324830181BBA1F01EBFF53049F958EA5AA65327BFE0.
        block_id: Query<Option<String>>,
        /// public key, e.g. b2fdE7jKfQg_XL2CT7jdw84XkTdpX3uiRgpgW-h6k6o=.
        pub_key: Query<Option<String>>,
        /// 0 - default, querying both AbarToBar and BarToAbar.
        /// 1 - AbarToBar.
        /// 2 - BarToAbar.
        bar: Query<Option<i32>>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_triple_masking_txs(
            self, block_id, pub_key, bar, start_time, end_time, page, page_size,
        )
        .await
        .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/txs/claim", method = "get", tag = "ApiTags::Transaction")]
    async fn get_claim_txs(
        &self,
        /// block hash.
        block_id: Query<Option<String>>,
        /// public key.
        pub_key: Query<Option<String>>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<TxsResponse> {
        service::tx::get_claim_txs(
            self, block_id, pub_key, start_time, end_time, page, page_size,
        )
        .await
        .map_err(handle_fetch_one_err)
    }

    #[allow(clippy::too_many_arguments)]
    #[oai(
        path = "/txs/prism/:address",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_prism_tx(
        &self,
        /// Bridge Contract deploy address, e.g. 0x2B7835AE05C9Cb5EF086e3BFe249e2658b450E8d.
        address: Path<String>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<PmtxsResponse> {
        service::tx::get_prism_tx(self, address, start_time, end_time, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/block/height/:height", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_height(
        &self,
        /// block height.
        height: Path<i64>,
    ) -> poem::Result<SimpleBlockResponse> {
        service::block::get_block_by_height(self, height)
            .await
            .map_err(handle_fetch_one_err)
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
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/block/hash/:hash", method = "get", tag = "ApiTags::Block")]
    async fn get_block_by_hash(
        &self,
        /// block hash.
        hash: Path<String>,
    ) -> poem::Result<SimpleBlockResponse> {
        service::block::get_block_by_hash(self, hash)
            .await
            .map_err(handle_fetch_one_err)
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
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/blocks", method = "get", tag = "ApiTags::Block")]
    async fn get_blocks(
        &self,
        /// starting height.
        start_height: Query<Option<i64>>,
        /// ending height.
        end_height: Query<Option<i64>>,
        /// starting timestamp.
        start_time: Query<Option<i64>>,
        /// ending timestamp.
        end_time: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
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
        .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/address/:address", method = "get", tag = "ApiTags::Address")]
    async fn get_address(
        &self,
        /// bech32 address, e.g. fra1p4vy5n9mlkdys7xczegj398xtyvw2nawz00nnfh4yr7fpjh297cqsxfv7v.
        address: Path<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<AddressResponse> {
        service::address::get_address(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/asset/:code", method = "get", tag = "ApiTags::Asset")]
    async fn get_asset(
        &self,
        /// base64 asset code, e.g. AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=.
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
        /// tx type, 0 - Findora native tx, 1 - EVM tx.
        ty: Query<Option<i32>>,
    ) -> poem::Result<ChainStatisticsResponse> {
        service::chain::statistics(self, ty)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator_list",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_list(&self) -> poem::Result<ValidatorListResponse> {
        service::validator::validator_list(self)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator_detail/:address",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_detail(
        &self,
        /// validator address, e.g. 917454FB61CFBDB1995BC57C7A821E41FFB1AF43.
        address: Path<String>,
    ) -> poem::Result<ValidatorDetailResponse> {
        service::validator::validator_detail(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/circulating_supply",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn circulating_supply(&self) -> poem::Result<CirculatingSupplyResponse> {
        service::validator::circulating_supply(self)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator/signed_count",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_signed_count(
        &self,
        /// validator address, e.g. 917454FB61CFBDB1995BC57C7A821E41FFB1AF43.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<ValidatorSignedCountResponse> {
        service::validator::validator_signed_count(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/delegator_list/:address",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn delegator_list(
        &self,
        /// delegator address, e.g. 000E33AB7471186F3B1DE9FC08BB9C480F453590.
        address: Path<String>,
    ) -> poem::Result<DelegatorListResponse> {
        service::validator::delegator_list(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/simple/price", method = "get", tag = "ApiTags::Price")]
    async fn simple_price(
        &self,
        ids: Query<String>,
        vs_currencies: Query<String>,
    ) -> poem::Result<SimplePriceResponse> {
        service::price::simple_price(self, ids, vs_currencies).await
    }

    #[oai(
        path = "/coins/:id/market_chart",
        method = "get",
        tag = "ApiTags::Price"
    )]
    async fn market_chart(
        &self,
        id: Path<String>,
        vs_currency: Query<String>,
        interval: Query<Option<String>>,
        days: Query<i32>,
    ) -> poem::Result<MarketChartResponse> {
        service::price::market_chart(self, id, vs_currency, interval, days).await
    }

    #[oai(path = "/address/count", method = "get", tag = "ApiTags::Address")]
    async fn address_count(
        &self,
        /// staring timestamp.
        start_time: Query<i64>,
        /// ending timestamp.
        end_time: Query<i64>,
    ) -> poem::Result<AddressCountResponse> {
        service::chain::address_count(self, start_time, end_time)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/txs/distribute", method = "get", tag = "ApiTags::Transaction")]
    async fn distribute(&self) -> poem::Result<DistributeResponse> {
        service::chain::distribute(self)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator_delegation",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn validator_delegation(
        &self,
        /// validator address, e.g. 000E33AB7471186F3B1DE9FC08BB9C480F453590.
        address: Query<String>,
    ) -> poem::Result<ValidatorDelegationResponse> {
        service::validator::validator_delegation(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/tx/prism/records/:address",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_prism_records(
        &self,
        /// bech32 address or RIMP160 address.
        address: Path<String>,
    ) -> poem::Result<PrismRecordResponse> {
        service::tx::get_prism_records(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/tx/prism/records/receive",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_prism_records_receive(
        &self,
        /// bech32 address or RIMP160 address.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<PrismRecordResponseNew> {
        service::tx::get_prism_records_receive_from(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/tx/prism/records/send",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_prism_records_send(
        &self,
        /// bech32 address or RIMP160 addrss.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<PrismRecordResponseNew> {
        service::tx::get_prism_records_send_to(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/tx/delegation", method = "get", tag = "ApiTags::Transaction")]
    async fn get_delegation_tx(
        &self,
        /// bech32 address, e.g. fra18fnyetvs2kc035xz78kyfcygmej8pk5h2kwczy03w6uewdphzfxsk74dym.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<DelegationResponse> {
        service::staking::get_tx_delegation(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/tx/undelegation",
        method = "get",
        tag = "ApiTags::Transaction"
    )]
    async fn get_undelegation(
        &self,
        /// bech32 address, e.g. fra18fnyetvs2kc035xz78kyfcygmej8pk5h2kwczy03w6uewdphzfxsk74dym.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<UnDelegationResponse> {
        service::staking::get_tx_undelegation(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/staking/claim", method = "get", tag = "ApiTags::Staking")]
    async fn get_claim(
        &self,
        /// bech32 address, e.g. fra1xczgryuz65as77gf8d5f07xd0wetd8qpm5hvgqkfgc60gxdjpmkshnq9ys.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<ClaimResponse> {
        service::staking::get_claim(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/validator/history",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn get_validator_history(
        &self,
        /// validator address, e.g. 9E6717392EFDCFA101E33449A7C2A238251315B1.
        address: Query<String>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<ValidatorHistoryResponse> {
        service::validator::validator_history(self, address, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/chain/claim/:address",
        method = "get",
        tag = "ApiTags::BlockChain"
    )]
    async fn get_claim_amount(&self, address: Path<String>) -> poem::Result<ClaimAmountResponse> {
        service::tx::get_claims_amount(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/staking/delegation_info/:address",
        method = "get",
        tag = "ApiTags::Staking"
    )]
    async fn get_delegation(&self, address: Path<String>) -> poem::Result<DelegationInfoResponse> {
        service::staking::delegation(self, address)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/staking/undelegation",
        method = "get",
        tag = "ApiTags::Staking"
    )]
    async fn get_undelegation_info(
        &self,
        /// base64 pubkey, e.g. OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=
        pubkey: Query<Option<String>>,
        /// staring timestamp.
        start: Query<Option<i64>>,
        /// ending timestamp.
        end: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<UndelegationResponse> {
        service::staking::get_undelegation_info(self, pubkey, start, end, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(path = "/staking/delegation", method = "get", tag = "ApiTags::Staking")]
    async fn get_delegation_info(
        &self,
        /// base64 pubkey, e.g. OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=
        pubkey: Query<Option<String>>,
        /// starting timestamp.
        start: Query<Option<i64>>,
        /// ending timestamp.
        end: Query<Option<i64>>,
        /// page index, the default is 1.
        page: Query<Option<i64>>,
        /// page size, the default is 10.
        page_size: Query<Option<i64>>,
    ) -> poem::Result<SimpleDelegationResponse> {
        service::staking::get_delegation_info(self, pubkey, start, end, page, page_size)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/staking/delegation/amount",
        method = "get",
        tag = "ApiTags::Staking"
    )]
    async fn get_delegation_amount(
        &self,
        /// base64 pubkey, e.g. OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=
        pubkey: Query<Option<String>>,
        /// starting timestamp.
        start: Query<Option<i64>>,
        /// ending timestamp.
        end: Query<Option<i64>>,
    ) -> poem::Result<DelegationAmountResponse> {
        service::staking::get_delegation_amount(self, pubkey, start, end)
            .await
            .map_err(handle_fetch_one_err)
    }

    #[oai(
        path = "/staking/undelegation/amount",
        method = "get",
        tag = "ApiTags::Staking"
    )]
    async fn get_undelegation_amount(
        &self,
        /// base64 pubkey, e.g. OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=
        pubkey: Query<Option<String>>,
        /// starting timestamp.
        start: Query<Option<i64>>,
        /// ending timestamp.
        end: Query<Option<i64>>,
    ) -> poem::Result<UndelegationAmountResponse> {
        service::staking::get_undelegation_amount(self, pubkey, start, end)
            .await
            .map_err(handle_fetch_one_err)
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
    /// Operations about Price
    Price,
    /// Operations about staking
    Staking,
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
    let mut opt = PoolOptions::new();
    opt = opt.max_connections(1000);
    let pool = opt.connect(&postgres_config).await.unwrap();

    // tendermint rpc
    let tendermint_rpc_client = TendermintRPC::new(
        Duration::from_secs(60),
        config.rpc.tendermint.to_string().parse().unwrap(),
    );
    let platform_rpc_client = TendermintRPC::new(
        Duration::from_secs(60),
        config.rpc.platform.to_string().parse().unwrap(),
    );

    let rds_client = redis::Client::open("redis://127.0.0.1").unwrap();

    let api = Api {
        redis_client: rds_client,
        platform: platform_rpc_client,
        tendermint: tendermint_rpc_client,
        storage: Mutex::new(pool),
    };

    let server_config = format!("http://{}:{}/api", config.server.addr, config.server.port);

    let api_service = OpenApiService::new(api, "explorer", "1.0").server(server_config);
    let ui = api_service.swagger_ui();

    let server_addr = format!("{}:{}", config.server.addr, config.server.port);
    Server::new(TcpListener::bind(server_addr))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .with(Cors::new()),
        )
        .await?;

    Ok(())
}
