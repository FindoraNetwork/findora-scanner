use module::schema::{Block as ModuleBlock, DelegationInfo};
use sqlx::{Error, PgPool, Row};

pub use sqlx::PgPool as SqlxPgPool;
pub use sqlx::Error as SqlxError;

#[cfg(feature = "static-check")]
use module::schema::LastHeight;

pub async fn connect() -> Result<PgPool, Error> {
    let conn_str = std::env::var("DATABASE_URL")
        .expect("Env var `DATABASE_URL` is required for the findora scanner.");
    PgPool::connect(&conn_str).await.map_err(Error::from)
}

#[cfg(not(feature = "static-check"))]
pub async fn save(block: ModuleBlock, pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
            "INSERT INTO block VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT(height) DO UPDATE SET block_id=$1, size=$3, time=$4, app_hash=$5, proposer=$6")
            .bind(&block.block_id)
            .bind(&block.height)
            .bind(&block.size)
            .bind(&block.timestamp)
            .bind(&block.app_hash)
            .bind(&block.proposer)
        .execute(pool)
        .await?;

    for tx in block.txs {
        sqlx::query(
                "INSERT INTO transaction VALUES ($1, $2, 0, $3, $4, $5, $6) ON CONFLICT(txid) DO UPDATE SET ty=0, block_id=$2, timestamp=$3, value=$4, code=$5, log=$6")
            .bind(&tx.txid)
            .bind(&tx.block_id)
            .bind(&tx.timestamp)
            .bind(&tx.value)
            .bind(&tx.code)
            .bind(&tx.log)
            .execute(pool)
            .await?;
    }

    for tx in block.evm_txs {
        sqlx::query(
                "INSERT INTO transaction VALUES ($1, $2, 1, $3, $4, $5, $6) ON CONFLICT(txid) DO UPDATE SET ty=1, block_id=$2, timestamp=$3, value=$4, code=$5, log=$6")
                .bind(&tx.txid)
                .bind(&tx.block_id)
                .bind(&tx.timestamp)
                .bind(&tx.value)
                .bind(&tx.code)
                .bind(&tx.log)
            .execute(pool)
            .await?;
    }

    for v in block.validators {
        sqlx::query(
                "INSERT INTO validators VALUES ($1, 0, $2) ON CONFLICT(address) DO UPDATE SET pubkey_type=0, pubkey=$2")
                .bind(&v.address)
                .bind(&v.pub_key.value)
        .execute(pool)
            .await?;

        let power = v.power as i64;

        let _ = sqlx::query(
                "INSERT INTO block_generation VALUES($1, $2, $3, $4, $5, $6) ON CONFLICT(height, address) DO UPDATE SET power=$3, priority=$4, signature=$5, time=$6")
                .bind(&block.height)
                .bind(&v.address)
                .bind(&power)
                .bind(&v.priority)
                .bind(v.signature.as_ref())
                .bind(v.timestamp.as_ref())
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(feature = "static-check")]
pub async fn save(block: ModuleBlock, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
            "INSERT INTO block VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT(height) DO UPDATE SET block_id=$1, size=$3, time=$4, app_hash=$5, proposer=$6",
                &block.block_id,
                &block.height,
                &block.size,
                &block.timestamp,
                &block.app_hash,
                &block.proposer,
        )
        .execute(pool)
        .await?;

    for tx in block.txs {
        sqlx::query!(
                "INSERT INTO transaction VALUES ($1, $2, 0, $3, $4, $5, $6) ON CONFLICT(txid) DO UPDATE SET ty=0, block_id=$2, time=$3, value=$4, code=$5, log=$6",
                &tx.txid, &tx.block_id, &tx.time, &tx.value, &tx.code, &tx.log
            )
            .execute(pool)
            .await?;
    }

    for tx in block.evm_txs {
        sqlx::query!(
                "INSERT INTO transaction VALUES ($1, $2, 1, $3, $4, $5, $6) ON CONFLICT(txid) DO UPDATE SET ty=1, block_id=$2, time=$3, value=$4, code=$5, log=$6",
                &tx.txid, &tx.block_id, &tx.time, &tx.value, &tx.code, &tx.log
            )
            .execute(pool)
            .await?;
    }

    for v in block.validators {
        sqlx::query!(
                "INSERT INTO validators VALUES ($1, 0, $2) ON CONFLICT(address) DO UPDATE SET pubkey_type=0, pubkey=$2",
                &v.address, &v.pub_key.value
        ).execute(pool)
            .await?;

        let power: i64 = v.power.try_into()?;

        let _ = sqlx::query!(
                "INSERT INTO block_generation VALUES($1, $2, $3, $4, $5, $6) ON CONFLICT(height, address) DO UPDATE SET power=$3, priority=$4, signature=$5, time=$6",
                    &block.height,
                    &v.address,
                    &power,
                    &v.priority,
                    v.signature.as_ref(),
                    v.timestamp.as_ref()
            ).execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(not(feature = "static-check"))]
pub async fn save_last_height(height: i64, pool: &PgPool) -> Result<(), Error> {
    sqlx::query("INSERT INTO last_height VALUES($1, $2) ON CONFLICT(tip) DO UPDATE SET height=$2")
        .bind("tip")
        .bind(&height)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(feature = "static-check")]
pub async fn save_last_height(height: i64, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO last_height VALUES($1, $2) ON CONFLICT(tip) DO UPDATE SET height=$2",
        "tip",
        &height,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(not(feature = "static-check"))]
pub async fn load_last_height(pool: &PgPool) -> Result<i64, Error> {
    let row = sqlx::query("SELECT * FROM last_height")
        .fetch_one(pool)
        .await?;
    row.try_get("height")
}

#[cfg(feature = "static-check")]
pub async fn load_last_height(pool: &PgPool) -> Result<i64, Error> {
    let lh = sqlx::query_as!(LastHeight, "SELECT * FROM last_height")
        .fetch_one(pool)
        .await?;

    Ok(lh.height)
}

pub async fn save_delegations(h: i64, info: &DelegationInfo, pool: &PgPool) -> Result<(), Error> {
    let info = serde_json::to_value(info).unwrap();
    sqlx::query(
        "INSERT INTO delegations VALUES($1, $2) ON CONFLICT(height) DO UPDATE SET info=$2;",
    )
    .bind(&h)
    .bind(&info)
    .execute(pool)
    .await?;
    Ok(())
}
