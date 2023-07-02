use crate::modules::tiny_url::service;
use crate::modules::common::model::Data;
use crate::modules::common::schema::RequestTinyUrl;
use crate::infrastructure::http_lib::Response;
use actix_web::{web, HttpResponse, post, get};
use actix_web::http::StatusCode;
use validator::{Validate, ValidationErrors};

#[post("")]
pub async fn create_tiny_url(body: web::Json<RequestTinyUrl>) -> HttpResponse {
    //validate the struct from body
    if let Err(errors) = body.validate() {
        let resp: Response<(), ValidationErrors> = Response::custom(
            StatusCode::BAD_REQUEST,
            StatusCode::BAD_REQUEST.as_str(),
            (),
            errors,
        );
        return HttpResponse::BadRequest().json(resp);
    }

    // set req after validation from body
    let req: RequestTinyUrl = body.into_inner();

    if !req.shortcode.is_empty() {
        if !service::is_valid_short_code(&req.shortcode) {
            let resp: Response<(), ()> =
                Response::error(StatusCode::BAD_REQUEST, "SHORT_CODE_FAILED_REGEX_PATTERN");
            return HttpResponse::Ok().json(resp);
        }

        if let Some(_) = service::get_single_data(&req.shortcode) {
            let resp: Response<(), ()> =
                Response::error(StatusCode::BAD_REQUEST, "SHORT_CODE_EXIST");
            return HttpResponse::Ok().json(resp);
        }
    }

    let dto_obj = Data {
        shortcode: req.shortcode,
        url: req.url,
        start_date: "".to_string(),
        last_seen_date: "".to_string(),
        redirect_count: 0,
    };

    return match service::create_data(dto_obj) {
        Ok(short_code) => {
            let resp: Response<String, ()> =
                Response::success(StatusCode::OK, short_code, "OK");
            HttpResponse::Ok().json(resp)
        }
        Err(err) => {
            let resp: Response<String, ()> =
                Response::error(StatusCode::INTERNAL_SERVER_ERROR, err.as_str());
            HttpResponse::InternalServerError().json(resp)
        }
    }
}

#[get("")]
pub async fn get_all_tiny_url() -> HttpResponse {
    let list: Vec<Data> = service::get_all_data();
    let resp: Response<Vec<Data>, ()> =
        Response::success(StatusCode::OK, list, "OK");
    return HttpResponse::Ok().json(resp);
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    let resp: Response<String, ()> =
        Response::success(StatusCode::OK, "healthy!".to_string(), "OK");
    return HttpResponse::Ok().json(resp);
}

#[get("/{shortcode}")]
pub async fn single_tiny_url(shortcode: web::Path<String>) -> HttpResponse {
    let shortcode: String = shortcode.into_inner();
    match service::get_single_data(&shortcode) {
        Some(data) => {
            service::update_stat(&data.shortcode);
            HttpResponse::Found()
                .append_header(("Location", data.url.clone()))
                .finish()
        }
        None => {
            let resp: Response<(), ()> =
                Response::error(StatusCode::NOT_FOUND, "SHORT_CODE_NOT_FOUND");
            return HttpResponse::NotFound().json(resp);
        }
    }
}

#[get("/{shortcode}/stats")]
pub async fn stat_single_tiny_url(shortcode: web::Path<String>) -> HttpResponse {
    let shortcode: String = shortcode.into_inner();
    match service::get_single_data(&shortcode) {
        Some(data) => {
            let resp: Response<Data, ()> =
                Response::success(StatusCode::OK, data, "OK");
            HttpResponse::Ok().json(resp)
        }
        None => {
            let resp: Response<(), ()> =
                Response::error(StatusCode::NOT_FOUND, "SHORT_CODE_NOT_FOUND");
            return HttpResponse::NotFound().json(resp);
        }
    }
}