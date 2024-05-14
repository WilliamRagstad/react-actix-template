pub mod db;
pub mod endpoints;
pub mod util;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct ConnState {
    conn: Mutex<mysql::Conn>,
}

/// The main function that starts the Actix server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_state = web::Data::new(ConnState {
        conn: Mutex::new(db::connect()),
    });

    let host_addr = "localhost:8080";
    println!("Starting server at http://{}", host_addr);
    HttpServer::new(move || {
        App::new()
            .app_data(conn_state.clone())
            .route("/hello", web::get().to(endpoints::hello))
    })
    .bind(host_addr)?
    .run()
    .await
}
