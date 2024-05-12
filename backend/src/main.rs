pub mod db;
pub mod endpoints;
pub mod util;

mod schema;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

struct ConnState {
    conn: Mutex<mysql::Conn>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Connect to MySQL using credentials from the TOML file
    // Connect to MySQL and create tables
    let conn = db::connect().expect("Failed to connect to MySQL.");
    let conn_state = web::Data::new(ConnState {
        conn: Mutex::new(conn),
    });

    // let stripe_secret_key =
    //     std::env::var("STRIPE_PUBLIC_KEY").expect("STRIPE_PUBLIC_KEY must be set");
    // let _stripe = Client::new(stripe_secret_key); // TODO: Implement the Stripe client

    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            // Register the index endpoint
            //.route("/", actix_web::web::get().to(backend::endpoints::page))
            .app_data(conn_state.clone())
            .service(web::scope("api/auth").configure(auth::endpoints))
            .route("/hello", web::get().to(endpoints::hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
