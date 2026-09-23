use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::Decimal;
use uuid::Uuid;

use crate::{middlewares::auth_middlewares::AuthUser, AppState};

#[derive(Serialize)]
pub struct UserProfileResponse {
    message: String,
    success: bool,
    data: UserProfileData,
}

#[derive(Serialize)]
pub struct UserProfileData {
    id: Uuid,
    username: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn get_user_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let user = db::users::get_user_by_id(&state.db, &auth_user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "user not found".to_string()))?;

    Ok(Json(UserProfileResponse {
        message: "user profile fetched successfully".to_string(),
        success: true,
        data: UserProfileData {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        },
    }))
}

#[derive(Serialize)]
pub struct UserAssetsData {
    pub asset_id: Uuid,
    pub available: Decimal,
    pub locked: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UserAssetsResponse {
    message: String,
    success: bool,
    data: Vec<UserAssetsData>,
}

pub async fn get_user_assets(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserAssetsResponse>, (StatusCode, String)> {
    let balances = db::user_assets::get_user_assets_by_user_id(&state.db, &auth_user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UserAssetsResponse {
        message: "successfully fetched user balances".to_string(),
        success: true,
        data: balances
            .into_iter()
            .map(|b| UserAssetsData {
                asset_id: b.asset_id,
                available: b.available,
                locked: b.locked,
                updated_at: b.updated_at,
            })
            .collect(),
    }))
}
