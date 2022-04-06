mod module;
mod utils;

use module::{config::*, rpc::*};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::Json, OpenApi, OpenApiService, Tags};
use utils::*;

pub struct Api {
    pub config: Config,
}

#[OpenApi]
impl Api {
    #[oai(
        path = "/circulating_supply",
        method = "get",
        tag = "ApiTags::MainNetPatch"
    )]
    async fn circulating_supply(&self) -> poem::Result<CirculatingSupplyResp> {
        let gcsr: GetCirculatingSupplyResp = request_get::<GetCirculatingSupplyResp>(format!(
            "{}{}",
            self.config.server.url, "circulating_supply"
        ))
        .await
        .map_err(err_handle)?;

        let gtsr: GetTotalSupplyResp = request_get::<GetTotalSupplyResp>(format!(
            "{}{}",
            self.config.server.url, "get_total_supply"
        ))
        .await
        .map_err(err_handle)?;

        Ok(CirculatingSupplyResp::Ok(Json(CirculatingSupply {
            global_circulating_supply: gtsr.global_circulating_supply,
            global_return_rate: gcsr.global_return_rate,
            global_delegation_amount: gcsr.global_delegation_amount,
        })))
    }

    #[oai(
        path = "/circulating_supply/total_circulating_supply",
        method = "get",
        tag = "ApiTags::MainNetPatch"
    )]
    async fn total_circulating_supply(&self) -> poem::Result<F64Resp> {
        let gtsr: GetTotalSupplyResp = request_get::<GetTotalSupplyResp>(format!(
            "{}{}",
            self.config.server.url, "get_total_supply"
        ))
        .await
        .map_err(err_handle)?;

        Ok(F64Resp::Ok(Json(gtsr.global_circulating_supply)))
    }

    #[oai(
        path = "/circulating_supply/global_delegation_amount",
        method = "get",
        tag = "ApiTags::MainNetPatch"
    )]
    async fn global_delegation_amount(&self) -> poem::Result<F64Resp> {
        let gcsr: GetCirculatingSupplyResp = request_get::<GetCirculatingSupplyResp>(format!(
            "{}{}",
            self.config.server.url, "circulating_supply"
        ))
        .await
        .map_err(err_handle)?;

        Ok(F64Resp::Ok(Json(gcsr.global_delegation_amount)))
    }

    #[oai(
        path = "/circulating_supply/global_return_rate",
        method = "get",
        tag = "ApiTags::MainNetPatch"
    )]
    async fn global_return_rate(&self) -> poem::Result<F64Resp> {
        let gcsr: GetCirculatingSupplyResp = request_get::<GetCirculatingSupplyResp>(format!(
            "{}{}",
            self.config.server.url, "circulating_supply"
        ))
        .await
        .map_err(err_handle)?;

        Ok(F64Resp::Ok(Json(gcsr.global_return_rate)))
    }
}

#[derive(Tags)]
enum ApiTags {
    MainNetPatch,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config_path = std::env::var("CONFIG_FILE_PATH").unwrap();
    let config = Config::new(&config_path)?;

    let api = Api {
        config: config.clone(),
    };

    let server_config = format!("http://{}:{}", config.server.addr, config.server.port);

    let api_service = OpenApiService::new(api, "temp-server", "1.0").server(server_config);
    let ui = api_service.swagger_ui();

    let server_addr = format!("{}:{}", config.server.addr, config.server.port);
    Server::new(TcpListener::bind(server_addr))
        .run(Route::new().nest("/", api_service).nest("/ui", ui))
        .await?;

    Ok(())
}
