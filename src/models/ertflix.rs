use crate::api::ertflix_client;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub year: u32,
    pub genre: Vec<String>,
    pub description: String,
}

impl From<ertflix_client::Tile> for Movie {
    fn from(tile: ertflix_client::Tile) -> Self {
        Self {
            id: tile.id,
            title: tile.title.unwrap_or_default(),
            year: tile.year.unwrap_or(1970), // Placeholder for year
            genre: Vec::new(),               // Placeholder for an empty list of genres
            description: tile.description.unwrap_or_default(), // Placeholder for description
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: String,
    pub title: String,
    pub seasons: Vec<Season>,
}

impl From<ertflix_client::Tile> for TVShow {
    fn from(tile: ertflix_client::Tile) -> Self {
        Self {
            id: tile.id,
            title: tile.title.unwrap_or(tile.codename),
            seasons: Vec::new(), // Placeholder for an empty list of seasons
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Season {
    pub season_number: u32,
    pub episodes: Vec<Episode>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub duration: u32, // duration in seconds
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub id: String,
}
