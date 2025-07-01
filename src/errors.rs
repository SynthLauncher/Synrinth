use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynrinthError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}