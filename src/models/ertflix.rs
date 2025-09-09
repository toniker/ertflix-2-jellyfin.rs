use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub year: u32,
    pub genre: Vec<String>,
    pub description: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: String,
    pub title: String,
    pub seasons: Vec<Season>,
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
    pub id: String
}

