use std::{fs::File, io::{BufReader, Write}, path::Path};

use reqwest::Client;
use zip::ZipArchive;

use crate::{models::mrpack::{MRPack, ModpackFile}};

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

pub fn read_modpack_file<E>(modpack: &Path) -> Result<MRPack, E> 
    where E: From<std::io::Error> + From<serde_json::Error>
{
    let path = modpack.join("modrinth.index.json");
    let json = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub async fn download_modpack_file<E>(client: &Client, instance_path: &Path, modpack_file: &ModpackFile) -> Result<(), E> 
    where E: From<std::io::Error> + From<reqwest::Error>
{
    let mut res = client.get(&modpack_file.downloads[0]).send().await?;
    let path = instance_path.join(&modpack_file.path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(&path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}

pub async fn download_modpack_files<E>(client: &Client, instance_path: &Path, modpack_files: &[ModpackFile]) -> Result<(), E> 
    where E: From<std::io::Error> + From<reqwest::Error>
{
    for modpack_file in modpack_files {
        download_modpack_file::<E>(&client, &instance_path, &modpack_file).await?;
    }

    Ok(())
}
