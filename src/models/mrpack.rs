use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

use crate::errors::SynrinthErr;

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
    Minecraft,
    Forge,
    Neoforge,
    FabricLoader,
    QuiltLoader,
}

impl TryFrom<&str> for DependencyID {
    type Error = SynrinthErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "minecraft" => Ok(Self::Minecraft),
            "fabric" | "fabric-loader" => Ok(Self::FabricLoader),
            "neoforge" => Ok(Self::Neoforge),
            "quilt" | "quilt-loader" => Ok(Self::QuiltLoader),
            "forge" => Ok(Self::Forge),
            invalid => Err(SynrinthErr::InvalidDependency(invalid.to_string())),
        }
    }
}
