use super::{context::Context, mutation::Mutation, query::Query};
use juniper::{self, EmptySubscription};

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::<Context>::new())
}
