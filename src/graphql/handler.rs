use crate::graphql::{context::Context, schema::Schema};
use actix_web::{route, web, Error, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

#[route("/graphql", method = "POST")]
pub async fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context::new();

    let res = data.execute(&schema, &ctx).await;
    Ok(HttpResponse::Ok().json(res))
}
