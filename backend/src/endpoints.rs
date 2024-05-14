use actix_web::{get, HttpRequest, HttpResponse, Responder};
use log::info;
use std::fs;

pub async fn hello(_req: HttpRequest) -> impl Responder {
    info!("GET /hello called");
    HttpResponse::Ok().body("Hello from Actix!")
}
