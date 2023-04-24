use super::{
    error::DatabaseError,
    sync::{SyncResponse, SyncSupport},
};
use crate::server_config::manticore::ManticoreConfig;
use mysql::{prelude::*, Conn, Error as MySqlError, OptsBuilder};
use r2d2::{Error, Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;
use std::sync::Arc;

const POOL_MAX_SIZE: u32 = 5;
const SPECIAL_TOKEN: &'static [(&str, &str)] = &[("\\", "\\\\"), ("'", "\\'"), ("\"", "\\\"")];

pub struct ManticoreWrapper {
    pool: Arc<Pool<MySqlConnectionManager>>,
    pub prefix: String,
}

fn healthcheck(_: MySqlConnectionManager, conn: &mut Conn) -> Result<(), MySqlError> {
    conn.query("SELECT 1").map(|_: Vec<String>| ())
}

impl ManticoreWrapper {
    pub async fn new(config: &ManticoreConfig) -> Result<Self, Error> {
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some(&config.uri))
            .tcp_port(config.port);
        let builder = if config.had_auth() {
            builder
                .user(Some(&config.user))
                .pass(Some(&config.password))
        } else {
            builder
        };
        let manager = MySqlConnectionManager::with_custom_healthcheck(builder, &healthcheck);
        let pool = Pool::builder().max_size(POOL_MAX_SIZE).build(manager)?;
        Ok(Self {
            pool: Arc::new(pool),
            prefix: String::from(&config.prefix),
        })
    }

    pub fn conn(&self) -> Result<PooledConnection<MySqlConnectionManager>, DatabaseError> {
        let pool = self.pool.clone();
        Ok(pool.get()?)
    }
}

/// @fixme - mid priority
/// potential SQL-injection? either better sanitizer or
/// somehow make manticore support prepared statement.
pub fn sanitize_param<T: Into<String>>(param: T) -> String {
    let mut sanitized = Into::<String>::into(param);
    for token in SPECIAL_TOKEN {
        sanitized = sanitized.replace(token.0, token.1);
    }
    sanitized
}

impl SyncSupport for ManticoreWrapper {
    fn schema_version(&self) -> Result<Option<i64>, DatabaseError> {
        const NO_TABLE_ERROR: &str = "unknown local table(s)";
        const VERSION_QUERY: &str = r#"
            SELECT value 
            FROM sync_data 
            WHERE MATCH('@field schema_version')
        "#;

        let mut conn = self.conn()?;
        let row: Option<String> = match conn.query_first(VERSION_QUERY) {
            Err(err) => {
                if format!("{:?}", err).contains(NO_TABLE_ERROR) {
                    return Ok(None);
                }
                return Err(err.into());
            }
            Ok(row) => row,
        };
        let version = match row {
            None => return Ok(None),
            Some(version) => version,
        };
        match version.parse::<i64>() {
            Err(err) => Err(DatabaseError::Other(format!("{:?}", err))),
            Ok(version) => Ok(Some(version)),
        }
    }

    fn set_schema_version(&self, version: i64) -> SyncResponse {
        let mut conn = self.conn()?;
        match self.schema_version()? {
            None => {
                conn.query_drop(format!(
                    "INSERT INTO sync_data (field, value) VALUES ('{}', '{}')",
                    "schema_version", version
                ))?;
            }
            Some(_) => {
                conn.query_drop(format!(
                    "UPDATE sync_data SET value = '{}' WHERE MATCH('@field {}')",
                    version, "schema_version"
                ))?;
            }
        };
        Ok(())
    }

    fn execute(&self, query: &str) -> SyncResponse {
        self.conn()?.query_drop(query)?;
        Ok(())
    }
}
