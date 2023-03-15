use super::{manticore::ManticoreConfig, scylla::ScyllaConfig, EnvParseError};

#[derive(Clone)]
pub struct DatabaseConfig {
    pub scylla: ScyllaConfig,
    pub manticore: ManticoreConfig,
}

impl DatabaseConfig {
    pub fn load() -> Result<Self, EnvParseError> {
        Ok(Self {
            scylla: ScyllaConfig::load()?,
            manticore: ManticoreConfig::load()?,
        })
    }
}
