use std::fmt;

use serde::{Deserialize, Serialize};

use super::{MonetizationStatus, SupportRequirement};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Search {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FacetOp {
    Eq,  // :
    Neq, // !=
    Gte, // >=
    Gt,  // >
    Lte, // <=
    Lt,  // <
}

#[derive(Debug, Deserialize, Serialize)]
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
