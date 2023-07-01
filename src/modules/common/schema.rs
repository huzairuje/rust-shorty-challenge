use serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestTinyUrl {
    pub shortcode: String,
    #[validate(custom = "validate_url")]
    pub url: String,
}

fn validate_url(url: &str) -> Result<(), ValidationError> {
    if url.len() < 1 {
        return Err(ValidationError::new(
            "url must have minimum length of 1 characters",
        ));
    }
    Ok(())
}