use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
    vec,
};

use reqwest::Client;
use tokio::fs::create_dir_all;
use zip::ZipArchive;

use crate::{
    errors::SynrinthErrors,
    structs::{FacetFilter, MRPack, ModpackFile, Project, ProjectFile, QueryParams, Search, ProjectVersion},
};

pub fn build_facets(facets: &Vec<Vec<FacetFilter>>) -> Result<Option<String>, SynrinthErrors> {
    if facets.is_empty() {
        return Ok(None);
    }

    let mut json_facets: Vec<Vec<String>> = vec![];

    for group in facets {
        if !group.is_empty() {
            json_facets.push(group.iter().map(|f| f.to_string()).collect());
        }
    }

    if json_facets.is_empty() {
        return Ok(None);
    }

    Ok(Some(serde_json::to_string(&json_facets)?))
}

pub async fn query_search(client: &Client, params: QueryParams) -> Result<Search, SynrinthErrors> {
    let mut url = "https://api.modrinth.com/v2/search".to_string();
    let mut query_parts = vec![];

    if let Some(query) = params.query {
        if !query.trim().is_empty() {
            query_parts.push(format!("query={}", &query));
        }
    }

    if let Some(facets) = params.facets {
        if let Some(facets_str) = build_facets(&facets)? {
            query_parts.push(format!("facets={}", &facets_str));
        }
    }

    if !query_parts.is_empty() {
        url = format!("{}?{}", url, query_parts.join("&"));
    }

    let res = client.get(url).send().await?.bytes().await?;
    let json: Search = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn query_project(client: &Client, slug: &str) -> Result<Project, SynrinthErrors> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let res = client.get(url).send().await?.bytes().await?;
    let json: Project = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn query_project_versions(client: &Client, slug: &str) -> Result<Vec<ProjectVersion>, SynrinthErrors> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);
    let res = client.get(url).send().await?.bytes().await?;
    let json: Vec<ProjectVersion> = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn download_project_file(client: &Client, project_file: &ProjectFile, dest: &Path) -> Result<PathBuf, SynrinthErrors> {
    let mut res = client.get(&project_file.url).send().await?;
    let path = dest.join(&project_file.filename);
    let mut file = File::create(&path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(path)
}

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

pub async fn read_modpack_file(modpack: &Path) -> Result<MRPack, SynrinthErrors> {
    let path = modpack.join("modrinth.index.json");
    let json = tokio::fs::read_to_string(path).await?;
    let mrpack: MRPack = serde_json::from_str(&json)?;
    Ok(mrpack)
}

pub async fn download_modpack_file(client: &Client, instance_path: &Path, modpack_file: &ModpackFile) -> Result<(), SynrinthErrors> {
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

pub async fn download_modpack_files(client: &Client, instance_path: &Path, modpack_files: &Vec<ModpackFile>) -> Result<(), SynrinthErrors> {
    for modpack_file in modpack_files {
        download_modpack_file(&client, &instance_path, &modpack_file).await?;
    }

    Ok(())
}
