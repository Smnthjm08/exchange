use axum::{
    routing::{get, post},
    Router,
};
use db::init_db;
use sqlx::PgPool;
pub mod middlewares;
pub mod routes;
pub mod utils;

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
    let mut api = Router::new()
        .route("/health", get(routes::health_routes::get_health))
        .route("/auth/login", post(routes::auth_routes::login_request))
        .route("/auth/signup", post(routes::auth_routes::signup_request))
        .route("/user/profile", get(routes::user_routes::get_user_profile))
        .route("/user/balances", get(routes::user_routes::get_user_assets));
    // CREATE and CANCEL ORDER

    if std::env::var("MOCK_DEPOSITS_ENABLED").is_ok() {
        api = api
            .route(
                "/deposit/mock",
                post(routes::deposit_routes::create_mock_deposit),
            )
            .route(
                "/deposit/mock/confirm",
                post(routes::deposit_routes::confirm_mock_deposit),
            );
    }

    let api = api.with_state(state);

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
