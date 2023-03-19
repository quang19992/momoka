use super::{EnvParseError, ServerConfig};
use std::result::Result;

const REDIS_URI: &str = "REDIS_URI";

#[derive(Clone)]
pub struct RedisConfig {
    pub uri: String,
}

impl RedisConfig {
    pub fn load() -> Result<RedisConfig, EnvParseError> {
        Ok(Self {
            uri: ServerConfig::get_str(REDIS_URI)?,
        })
    }
}
