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

pub async fn download_modpack_file(client: &Client, modpack_file: &ModpackFile) -> Result<(), SynrinthErrors> {
    let mut res = client.get(&modpack_file.downloads[0]).send().await?;
    
    if let Some(parent) = modpack_file.path.parent() {
        create_dir_all(parent).await?;
    }
    let mut file = File::create(&modpack_file.path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}

pub async fn download_modpack_files(client: &Client, modpack_files: &Vec<ModpackFile>) -> Result<(), SynrinthErrors> {
    for modpack_file in modpack_files {
        download_modpack_file(&client, &modpack_file).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use reqwest::Client;

    use crate::{api::{download_modpack_files, read_modpack_file}, errors::SynrinthErrors};

    // use crate::{api::{download_mod, download_mods, read_modpack_file, search}, errors::SynrinthErrors, structs::{FacetOp, FacetType}};

    // #[tokio::test]
    // async fn search_test() {
    //     let client = Client::new();
    //     let params = super::QueryParams {
    //         query: Some("map".to_string()),
    //         facets: Some(vec![vec![super::FacetFilter {
    //             facet: FacetType::Downloads,
    //             op: FacetOp::Eq,
    //             value: "1000".to_string(),
    //         }]]),
    //     };

    //     let result = search(&client, params).await;
    //     assert!(result.is_ok());
    // }

    // #[tokio::test]
    // async fn get_project_test() {
    //     let client = Client::new();
    //     let slug = "map";
    //     let result = super::get_project(&client, slug).await;
    //     assert!(result.is_ok());
    // }

    // #[tokio::test]
    // async fn get_version_project_test() {
    //     let client = Client::new();
    //     let slug = "map";
    //     let result = super::get_version_project(&client, slug).await;
    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn unpack_mrpack_test() {
        let mrpack_path = Path::new("./Fabulously.Optimized-v9.0.0-beta.3.mrpack");
        let output_dir = Path::new("test_output");

        let result = super::unpack_modpack(&mrpack_path, &output_dir).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_modpack_file_test() -> Result<(), SynrinthErrors> {
        let client = Client::new();
        let mrpack = read_modpack_file(Path::new("test_output")).await?;
        download_modpack_files(&client, &mrpack.files).await?;
        Ok(())
    }
}