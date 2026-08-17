use axum::{
    Router,
    response::Json,
    routing::{get, post},
};
use db::init_db;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = init_db(&database_url).await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;

    println!("Database ready");

    // build our application with a route
    let api = Router::new()
        .route("/health", get(get_health))
        .route("/auth", post(login));

    let app = Router::new().route("/", get(root)).nest("/api/v1", api);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy!".to_string(),
    })
}

#[derive(Serialize)]
struct AuthData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    message: String,
    success: bool,
    data: AuthData,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(Json(payload): Json<LoginRequest>) -> Json<AuthResponse> {
    Json(AuthResponse {
        message: "req".to_string(),
        success: true,
        data: AuthData {
            username: payload.username.to_string(),
            password: payload.password.to_string(),
        },
    })
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
