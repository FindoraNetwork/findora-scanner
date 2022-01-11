use axum::extract::Path;
use axum::Json;
use module::db::tx::Transaction;

pub async fn tx(Path(_tx_id): Path<String>) -> Json<Transaction> {
    Json(Transaction::default())
}
