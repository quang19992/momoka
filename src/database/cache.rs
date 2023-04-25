use super::{error::DatabaseError, redis::RedisWrapper};
use r2d2_redis::redis::RedisError;
use serde_cbor::Error as SerdeCborError;
use std::{convert, sync::Arc};

pub struct CacheWrapper {
    redis: RedisWrapper,
}

#[derive(Debug, Clone)]
pub enum CacheError {
    RedisError(Arc<RedisError>),
    SerdeCborError(Arc<SerdeCborError>),
}

impl convert::From<RedisError> for CacheError {
    fn from(err: RedisError) -> Self {
        CacheError::RedisError(Arc::new(err))
    }
}

impl convert::From<SerdeCborError> for CacheError {
    fn from(err: SerdeCborError) -> Self {
        CacheError::SerdeCborError(Arc::new(err))
    }
}

pub trait CacheModule {
    fn set<K: Into<String>>(
        &self,
        key: K,
        value: Vec<u8>,
        expire: usize,
    ) -> Result<(), DatabaseError>;
    fn get<K: Into<String>>(&self, key: K) -> Result<Vec<u8>, DatabaseError>;
}

impl CacheWrapper {
    pub fn new(redis: RedisWrapper) -> Self {
        Self { redis }
    }

    #[allow(dead_code)]
    pub fn set<K, V>(&self, key: K, value: &V, expire: usize) -> Result<(), DatabaseError>
    where
        K: std::convert::Into<String>,
        V: serde::ser::Serialize,
    {
        let cbor = match serde_cbor::to_vec(&value) {
            Err(err) => {
                return Err(DatabaseError::CacheError(CacheError::SerdeCborError(
                    Arc::new(err),
                )))
            }
            Ok(cbor) => cbor,
        };
        self.redis.set(key, cbor, expire)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get<K, V>(&self, key: K) -> Result<V, DatabaseError>
    where
        K: std::convert::Into<String>,
        V: for<'a> serde::de::Deserialize<'a>,
    {
        let cbor: Vec<u8> = self.redis.get(key)?;
        match serde_cbor::de::from_slice::<V>(&cbor[..]) {
            Err(err) => Err(DatabaseError::CacheError(CacheError::SerdeCborError(
                Arc::new(err),
            ))),
            Ok(cbor) => Ok(cbor),
        }
    }
}
