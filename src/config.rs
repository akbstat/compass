use crate::errors::Result;
use dotenv::dotenv;
use std::{
    env,
    path::{Path, PathBuf},
};
use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub log_dir: PathBuf,
    pub database_url_vestige: String,
    pub database_url_sdtm: String,
    pub addr: String,
    pub env: String,
    pub log_level: Level,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenv().ok();
        let addr = format!("0.0.0.0:{}", env::var("SERVER_PORT")?);
        let database_url_vestige = env::var("DATABASE_URL_VESTIGE")?;
        let database_url_sdtm = env::var("DATABASE_URL_SDTM")?;

        let (log_level, env) = match env::var("COMPASS_ENV") {
            Ok(value) => {
                let log_level = if value.eq("PROD") {
                    Level::INFO
                } else {
                    Level::DEBUG
                };
                (log_level, value.to_lowercase())
            }
            Err(_) => (Level::INFO, "prod".to_string()),
        };

        let log_dir = Path::new(&env::var("LOG_DIR")?).to_path_buf();
        Ok(Config {
            log_dir,
            database_url_vestige,
            database_url_sdtm,
            addr,
            env,
            log_level,
        })
    }
}
