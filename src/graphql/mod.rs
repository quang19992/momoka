use actix_web::web;

mod handler;
pub mod mutation;
pub mod query;
pub mod context;
pub mod schema;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::graphql);
}
