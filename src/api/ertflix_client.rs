use std::error;
use crate::config;
use crate::models::ertflix;
use log::{debug, error, info, trace, warn};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub section_contents: Vec<SectionContents>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct SectionContents {
    pub toplist_codename: Option<String>,
    pub section_id: i32,
    pub tiles_ids: Option<Vec<Tile>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub origin_entity_id: i32,
    pub codename: String,
    pub id: String,
    pub year: Option<u32>,
    pub description: Option<String>,
    pub title: Option<String>,
}

pub struct DefaultErtflixClient {
    pub client: Client,
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GetTilesRequestBody {
    platform_codename: String,
    requested_tiles: Vec<RequestedTile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RequestedTile {
    id: String,
}

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Parse(serde_json::Error),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
            Error::Custom(s) => write!(f, "Custom error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Request(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
            Error::Custom(_) => None,
        }
    }
}

pub trait ErtflixClient {
    fn new(base_url: &str) -> Self
    where
        Self: Sized;

    async fn get_collections<CollectionCategory>(
        &self,
        filtering_strategy: fn(SectionContents) -> CollectionCategory,
    ) -> Result<Vec<CollectionCategory>, Box<dyn error::Error>>;

    async fn get_movies(&self) -> Result<Vec<ertflix::Movie>, Box<dyn error::Error>>;

    async fn get_tv_shows(&self) -> Result<Vec<ertflix::TVShow>, Box<dyn error::Error>>;

    async fn get_section_content(
        &self,
        section_codename: String,
    ) -> Result<Vec<SectionContents>, Box<dyn error::Error>>;

