use actix_web::{post, web, HttpResponse, Responder};
use crate::models::url_data::UrlData;
use crate::services::priority_queue_service::PriorityQueueService;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::debug;

#[post("/add_url")]
async fn add_url(data: web::Json<UrlData>, queue_service: web::Data<Arc<Mutex<PriorityQueueService>>>) -> impl Responder {
    debug!("Received request to add URL: {}", data.url);
    
    let priority = data.priority.unwrap_or(10.0);
    debug!("Priority set to: {}", priority);

    {
        let mut service = queue_service.lock().await;
        debug!("Lock acquired on PriorityQueueService");
        service.add_url(&data.url, priority).await;
        debug!("URL added to the queue: {}", data.url);
    }

    HttpResponse::Ok().json(format!("URL added to the queue: {}", data.url))
}

