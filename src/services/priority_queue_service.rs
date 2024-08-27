use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{info, warn, error};
extern crate redis;

pub struct PriorityQueueService {
    connection_manager: ConnectionManager,
    queue_name: String,
    retry_metadata: String,
    dead_letter_queue: String,
    time_weight: f64,
    max_retries: u32,
    initial_backoff: u64,
}

impl PriorityQueueService {
    pub async fn new(redis_url: &str, time_weight: f64, max_retries: u32, initial_backoff: u64) -> Self {
        let client = Client::open(redis_url).expect("Invalid Redis URL");
        let connection_manager = ConnectionManager::new(client).await.unwrap();
        Self {
            connection_manager,
            queue_name: "url_priority_queue".into(),
            retry_metadata: "url_priority_queue:metadata".into(),
            dead_letter_queue: "url_priority_queue:dead_letter".into(),
            time_weight,
            max_retries,
            initial_backoff,
        }
    }

    async fn get_timestamp() -> f64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs_f64()
    }

    pub async fn add_url(&self, url: &str, priority: f64) {
        let timestamp = Self::get_timestamp().await;
        let mut conn = self.connection_manager.clone();

        conn.zadd(&self.queue_name, url, priority).await.unwrap();
        conn.hset(&self.retry_metadata, url, format!("{}:0", timestamp)).await.unwrap();

        info!("Added URL: {} with priority: {} to the queue.", url, priority);
    }

    pub async fn fetch_next_url(&self) -> Option<(String, f64)> {
        let mut conn = self.connection_manager.clone();
        let urls: Vec<(String, f64)> = conn.zrangebyscore_withscores(&self.queue_name, "-inf", "+inf").await.unwrap();

        if urls.is_empty() {
            info!("No URLs available in the queue.");
            return None;
        }

        let mut highest_priority_url = None;
        let mut lowest_adjusted_score = f64::INFINITY;

        for (url, priority) in urls {
            let metadata: String = conn.hget(&self.retry_metadata, &url).await.unwrap();
            let parts: Vec<&str> = metadata.split(':').collect();
            let timestamp: f64 = parts[0].parse().unwrap();
            let time_in_queue = Self::get_timestamp().await - timestamp;
            let adjusted_score = self.calculate_adjusted_priority(priority, time_in_queue);

            if adjusted_score < lowest_adjusted_score {
                lowest_adjusted_score = adjusted_score;
                highest_priority_url = Some((url, lowest_adjusted_score));
            }
        }

        if let Some((url, adjusted_score)) = &highest_priority_url {
            conn.zrem(&self.queue_name, url).await.unwrap();
            conn.hdel(&self.retry_metadata, url).await.unwrap();
            info!("Fetched URL: {} with adjusted score: {} from the queue.", url, adjusted_score);
        }

        highest_priority_url
    }

    pub async fn retry_url(&self, url: &str, priority: f64) {
        let mut conn = self.connection_manager.clone();
        let metadata: Option<String> = conn.hget(&self.retry_metadata, url).await.unwrap();
        let retries = if let Some(metadata) = metadata {
            let parts: Vec<&str> = metadata.split(':').collect();
            parts[1].parse::<u32>().unwrap() + 1
        } else {
            1
        };

        if retries >= self.max_retries {
            warn!("URL {} reached max retries. Moving to dead-letter queue.", url);
            self.move_to_dead_letter_queue(url).await;
            return;
        }

        let backoff_delay = self.initial_backoff * (2u64.pow(retries - 1));
        let adjusted_priority = priority - (self.time_weight * backoff_delay as f64);
        let next_retry_time = Self::get_timestamp().await + backoff_delay as f64;

        conn.zadd(&self.queue_name, url, adjusted_priority).await.unwrap();
        conn.hset(&self.retry_metadata, url, format!("{}:{}", next_retry_time, retries)).await.unwrap();

        info!("Retrying URL: {} with adjusted priority: {} after {} seconds", url, adjusted_priority, backoff_delay);
    }

    async fn move_to_dead_letter_queue(&self, url: &str) {
        let mut conn = self.connection_manager.clone();
        conn.lpush(&self.dead_letter_queue, url).await.unwrap();
        conn.zrem(&self.queue_name, url).await.unwrap();
        conn.hdel(&self.retry_metadata, url).await.unwrap();
        error!("Moved URL: {} to the dead-letter queue.", url);
    }

    pub async fn process_dead_letter_queue(&self) {
        let mut conn = self.connection_manager.clone();
        loop {
            let url: Option<String> = conn.rpop(&self.dead_letter_queue).await.unwrap();
            if let Some(url) = url {
                error!("Processing URL from dead-letter queue: {}", url);
                // Further handling of dead-letter URLs as needed.
            } else {
                info!("Dead-letter queue is empty.");
                break;
            }
        }
    }

    fn calculate_adjusted_priority(&self, priority: f64, time_in_queue: f64) -> f64 {
        priority - (self.time_weight * time_in_queue)
    }
}
