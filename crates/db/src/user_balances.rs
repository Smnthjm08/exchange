use common::types::UserBalances;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_balances_by_user_id(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Option<UserBalances>, sqlx::Error> {
    let row = sqlx::query!(r#"SELECT id, user_id, available, locked, created_at, updated_at FROM user_balances WHERE user_id = $1"#, user_id).fetch_optional(pool).await?;

    Ok(row.map(|r| UserBalances {
        id: r.id,
        user_id: r.user_id,
        available: r.available,
        locked: r.locked,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}
