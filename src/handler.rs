use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    #[serde(rename = "retCode")]
    pub code: String,
    #[serde(rename = "retMsg")]
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<APIError> code={}, msg={}", self.code, self.message)
    }
}


impl Error for ValidationError {}
impl Error for APIError {}


pub fn validate_params(params: &HashMap<String, Option<String>>) -> Result<(), ValidationError> {
    let mut seen_keys = HashMap::new();

    for (key, value) in params {
        if key.is_empty() {
            return Err(ValidationError {
                message: "empty key found in parameters".to_string(),
            });
        }
        if seen_keys.contains_key(key) {
            return Err(ValidationError {
                message: format!("duplicate key found in parameters: {}", key),
            });
        }
        if value.is_none() {
            return Err(ValidationError {
                message: format!("parameter for key '{}' is nil", key),
            });
        }
        seen_keys.insert(key, true);
    }
    Ok(())
}

pub fn is_api_error(e: &dyn Error) -> bool {
    e.downcast_ref::<APIError>().is_some()
}