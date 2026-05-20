use actix_web::{HttpResponse, Responder};

pub async fn onramp_fund_handler() -> impl Responder {
    HttpResponse::Ok().body("onramp_fund_handler")
}