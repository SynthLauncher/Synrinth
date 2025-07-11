use reqwest::Client;

use crate::{
    errors::SynrinthErr,
    models::search::{FacetFilter, QueryParams, Search},
};

#[macro_export]
macro_rules! facet_filters {
    (
        $( [ $($facet:ident $op:tt $val:expr),+ ] ),+
    ) => {{
        use $crate::models::search::{FacetFilter, FacetType, FacetOp};
        let mut filters = Vec::new();

        $(
            let mut group = Vec::new();
            $(
                let op = stringify!($op).into();

                group.push(FacetFilter {
                    facet: FacetType::$facet,
                    op,
                    value: $val.to_string(),
                });
            )+

            filters.push(group);
        )+

        filters
    }};
}

pub fn build_facets(facets: &Vec<Vec<FacetFilter>>) -> Result<Option<String>, SynrinthErr> {
    if facets.is_empty() {
        return Ok(None);
    }

    let mut json_facets: Vec<Vec<String>> = Vec::new();

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

pub async fn query_search(client: &Client, params: QueryParams) -> Result<Search, SynrinthErr> {
    let mut query_parts = Vec::new();

    if let Some(query) = params.query {
        if !query.trim().is_empty() {
            query_parts.push(format!("query={}", &query));
        }
    }

    if let Some(index) = params.index {
        query_parts.push(format!("index={}", &index));
    }

    if let Some(limit) = params.limit {
        query_parts.push(format!("limit={}", &limit));
    }

    if let Some(offset) = params.offset {
        query_parts.push(format!("offset={}", offset));
    }

    if let Some(facets) = params.facets {
        if let Some(facets_str) = build_facets(&facets)? {
            query_parts.push(format!("facets={}", &facets_str));
        }
    }

    let url = if query_parts.is_empty() {
        "https://api.modrinth.com/v2/search".to_string()
    } else {
        format!(
            "https://api.modrinth.com/v2/search?{}",
            query_parts.join("&")
        )
    };

    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}
