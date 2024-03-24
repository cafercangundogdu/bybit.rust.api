use fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    #[serde(rename = "retCode")]
    pub code: String,
    #[serde(rename = "retMsg")]
    pub message: String,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<APIError> code={}, msg={}", self.code, self.message)
    }
}

impl Error for APIError {}