    async fn get_tiles<TileType>(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<TileType>, Box<dyn error::Error>> where
        TileType: From<Tile>;
}

impl ErtflixClient for DefaultErtflixClient {
    fn new(base_url: &str) -> Self {
        info!("Creating new DefaultErtflixClient with base_url: {}", base_url);

        DefaultErtflixClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn get_collections<CollectionCategory>(
        &self,
        filtering_strategy: fn(SectionContents) -> CollectionCategory,
    ) -> Result<Vec<CollectionCategory>, Box<dyn error::Error>> {
        let url = format!(
            "https://{base_url}/v1/InsysGoPage/GetPageContent?platformCodename=www&pageCodename=mainpage&limit=100&page=1&$headers=%7B%22X-Api-Date-Format%22:%22iso%22,%22X-Api-Camel-Case%22:true%7D",
            base_url = self.base_url
        );

        info!("Fetching collections from Ertflix API");
        debug!("Request URL: {}", url);
        trace!("Making HTTP GET request to collections endpoint");
        let response = self.client.get(url).with_ertflix_headers().send().await;

        let response_str = match response {
            Ok(res) => {
                debug!("Received response with status: {}", res.status());
                match res.text().await {
                    Ok(text) => {
                        trace!("Response body length: {} bytes", text.len());
                        text
                    }
                    Err(e) => {
                        error!("Failed to read response text: {}", e);
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                error!("HTTP request failed: {}", e);
                return Err(Box::new(e));
            }
        };
        // Deserialize into the new top-level struct
        let top_level_response: Result<ApiResponse, Box<dyn error::Error>> = match serde_json::from_str::<ApiResponse>(&response_str) {
            Ok(data) => {
                debug!("Successfully parsed API response");
                trace!("Parsed {} section contents", data.section_contents.len());
                Ok(data)
            }
            Err(e) => {
                println!("Failed to parse JSON: {:?}", e);
                error!("Failed to parse JSON response: {}", e);
                debug!("Response body: {}", response_str);
                return Err(Box::new(Error::Parse(e)));
            }
        };

        // Now you can access the content
        let api_response_content: Vec<SectionContents> = top_level_response?
            .section_contents
            .into_iter()
            .filter(|s| s.toplist_codename.is_some())
            .filter(|s| {
                let has_toplist = s.toplist_codename.is_some();
                if has_toplist {
                    trace!("Including section {} with toplist: {:?}", s.section_id, s.toplist_codename);
                } else {
                    trace!("Filtering out section {} (no toplist)", s.section_id);
                }
                has_toplist
            })
            .collect();
        debug!("Filtered to {} sections with toplists", api_response_content.len());

        let collections: Vec<CollectionCategory> = api_response_content
            .into_iter()
            .map(filtering_strategy)
            .collect();
        info!("Successfully processed {} collections", collections.len());
        Ok(collections)
    }

    async fn get_movies(&self) -> Result<Vec<ertflix::Movie>, Box<dyn error::Error>> {
        info!("Fetching movies from Ertflix");
        debug!("Getting section content for movies: oles-oi-tainies-1");
        let section_contents = self
            .get_section_content("oles-oi-tainies-1".to_string())
            .await?;

        let movie_section = match section_contents.first() {
            Some(section) => {
                debug!("Found movie section with ID: {}", section.section_id);
                section
            }
            None => {
                warn!("No movie section found in response");
                return Err(Box::new(Error::Custom("No movie section found".to_string())));
            }
        };
        let movie_ids: Vec<String> = match &movie_section.tiles_ids {
            Some(tiles) => {
                info!("Found {} movie tiles", tiles.len());
                tiles
            }
            None => {
                warn!("No movie tiles found in section");
                return Err(Box::new(Error::Custom("No tiles found".to_string())));
            }
        }
            .iter()
            .map(|tile| tile.id.clone())
            .collect();
        debug!("Fetching details for {} movies", movie_ids.len());

        let movies: Vec<ertflix::Movie> = self.get_tiles(movie_ids).await?;
        info!("Successfully fetched {} movies", movies.len());

        Ok(movies)
    }

    async fn get_tv_shows(&self) -> Result<Vec<ertflix::TVShow>, Box<dyn error::Error>> {
        info!("Fetching TV shows from Ertflix");
        debug!("Getting section content for TV shows: ert-seires-plereis");

        let section_contents = self.get_section_content("ert-seires-plereis".to_string()).await?;

        let tv_section = match section_contents.first() {
            Some(section) => {
                debug!("Found TV shows section with ID: {}", section.section_id);
                section
            }
            None => {
                warn!("No TV shows section found in response");
                return Err(Box::new(Error::Custom("No TV shows section found".to_string())));
            }
        };
        let tv_ids: Vec<String> = match &tv_section.tiles_ids {
            Some(tiles) => {
                info!("Found {} TV show tiles", tiles.len());
                tiles
            }
            None => {
                warn!("No TV show tiles found in section");
                return Err(Box::new(Error::Custom("No tiles found".to_string())));
            }
        }.iter().map(|tile| tile.id.clone()).collect();
        debug!("Fetching details for {} TV shows", tv_ids.len());

        let shows: Vec<ertflix::TVShow> = self.get_tiles(tv_ids).await?;
        info!("Successfully fetched {} TV shows", shows.len());
        Ok(shows)
    }

    async fn get_section_content(
        &self,
        section_codename: String,
    ) -> Result<Vec<SectionContents>, Box<dyn error::Error>> {
        let url = format!(
            "https://{base_url}/v1/InsysGoPage/GetSectionContent?platformCodename=www&sectionCodename={section_codename}&page=1&ignoreLimit=true&limit=1000&$headers=%7B%22X-Api-Date-Format%22:%22iso%22,%22X-Api-Camel-Case%22:true%7D",
            base_url = self.base_url,
        );
        let response = self.client.get(&url).with_ertflix_headers().send().await;

        info!("Fetching section content for: {}", section_codename);
        debug!("Request URL: {}", url);
        trace!("Making HTTP GET request to section content endpoint");


        match response {
            Ok(res) => {
                let status = res.status();
                debug!("Received response with status: {}", status);
                if !status.is_success() {
                    warn!("Non-success status code: {}", status);
                }

                match res.text().await {
                    Ok(response_str) => {
                        trace!("Response body length: {} bytes", response_str.len());
                        match serde_json::from_str(&response_str) {
                            Ok(section_contents) => {
                                let contents: Vec<SectionContents> = section_contents;
                                info!("Successfully fetched {} section contents for {}", contents.len(), section_codename);
                                Ok(contents)
                            }
                            Err(e) => {
                                error!("Failed to parse section content JSON: {}", e);
                                debug!("Response body: {}", response_str);
                                Err(Box::new(Error::Parse(e)))
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read response text: {}", e);
                        Err(Box::new(Error::Request(e)))
                    }
                }
            }
            Err(e) => {
                error!("HTTP request failed for section {}: {}", section_codename, e);
                Err(Box::new(Error::Request(e)))
            }
        }
    }

    async fn get_tiles<TileType>(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<TileType>, Box<dyn error::Error>>
    where
        TileType: From<Tile>,
    {
        let url = format!(
            "https://{base_url}/v2/Tile/GetTiles?$headers=%7B%22Content-Type%22:%22application%2Fjson%3Bcharset%3Dutf-8%22,%22X-Api-Date-Format%22:%22iso%22,%22X-Api-Camel-Case%22:true%7D",
            base_url = self.base_url
        );

        info!("Fetching tile details for {} items", ids.len());
        debug!("Request URL: {}", url);
        trace!("Tile IDs: {:?}", ids);

        let request_body: GetTilesRequestBody = GetTilesRequestBody {
            platform_codename: "www".to_string(),
            requested_tiles: ids
                .iter()
                .map(|id| {
                    let id = id.clone();
                    RequestedTile { id }
                })
                .collect(),
        };

        trace!("Request body prepared with {} tiles", request_body.requested_tiles.len());
        let response = self
        .client
        .post(url)
        .with_ertflix_headers()
        .json(&serde_json::json!(request_body))
        .send()
        .await;

        match response {
            Ok(res) => {
                let status = res.status();
                debug!("Received tiles response with status: {}", status);
                if !status.is_success() {
                    warn!("Non-success status code for tiles request: {}", status);
                }

                match res.text().await {
                    Ok(response_str) => {
                        trace!("Tiles response body length: {} bytes", response_str.len());
                        match serde_json::from_str(&response_str) {
                            Ok(tiles) => {
                                let tiles: Vec<Tile> = tiles;
                                debug!("Successfully parsed {} tiles", tiles.len());

                                let tile_types: Vec<TileType> = tiles.into_iter().map(|tile| {
                                    trace!("Converting tile: {} ({})", tile.title.as_deref().unwrap_or("Unknown"), tile.id);
                                    TileType::from(tile)
                                }).collect();

                                info!("Successfully fetched and converted {} tiles", tile_types.len());
                                Ok(tile_types)
                            }
                            Err(e) => {
                                error!("Failed to parse tiles JSON: {}", e);
                                debug!("Response body: {}", response_str);
                                Err(Box::new(Error::Parse(e)))
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read tiles response text: {}", e);
                        Err(Box::new(Error::Request(e)))
                    }
                }
            }
            Err(e) => {
                error!("HTTP request failed for tiles: {}", e);
                Err(Box::new(Error::Request(e)))
            }
        }
    }
}


trait ErtflixRequestBuilder {
    fn with_ertflix_headers(self) -> Self;
}

impl ErtflixRequestBuilder for RequestBuilder {
    fn with_ertflix_headers(self) -> Self {
        self.header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:142.0) Gecko/20100101 Firefox/142.0",
        )
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
            .timeout(Duration::from_secs(config::TIMEOUT_SECONDS))
    }
}
