use super::{manticore::ManticoreConfig, redis::RedisConfig, scylla::ScyllaConfig, EnvParseError};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub scylla: ScyllaConfig,
    pub manticore: ManticoreConfig,
    pub redis: RedisConfig,
}

impl DatabaseConfig {
    pub fn load() -> Result<Self, EnvParseError> {
        Ok(Self {
            scylla: ScyllaConfig::load()?,
            manticore: ManticoreConfig::load()?,
            redis: RedisConfig::load()?,
        })
    }
}
