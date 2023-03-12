use std::{env, result::Result};
use crate::server_config::database::DatabaseConfig;

pub mod database;
pub mod scylla;

const RUST_ENV: &str = "RUST_ENV";
const RUST_LOG: &str = "RUST_LOG";
const HTTP_PORT: &str = "HTTP_PORT";
const NUM_WORKER: &str = "NUM_WORKER";

#[derive(Clone)]
pub struct ServerConfig {
    pub is_production: bool,
    pub http_port: u16,
    pub num_worker: usize,
    pub database: DatabaseConfig,
}

#[derive(Clone, Debug)]
pub enum EnvParseError {
    InvalidNumber,
}

impl ServerConfig {
    pub fn get_str(key: &str) -> String {
        match env::var(key) {
            Ok(val) => val,
            Err(_) => "".to_string(),
        }
    }

    pub fn get_num<T: std::str::FromStr>(key: &str) -> Result<T, EnvParseError> {
        match key.parse::<T>() {
            Ok(num) => Ok(num),
            Err(_) => Err(EnvParseError::InvalidNumber),
        }
    }

    pub fn load() -> Result<Self, EnvParseError> {
        dotenv::dotenv().ok();

        let is_production: bool = Self::get_str(&RUST_ENV) == "production";
        let rust_log: &str = if is_production { "INFO" } else { "DEBUG" };

        let http_port: u16 = match Self::get_num::<u16>(&HTTP_PORT) {
            Ok(val) => val,
            Err(_) => 8080,
        };

        let num_worker: usize = match Self::get_num::<usize>(&NUM_WORKER) {
            Ok(val) => val,
            Err(_) => 2,
        };

        env::set_var(RUST_LOG, rust_log);
        Ok(Self {
            is_production: false,
            http_port,
            num_worker,
            database: DatabaseConfig::load()?,
        })
    }
}
