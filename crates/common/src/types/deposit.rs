use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub enum DepositStatus{
    Pending,
    Confirmed,
    Failed
}

pub struct Deposit{
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Decimal,
    pub status: DepositStatus,
    pub external_ref: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}