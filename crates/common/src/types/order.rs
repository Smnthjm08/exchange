use crate::types::types::{OrderStatus, OrderType, Side};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct Order {
    pub order_id: Uuid,
    pub market: String,
    pub order_type: OrderType,
    pub side: Side,
    pub price: Decimal,
    pub qty: Decimal,
    pub margin: Decimal,
    pub status: OrderStatus,
    pub user_id: Uuid,
}
