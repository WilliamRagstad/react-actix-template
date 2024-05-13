use dotenvy::dotenv;
use mysql::prelude::*;
use std::env;

use crate::util;

/// Connect to the MySQL database.
///
/// # Returns
/// The connection to the MySQL database if successful.
/// Panics if the connection fails.
pub fn connect() -> mysql::Conn {
    dotenv().ok();
    let db_url = {
        if util::is_development_mode() {
            println!("Running in development mode. This is not recommended for production use.");
            env::var("DEV_DATABASE_URL").expect("DEV_DATABASE_URL must be set")
        } else {
            println!("Running in production mode.");
            env::var("DATABASE_URL").expect("DATABASE_URL must be set")
        }
    };
    assert!(!db_url.is_empty(), "DATABASE_URL must not be empty");

    println!("Connecting to: {}", db_url);
    mysql::Conn::new(db_url).expect("Failed to connect to MySQL.")
}
