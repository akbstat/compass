use crate::errors::Result;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub log_dir: String,
    pub database_url_vestige: String,
    pub database_url_sdtm: String,
    pub addr: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenv().ok();
        let addr = format!("0.0.0.0:{}", env::var("SERVER_PORT")?);
        let database_url_vestige = env::var("DATABASE_URL_VESTIGE")?;
        let database_url_sdtm = env::var("DATABASE_URL_SDTM")?;
        let log_dir = env::var("LOG_DIR")?;
        Ok(Config {
            log_dir,
            database_url_vestige,
            database_url_sdtm,
            addr,
        })
    }
}
