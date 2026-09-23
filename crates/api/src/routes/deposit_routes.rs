use axum::{extract::State, http::StatusCode, Json};
use common::types::DepositStatus;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use uuid::Uuid;

use crate::{middlewares::auth_middlewares::AuthUser, AppState};

#[derive(Deserialize)]
pub struct CreatePendingDepositRequestPayload {
    asset_symbol: String,
    amount: Decimal,
}

#[derive(Deserialize)]
pub struct ConfirmDepositRequestPayload {
    external_ref: String,
}

#[derive(Serialize)]
pub struct DepositData {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub amount: Decimal,
    pub status: String,
    pub external_ref: String,
}

#[derive(Serialize)]
pub struct DepositResponse {
    message: String,
    success: bool,
    data: DepositData,
}

fn status_str(status: &DepositStatus) -> &'static str {
    match status {
        DepositStatus::Pending => "pending",
        DepositStatus::Confirmed => "confirmed",
        DepositStatus::Failed => "failed",
    }
}

pub async fn create_mock_deposit(
    State(app_state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePendingDepositRequestPayload>,
) -> Result<Json<DepositResponse>, (StatusCode, String)> {
    let asset = db::deposit::get_asset_by_symbol(&app_state.db, &payload.asset_symbol)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "unknown asset".to_string()))?;

    let external_ref = Uuid::new_v4().to_string();

    let deposit = db::deposit::create_pending_deposit(
        &app_state.db,
        auth_user.id,
        asset.id,
        payload.amount,
        &external_ref,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DepositResponse {
        message: "deposit created".to_string(),
        success: true,
        data: DepositData {
            id: deposit.id,
            asset_id: deposit.asset_id,
            amount: deposit.amount,
            status: status_str(&deposit.status).to_string(),
            external_ref: deposit.external_ref,
        },
    }))
}

// Stands in for the provider's webhook call, so it isn't (and shouldn't be) behind AuthUser —
// a real provider wouldn't carry our JWT. Swap in webhook signature verification here
// when a real provider replaces this mock.
pub async fn confirm_mock_deposit(
    State(app_state): State<AppState>,
    Json(payload): Json<ConfirmDepositRequestPayload>,
) -> Result<Json<DepositResponse>, (StatusCode, String)> {
    let deposit = db::deposit::confirm_deposit(&app_state.db, &payload.external_ref)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                "unknown or unconfirmable deposit".to_string(),
            ),
            e => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })?;

    Ok(Json(DepositResponse {
        message: "deposit confirmed".to_string(),
        success: true,
        data: DepositData {
            id: deposit.id,
            asset_id: deposit.asset_id,
            amount: deposit.amount,
            status: status_str(&deposit.status).to_string(),
            external_ref: deposit.external_ref,
        },
    }))
}
