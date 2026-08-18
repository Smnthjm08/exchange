use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
pub async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy!".to_string(),
    })
}
