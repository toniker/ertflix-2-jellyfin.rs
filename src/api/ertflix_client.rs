use crate::models::ertflix;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize};
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub section_contents: Vec<SectionContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SectionContents {
    pub toplist_codename: Option<String>,
    pub section_id: i32,
    pub tiles_ids: Option<Vec<Tile>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub origin_entity_id: i32,
    pub codename: String,
    pub id: String,
}

pub struct ErtflixClient {
    client: Client,
    base_url: String,
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::ParseError(e) => write!(f, "Parse error: {}", e),
            Error::CustomError(s) => write!(f, "Custom error: {}", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::RequestError(ref e) => Some(e),
            Error::ParseError(ref e) => Some(e),
            Error::CustomError(_) => None,
        }
    }
}

impl ErtflixClient {
    pub fn new(base_url: &str) -> Self {
        ErtflixClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_collections<CollectionCategory>(
        &self,
        filtering_strategy: fn(SectionContents) -> CollectionCategory,
    ) -> Result<Vec<CollectionCategory>, Box<dyn std::error::Error>> {
        let url = "https://api.app.ertflix.gr/v1/InsysGoPage/GetPageContent?platformCodename=www&pageCodename=mainpage&limit=100&page=1&$headers=%7B%22X-Api-Date-Format%22:%22iso%22,%22X-Api-Camel-Case%22:true%7D";

        let response = self.client.get(url).with_ertflix_headers().send().await;

        let response_str = response?.text().await?;

        // Deserialize into the new top-level struct
        let top_level_response: ApiResponse = match serde_json::from_str(&response_str) {
            Ok(data) => data,
            Err(e) => {
                println!("Failed to parse JSON: {:?}", e);
                return Err(Box::new(e));
            }
        };

        // Now you can access the content
        let api_response_content: Vec<SectionContents> = top_level_response
            .section_contents
            .into_iter()
            .filter(|s| s.toplist_codename.is_some())
            .collect();

        let collections: Vec<CollectionCategory> = api_response_content
            .into_iter()
            .map(filtering_strategy)
            .collect();

        Ok(collections)
    }
}

trait ErtflixRequestBuilderExt {
    fn with_ertflix_headers(self) -> Self;
}

impl ErtflixRequestBuilderExt for RequestBuilder {
    fn with_ertflix_headers(self) -> Self {
        self        
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0")
        .header("Accept", "*/*")
        .header("Accept-Language", "en")
        .header("Origin", "https://www.ertflix.gr")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache")
        .header("TE", "trailers")
    }
}