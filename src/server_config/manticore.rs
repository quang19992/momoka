use super::{EnvParseError, ServerConfig};
use std::result::Result;

const MANTICORE_URI: &str = "MANTICORE_URI";
const MANTICORE_PORT: &str = "MANTICORE_PORT";
const MANTICORE_USER: &str = "MANTICORE_USER";
const MANTICORE_PASSWORD: &str = "MANTICORE_PASSWORD";
const MANTICORE_PREFIX: &str = "MANTICORE_PREFIX";

#[derive(Clone)]
pub struct ManticoreConfig {
    pub uri: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub prefix: String,
}

impl ManticoreConfig {
    pub fn load() -> Result<ManticoreConfig, EnvParseError> {
        Ok(Self {
            uri: ServerConfig::get_str(MANTICORE_URI)?,
            port: ServerConfig::get_num::<u16>(MANTICORE_PORT)?,
            user: ServerConfig::get_str(MANTICORE_USER).unwrap_or("".to_string()),
            password: ServerConfig::get_str(MANTICORE_PASSWORD).unwrap_or("".to_string()),
            prefix: ServerConfig::get_str(MANTICORE_PREFIX).unwrap_or("".to_string()),
        })
    }

    pub fn had_auth(&self) -> bool {
        self.user != "" && self.password != ""
    }
}
