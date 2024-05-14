use dotenvy::dotenv;
use log::info;
use mysql::prelude::*;
use std::env;

use crate::util;

/// Connect to the MySQL database.
///
/// # Returns
/// The connection to the MySQL database if successful.
/// Panics if the connection fails.
pub fn connect() -> mysql::Conn {
    info!("Connecting to MySQL.");
    dotenv().ok();
    let db_url = {
        if util::is_development_mode() {
            info!("Running in development mode. This is not recommended for production use.");
            env::var("DEV_DATABASE_URL").expect("DEV_DATABASE_URL must be set")
        } else {
            info!("Running in production mode.");
            env::var("DATABASE_URL").expect("DATABASE_URL must be set")
        }
    };
    assert!(!db_url.is_empty(), "DATABASE_URL must not be empty");

    info!("Connecting to: {}", db_url);
    let conn = match mysql::Conn::new(db_url) {
        Ok(conn) => conn,
        Err(e) => {
            panic!("Error connecting to MySQL: {}", e);
        }
    };
    conn
}
