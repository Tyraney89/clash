use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to fetch player data: {0}")]
    FetchError(String),
    
    #[error("Invalid player tag: {0}")]
    InvalidPlayerTag(String),
    
    #[error("API error: {reason} - {message}")]
    ClashApiError { reason: String, message: String },
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::FetchError(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::InternalError(format!("JSON parsing error: {}", err))
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(err: actix_web::Error) -> Self {
        ApiError::InternalError(err.to_string())
    }
}
