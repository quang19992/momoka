use super::{error::DatabaseError, sync::{SyncSupport, SyncResponse}};
use crate::server_config::scylla::ScyllaConfig;
use scylla::{Session, SessionBuilder};
use std::result::Result;

pub struct ScyllaWrapper {
    pub session: Session,
}

impl ScyllaWrapper {
    pub async fn new(config: &ScyllaConfig) -> Result<Self, DatabaseError> {
        let builder: SessionBuilder = SessionBuilder::new()
            .known_node(&config.uri)
            .use_keyspace(&config.keyspace, true);
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

impl SyncSupport for ScyllaWrapper {
    fn execute(&self, query: &str) -> SyncResponse {
        futures::executor::block_on(self.session.query(query, &[]))?;
        Ok(())
    }
}
