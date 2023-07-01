use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub enable_log: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
}

impl Config {
    pub fn from_env() -> Result<Config, dotenv::Error> {
        dotenv::dotenv().ok();
        let enable_log = env::var("ENABLE_LOG").ok();
        let host = env::var("HOST").ok();
        let port = env::var("PORT")
            .ok()
            .map(|p| p.parse::<i32>().ok())
            .flatten();

        Ok(Config {
            enable_log,
            host,
            port,
        })
    }
}
