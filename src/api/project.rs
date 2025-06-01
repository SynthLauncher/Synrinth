use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use reqwest::Client;

use crate::{
    errors::SynrinthError,
    models::project::{Project, ProjectFile, ProjectVersion},
};

pub async fn query_project(client: &Client, slug: &str) -> Result<Project, SynrinthError> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let res = client.get(url).send().await?.bytes().await?;
    let json: Project = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn query_project_versions(
    client: &Client,
    slug: &str,
) -> Result<Vec<ProjectVersion>, SynrinthError> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);
    let res = client.get(url).send().await?.bytes().await?;
    let json: Vec<ProjectVersion> = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn query_project_version(
    client: &Client,
    slug: &str,
    version: &str,
) -> Result<ProjectVersion, SynrinthError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let res = client.get(url).send().await?.bytes().await?;
    let json: ProjectVersion = serde_json::from_slice(&res)?;
    Ok(json)
}

pub async fn download_project_file(
    client: &Client,
    project_file: &ProjectFile,
    dest: &Path,
) -> Result<PathBuf, SynrinthError> {
    let mut res = client.get(&project_file.url).send().await?;
    let path = dest.join(&project_file.filename);
    let mut file = File::create(&path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(path)
}
