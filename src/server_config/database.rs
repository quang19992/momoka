use super::{scylla::ScyllaConfig, EnvParseError};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub scylla: ScyllaConfig,
}

impl DatabaseConfig {
    pub fn load() -> Result<Self, EnvParseError> {
        Ok(Self {
            scylla: ScyllaConfig::load()?,
        })
    }
}
