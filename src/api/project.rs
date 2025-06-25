use crate::models::project::{Project, ProjectFile, ProjectVersion};
use reqwest::Client;
use std::{
    io::Write,
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
///     let project = query_project::<Box<dyn std::error::Error>>(&client, "map").await?;
///     println!("{:#?}", project);
///     Ok(())
/// }
/// ```
#[must_use]
pub async fn query_project<E>(client: &Client, slug: &str) -> Result<Project, E>
where
    E: From<reqwest::Error> + From<serde_json::Error>,
{
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
///     let projects = query_project_versions::<Box<dyn std::error::Error>>(&client, "map").await?;
///     
///     for project in projects {
///         println!("{:#?}", project);
///     }
///
///     Ok(())
/// }
/// ```
#[must_use]
pub async fn query_project_versions<E>(
    client: &Client,
    slug: &str,
) -> Result<Vec<ProjectVersion>, E>
where
    E: From<reqwest::Error> + From<serde_json::Error>,
{
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
pub async fn query_project_version<E>(
    client: &Client,
    slug: &str,
    version: &str,
) -> Result<ProjectVersion, E>
where
    E: From<reqwest::Error> + From<serde_json::Error>,
{
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

/// Downloads the file specified by `project_file` to the given destination directory.
///
/// # Arguments
///
/// * `client` - A reqwest HTTP client to perform the request.
/// * `project_file` - A reference to the [`ProjectFile`] containing the URL and filename.
/// * `dest` - The directory path where the file should be saved.
///
/// # Returns
///
/// Returns a `Result` containing the full path to the downloaded file on success,
/// or an error `E` convertible from either `reqwest::Error` or `std::io::Error`.
///
/// # Example
///
/// ```no_run
/// # async fn download_example() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = reqwest::Client::new();
/// # let project_file = /* get ProjectFile */ todo!();
/// let path = download_project_file::<Box<dyn std::error::Error>>(&client, &project_file, std::path::Path::new("/tmp")).await?;
/// println!("Downloaded to {:?}", path);
/// # Ok(())
/// # }
/// ```
pub async fn download_project_file<E>(
    client: &Client,
    project_file: &ProjectFile,
    dest: &Path,
) -> Result<PathBuf, E>
where
    E: From<reqwest::Error> + From<std::io::Error>,
{
    let mut res = client.get(&project_file.url).send().await?;
    let path = dest.join(&project_file.filename);
    let mut file = std::fs::File::create(&path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use crate::api::project::{query_project, query_project_version, query_project_versions};
    use reqwest::Client;

    #[tokio::test]
    async fn query_project_test() {
        let client = Client::new();

        match query_project::<Box<dyn std::error::Error>>(&client, "map").await {
            Ok(project) => println!("{:#?}", project),
            Err(e) => panic!("query_project failed: {}", e),
        }
    }

    #[tokio::test]
    async fn query_project_versions_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let projects = query_project_versions::<Box<dyn std::error::Error>>(&client, "map").await?;

        for project in projects {
            println!("{:#?}", project);
        }

        Ok(())
    }

    #[tokio::test]
    async fn query_project_version_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let project_version = query_project_version::<Box<dyn std::error::Error>>(&client, "map", "1.2").await?;

        println!("{:#?}", project_version);

        Ok(())
    }
}
