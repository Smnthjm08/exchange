use actix_web::{HttpResponse, Responder};

pub async fn login_handler() -> impl Responder {
    HttpResponse::Ok().body("login_handler")
}

pub async fn signup_handler() -> impl Responder {
    HttpResponse::Ok().body("signup_handler")
}
