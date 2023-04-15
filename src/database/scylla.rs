use super::{
    error::DatabaseError,
    sync::{SyncResponse, SyncSupport},
};
use crate::server_config::scylla::ScyllaConfig;
use scylla::{IntoTypedRows, Session, SessionBuilder};
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

    pub async fn get_schema_version(&self, target: &str) -> Result<Option<i64>, DatabaseError> {
        const VERSION_QUERY: &str =
            "SELECT source, schema_version FROM sync_data WHERE source = ?;";
        let rows = self.session.query(VERSION_QUERY, vec![target]).await?.rows;
        let rows = match rows {
            Some(rows) => rows,
            None => return Ok(None),
        };
        for row in rows.into_typed::<(String, i64)>() {
            let (_, version): (String, i64) = row?;
            return Ok(Some(version));
        }
        Ok(None)
    }
}

impl SyncSupport for ScyllaWrapper {
    fn execute(&self, query: &str) -> SyncResponse {
        futures::executor::block_on(self.session.query(query, &[]))?;
        Ok(())
    }
}
