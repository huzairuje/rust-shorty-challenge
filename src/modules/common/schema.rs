use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTinyUrl {
    pub shortcode: String,
    pub url: String,
}