use super::error::DatabaseError;
use super::cache::{CacheModule, CacheError};
use crate::server_config::redis::RedisConfig;
use r2d2::{Error, Pool, PooledConnection};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::sync::Arc;

pub struct RedisWrapper {
    pub pool: Arc<Pool<RedisConnectionManager>>,
}

impl RedisWrapper {
    pub async fn new(config: &RedisConfig) -> Result<Self, Error> {
        let manager = RedisConnectionManager::new(config.uri.to_owned()).unwrap();
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        Ok(RedisWrapper {
            pool: Arc::new(pool),
        })
    }

    pub fn conn(&self) -> Result<PooledConnection<RedisConnectionManager>, DatabaseError> {
        let pool = self.pool.clone();
        Ok(pool.get()?)
    }
}

impl CacheModule for RedisWrapper {
    fn set<K: Into<String>>(&self, key: K, value: Vec<u8>, expire: usize) -> Option<CacheError> {
        todo!()
    }

    fn get<K: Into<String>>(&self, key: K) -> Result<Vec<u8>, CacheError> {
        todo!()
    }
}
