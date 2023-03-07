use crate::server_config::ServerConfig;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

mod graphql;
mod server_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match ServerConfig::load() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err),
    };
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = std::sync::Arc::new(graphql::schema::create_schema());

    log::info!("starting server on port {}", config.http_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .configure(graphql::route)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(config.num_worker)
    .bind(("0.0.0.0", config.http_port))?
    .run()
    .await
}
