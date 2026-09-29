use common::types::Market;
use sqlx::PgPool;

pub async fn get_market_by_symbol(pool: &PgPool, symbol: &str) -> Result<Option<Market>, sqlx::Error> {
    sqlx::query_as!(
        Market,
        r#"SELECT id, symbol, base_asset_id, quote_asset_id, tick_size, min_qty, status
           FROM markets WHERE symbol = $1"#,
        symbol
    )
    .fetch_optional(pool)
    .await
}
