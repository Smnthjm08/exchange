use actix_web::{
    self, get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use db::init_db;
mod utils;
use dotenv::dotenv;
use sqlx::PgPool;

use crate::routes::{
    auth_route::{login_handler, signup_handler},
    equity_route::get_available_equity,
    onramp_route::onramp_fund_handler,
    order_route::{create_order_handler, delete_order_by_id_handler},
    orders_route::{get_open_orders_handler, get_orders_handler},
    positions_route::{get_closed_postions, get_open_positions},
};

mod routes;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("healthy!")
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env = std::env::var("ENV")?;
    let database_url = std::env::var("DATABASE_URL")?;
    println!("env: {}", env);

    let pool = init_db(&database_url).await?;

    sqlx::migrate!("../migrations").run(&pool).await?;

    let state = web::Data::new(AppState { db: pool });

    println!("Database ready");

    println!("🚀 Server started successfully!");

    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::scope("/api/v1")
                // health - done
                .service(health)
                // auth
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to(login_handler))
                        .route("/signup", web::post().to(signup_handler)),
                )
                // onramp
                .route("/onramp", web::post().to(onramp_fund_handler))
                // equity
                .service(
                    web::scope("/equity").route("/available", web::get().to(get_available_equity)),
                )
                // order
                .service(
                    web::scope("/order")
                        .route("/", web::post().to(create_order_handler))
                        .route("/{id}", web::delete().to(delete_order_by_id_handler)),
                )
                // postions
                .service(
                    web::scope("/postions")
                        .route("/open/{market_id}", web::get().to(get_open_positions))
                        .route("/closed/{market_id}", web::get().to(get_closed_postions)),
                )
                // orders
                .service(
                    web::scope("/orders")
                        .route("/open/{market_id}", web::post().to(get_open_orders_handler))
                        .route("/{market_id}", web::delete().to(get_orders_handler)),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
