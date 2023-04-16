use crate::server_config::ServerConfig;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::sync::Arc;

mod database;
mod graphql;
mod server_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match ServerConfig::load() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err),
    };
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = Arc::new(graphql::schema::create_schema());
    let database = Arc::new(
        database::bundle::Database::new(&config.database)
            .await
            .unwrap_or_else(|err| panic!("{:?}", err)),
    );

    let database_clone = database.clone();
    tokio::task::spawn_blocking(|| {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let _ = tokio::spawn(async move { database_clone.sync().await })
                .await
                .unwrap_or_else(|err| panic!("{:?}", err));
        });
    });

    log::info!("starting server on port {}", config.http_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(database.clone()))
            .configure(graphql::route)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(config.num_worker)
    .bind(("0.0.0.0", config.http_port))?
    .run()
    .await
}
