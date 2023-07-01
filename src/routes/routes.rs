use crate::modules::tiny_url::route as tiny_url_routes;
use actix_web::web;

pub fn initiate_routes(conf: &mut web::ServiceConfig) {
    //register all the routes here
    let scope = web::scope("/api/v1")
        .configure(tiny_url_routes::routes);

    conf.service(scope);
}
