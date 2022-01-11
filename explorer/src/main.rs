mod service;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let service = Router::new().route("/tx/:tx_id", get(service::tx));

    axum::Server::bind(&"0.0.0.0:8778".parse().unwrap())
        .serve(service.into_make_service())
        .await
        .unwrap();
}
