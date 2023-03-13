use crate::database::bundle::Database;
use actix_web::web;
use juniper;
use std::sync::Arc;

pub struct Context {
    pub database: web::Data<Arc<Database>>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(database: web::Data<Arc<Database>>) -> Self {
        Self { database }
    }
}
