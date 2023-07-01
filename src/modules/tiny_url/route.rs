use crate::modules::tiny_url::handler;
use actix_web::web;

pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/tiny-url")
        .service(handler::get_all_tiny_url)
        .service(handler::create_tiny_url)
        .service(handler::single_tiny_url)
        .service(handler::stat_single_tiny_url);

    conf.service(scope);
}