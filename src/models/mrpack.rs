use std::{collections::HashMap, fmt, path::PathBuf};

use serde::Deserialize;

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

impl From<&str> for DependencyID {
    fn from(value: &str) -> Self {
        match value {
            "minecraft" => DependencyID::Minecraft,
            "fabric" | "fabric-loader" => DependencyID::FabricLoader,
            "neoforge" => DependencyID::Neoforge,
            "quilt" | "quilt-loader" => DependencyID::QuiltLoader,
            "forge" => DependencyID::Forge,
            _ => panic!("Couldn't convert the string into Dependency ID"),
        }
    }
}
