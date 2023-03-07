use actix_web::web;

pub mod context;
mod handler;
pub mod mutation;
pub mod query;
pub mod schema;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::graphql);
}
