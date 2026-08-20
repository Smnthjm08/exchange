use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use uuid::Uuid;

use crate::{AppState, middlewares::auth_middlewares::AuthUser};


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