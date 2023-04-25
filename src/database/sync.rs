use super::{bundle::Database, error::DatabaseError};
use async_recursion::async_recursion;
use futures::future::BoxFuture;
use std::{future::Future, result::Result, sync::Arc};

const QUERY_BATCH_SIZE: usize = 10;

pub type SyncResponse = Result<(), DatabaseError>;

#[derive(Clone)]
pub struct SyncFn {
    pub function: Arc<dyn Fn(Arc<Database>) -> BoxFuture<'static, SyncResponse> + std::marker::Send + Sync>,
}

impl SyncFn {
    #[allow(unused)]
    pub fn new<F: Future<Output = SyncResponse> + std::marker::Send + 'static>(
        function: fn(Arc<Database>) -> F,
    ) -> Self {
        Self {
            function: Arc::new(move |database| Box::pin(function(database))),
        }
    }
}

pub trait SyncSupport {
    fn name(&self) -> String;
    fn schema_version(&self) -> Result<Option<i64>, DatabaseError>;
    fn set_schema_version(&self, version: i64) -> Result<(), DatabaseError>;
    fn execute(&self, query: &str) -> SyncResponse;
}

// TODO - implement this
// this will help in case of a sync failure
pub struct SyncState;

#[allow(unused)]
#[derive(Clone)]
pub enum Synchronizer {
    Simple(Vec<String>),
    Custom(SyncFn),
    Mixed(Vec<Synchronizer>),
}

impl Synchronizer {
    #[allow(unused)]
    pub fn execute<T: SyncSupport>(
        &self,
        bundle: Arc<Database>,
        database: Arc<T>,
        state: Option<SyncState>,
    ) -> SyncResponse {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                match self {
                    Synchronizer::Simple(queries) => Self::execute_simple_sync(database, queries.to_vec()).await,
                    Synchronizer::Custom(function) => Self::execute_custom_sync(bundle, function.clone()).await,
                    Synchronizer::Mixed(synchronizers) => {
                        Self::execute_mixed_sync(synchronizers.to_vec(), bundle, database, state).await
                    }
                }
            })
        })
    }

    async fn execute_simple_sync<T: SyncSupport>(
        database: Arc<T>,
        queries: Vec<String>,
    ) -> SyncResponse {
        let mut iter = queries.chunks(QUERY_BATCH_SIZE);
        while let Some(batch) = iter.next() {
            Self::execute_batch_query(database.clone(), batch.to_vec()).await?;
        }
        Ok(())
    }

    async fn execute_custom_sync(bundle: Arc<Database>, function: SyncFn) -> SyncResponse {
        (function.function)(bundle).await
    }

    async fn execute_mixed_sync<T: SyncSupport>(
        synchronizers: Vec<Self>,
        bundle: Arc<Database>,
        database: Arc<T>,
        _state: Option<SyncState>,
    ) -> SyncResponse {
        for synchronizer in synchronizers.iter() {
            synchronizer
                .execute(bundle.clone(), database.clone(), None)?;
        }
        Ok(())
    }

    async fn execute_batch_query<T: SyncSupport>(
        database: Arc<T>,
        queries: Vec<String>,
    ) -> SyncResponse {
        futures::future::try_join_all(
            queries
                .into_iter()
                .map(|query| Self::execute_query(database.clone(), query.clone())),
        )
        .await?;
        Ok(())
    }

    async fn execute_query<T: SyncSupport>(database: Arc<T>, query: String) -> SyncResponse {
        database.execute(&query)
    }
}

pub async fn execute<T: SyncSupport>(
    synchronizers: Vec<Synchronizer>,
    bundle: Arc<Database>,
    database: Arc<T>,
) -> SyncResponse {
    if synchronizers.is_empty() {
        return Ok(());
    }

    let current_version = match database.clone().schema_version()? {
        None => {
            log::debug!("[{}] starting master synchronizer", database.clone().name());
            let synchronizer = synchronizers[0].clone();
            synchronizer.execute(bundle.clone(), database.clone(), None)?;
            database
                .clone()
                .set_schema_version(synchronizers.len() as i64 - 1)?;
            log::debug!(
                "[{}] schema now in sync with master schema",
                database.clone().name()
            );
            return Ok(());
        }
        Some(version) => version,
    };

    for i in (current_version as usize)..(synchronizers.len() - 1) {
        log::debug!(
            "[{}] starting synchronizer from #{} to #{})",
            database.clone().name(),
            i - 1,
            i
        );
        let synchronizer = synchronizers[i].clone();
        synchronizer.execute(bundle.clone(), database.clone(), None)?;
        database.clone().set_schema_version(i as i64)?;
        log::debug!(
            "[{}] schema now in sync with schema #{}",
            database.clone().name(),
            i
        );
    }
    Ok(())
}
