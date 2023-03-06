use super::context::Context;
use juniper::FieldResult;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn empty(_ctx: &Context) -> FieldResult<bool> {
        Ok(true)
    }
}
