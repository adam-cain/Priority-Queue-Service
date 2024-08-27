use actix_web::{post, web, HttpResponse, Responder};
use crate::models::url_data::UrlData;
use crate::services::priority_queue_service::PriorityQueueService;
use std::sync::Arc;
use tokio::sync::Mutex;

#[post("/retry_url")]
async fn retry_url(data: web::Json<UrlData>, queue_service: web::Data<Arc<Mutex<PriorityQueueService>>>) -> impl Responder {
    let priority = data.priority.unwrap_or(10.0);
    queue_service.lock().await.retry_url(&data.url, priority).await;
    HttpResponse::Ok().json(format!("URL scheduled for retry: {}", data.url))
}
