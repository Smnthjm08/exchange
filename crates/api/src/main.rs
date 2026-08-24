use axum::{
    Router,
    routing::{get, post},
};
use db::init_db;
use sqlx::PgPool;
pub mod routes;
pub mod utils;
pub mod middlewares;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = init_db(&database_url).await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;

    println!("Database ready");

    let state = AppState { db: pool };

    // build our application with a route
    let api = Router::new()
        .route("/health", get(routes::health_routes::get_health))
        .route("/auth/login", post(routes::auth_routes::login_request))
        .route("/auth/signup", post(routes::auth_routes::signup_request))
        .route("/user/profile", get(routes::user_routes::get_user_profile))
        .route("/user/balances", get(routes::user_routes::get_user_balances))
        // In this app, onramp should mean: add fiat or crypto into the user’s exchange account, then credit their internal balance after payment confirmation.
        .route("/onramp", get(routes::user_routes::get_user_balances))
        .with_state(state);

    let app = Router::new().route("/", get(root)).nest("/api/v1", api);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("app running");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "exchange api"
}
