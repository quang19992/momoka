use super::error::DatabaseError;
use crate::server_config::scylla::ScyllaConfig;
use scylla::{Session, SessionBuilder};
use std::result::Result;

pub struct ScyllaWrapper {
    session: Session,
}

impl ScyllaWrapper {
    pub async fn new(config: &ScyllaConfig) -> Result<Self, DatabaseError> {
        let builder: SessionBuilder = SessionBuilder::new().known_node(&config.uri);
        let builder = if config.had_auth() {
            builder.user(&config.user, &config.password)
        } else {
            builder
        };
        Ok(Self {
            session: builder.build().await?,
        })
    }
}
