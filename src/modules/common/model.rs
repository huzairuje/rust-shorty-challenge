use serde::{Deserialize, Serialize};
use std::sync::{ MutexGuard, Mutex };
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub shortcode: String,
    pub url: String,
    pub start_date: String,
    pub last_seen_date: String,
    pub redirect_count: i32,
}

lazy_static! {
    pub static ref LIST_ALL_TINY_URL: Mutex<Vec<Data>> = Mutex::new(Vec::new());
}

pub fn list_all_tiny_url() -> MutexGuard<'static, Vec<Data>> {
    LIST_ALL_TINY_URL.lock().unwrap()
}