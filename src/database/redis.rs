use super::cache::{CacheError, CacheModule};
use super::error::DatabaseError;
use crate::server_config::redis::RedisConfig;
use r2d2::{Error, Pool, PooledConnection};
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use std::{convert::Into, ops::DerefMut, sync::Arc};

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
    fn set<K: Into<String>>(
        &self,
        key: K,
        value: Vec<u8>,
        expire: usize,
    ) -> Result<(), DatabaseError> {
        let mut conn = self.conn()?;
        redis::Cmd::pset_ex(Into::<String>::into(key), value, expire).execute(conn.deref_mut());
        Ok(())
    }

    fn get<K: Into<String>>(&self, key: K) -> Result<Vec<u8>, DatabaseError> {
        let mut conn = self.conn()?;
        match redis::Cmd::get(Into::<String>::into(key)).query(conn.deref_mut()) {
            Err(err) => Err(DatabaseError::CacheError(Into::<CacheError>::into(err))),
            Ok(cbor) => Ok(cbor),
        }
    }
}
