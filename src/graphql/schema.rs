use super::{mutation::Mutation, query::Query, context::Context};
use juniper::{self, EmptySubscription};

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::<Context>::new())
}
