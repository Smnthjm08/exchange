use axum::{
    extract::FromRequestParts,
    http::{header, StatusCode},
};
use uuid::Uuid;

use crate::{AppState, utils::decode_jwt};

pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "missing authorization header".to_string(),
            ))?;

            let data = decode_jwt(token.to_string()).map_err(|_| {
                (StatusCode::UNAUTHORIZED, "invalid token".to_string())
            })?;

            Ok(AuthUser { id: data.claims.id, email: data.claims.email })
    }
}
