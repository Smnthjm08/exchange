use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use common::types::{OrderStatus, OrderType, Side};
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use uuid::Uuid;

use crate::{middlewares::auth_middlewares::AuthUser, AppState};

#[derive(Serialize)]
pub struct OrderData {
    id: Uuid,
    user_id: Uuid,
    market_symbol: String,
    side: Side,
    r#type: OrderType,
    price: Decimal,
    qty: Decimal,
    filled_qty: Decimal,
    status: OrderStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateOrderPayload {
    market_symbol: String,
    side: Side,
    r#type: OrderType,
    price: Decimal,
    qty: Decimal,
}

#[derive(Serialize)]
pub struct OrderResponse {
    message: String,
    success: bool,
    data: OrderData,
}

pub async fn create_orders(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateOrderPayload>,
) -> Result<Json<OrderResponse>, (StatusCode, String)> {
    let bad_request = |msg: &str| (StatusCode::BAD_REQUEST, msg.to_string());

    // Market orders have no price to size the lock from; they arrive with the engine (M4).
    if matches!(payload.r#type, OrderType::Market) {
        return Err(bad_request("only limit orders are supported"));
    }
    // Guards lock_balance: a negative amount would pass `available >= $3` and move funds backwards.
    if payload.price <= Decimal::ZERO || payload.qty <= Decimal::ZERO {
        return Err(bad_request("price and qty must be positive"));
    }

    let market = db::markets::get_market_by_symbol(&app_state.db, &payload.market_symbol)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "unknown market".to_string()))?;

    if market.status != "active" {
        return Err(bad_request("market is not active"));
    }
    if !(payload.price % market.tick_size).is_zero() {
        return Err(bad_request("price must be a multiple of the tick size"));
    }
    if payload.qty < market.min_qty {
        return Err(bad_request("qty is below the market minimum"));
    }

    // Buy locks quote (price * qty), sell locks base (qty).
    let (lock_asset_id, lock_amount) = match payload.side {
        Side::Buy => (
            market.quote_asset_id,
            payload
                .price
                .checked_mul(payload.qty)
                .ok_or_else(|| bad_request("order value too large"))?,
        ),
        Side::Sell => (market.base_asset_id, payload.qty),
    };

    let mut tx = app_state
        .db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Returning early drops `tx`, which rolls it back.
    let locked = db::user_assets::lock_balance(&mut tx, &auth_user.id, &lock_asset_id, lock_amount)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if !locked {
        return Err(bad_request("insufficient balance"));
    }

    let order: common::types::Order = db::orders::create_order(
        &mut tx,
        auth_user.id,
        market.id,
        payload.side,
        payload.r#type,
        payload.price,
        payload.qty,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(OrderResponse {
        message: "order placed".to_string(),
        success: true,
        data: OrderData {
            id: order.id,
            user_id: order.user_id,
            market_symbol: market.symbol,
            side: order.side,
            r#type: order.order_type,
            price: order.price,
            qty: order.qty,
            filled_qty: order.filled_qty,
            status: order.status,
            created_at: order.created_at,
            updated_at: order.updated_at,
        },
    }))
}

pub async fn cancel_orders(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<Uuid>,
)-> Result<Json<OrderResponse>, (StatusCode, String)> {
    todo!()
}
