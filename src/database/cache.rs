use super::{redis::RedisWrapper, error::DatabaseError};
use std::convert;
use r2d2_redis::redis::RedisError;
use serde_cbor::Error as SerdeCborError;

pub struct CacheWrapper {
    redis: RedisWrapper,
}

#[derive(Debug)]
pub enum CacheError {
    DatabaseError(DatabaseError),
    RedisError(RedisError),
    SerdeCborError(SerdeCborError),
}

impl convert::From<DatabaseError> for CacheError {
    fn from(err: DatabaseError) -> Self {
        CacheError::DatabaseError(err)
    }
}

impl convert::From<RedisError> for CacheError {
    fn from(err: RedisError) -> Self {
        CacheError::RedisError(err)
    }
}

impl convert::From<SerdeCborError> for CacheError {
    fn from(err: SerdeCborError) -> Self {
        CacheError::SerdeCborError(err)
    }
}

pub trait CacheModule {
    fn set<K: Into<String>>(&self, key: K, value: Vec<u8>, expire: usize) -> Option<CacheError>;
    fn get<K: Into<String>>(&self, key: K) -> Result<Vec<u8>, CacheError>;
}

impl CacheWrapper {
    pub fn new(redis: RedisWrapper) -> Self {
        Self { redis }
    }

    pub fn set<K, V>(&self, key: K, value: &V, expire: usize) -> Option<CacheError> 
    where
        K: std::convert::Into<String>,
        V: serde::ser::Serialize,
    {
        let cbor = serde_cbor::to_vec(&value).ok()?;
        self.redis.set(key, cbor, expire)
    }

    pub fn get<K, V>(&self, key: K) -> Result<V, CacheError>
    where
        K: std::convert::Into<String>,
        V: for<'a> serde::de::Deserialize<'a>,
    {
        let cbor : Vec<u8> = self.redis.get(key)?;
        Ok(serde_cbor::de::from_slice::<V>(&cbor[..])?)
    }
}
