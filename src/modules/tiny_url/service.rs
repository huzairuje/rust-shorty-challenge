use crate::modules::common::model::{Data, list_all_tiny_url, LIST_ALL_TINY_URL};
use chrono::prelude::*;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::rngs::ThreadRng;
use regex::Regex;

pub fn get_all_data() -> Vec<Data> {
    list_all_tiny_url().clone()
}

pub fn get_single_data(short_code: &str) -> Option<Data> {
    if let Some(data) = list_all_tiny_url().iter().find(|v| v.shortcode == short_code) {
        Some(Data {
            shortcode: data.shortcode.clone(),
            url: data.url.clone(),
            start_date: data.start_date.clone(),
            last_seen_date: data.last_seen_date.clone(),
            redirect_count: data.redirect_count,
        })
    } else {
        None
    }
}

pub fn update_stat(short_code: &str) {
    let time_now : String = Utc::now().to_rfc3339();
    if let Some(data) = list_all_tiny_url().iter_mut().find(|v| v.shortcode == short_code) {
        data.redirect_count += 1;
        data.last_seen_date = time_now;
    }
}

pub fn create_data(mut data_req: Data) -> Result<String, String> {
    if data_req.shortcode.is_empty() {
        data_req.shortcode = generate_short_code()?;
        if !is_valid_short_code(&data_req.shortcode) {
            return Err("Shortcode generated is not valid, you can try again".to_string());
        }
    }

    let time_now = Utc::now().to_rfc3339();
    let new_data = Data {
        shortcode: data_req.shortcode,
        url: data_req.url,
        start_date: time_now.clone(),
        last_seen_date: time_now,
        redirect_count: 0,
    };

    LIST_ALL_TINY_URL.lock().unwrap().push(new_data.clone());

    Ok(new_data.shortcode)
}

fn generate_short_code() -> Result<String, String> {
    let rng: ThreadRng = rand::thread_rng();
    let short_code: String = rng
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(|c| c as char)
        .collect();
    Ok(short_code)
}

pub fn is_valid_short_code(short_code: &str) -> bool {
    let regex_query_param : Regex = Regex::new(r"^[0-9a-zA-Z_]{6}$").unwrap();
    regex_query_param.is_match(short_code)
}