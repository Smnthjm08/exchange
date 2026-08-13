use rust_decimal::Decimal;
use uuid::Uuid;

use crate::types::types::Side;

pub struct Position {
    pub market: String,
    pub side: Side,
    pub qty: Decimal,
    pub margin: Decimal,
    pub average_price: Decimal,
    pub liquidation_price: Decimal,
    pub pnl: Decimal,
    pub user_id: Uuid,
}
