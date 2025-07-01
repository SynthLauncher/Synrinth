use crate::{
    errors::SynrinthErr,
    models::project::{Project, ProjectFile, ProjectVersion},
};
use reqwest::Client;
use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

/// Fetches the project details for provided slug from the Modrinth API
///
/// # Args
///
/// * `client` - A reqwest HTTP client.
/// * `slug` - The project identifier string.
///
/// # Returns
///
/// Returns a `Result` with deserialized `Project` on success,
/// on an Error `E` convertible from either `reqwest::Error` or `serde_json::Error`
///
/// # Example
/// ```no_run
/// async fn run() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::Client::new();
///     let project = query_project(&client, "map").await?;
///     println!("{:#?}", project);
///     Ok(())
/// }
/// ```
#[must_use]
pub async fn query_project(client: &Client, slug: &str) -> Result<Project, SynrinthErr> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

/// Fetches all versions of the project identified by `slug` from the Modrinth API
///
/// # Args
///
/// * `client` - A reqwest HTTP client.
/// * `slug` - The project identifier string.
///
/// # Returns
///
/// Returns a `Result` with Vec<ProjectVersion> on success,
/// or an Error `E` convertible from either `reqwest::Error or `serde_json::Error`
///
/// # Example
/// ```no_run
/// async fn run() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::Client::new();
///     let projects = query_project_versions(&client, "map").await?;
///     
///     for project in projects {
///         println!("{:#?}", project);
///     }
///
///     Ok(())
/// }
/// ```
#[must_use]
pub async fn query_project_versions(
    client: &Client,
    slug: &str,
) -> Result<Vec<ProjectVersion>, SynrinthErr> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

/// Fetches details of a particular project version by `slug` and `version` from the Modrinth API
///
/// # Args
///
/// * `client` - A reqwest HTTP client.
/// * `slug` - The project identifier string.
/// * `version` - The version identifier string.
///
/// # Returns
///
/// Returns a `Result` with ProjectVersion on success,
/// or an Error `E` convertible from either `reqwest::Error` or `serde_json::Error`
///
/// # Example
/// ```no_run
/// async fn run() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::Client::new();
///     let project_version = query_project_version::<Box<dyn std::error::Error>>(&client, "map", "1.2").await?;
///     println!("{:#?}", project_version);
///     Ok(())
/// }
/// ```
#[must_use]
pub async fn query_project_version(
    client: &Client,
    slug: &str,
    version: &str,
) -> Result<ProjectVersion, SynrinthErr> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

pub async fn download_project_file(
    client: &Client,
    project_file: &ProjectFile,
    dest: &Path,
) -> Result<PathBuf, SynrinthErr> {
    let mut res = client.get(&project_file.url).send().await?;
    let path = dest.join(&project_file.filename);

    let file = std::fs::File::create(&path)?;
    let mut writer = BufWriter::new(file);

    while let Some(chunk) = res.chunk().await? {
        writer.write_all(&chunk)?;
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use crate::api::project::{query_project, query_project_version, query_project_versions};
    use reqwest::Client;

    #[tokio::test]
    async fn query_project_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let project = query_project(&client, "map").await?;

        println!("{:#?}", project);

        Ok(())
    }

    #[tokio::test]
    async fn query_project_versions_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let projects = query_project_versions(&client, "map").await?;

        for project in projects {
            println!("{:#?}", project);
        }

        Ok(())
    }

    #[tokio::test]
    async fn query_project_version_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let project_version = query_project_version(&client, "map", "1.2").await?;

        println!("{:#?}", project_version);

        Ok(())
    }
}
