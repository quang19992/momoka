use super::{
    error::DatabaseError,
    sync::{SyncResponse, SyncSupport},
};
use crate::server_config::scylla::ScyllaConfig;
use scylla::{transport::errors, IntoTypedRows, Session, SessionBuilder};
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
        const VERSION_QUERY: &str = "SELECT value FROM sync_data WHERE field = ?;";
        let query = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                self.session.query(VERSION_QUERY, vec![VERSION_FIELD]).await
            })
        });
        let query = match query {
            Err(err) => match err {
                errors::QueryError::DbError(err, message) => match err {
                    errors::DbError::Invalid => return Ok(None),
                    _ => {
                        return Err(DatabaseError::ScyllaQueryError(
                            errors::QueryError::DbError(err, message),
                        ))
                    }
                },
                _ => return Err(DatabaseError::ScyllaQueryError(err)),
            },
            Ok(query) => query,
        };
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

    fn set_schema_version(&self, version: i64) -> SyncResponse {
        const VERSION_FIELD: &str = "schema_version";
        const VERSION_QUERY: &str = "UPDATE sync_data SET value = ? WHERE field = ?;";
        let _ = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                self.session
                    .query(VERSION_QUERY, (version, VERSION_FIELD))
                    .await
            })
        })?;
        Ok(())
    }

    fn execute(&self, query: &str) -> SyncResponse {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current()
                .block_on(async move { self.session.query(query, &[]).await })
        })?;
        Ok(())
    }
}
