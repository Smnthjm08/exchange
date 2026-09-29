use common::types::{Order, OrderStatus, OrderType, Side};
use sqlx::types::Decimal;
use uuid::Uuid;

// Takes the caller's transaction so the insert commits together with `lock_balance`.
pub async fn create_order(
    tx: &mut sqlx::PgConnection,
    user_id: Uuid,
    market_id: Uuid,
    side: Side,
    order_type: OrderType,
    price: Decimal,
    qty: Decimal,
) -> Result<Order, sqlx::Error> {
    let id = Uuid::new_v4();

    let row = sqlx::query!(
        r#"INSERT INTO orders (id, user_id, market_id, side, type, price, qty)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING created_at, updated_at"#,
        id,
        user_id,
        market_id,
        side.as_str(),
        order_type.as_str(),
        price,
        qty
    )
    .fetch_one(tx)
    .await?;

    Ok(Order {
        id,
        user_id,
        market_id,
        side,
        order_type,
        price,
        qty,
        filled_qty: Decimal::ZERO,
        status: OrderStatus::Open,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}
