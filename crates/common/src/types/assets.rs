// CREATE TABLE IF NOT EXISTS assets (
//     id UUID PRIMARY KEY,
//     name TEXT NOT NULL,
//     symbol VARCHAR(10) NOT NULL,
//     decimals INTEGER NOT NULL,
//     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
//     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
//     UNIQUE (symbol)
// );

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Assets{
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
