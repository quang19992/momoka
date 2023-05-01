use actix_web::web;

pub mod context;
pub mod error;
mod handler;
pub mod mutation;
pub mod query;
pub mod schema;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::graphql);
}
