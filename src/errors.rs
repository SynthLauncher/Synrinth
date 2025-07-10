use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynrinthErr {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
