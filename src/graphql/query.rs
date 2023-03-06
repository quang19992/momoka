use super::context::Context;
use juniper::FieldResult;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn health_check(_ctx: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}
