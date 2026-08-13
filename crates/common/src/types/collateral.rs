use rust_decimal::Decimal;

pub struct Collateral {
    pub available: Decimal,
    pub locked: Decimal,
}
