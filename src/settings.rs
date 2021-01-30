use std::env;
use config::{ConfigError, Config, File};
use serde::{Deserialize};


#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub db_name: String,
    pub collection_name: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
}


impl Settings {
    pub fn new(logger : &slog::Logger) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        let env = env::var("ENV_FILE").unwrap_or_else(|_| "dev".into());
        info!(logger, "Env: {}", env);

        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        s.try_into()
    }
}