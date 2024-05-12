use actix_web::{get, HttpRequest, HttpResponse, Responder};
use std::fs;

pub async fn hello(_req: HttpRequest) -> impl Responder {
    // Here you can process the request as needed
    // For now, let's just return a simple response
    HttpResponse::Ok().body("Hello from Actix!")
}
