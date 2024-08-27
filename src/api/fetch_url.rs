use actix_web::{get, web, HttpResponse, Responder};
use crate::services::priority_queue_service::PriorityQueueService;
use std::sync::Arc;
use tokio::sync::Mutex;

#[get("/fetch_url")]
async fn fetch_url(queue_service: web::Data<Arc<Mutex<PriorityQueueService>>>) -> impl Responder {
    if let Some((url, score)) = queue_service.lock().await.fetch_next_url().await {
        HttpResponse::Ok().json(format!("Fetched URL: {} with score: {}", url, score))
    } else {
        HttpResponse::Ok().json("No URLs available in the queue")
    }
}
