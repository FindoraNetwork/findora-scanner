use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, OpenApiService, Tags};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

pub struct Api {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct GetTotalSupplyResp {
    pub global_adjusted_circulating_supply: f64,
    pub global_circulating_supply: f64,
    pub global_total_supply: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct GetCirculatingSupplyResp {
    pub global_circulating_supply: f64,
    pub global_delegation_amount: f64,
    pub global_return_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Object)]
pub struct CirculatingSupply {
    pub global_circulating_supply: f64,
    pub global_return_rate: f64,
    pub global_delegation_amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u64,
    pub url: String,
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

#[derive(ApiResponse)]
pub enum CirculatingSupplyResp {
    #[oai(status = 200)]
    Ok(Json<CirculatingSupply>),
}

#[derive(ApiResponse)]
pub enum TotalCirculatingSupplyResp {
    #[oai(status = 200)]
    Ok(Json<f64>),
}

#[OpenApi]
impl Api {
    #[oai(
        path = "/circulating_supply",
        method = "get",
        tag = "ApiTags::MainNetPatch"
    )]
    async fn circulating_supply(&self) -> poem::Result<CirculatingSupplyResp> {
        let err_handle = |e: reqwest::Error| -> poem::Error {
            poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        };

        let gcsr = reqwest::get(format!(
            "{}{}",
            self.config.server.url, "circulating_supply"
        ))
        .await
        .map_err(err_handle)?
        .json::<GetCirculatingSupplyResp>()
        .await
        .map_err(err_handle)?;

        let gtsr = reqwest::get(format!("{}{}", self.config.server.url, "get_total_supply"))
            .await
            .map_err(err_handle)?
            .json::<GetTotalSupplyResp>()
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
    async fn total_circulating_supply(&self) -> poem::Result<TotalCirculatingSupplyResp> {
        let err_handle = |e: reqwest::Error| -> poem::Error {
            poem::Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        };

        let gtsr = reqwest::get(format!("{}{}", self.config.server.url, "get_total_supply"))
            .await
            .map_err(err_handle)?
            .json::<GetTotalSupplyResp>()
            .await
            .map_err(err_handle)?;

        Ok(TotalCirculatingSupplyResp::Ok(Json(
            gtsr.global_circulating_supply,
        )))
    }
}

#[derive(Tags)]
enum ApiTags {
    MainNetPatch,
}

#[tokio::main]
async fn main() -> Result<()> {
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
