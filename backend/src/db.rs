use dotenvy::dotenv;
use mysql::prelude::*;
use std::env;

use crate::util;

pub fn connect() -> Result<mysql::Conn, String> {
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
    assert!(
        db_url.starts_with("mysql://"),
        "DATABASE_URL must start with mysql://"
    );
    assert!(db_url.contains('@'), "DATABASE_URL must contain @");
    assert!(db_url.contains('/'), "DATABASE_URL must contain /");

    println!("Connecting to: {}", db_url);
    mysql::Conn::new(db_url).map_err(|err| format!("Failed to connect to MySQL: {}", err))
}
