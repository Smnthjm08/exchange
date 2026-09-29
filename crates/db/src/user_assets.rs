use common::types::UserAssets;
use sqlx::{PgPool, types::Decimal};
use uuid::Uuid;

pub async fn get_user_assets_by_user_id(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<UserAssets>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT id, user_id, asset_id, available, locked, created_at, updated_at FROM user_assets WHERE user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| UserAssets {
            id: r.id,
            user_id: r.user_id,
            asset_id: r.asset_id,
            available: r.available,
            locked: r.locked,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}


pub async fn credit_available(
    tx: &mut sqlx::PgConnection,
    user_id: &Uuid, asset_id: &Uuid, amount: Decimal,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO user_assets (id, user_id, asset_id, available, locked)
           VALUES ($1, $2, $3, $4, 0)
           ON CONFLICT (user_id, asset_id)
           DO UPDATE SET available = user_assets.available + EXCLUDED.available, updated_at = NOW()"#,
        Uuid::new_v4(), user_id, asset_id, amount
    ).execute(tx).await?;
    Ok(())
}

// Check and lock in one statement: a separate SELECT-then-UPDATE lets two conc req
pub async fn lock_balance(
    tx: &mut sqlx::PgConnection,
    user_id: &Uuid, asset_id: &Uuid, amount: Decimal,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"UPDATE user_assets
           SET available = available - $3, locked = locked + $3, updated_at = NOW()
           WHERE user_id = $1 AND asset_id = $2 AND available >= $3"#,
        user_id, asset_id, amount
    ).execute(tx).await?;
    Ok(result.rows_affected() == 1)
}
