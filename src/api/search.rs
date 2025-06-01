use reqwest::Client;

use crate::{errors::SynrinthError, models::search::{FacetFilter, QueryParams, Search}};

pub fn build_facets(facets: &Vec<Vec<FacetFilter>>) -> Result<Option<String>, SynrinthError> {
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

pub async fn query_search(client: &Client, params: QueryParams) -> Result<Search, SynrinthError> {
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
