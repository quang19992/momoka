use super::{error::DatabaseError, scylla::ScyllaWrapper};
use crate::server_config::database::DatabaseConfig;
use std::result::Result;

pub struct Database {
    scylla: ScyllaWrapper,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let scylla = ScyllaWrapper::new(&config.scylla);
        let polls = futures::join!(scylla);
        Ok(Self { scylla: polls.0? })
    }
}
