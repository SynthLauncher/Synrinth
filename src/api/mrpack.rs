use std::{fs::File, io::{BufReader, Write}, path::Path};

use reqwest::Client;
use tokio::fs::create_dir_all;
use zip::ZipArchive;

use crate::{errors::SynrinthError, models::mrpack::{MRPack, ModpackFile}};


pub async fn unpack_modpack(mrpack: &Path, output_dir: &Path) -> zip::result::ZipResult<()> {
    let file = File::open(mrpack)?;
    let reader = BufReader::new(file);

    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(output_dir).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&out_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub async fn read_modpack_file(modpack: &Path) -> Result<MRPack, SynrinthError> {
    let path = modpack.join("modrinth.index.json");
    let json = tokio::fs::read_to_string(path).await?;
    let mrpack: MRPack = serde_json::from_str(&json)?;
    Ok(mrpack)
}

pub async fn download_modpack_file(client: &Client, instance_path: &Path, modpack_file: &ModpackFile) -> Result<(), SynrinthError> {
    let mut res = client.get(&modpack_file.downloads[0]).send().await?;
    let path = instance_path.join(&modpack_file.path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent).await?;
    }
    let mut file = File::create(&path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}

pub async fn download_modpack_files(client: &Client, instance_path: &Path, modpack_files: &Vec<ModpackFile>) -> Result<(), SynrinthError> {
    for modpack_file in modpack_files {
        download_modpack_file(&client, &instance_path, &modpack_file).await?;
    }

    Ok(())
}
