
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

use crate::{config::CrankerConfig, services};

#[derive(Deserialize)]
struct UpdatePoolRequest {
    pool: String,
}

#[get("/")]
async fn hello_cranker() -> HttpResponse {
    HttpResponse::Ok().body("Cranker service is running")
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Cranker service is healthy")
}

#[post("/cranker/update")]
async fn update_pool(
    body: web::Json<UpdatePoolRequest>,
    config: web::Data<CrankerConfig>,
) -> HttpResponse {
    match services::cranker_service::update(&body.pool, &config.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(e) => {
            tracing::error!("Failed to update pool: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update pool")
        }
    }
}
