use crate::errors::Result;
use dotenv::dotenv;
use std::env;
use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub log_dir: String,
    pub database_url: String,
    pub addr: String,
    pub log_level: Level,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenv().ok();
        let addr = format!("0.0.0.0:{}", env::var("SERVER_PORT")?);
        let database_url = env::var("DATABASE_URL")?;
        let log_dir = env::var("LOG_DIR")?;
        let log_level = match env::var("LOG_LEVEL") {
            Ok(value) => {
                if value.eq("DEV") {
                    Level::DEBUG
                } else {
                    Level::INFO
                }
            }
            Err(_) => Level::INFO,
        };
        Ok(Config {
            log_dir,
            database_url,
            addr,
            log_level,
        })
    }
}
