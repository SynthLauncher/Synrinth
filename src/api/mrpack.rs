use std::{fs::File, io::{BufReader, BufWriter, Write}, path::Path};
use reqwest::Client;
use zip::ZipArchive;
use crate::{errors::SynrinthErr, models::mrpack::{MRPack, ModpackFile}};

pub async fn unpack_modpack(mrpack: &Path, output_dir: &Path) -> Result<(), SynrinthErr> {
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

pub fn read_modpack_file(modpack: &Path) -> Result<MRPack, SynrinthErr> {
    let json = std::fs::read_to_string(modpack.join("modrinth.index.json"))?;
    Ok(serde_json::from_str(&json)?)
}

pub async fn download_modpack_file(client: &Client, path: &Path, modpack_file: &ModpackFile) -> Result<(), SynrinthErr> {
    let mut res = client.get(&modpack_file.downloads[0]).send().await?;
    let path = path.join(&modpack_file.path);
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    while let Some(chunk) = res.chunk().await? {
        writer.write_all(&chunk)?;
    }

    Ok(())
}

pub async fn download_modpack_files(client: &Client, instance_path: &Path, modpack_files: &[ModpackFile]) -> Result<(), SynrinthErr> {
    for modpack_file in modpack_files {
        download_modpack_file(&client, instance_path, modpack_file).await?;
    }
    
    Ok(())
}