use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportRequirement {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusType {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Withheld,
    Scheduled,
    Private,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatusType {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}

#[derive(Debug, Deserialize)]
pub struct GalleryImage {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: String, // format: ISO-8601
    pub ordering: i32,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DonationURL {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub slug: String,
    pub titile: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: SupportRequirement,
    pub server_side: SupportRequirement,
    pub body: String,
    pub status: StatusType,
    pub requested_status: Option<RequestedStatusType>,
    pub additional_categories: Vec<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Vec<DonationURL>,
    pub project_type: ProjectType,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub color: Option<i32>,
    pub thread_id: String,
    pub monetization_status: MonetizationStatus,
    pub id: String,
    pub team: String,
    pub body_url: Option<String>,
    pub moderator_message: ModeratorMessage,
    pub published: String,        // format: ISO-8601
    pub updated: String,          // format: ISO-8601
    pub approved: Option<String>, // format: ISO-8601
    pub queued: Option<String>,   // format: ISO-8601
    pub followers: u32,
    pub license: License,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub gallery: Vec<GalleryImage>,
}
