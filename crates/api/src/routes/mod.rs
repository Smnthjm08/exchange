pub mod auth_routes;
pub mod health_routes;
pub mod user_routes;

pub use auth_routes::{login_request,signup_request};
pub use health_routes::get_health;
pub use user_routes::{get_user_profile, get_user_balances};