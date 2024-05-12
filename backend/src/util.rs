use dotenvy::dotenv;

pub(crate) fn is_development_mode() -> bool {
    dotenv().ok();
    !std::env::var("DEV_MODE")
        .unwrap_or("".to_string())
        .is_empty()
}
