use super::{bundle::Database, error::DatabaseError};
use async_recursion::async_recursion;
use futures::future::BoxFuture;
use std::{future::Future, result::Result, sync::Arc};

const QUERY_BATCH_SIZE: usize = 10;

pub type SyncResponse = Result<(), DatabaseError>;

pub struct SyncFn {
    pub function: Box<dyn Fn(Arc<Database>) -> BoxFuture<'static, SyncResponse>>,
}

impl SyncFn {
    #[allow(unused)]
    pub fn new<F: Future<Output = SyncResponse> + std::marker::Send + 'static>(
        function: fn(Arc<Database>) -> F,
    ) -> Self {
        Self {
            function: Box::new(move |database| Box::pin(function(database))),
        }
    }
}

pub trait SyncSupport {
    fn schema_version(&self) -> Result<Option<i64>, DatabaseError>;
    fn set_schema_version(&self, version: i64) -> Result<(), DatabaseError>;
    fn execute(&self, query: &str) -> SyncResponse;
}

// TODO - implement this
// this will help in case of a sync failure
pub struct SyncState;

#[allow(unused)]
pub enum Synchronizer<'a> {
    Simple(&'a [&'a str]),
    Custom(&'a SyncFn),
    Mixed(Arc<&'a [&'a Synchronizer<'a>]>),
}

impl Synchronizer<'_> {
    #[allow(unused)]
    #[async_recursion(?Send)]
    pub async fn execute<T: SyncSupport + std::marker::Copy>(
        &self,
        bundle: Arc<Database>,
        database: Arc<T>,
        state: Option<SyncState>,
    ) -> SyncResponse {
        match self {
            Synchronizer::Simple(queries) => Self::execute_simple_sync(database, queries).await,
            Synchronizer::Custom(function) => Self::execute_custom_sync(bundle, function).await,
            Synchronizer::Mixed(synchronizers) => {
                Self::execute_mixed_sync(synchronizers, bundle, database, state).await
            }
        }
    }

    async fn execute_simple_sync<'a, T: SyncSupport>(
        database: Arc<T>,
        queries: &'a [&'a str],
    ) -> SyncResponse {
        let mut iter = queries.chunks(QUERY_BATCH_SIZE);
        while let Some(batch) = iter.next() {
            Self::execute_batch_query(database.clone(), batch).await?;
        }
        Ok(())
    }

    async fn execute_custom_sync<'a>(bundle: Arc<Database>, function: &'a SyncFn) -> SyncResponse {
        (function.function)(bundle).await
    }

    async fn execute_mixed_sync<'a, T: SyncSupport + std::marker::Copy>(
        synchronizers: &'a [&'a Self],
        bundle: Arc<Database>,
        database: Arc<T>,
        _state: Option<SyncState>,
    ) -> SyncResponse {
        for synchronizer in synchronizers.iter() {
            synchronizer
                .execute(bundle.clone(), database.clone(), None)
                .await?;
        }
        Ok(())
    }

    async fn execute_batch_query<'a, T: SyncSupport>(
        database: Arc<T>,
        queries: &'a [&'a str],
    ) -> SyncResponse {
        futures::future::try_join_all(
            queries
                .into_iter()
                .map(|query| Self::execute_query(database.clone(), query)),
        )
        .await?;
        Ok(())
    }

    async fn execute_query<'a, T: SyncSupport>(database: Arc<T>, query: &'a str) -> SyncResponse {
        database.execute(query)
    }
}

pub async fn execute<T: SyncSupport + std::marker::Copy>(
    synchronizers: Arc<Vec<&Synchronizer<'_>>>,
    bundle: Arc<Database>,
    database: Arc<T>,
) -> SyncResponse {
    if synchronizers.is_empty() {
        return Ok(());
    }

    let current_version = match database.clone().schema_version()? {
        None => {
            synchronizers[0]
                .execute(bundle.clone(), database.clone(), None)
                .await?;
            database
                .clone()
                .set_schema_version(synchronizers.len() as i64 - 1)?;
            return Ok(());
        }
        Some(version) => version,
    };

    for i in (current_version as usize)..synchronizers.len() {
        synchronizers[i]
            .execute(bundle.clone(), database.clone(), None)
            .await?;
        database.clone().set_schema_version(i as i64)?;
    }
    Ok(())
}
