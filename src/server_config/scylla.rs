use std::result::Result;
use super::{EnvParseError, ServerConfig};

const SCYLLA_URI: &str = "SCYLLA_URI";
const SCYLLA_USER: &str = "SCYLLA_USER";
const SCYLLA_PASSWORD: &str = "SCYLLA_PASSWORD";

#[derive(Clone)]
pub struct ScyllaConfig {
    pub uri: String,
    pub user: String,
    pub password: String,
}

impl ScyllaConfig {
    pub fn load() -> Result<ScyllaConfig, EnvParseError> {
        Ok(Self {
            uri: ServerConfig::get_str(SCYLLA_URI),
            user: ServerConfig::get_str(SCYLLA_USER),
            password: ServerConfig::get_str(SCYLLA_PASSWORD),
        })
    }

    pub fn had_auth(&self) -> bool {
        self.user == "" && self.password == ""
    }
}
