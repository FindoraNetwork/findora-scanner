use module::schema::PrismTxResult;
use sqlx::{Error, PgPool, Row};

pub use sqlx::Error as SqlxError;
pub use sqlx::PgPool as SqlxPgPool;

#[cfg(feature = "static-check")]
use module::schema::LastHeight;

pub async fn connect() -> Result<PgPool, Error> {
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var `DATABASE_URL` is required for the findora scanner.");
    PgPool::connect(&conn_str).await.map_err(Error::from)
}

#[cfg(not(feature = "static-check"))]
pub async fn save(res: Vec<PrismTxResult>, pool: &PgPool) -> Result<(), Error> {
    for tr in res {
        sqlx::query("INSERT INTO e2n VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,sender=$3,receiver=$4,asset=$5,amount=$6,decimal=$7,height=$8,timestamp=$9,value=$10")
            .bind(&tr.tx_hash)
            .bind(&tr.block_hash)
            .bind(&tr.sender)
            .bind(&tr.receiver)
            .bind(&tr.asset)
            .bind(&tr.amount)
            .bind(tr.decimal)
            .bind(tr.height)
            .bind(tr.timestamp)
            .bind(&tr.value)
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(feature = "static-check")]
pub async fn save(res: Vec<PrismTxResult>, pool: &PgPool) -> Result<(), Error> {
    for tr in res {
        sqlx::query!(
                "INSERT INTO e2n VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,sender=$3,receiver=$4,asset=$5,amount=$6,decimal=$7,height=$8,timestamp=$9,value=$10",
                &tr.tx_hash, &tr.block_hash, &tr.sender, &tr.receiver, &tr.asset, &tr.amount, &tr.decimal, &tr.height, &tr.timestamp, &tr.value
            )
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(not(feature = "static-check"))]
pub async fn save_last_height(height: i64, pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO e2n_last_height VALUES($1, $2) ON CONFLICT(tip) DO UPDATE SET height=$2",
    )
    .bind("tip")
    .bind(height)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(feature = "static-check")]
pub async fn save_last_height(height: i64, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO e2n_last_height VALUES($1, $2) ON CONFLICT(tip) DO UPDATE SET height=$2",
        "tip",
        &height,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(not(feature = "static-check"))]
pub async fn load_last_height(pool: &PgPool) -> Result<i64, Error> {
    let row = sqlx::query("SELECT * FROM e2n_last_height")
        .fetch_one(pool)
        .await?;
    row.try_get("height")
}

#[cfg(feature = "static-check")]
pub async fn load_last_height(pool: &PgPool) -> Result<i64, Error> {
    let lh = sqlx::query_as!(LastHeight, "SELECT * FROM e2n_last_height")
        .fetch_one(pool)
        .await?;

    Ok(lh.height)
}
