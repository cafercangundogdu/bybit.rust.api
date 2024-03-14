use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

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