use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct UserAssets {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub available: Decimal,
    pub locked: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
