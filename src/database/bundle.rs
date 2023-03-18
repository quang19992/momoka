use super::{
    cache::CacheWrapper, error::DatabaseError, manticore::ManticoreWrapper, redis::RedisWrapper,
    scylla::ScyllaWrapper,
};
use crate::server_config::database::DatabaseConfig;
use std::result::Result;

pub struct Database {
    pub scylla: ScyllaWrapper,
    pub manticore: ManticoreWrapper,
    pub cache: CacheWrapper,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let scylla = ScyllaWrapper::new(&config.scylla);
        let manticore = ManticoreWrapper::new(&config.manticore);
        let redis = RedisWrapper::new(&config.redis);
        let polls = futures::join!(scylla, manticore, redis);
        Ok(Self {
            scylla: polls.0?,
            manticore: polls.1?,
            cache: CacheWrapper::new(polls.2?),
        })
    }
}
