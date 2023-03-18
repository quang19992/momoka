use super::redis::RedisWrapper;

pub struct CacheWrapper {
    redis: RedisWrapper,
}

impl CacheWrapper {
    pub fn new(redis: RedisWrapper) -> Self {
        Self { redis }
    }
}
