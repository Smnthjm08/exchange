use actix_web::{HttpResponse, Responder};

pub async fn get_available_equity() -> impl Responder{
    HttpResponse::Ok().body("get_available_equity")
}