use common::types::{Assets, Deposit, DepositStatus};
use sqlx::{types::Decimal, PgPool};
use uuid::Uuid;

pub async fn get_asset_by_symbol(
    pool: &PgPool,
    symbol: &str,
) -> Result<Option<Assets>, sqlx::Error> {
    let row = sqlx::query!(r#"SELECT id, name, symbol, decimals, created_at, updated_at FROM assets WHERE symbol = $1"#, symbol).fetch_optional(pool).await?;

    Ok(row.map(|r| Assets {
        id: r.id,
        name: r.name,
        symbol: r.symbol,
        decimals: r.decimals,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn create_pending_deposit(
    pool: &PgPool,
    user_id: Uuid,
    asset_id: Uuid,
    amount: Decimal,
    external_ref: &str,
) -> Result<Deposit, sqlx::Error> {
    let id = Uuid::new_v4();

    let row = sqlx::query!(
        r#"INSERT INTO deposits (id, user_id, asset_id, amount, status, external_ref)
           VALUES ($1, $2, $3, $4, 'pending', $5)
           RETURNING created_at, updated_at, external_ref"#,
        id,
        user_id,
        asset_id,
        amount,
        external_ref
    )
    .fetch_one(pool)
    .await?;

    Ok(Deposit {
        id,
        user_id,
        asset_id,
        amount,
        status: DepositStatus::Pending,
        external_ref: row.external_ref,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn confirm_deposit(pool: &PgPool, external_ref: &str) -> Result<Deposit, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let updated = sqlx::query!(
        r#"UPDATE deposits SET status='confirmed', updated_at=NOW()
           WHERE external_ref=$1 AND status='pending'
           RETURNING id, user_id, asset_id, amount, external_ref, created_at, updated_at"#,
        external_ref
    )
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(row) = updated {
        crate::user_assets::credit_available(&mut tx, &row.user_id, &row.asset_id, row.amount)
            .await?;
        tx.commit().await?;

        return Ok(Deposit {
            id: row.id,
            user_id: row.user_id,
            asset_id: row.asset_id,
            amount: row.amount,
            status: DepositStatus::Confirmed,
            external_ref: row.external_ref,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }

    let existing = sqlx::query!(
        r#"SELECT id, user_id, asset_id, amount, status, external_ref, created_at, updated_at
           FROM deposits WHERE external_ref=$1"#,
        external_ref
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    if existing.status != "confirmed" {
        return Err(sqlx::Error::RowNotFound);
    }

    tx.rollback().await?;

    Ok(Deposit {
        id: existing.id,
        user_id: existing.user_id,
        asset_id: existing.asset_id,
        amount: existing.amount,
        status: DepositStatus::Confirmed,
        external_ref: existing.external_ref,
        created_at: existing.created_at,
        updated_at: existing.updated_at,
    })
}

pub async fn get_user_deposits(pool: &PgPool, user_id: Uuid) -> Result<Vec<Deposit>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT id, user_id, asset_id, amount, status, external_ref, created_at, updated_at
           FROM deposits WHERE user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Deposit {
            id: r.id,
            user_id: r.user_id,
            asset_id: r.asset_id,
            amount: r.amount,
            status: match r.status.as_str() {
                "confirmed" => DepositStatus::Confirmed,
                _ => DepositStatus::Pending,
            },
            external_ref: r.external_ref,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}
