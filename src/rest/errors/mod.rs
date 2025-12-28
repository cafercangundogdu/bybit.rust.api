use thiserror::Error;

#[derive(Error, Debug)]
pub enum BybitError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Serialization error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),

    #[error("API error: {0}")]
    Api(ErrorCodes),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type BybitResult<T> = std::result::Result<T, BybitError>;

include!(concat!(env!("OUT_DIR"), "/error_codes.rs"));
