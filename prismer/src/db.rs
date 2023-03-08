use module::schema::TxResult;
use sqlx::{Error, PgPool};

pub use sqlx::Error as SqlxError;
pub use sqlx::PgPool as SqlxPgPool;

use module::schema::LastHeight;

pub async fn connect() -> Result<PgPool, Error> {
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var `DATABASE_URL` is required for the findora scanner.");
    PgPool::connect(&conn_str).await.map_err(Error::from)
}

pub async fn save(res: Vec<TxResult>, pool: &PgPool) -> Result<(), Error> {
    for tr in res {
        sqlx::query!(
                "INSERT INTO result VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1, block_hash=$2, height=$3, timestamp=$4, code=$5, ty=$6, value=$7",
                &tr.tx_hash, &tr.block_hash, &tr.height, &tr.timestamp, &tr.code, &tr.ty, &tr.value
            )
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn save_last_height(height: i64, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO result_last_height VALUES($1, $2) ON CONFLICT(tip) DO UPDATE SET height=$2",
        "tip",
        &height,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn load_last_height(pool: &PgPool) -> Result<i64, Error> {
    let lh = sqlx::query_as!(LastHeight, "SELECT * FROM result_last_height")
        .fetch_one(pool)
        .await?;

    Ok(lh.height)
}