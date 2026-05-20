use actix_web::{HttpResponse, Responder};

pub async fn signin_handler() -> impl Responder {
    HttpResponse::Ok().body("signin_handler")
}

pub async fn signup_handler() -> impl Responder {
    HttpResponse::Ok().body("signup_handler")
}
