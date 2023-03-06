use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer, web};

mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = std::sync::Arc::new(graphql::schema::create_schema());

    log::info!("starting server on port {}", 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .configure(graphql::route)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
