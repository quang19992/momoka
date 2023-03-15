use crate::server_config::database::DatabaseConfig;
use std::{env, result::Result, str::FromStr};

pub mod database;
pub mod manticore;
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
    KeyNotFound(String),
    KeyIsEmpty(String),
    InvalidNumber(String, String),
}

impl ServerConfig {
    pub fn get_str(key: &str) -> Result<String, EnvParseError> {
        match env::var(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(EnvParseError::KeyNotFound(key.to_string())),
        }
    }

    pub fn get_num<T: FromStr>(key: &str) -> Result<T, EnvParseError>
    where
        T::Err: std::fmt::Debug,
    {
        match Self::get_str(key)?.parse::<T>() {
            Ok(num) => Ok(num),
            Err(err) => Err(EnvParseError::InvalidNumber(
                key.to_string(),
                format!("{:?}", err),
            )),
        }
    }

    pub fn load() -> Result<Self, EnvParseError> {
        dotenv::dotenv().ok();

        let is_production: bool =
            Self::get_str(&RUST_ENV).unwrap_or("".to_string()) == "production";
        let rust_log: &str = if is_production { "INFO" } else { "DEBUG" };
        env::set_var(RUST_LOG, rust_log);
        Ok(Self {
            is_production,
            http_port: Self::get_num::<u16>(&HTTP_PORT).unwrap_or(8080),
            num_worker: Self::get_num::<usize>(&NUM_WORKER).unwrap_or(2),
            database: DatabaseConfig::load()?,
        })
    }
}
