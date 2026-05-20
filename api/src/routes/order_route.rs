use actix_web::{HttpResponse, Responder};

pub async fn create_order_handler() -> impl Responder {
    HttpResponse::Ok().body("create_order_handler")
}

pub async fn delete_order_by_id_handler() -> impl Responder {
    HttpResponse::Ok().body("delete_order_handler")
}
