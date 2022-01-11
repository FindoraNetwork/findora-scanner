use tokio_postgres::Client;

use crate::{block::Block, Result};

pub async fn save(block: Block, client: &Client) -> Result<()> {
    let _ = client
        .execute(
            "INSERT INTO block VALUES ($1, $2, $3, $4, $5)",
            &[
                &block.block_id,
                &block.height,
                &block.timestamp,
                &block.app_hash,
                &block.proposer,
            ],
        )
        .await?;

    for tx in block.txs {
        let _ = client
            .execute(
                "INSERT INTO transaction VALUES ($1, 0, $2, $3, $4)",
                &[&tx.txid, &tx.value, &tx.code, &tx.log],
            )
            .await?;
    }

    for tx in block.evm_txs {
        let _ = client
            .execute(
                "INSERT INTO transaction VALUES ($1, 1, $2, $3, $4)",
                &[&tx.txid, &tx.value, &tx.code, &tx.log],
            )
            .await?;
    }

    for v in block.validators {
        let _ = client
            .execute(
                "INSERT INTO validators VALUES ($1, 0, $2)",
                &[&v.address, &v.pub_key.value],
            )
            .await?;

        let power: i64 = v.power.try_into()?;

        let _ = client
            .execute(
                "INSERT INTO block_generation VALUES($1, $2, $3, $4, $5, $6)",
                &[
                    &block.height,
                    &v.address,
                    &power,
                    &v.priority,
                    &v.signature,
                    &v.timestamp,
                ],
            )
            .await?;
    }

    Ok(())
}
