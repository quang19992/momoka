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
}

impl SyncSupport for ScyllaWrapper {
    fn schema_version(&self) -> Result<Option<i64>, DatabaseError> {
        const VERSION_FIELD: &str = "schema_version";
        const VERSION_QUERY: &str =
            "SELECT value FROM sync_data WHERE field = ?;";
        let query = self.session.query(VERSION_QUERY, vec![VERSION_FIELD]);
        let query = futures::executor::block_on(query)?;
        let rows = match query.rows {
            Some(rows) => rows,
            None => return Ok(None),
        };
        for row in rows.into_typed::<(String,)>() {
            let (version,): (String,) = row?;
            let version: i64 = version.parse::<i64>().unwrap_or_else(|err| {
                log::error!("invalid schema version {:?}", err);
                panic!("invalid schema version {:?}", err);
            });
            return Ok(Some(version));
        }
        Ok(None)
    }

    fn execute(&self, query: &str) -> SyncResponse {
        futures::executor::block_on(self.session.query(query, &[]))?;
        Ok(())
    }
}
