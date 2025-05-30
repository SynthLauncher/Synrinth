use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynrinthErrors {
    #[error("[Synrinth] Serialization error: {0}")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("[Synrinth] Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("[Synthrinth] IO Error: {0}")]
    IOError(#[from] std::io::Error)
}