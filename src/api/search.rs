use reqwest::Client;

use crate::{models::search::{FacetFilter, QueryParams, Search}};

pub fn build_facets<E>(facets: &[&[FacetFilter]]) -> Result<Option<String>, E> 
    where E: From<serde_json::error::Error>
{
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

pub async fn query_search<E>(client: &Client, params: QueryParams) -> Result<Search, E> 
    where E: From<reqwest::Error> + From<serde_json::error::Error>
{
    let mut url = "https://api.modrinth.com/v2/search".to_string();
    let mut query_parts = vec![];

    if let Some(query) = params.query {
        if !query.trim().is_empty() {
            query_parts.push(format!("query={}", &query));
        }
    }

    if let Some(facets) = params.facets {
        let inner: Vec<&[FacetFilter]> = facets.iter().map(|x| x.as_slice()).collect();
        if let Some(facets_str) = build_facets::<E>(&inner)? {
            query_parts.push(format!("facets={}", &facets_str));
        }
    }

    if !query_parts.is_empty() {
        url = format!("{}?{}", url, query_parts.join("&"));
    }

    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}
