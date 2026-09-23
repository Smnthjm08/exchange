use sqlx::{PgPool, postgres::PgPoolOptions};
pub mod users;
pub mod user_assets;
pub mod deposit;

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
