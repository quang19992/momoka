use juniper;

#[derive(Clone)]
pub struct Context;

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}
