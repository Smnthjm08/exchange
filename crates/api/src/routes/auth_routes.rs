use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{utils::encode_jwt, AppState};

#[derive(Serialize, Deserialize)]
pub struct LoginRequestPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    success: bool,
    data: LoginData,
}

#[derive(Serialize)]
pub struct LoginData {
    access_token: String,
    id: Uuid,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupRequestPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    message: String,
    success: bool,
    data: SignupData,
}

#[derive(Serialize)]
pub struct SignupData {
    access_token: String,
    id: Uuid,
    username: String,
    email: String,
}

pub async fn login_request(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequestPayload>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = db::users::get_user_by_email(&app_state.db, &payload.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "invalid username or password".to_string(),
        ))?;

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "stored password hash is invalid".to_string(),
        )
    })?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "invalid email or password".to_string(),
            )
        })?;

    let access_token = encode_jwt(user.email.clone(), user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate access token".to_string(),
        )
    })?;

    Ok(Json(LoginResponse {
        message: "logged in successfully".to_string(),
        success: true,
        data: LoginData {
            access_token,
            id: user.id,
            username: user.username,
            email: user.email,
        },
    }))
}

pub async fn signup_request(
    State(app_state): State<AppState>,
    Json(payload): Json<SignupRequestPayload>,
) -> Result<(StatusCode, Json<SignupResponse>), (StatusCode, String)> {
    let existing_user = db::users::get_user_by_email(&app_state.db, &payload.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, "user already exists".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to hash password".to_string(),
            )
        })?
        .to_string();

    let user = db::users::create_user(
        &app_state.db,
        &payload.username,
        &payload.email,
        &password_hash,
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error creating user".to_string(),
        )
    })?;

    let access_token = encode_jwt(user.email.clone(), user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to generate access token".to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(SignupResponse {
            message: "user created successfully".to_string(),
            success: true,
            data: SignupData {
                access_token,
                id: user.id,
                username: user.username,
                email: user.email,
            },
        }),
    ))
}
