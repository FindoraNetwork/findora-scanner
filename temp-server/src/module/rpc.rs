use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{Deserialize, Serialize};

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

#[derive(ApiResponse)]
pub enum CirculatingSupplyResp {
    #[oai(status = 200)]
    Ok(Json<CirculatingSupply>),
}

#[derive(ApiResponse)]
pub enum F64Resp {
    #[oai(status = 200)]
    Ok(Json<f64>),
}
