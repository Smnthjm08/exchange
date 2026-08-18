use common::types::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, username, password, email, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.id,
        username: r.username,
        password: r.password,
        email: r.email,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4();

    let row = sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, password, email, created_at, updated_at
        "#,
        id,
        username,
        password,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: row.id,
        username: row.username,
        password: row.password,
        email: row.email,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
