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

    HttpServer::new(move || {
        App::new()
            .app_data(conn_state.clone())
            .service(web::scope("api/auth").configure(auth::endpoints))
            .route("/hello", web::get().to(endpoints::hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    println!("Server running at http://localhost:8080");
}
