use std::{fs::File, io::{BufReader, Write}, path::Path};
use reqwest::Client;
use thiserror::Error;
use zip::ZipArchive;
use crate::models::mrpack::{MRPack, ModpackFile};

#[derive(Debug, Error)]
pub enum ModpackError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}

pub async fn unpack_modpack(mrpack: &Path, output_dir: &Path) -> Result<(), ModpackError> {
    let mut archive = ZipArchive::new(BufReader::new(File::open(mrpack)?))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::io::copy(&mut file, &mut File::create(&out_path)?)?;
        }
    }
    Ok(())
}

pub fn read_modpack_file(modpack: &Path) -> Result<MRPack, ModpackError> {
    let json = std::fs::read_to_string(modpack.join("modrinth.index.json"))?;
    Ok(serde_json::from_str(&json)?)
}

pub async fn download_modpack_file(client: &Client, instance_path: &Path, modpack_file: &ModpackFile) -> Result<(), ModpackError> {
    let mut res = client.get(&modpack_file.downloads[0]).send().await?;
    let path = instance_path.join(&modpack_file.path);
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let mut file = File::create(&path)?;
    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }
    Ok(())
}

pub async fn download_modpack_files(client: &Client, instance_path: &Path, modpack_files: &[ModpackFile]) -> Result<(), ModpackError> {
    for modpack_file in modpack_files {
        download_modpack_file(&client, instance_path, modpack_file).await?;
    }
    Ok(())
}