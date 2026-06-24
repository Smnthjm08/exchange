use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{utils, AppState};
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub name: String,
    pub email: String,
    pub updated_at: String,
    pub created_at: String,
    pub access_token: String,
}

pub async fn login_handler(
    app_data: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = &app_data.db;

    let user = sqlx::query!(
        r#"
        SELECT id, email, password, name, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        req.email
    )
    .fetch_optional(pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    match user {
        Some(user) => {
            if user.password == req.password {
                println!("Found user {:?}", user.id);

                let access_token = utils::encode_jwt(user.email.clone(), user.id)
                    .map_err(error::ErrorInternalServerError)?;

                Ok(HttpResponse::Ok().json(AuthResponse {
                    name: user.name,
                    email: user.email,
                    created_at: user.created_at.to_string(),
                    updated_at: user.updated_at.to_string(),
                    access_token: access_token,
                }))
            } else {
                Ok(HttpResponse::Unauthorized().body("Invalid password"))
            }
        }
        None => Ok(HttpResponse::Unauthorized().body("User not found")),
    }
}

pub async fn signup_handler(
    app_data: web::Data<AppState>,
    req: web::Json<SignUpRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = &app_data.db;

    let user = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE email = $1
        "#,
        req.email
    )
    .fetch_optional(pool)
    .await
    .map_err(error::ErrorInternalServerError)?;

    match user {
        Some(_) => Ok(HttpResponse::BadRequest().body("User already exists with this email")),

        None => {
            let user_id = uuid::Uuid::new_v4();

            let user = sqlx::query!(
                r#"
                INSERT INTO users (
                    id,
                    name,
                    email,
                    password
                )
                VALUES ($1, $2, $3, $4)
                RETURNING created_at, updated_at
                "#,
                user_id,
                req.name,
                req.email,
                req.password
            )
            .fetch_one(pool)
            .await
            .map_err(error::ErrorInternalServerError)?;

            let access_token = utils::encode_jwt(req.email.clone(), user_id)
                .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(AuthResponse {
                name: req.name.clone(),
                email: req.email.clone(),
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
                access_token: access_token,
            }))
        }
    }
}
