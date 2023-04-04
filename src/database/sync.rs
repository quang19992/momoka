use super::{bundle::Database, error::DatabaseError};
use futures::future::BoxFuture;
use std::{future::Future, result::Result, sync::Arc};

type SyncResponse = Result<(), DatabaseError>;

struct SyncFn {
    function: Box<dyn Fn(Database) -> BoxFuture<'static, SyncResponse>>,
}

impl SyncFn {
    fn new<F: Future<Output = SyncResponse> + std::marker::Send + 'static>(
        function: fn(Database) -> F,
    ) -> Self {
        Self {
            function: Box::new(move |database| Box::pin(function(database))),
        }
    }
}

trait SyncSupport {}

// TODO - implement this
// this will help in case of a sync failure
struct SyncState;

enum Synchronizer<'a> {
    Simple(&'a [&'a str]),
    Custom(&'a SyncFn),
    Mixed(Arc<&'a [&'a Synchronizer<'a>]>),
}

impl Synchronizer<'_> {
    async fn execute<T: SyncSupport>(
        bundle: Database,
        database: T,
        state: Option<SyncState>,
    ) -> SyncResponse {
        todo!()
    }
}
