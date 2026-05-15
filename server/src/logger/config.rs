use std::env;
use dotenvy::dotenv;

pub fn get_log_level() -> u8 {
    dotenv().ok();

    let env_level: &str = &env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string());

    match env_level {
        "DEBUG" => 0,
        "INFO" => 1,
        "WARNING" => 2,
        "ERROR" => 3,
        _ => 1,
    }
}