use actix_web::{get, HttpRequest, HttpResponse, Responder};
use std::fs;

pub async fn hello(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix!")
}
