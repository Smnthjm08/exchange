use actix_web::{HttpResponse, Responder};

pub async fn get_open_orders_handler() -> impl Responder {
    HttpResponse::Ok().body("get_open_orders_handler")
}

pub async fn get_orders_handler() -> impl Responder {
    HttpResponse::Ok().body("get_orders_handler")
}
