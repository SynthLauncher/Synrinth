use std::{collections::HashMap, fmt, path::PathBuf};

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
    Listed,
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
    Listed,
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
    pub title: String,
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
    pub moderator_message: Option<ModeratorMessage>,
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

#[derive(Debug, Deserialize)]
pub struct Hit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: SupportRequirement,
    pub server_side: SupportRequirement,
    pub project_type: String,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub color: Option<i32>,
    pub thread_id: Option<String>,
    pub monetization_status: Option<MonetizationStatus>,
    pub project_id: String,
    pub author: String,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub follows: u32,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub gallery: Vec<String>,
    pub featured_gallery: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacetType {
    ProjectType,
    Categories,
    Versions,
    ClientSide,
    ServerSide,
    OpenSource,
    Title,
    Author,
    Follows,
    ProjectId,
    License,
    Downloads,
    Color,
    CreatedTimestamp,
    ModifiedTimestamp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FacetOp {
    Eq,  // :
    Neq, // !=
    Gte, // >=
    Gt,  // >
    Lte, // <=
    Lt,  // <
}

#[derive(Debug, Deserialize)]
pub struct FacetFilter {
    pub facet: FacetType,
    pub op: FacetOp,
    pub value: String,
}

impl fmt::Display for FacetOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            FacetOp::Eq => ":",
            FacetOp::Neq => "!=",
            FacetOp::Gte => ">=",
            FacetOp::Gt => ">",
            FacetOp::Lte => "<=",
            FacetOp::Lt => "<",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for FacetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FacetType::ProjectType => "project_type",
            FacetType::Categories => "categories",
            FacetType::Versions => "versions",
            FacetType::ClientSide => "client_side",
            FacetType::ServerSide => "server_side",
            FacetType::OpenSource => "open_source",
            FacetType::Title => "title",
            FacetType::Author => "author",
            FacetType::Follows => "follows",
            FacetType::ProjectId => "project_id",
            FacetType::License => "license",
            FacetType::Downloads => "downloads",
            FacetType::Color => "color",
            FacetType::CreatedTimestamp => "created_timestamp",
            FacetType::ModifiedTimestamp => "modified_timestamp",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FacetFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.facet, self.op, self.value)
    }
}

pub struct QueryParams {
    pub query: Option<String>,
    pub facets: Option<Vec<Vec<FacetFilter>>>,
}

#[derive(Debug, Deserialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u32,
    pub file_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectVersion {
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: Option<String>,
    pub data_published: Option<String>,
    pub downloads: u32,
    pub version_type: String,
    pub status: StatusType,
    pub requested_status: Option<RequestedStatusType>,
    pub files: Vec<ProjectFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MRPack {
    pub dependencies: HashMap<DependencyID, String>,
    pub files: Vec<ModpackFile>,
    pub format_version: u32,
    pub game: String,
    pub name: String,
    pub version_id: String,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModpackFile {
    pub path: PathBuf,
    pub hashes: FileHashes,
    pub env: Option<Env>,
    pub downloads: Vec<String>,
    pub file_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
    pub other_hashes: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Env {
    pub client: EnvTypes,
    pub server: EnvTypes,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnvTypes {
    Required,
    Optional,
    Unsupported,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DependencyID {
    Minecraft, // Vanilla
    Forge,
    Neoforge,
    FabricLoader,
    QuiltLoader,
}
