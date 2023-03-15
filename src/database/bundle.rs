use super::{error::DatabaseError, manticore::ManticoreWrapper, scylla::ScyllaWrapper};
use crate::server_config::database::DatabaseConfig;
use std::result::Result;

pub struct Database {
    pub scylla: ScyllaWrapper,
    pub manticore: ManticoreWrapper,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let scylla = ScyllaWrapper::new(&config.scylla);
        let manticore = ManticoreWrapper::new(&config.manticore);
        let polls = futures::join!(scylla, manticore);
        Ok(Self {
            scylla: polls.0?,
            manticore: polls.1?,
        })
    }
}
