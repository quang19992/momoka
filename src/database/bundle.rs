use super::{
    cache::CacheWrapper, error::DatabaseError, manticore::ManticoreWrapper, redis::RedisWrapper,
    scylla::ScyllaWrapper,
};
use crate::database::sync::SyncSupport;
use crate::server_config::database::DatabaseConfig;
use std::{result::Result, sync::Arc};

pub struct Database {
    pub scylla: Arc<ScyllaWrapper>,
    pub manticore: Arc<ManticoreWrapper>,
    pub cache: Arc<CacheWrapper>,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let scylla = ScyllaWrapper::new(&config.scylla);
        let manticore = ManticoreWrapper::new(&config.manticore);
        let redis = RedisWrapper::new(&config.redis);
        let polls = futures::join!(scylla, manticore, redis);
        Ok(Self {
            scylla: Arc::new(polls.0?),
            manticore: Arc::new(polls.1?),
            cache: Arc::new(CacheWrapper::new(polls.2?)),
        })
    }
}

pub async fn sync(bundle: Arc<Database>) -> Result<(), DatabaseError> {
    log::info!("Started schema synchronization job");
    let scylla_synchronizers = super::scylla::schema::synchronizers();
    let scylla = super::sync::execute(
        scylla_synchronizers.clone().to_vec(),
        bundle.clone(), 
        bundle.clone().scylla.clone(),
    );
    let polls = futures::join!(scylla);
    Ok(())
}
