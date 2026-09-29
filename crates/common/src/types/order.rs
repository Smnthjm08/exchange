use crate::types::types::{OrderStatus, OrderType, Side};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub market_id: Uuid,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Decimal,
    pub qty: Decimal,
    pub filled_qty: Decimal,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
