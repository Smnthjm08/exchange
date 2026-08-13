use rust_decimal::Decimal;
use uuid::Uuid;

pub struct Fill {
    pub market: String,
    pub maker: Uuid,
    pub taker: Uuid,
    pub long: Uuid,
    pub short: Uuid,
    pub qty: Decimal,
    pub price: Decimal,
}
