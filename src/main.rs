use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::CrankerConfig;
use dotenv::dotenv;
use routes::cranker_route::{health_check, hello_cranker, update_pool};
use std::sync::Arc;

pub mod config;
pub mod routes;
pub mod services;
pub mod tx_utils;
pub mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Cranker...");

    let config = Arc::new(
        CrankerConfig::get_config()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
    );
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .app_data(web::Data::new(config.clone()))
            .service(update_pool)
            .service(hello_cranker)
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

// cargo watch -c -w src -x run