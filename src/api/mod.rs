use actix_web::web;

mod add_url;
mod fetch_url;
mod retry_url;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_url::add_url)
       .service(fetch_url::fetch_url)
       .service(retry_url::retry_url);
}
