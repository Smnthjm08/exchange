use actix_web::{HttpResponse, Responder};

pub async fn get_open_positions() -> impl Responder {
    HttpResponse::Ok().body("get_open_positions")
}

pub async fn get_closed_postions() -> impl Responder {
    HttpResponse::Ok().body("get_closed_postions")
}
