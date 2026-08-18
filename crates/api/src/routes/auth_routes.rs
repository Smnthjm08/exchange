use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct LoginRequestPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupPayload {
    username: String,
    email: Option<String>,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    success: bool,
    data: LoginRequestPayload,
}

#[derive(Serialize)]
pub struct SignupResponse {
    message: String,
    success: bool,
    data: SignupPayload,
}

pub async fn login_request(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequestPayload>,
) -> Json<LoginResponse> {
    Json(LoginResponse {
        message: "login request".to_string(),
        success: true,
        data: LoginRequestPayload {
            username: payload.username,
            password: payload.password,
        },
    })
}

pub async fn signin_request(
    State(app_state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> Json<SignupResponse> {
    Json(SignupResponse {
        message: "signup request".to_string(),
        success: true,
        data: SignupPayload {
            username: payload.username,
            password: payload.password,
            email: payload.email,
        },
    })
}
