use actix_web::cookie::time::error;

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
        // This is a placeholder for actual implementation
        unimplemented!();
    }

    pub async fn get_movies(&self) -> Result<Vec<ertflix::Movie>, Box<dyn std::error::Error>> {
        // Logic to interact with ERTFLIX and retrieve Movies
        // This is a placeholder for actual implementation
        unimplemented!();
    }

    pub async fn get_collections(&self) -> Result<ertflix::ErtflixCollection, Box<dyn std::error::Error>> {
        let response = self.ertflix_client.get_collections().await?;
        
        println!("{:?}", response);
        unimplemented!()
    }

    pub fn convert_to_jellyfin_tv_show(&self, tv_show: ertflix::TVShow) -> jellyfin::TVShow {
        // Logic to convert ERTFLIX TV Show to Jellyfin format
        // This is a placeholder for actual implementation
        jellyfin::TVShow::default()
    }

    pub fn convert_to_jellyfin_movie(&self, movie: ertflix::Movie) -> jellyfin::Movie {
        // Logic to convert ERTFLIX Movie to Jellyfin format
        // This is a placeholder for actual implementation
        jellyfin::Movie::default()
    }
}