use common::types::UserAssets;
use sqlx::PgPool;
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
