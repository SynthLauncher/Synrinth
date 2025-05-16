use std::{fs::File, io::Write, path::Path, vec};

use reqwest::Client;

use crate::structs::{FacetFilter, Project, ProjectFile, QueryParams, Search, VersionProject};

pub fn build_facets(facets: &Vec<Vec<FacetFilter>>) -> Option<String> {
    if facets.is_empty() {
        return None;
    }

    let mut json_facets: Vec<Vec<String>> = vec![];

    for group in facets {
        if !group.is_empty() {
            json_facets.push(group.iter().map(|f| f.to_string()).collect());
        }
    }

    if json_facets.is_empty() {
        return None;
    }

    Some(serde_json::to_string(&json_facets).unwrap())
}

pub async fn search(client: &Client, params: QueryParams) -> Search {
    let mut url = "https://api.modrinth.com/v2/search".to_string();
    let mut query_parts = vec![];

    if let Some(query) = params.query {
        if !query.trim().is_empty() {
            query_parts.push(format!("query={}", &query));
        }
    }

    if let Some(facets) = params.facets {
        if let Some(facets_str) = build_facets(&facets) {
            query_parts.push(format!("facets={}", &facets_str));
        }
    }

    if !query_parts.is_empty() {
        url = format!("{}?{}", url, query_parts.join("&"));
    }

    println!("{}", url);

    let res = client.get(url).send().await.unwrap().bytes().await.unwrap();
    let json: Search = serde_json::from_slice(&res).unwrap();
    json
}

pub async fn get_project(client: &Client, slug: &str) -> Project {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let res = client.get(url).send().await.unwrap().bytes().await.unwrap();
    let json: Project = serde_json::from_slice(&res).unwrap();
    json
}

pub async fn get_version_project(client: &Client, slug: &str) -> Vec<VersionProject> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);
    let res = client.get(url).send().await.unwrap().bytes().await.unwrap();
    let json: Vec<VersionProject> = serde_json::from_slice(&res).unwrap();
    json
}

pub async fn download_file(client: &Client, project_file: &ProjectFile, dest: &Path) {
    let mut res = client.get(&project_file.url).send().await.unwrap();

    let mut file = File::create(dest.join(&project_file.filename)).unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        file.write_all(&chunk).unwrap();
    }
}

#[tokio::test]
async fn test() {
    let client: Client = Client::new();
    let slug = "xaeros-world-map";
    let version_projects = get_version_project(&client, &slug).await;
    let project_file = &version_projects[0].files[0];
    download_file(&client, &project_file, &Path::new("./")).await;
}
