use module::schema::Block as ModuleBlock;
use serde_json::Value;
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
pub async fn save(block: ModuleBlock, pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
            "INSERT INTO block VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT(block_hash) DO UPDATE SET block_hash=$1, size=$3, tx_count=$4, time=$5, app_hash=$6, proposer=$7, block_data=$8")
            .bind(&block.block_hash)
            .bind(block.height)
            .bind(block.size)
            .bind(block.tx_count)
            .bind(block.timestamp)
            .bind(&block.app_hash)
            .bind(&block.proposer)
            .bind(&block.block_data)
            .execute(pool)
            .await?;

    for tx in block.txs {
        sqlx::query(
            "INSERT INTO transaction VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,height=$3,timestamp=$4,code=$5,ty=$6,ty_sub=$7,sender=$8,receiver=$9,log=$10,origin=$11,result=$12,value=$13")
            .bind(&tx.tx_hash)
            .bind(&tx.block_hash)
            .bind(tx.height)
            .bind(tx.timestamp)
            .bind(tx.code)
            .bind(tx.ty)
            .bind(tx.ty_sub)
            .bind(&tx.sender)
            .bind(&tx.receiver)
            .bind(&tx.log)
            .bind(&tx.origin)
            .bind(&tx.result)
            .bind(&tx.value)
            .execute(pool)
            .await?;
    }

    for tx in block.evm_txs {
        sqlx::query(
            "INSERT INTO transaction VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,height=$3,timestamp=$4,code=$5,ty=$6,ty_sub=$7,sender=$8,receiver=$9,log=$10,origin=$11,result=$12,value=$13")
            .bind(&tx.tx_hash)
            .bind(&tx.block_hash)
            .bind(tx.height)
            .bind(tx.timestamp)
            .bind(tx.code)
            .bind(tx.ty)
            .bind(tx.ty_sub)
            .bind(&tx.sender)
            .bind(&tx.receiver)
            .bind(&tx.log)
            .bind(&tx.origin)
            .bind(&tx.result)
            .bind(&tx.value)
            .execute(pool)
            .await?;
    }

    for tx in block.v2_convert_account_txs {
        save_n2e_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            &tx.receiver,
            &tx.asset,
            &tx.amount,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_delegation_txs {
        save_delegation_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            &tx.validator,
            &tx.new_validator,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_undelegation_txs {
        save_undelegation_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            &tx.target_validator,
            &tx.new_delegator,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_claim_txs {
        save_claim_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_asset_txs {
        save_asset_tx(
            &tx.asset,
            &tx.tx_hash,
            &tx.block_hash,
            &tx.issuer,
            tx.height,
            tx.timestamp,
            tx.issued,
            &tx.content,
            pool,
        )
        .await?;
    }

    for addr in block.evm_addrs {
        sqlx::query("INSERT INTO evm_addrs(tx,address,timestamp) VALUES ($1,$2,$3)")
            .bind(&addr.tx)
            .bind(&addr.address)
            .bind(addr.timestamp)
            .execute(pool)
            .await?;
    }
    for addr in block.native_addrs {
        sqlx::query("INSERT INTO native_addrs(tx,address,timestamp) VALUES ($1,$2,$3)")
            .bind(&addr.tx)
            .bind(&addr.address)
            .bind(addr.timestamp)
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
                .bind(block.height)
                .bind(&v.address)
                .bind(power)
                .bind(v.priority)
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
            "INSERT INTO block VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT(height) DO UPDATE SET block_hash=$1, size=$3, tx_count=$4, time=$5, app_hash=$6, proposer=$7, block_data=$8",
                &block.block_hash,
                &block.height,
                &block.size,
                &block.tx_count,
                &block.timestamp,
                &block.app_hash,
                &block.proposer,
                &block.block_data,
        )
        .execute(pool)
        .await?;

    for tx in block.txs {
        sqlx::query!(
                "INSERT INTO transaction VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,height=$3,timestamp=$4,code=$5,ty=$6,ty_sub=$7,sender=$8,receiver=$9,log=$10,origin=$11,result=$12,value=$13",
                &tx.tx_hash, &tx.block_hash, &tx.height, &tx.timestamp, &tx.code, &tx.ty, &tx.ty_sub, &tx.sender,&tx.receiver,&tx.log, &tx.origin, &tx.result, &tx.value
            )
            .execute(pool)
            .await?;
    }

    for tx in block.evm_txs {
        sqlx::query!(
                "INSERT INTO transaction VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) ON CONFLICT(tx_hash) DO UPDATE SET tx_hash=$1,block_hash=$2,height=$3,timestamp=$4,code=$5,ty=$6,ty_sub=$7,sender=$8,receiver=$9,log=$10,origin=$11,result=$12,value=$13",
                 &tx.tx_hash, &tx.block_hash, &tx.height, &tx.timestamp, &tx.code, &tx.ty, &tx.ty_sub, &tx.sender,&tx.receiver,&tx.log, &tx.origin, &tx.result, &tx.value
            )
            .execute(pool)
            .await?;
    }

    for tx in block.v2_convert_account_txs {
        save_n2e_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            &tx.receiver,
            &tx.asset,
            &tx.amount,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_delegation_txs {
        save_delegation_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            &tx.validator,
            &tx.new_validator,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_undelegation_txs {
        save_undelegation_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            &tx.target_validator,
            &tx.new_delegator,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_claim_txs {
        save_claim_tx(
            &tx.tx_hash,
            &tx.block_hash,
            &tx.sender,
            tx.amount,
            tx.height,
            tx.timestamp,
            &tx.content,
            pool,
        )
        .await?;
    }

    for tx in block.v2_asset_txs {
        save_asset_tx(
            &tx.asset,
            &tx.tx_hash,
            &tx.block_hash,
            &tx.issuer,
            tx.height,
            tx.timestamp,
            tx.issued,
            &tx.content,
            pool,
        )
        .await?;
    }
    for addr in block.evm_addrs {
        sqlx::query("INSERT INTO evm_addrs(tx,address,timestamp) VALUES ($1,$2,$3)")
            .bind(&addr.tx)
            .bind(&addr.address)
            .bind(addr.timestamp)
            .execute(pool)
            .await?;
    }
    for addr in block.native_addrs {
        sqlx::query("INSERT INTO native_addrs(tx,address,timestamp) VALUES ($1,$2,$3)")
            .bind(&addr.tx)
            .bind(&addr.address)
            .bind(addr.timestamp)
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
        .bind(height)
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// migrate
////////////////////////////////////////////////////////////////////////////////////////////////////
#[allow(clippy::too_many_arguments)]
pub async fn save_evm_tx(
    tx: &str,
    block: &str,
    evm_tx_hash: &str,
    sender: &str,
    receiver: &str,
    amount: &str,
    height: i64,
    timestamp: i64,
    content: Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO evm_txs VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,evm_tx=$3,sender=$4,receiver=$5,amount=$6,height=$7,timestamp=$8,content=$9")
        .bind(tx)
        .bind(block)
        .bind(evm_tx_hash)
        .bind(sender)
        .bind(receiver)
        .bind(amount)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn save_tx_type(tx: &str, ty: i32, pool: &PgPool) -> Result<(), Error> {
    sqlx::query("INSERT INTO tx_types VALUES($1,$2) ON CONFLICT(tx) DO UPDATE SET tx=$1,ty=$2")
        .bind(tx)
        .bind(ty)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_n2e_tx(
    tx: &str,
    block: &str,
    sender: &str,
    receiver: &str,
    asset: &str,
    amount: &str,
    height: i64,
    timestamp: i64,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO n2e VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,sender=$3,receiver=$4,asset=$5,amount=$6,height=$7,timestamp=$8,content=$9")
        .bind(tx)
        .bind(block)
        .bind(sender)
        .bind(receiver)
        .bind(asset)
        .bind(amount)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_native_tx(
    tx: &str,
    block: &str,
    address: &str,
    height: i64,
    timestamp: i64,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO native_txs VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,address=$3,height=$4,timestamp=$5,content=$6")
        .bind(tx)
        .bind(block)
        .bind(address)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_delegation_tx(
    tx: &str,
    block: &str,
    sender: &str,
    amount: i64,
    validator: &str,
    new_validator: &str,
    timestamp: i64,
    height: i64,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO delegations VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,sender=$3,amount=$4,validator=$5,new_validator=$6,height=$7,timestamp=$8,content=$9")
        .bind(tx)
        .bind(block)
        .bind(sender)
        .bind(amount)
        .bind(validator)
        .bind(new_validator)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_undelegation_tx(
    tx: &str,
    block: &str,
    sender: &str,
    amount: i64,
    target_validator: &str,
    new_delegator: &str,
    height: i64,
    timestamp: i64,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO undelegations VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,sender=$3,amount=$4,target_validator=$5,new_delegator=$6,height=$7,timestamp=$8,content=$9")
        .bind(tx)
        .bind(block)
        .bind(sender)
        .bind(amount)
        .bind(target_validator)
        .bind(new_delegator)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_claim_tx(
    tx: &str,
    block: &str,
    sender: &str,
    amount: i64,
    height: i64,
    timestamp: i64,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO claims VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(tx) DO UPDATE SET tx=$1,block=$2,sender=$3,amount=$4,height=$5,timestamp=$6,content=$7")
        .bind(tx)
        .bind(block)
        .bind(sender)
        .bind(amount)
        .bind(height)
        .bind(timestamp)
        .bind(content)
        .execute(pool)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn save_asset_tx(
    asset: &str,
    tx: &str,
    block: &str,
    issuer: &str,
    height: i64,
    timestamp: i64,
    ty: i32,
    content: &Value,
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO assets VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT(asset,tx,ty) DO UPDATE SET asset=$1,tx=$2,block=$3,issuer=$4,height=$5,timestamp=$6,ty=$7,content=$8")
        .bind(asset)
        .bind(tx)
        .bind(block)
        .bind(issuer)
        .bind(height)
        .bind(timestamp)
        .bind(ty)
        .bind(content)
        .execute(pool)
        .await?;

    Ok(())
}
