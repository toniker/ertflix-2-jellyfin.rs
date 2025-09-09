use crate::api::ertflix_client;
use crate::api::ertflix_client::ErtflixClient;
use crate::models::ertflix;
use crate::models::jellyfin;

pub struct MediaService {
    ertflix_client: ErtflixClient,
}

impl MediaService {
    pub fn new(ertflix_client: ErtflixClient) -> Self {
        MediaService { ertflix_client }
    }

    pub async fn get_tv_shows(&self) -> Result<Vec<ertflix::TVShow>, Box<dyn std::error::Error>> {
        // Logic to interact with ERTFLIX and retrieve TV shows
        let response = self
            .ertflix_client
            .get_collections(|section_contents| section_contents)
            .await?
            .into_iter()
            .filter(|section| {
                section.toplist_codename == Some("ert-seires-plereis".to_string())
            })
            .next_back();

        if response.is_none() {
            println!("No TV section found");
            return Err(Box::new(ertflix_client::Error::CustomError(String::from(
                "No TV section found",
            ))));
        }

        let section = response.unwrap();
        let shows: Vec<ertflix::TVShow> = section
            .tiles_ids
            .unwrap_or_default()
            .into_iter()
            .map(|tile| {
                ertflix::TVShow {
                    id: tile.id,
                    title: tile.codename,
                    seasons: Vec::new(), // Placeholder for an empty list of seasons
                }
            })
            .collect();
        Ok(shows)
    }

    pub async fn get_movies(&self) -> Result<Vec<ertflix::Movie>, Box<dyn std::error::Error>> {
        // Logic to interact with ERTFLIX and retrieve Movies
        let response = self
            .ertflix_client
            .get_collections(|section_contents| section_contents)
            .await?
            .into_iter()
            .filter(|section| {
                section.toplist_codename == Some("ert-oles-oi-tainies".to_string())
            })
            .next_back();

        match response {
            Some(section) => {
                let movies: Vec<ertflix::Movie> = section
                    .tiles_ids
                    .unwrap_or_default()
                    .into_iter()
                    .map(|tile| {
                        ertflix::Movie {
                            id: tile.id,
                            title: tile.codename,
                            description: String::new(), // Placeholder
                            year: 2025,
                            genre: vec![String::from("Placeholder")], // Placeholder
                        }
                    })
                    .collect();
                Ok(movies)
            }
            None => {
                println!("No movie section found");
                Err(Box::new(ertflix_client::Error::CustomError(String::from(
                    "No movie section found",
                ))))
            }
        }
    }

    pub async fn get_collections(&self, user_id: &str) -> Result<Vec<jellyfin::Collection>, Box<dyn std::error::Error>> {
        let collections = self
            .ertflix_client
            .get_collections(|section_contents| section_contents)
            .await?
            .into_iter()
            .map(|section| ertflix::Collection {
                name: section.toplist_codename.unwrap_or_default(),
                id: section.section_id.to_string(),
            })
            .map(jellyfin::Collection::from)
            .collect();
        Ok(collections)
    }

    fn convert_to_jellyfin_tv_show(&self, tv_show: ertflix::TVShow) -> jellyfin::TVShow {
        // Logic to convert ERTFLIX TV Show to Jellyfin format
        // This is a placeholder for actual implementation
        jellyfin::TVShow::default()
    }

    fn convert_to_jellyfin_movie(&self, movie: ertflix::Movie) -> jellyfin::Movie {
        // Logic to convert ERTFLIX Movie to Jellyfin format
        // This is a placeholder for actual implementation
        jellyfin::Movie::default()
    }
}
