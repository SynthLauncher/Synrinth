use serde::{Deserialize, Serialize};

pub mod project;
pub mod search;
pub mod mrpack;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API error: {0}")]
    Api(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SupportRequirement {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum StatusType {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Listed,
    Processing,
    Withheld,
    Scheduled,
    Private,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatusType {
    Approved,
    Archived,
    Unlisted,
    Listed,
    Private,
    Draft,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GalleryImage {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: String, // format: ISO-8601
    pub ordering: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    pub url: Option<String>,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DonationURL {
    pub id: String,
    pub platform: String,
    pub url: String,
}
