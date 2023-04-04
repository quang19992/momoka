use super::{bundle::Database, error::DatabaseError};
use async_recursion::async_recursion;
use futures::future::BoxFuture;
use std::{future::Future, result::Result, sync::Arc};

type SyncResponse = Result<(), DatabaseError>;

struct SyncFn {
    function: Box<dyn Fn(Arc<Database>) -> BoxFuture<'static, SyncResponse>>,
}

impl SyncFn {
    fn new<F: Future<Output = SyncResponse> + std::marker::Send + 'static>(
        function: fn(Arc<Database>) -> F,
    ) -> Self {
        Self {
            function: Box::new(move |database| Box::pin(function(database))),
        }
    }
}

trait SyncSupport {
    fn execute(query: &str) -> SyncResponse;
}

// TODO - implement this
// this will help in case of a sync failure
struct SyncState;

enum Synchronizer<'a> {
    Simple(&'a [&'a str]),
    Custom(&'a SyncFn),
    Mixed(Arc<&'a [&'a Synchronizer<'a>]>),
}

impl Synchronizer<'_> {
    #[async_recursion(?Send)]
    async fn execute<T: SyncSupport + std::marker::Copy>(
        &self,
        bundle: Arc<Database>,
        database: T,
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
        database: T,
        queries: &'a [&'a str],
    ) -> SyncResponse {
        todo!()
    }

    async fn execute_custom_sync<'a>(bundle: Arc<Database>, function: &'a SyncFn) -> SyncResponse {
        (function.function)(bundle).await
    }

    async fn execute_mixed_sync<'a, T: SyncSupport + std::marker::Copy>(
        synchronizers: &'a [&'a Self],
        bundle: Arc<Database>,
        database: T,
        _state: Option<SyncState>,
    ) -> SyncResponse {
        for synchronizer in synchronizers.iter() {
            if let Err(err) = synchronizer.execute(bundle.clone(), database, None).await {
                return Err(err);
            }
        }
        Ok(())
    }
}
