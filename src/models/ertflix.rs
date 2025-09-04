use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Movie {
    id: String,
    title: String,
    year: u32,
    genre: Vec<String>,
    description: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TVShow {
    id: String,
    title: String,
    seasons: Vec<Season>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Season {
    season_number: u32,
    episodes: Vec<Episode>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Episode {
    id: String,
    title: String,
    duration: u32, // duration in seconds
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ErtflixCollection {
    movies: Vec<Movie>,
    tv_shows: Vec<TVShow>,
}
