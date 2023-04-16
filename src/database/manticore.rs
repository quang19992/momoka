use super::{
    error::DatabaseError,
    sync::{SyncResponse, SyncSupport},
};
use crate::server_config::manticore::ManticoreConfig;
use mysql::{prelude::*, OptsBuilder};

use r2d2::{Error, Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;
use std::sync::Arc;

const POOL_MAX_SIZE: u32 = 5;

pub struct ManticoreWrapper {
    pool: Arc<Pool<MySqlConnectionManager>>,
    pub prefix: String,
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
        let manager = MySqlConnectionManager::new(builder);
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

impl SyncSupport for ManticoreWrapper {
    fn schema_version(&self) -> Result<Option<i64>, DatabaseError> {
        todo!()
    }

    fn execute(&self, query: &str) -> SyncResponse {
        self.conn()?.query_drop(query)?;
        Ok(())
    }
}
