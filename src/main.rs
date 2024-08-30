use actix_web::{App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod services;
mod config;
mod utils;
mod models;

use services::priority_queue_service::PriorityQueueService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::logging::init_logger();
    let queue_service = Arc::new(Mutex::new(PriorityQueueService::new(
        &config::config::get_redis_url(),
        0.1,
        3,
        5,
    ).await));

    HttpServer::new(move || {
        App::new()
            .app_data(queue_service.clone())
            .configure(api::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
