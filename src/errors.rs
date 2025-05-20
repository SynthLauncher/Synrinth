use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynrinthErrors {
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error)
}