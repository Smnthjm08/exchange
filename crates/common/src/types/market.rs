use rust_decimal::Decimal;
use uuid::Uuid;

pub struct Market {
    pub id: Uuid,
    pub symbol: String,
    pub base_asset_id: Uuid,
    pub quote_asset_id: Uuid,
    pub tick_size: Decimal,
    pub min_qty: Decimal,
    pub status: String,
}
