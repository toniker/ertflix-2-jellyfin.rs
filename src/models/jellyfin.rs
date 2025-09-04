use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub year: i32,
    pub genre: Vec<String>,
    pub overview: String,
    pub poster_url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: String,
    pub title: String,
    pub seasons: Vec<Season>,
    pub overview: String,
    pub poster_url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub title: String,
    pub season_number: i32,
    pub episodes: Vec<Episode>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub season_number: i32,
    pub episode_number: i32,
    pub overview: String,
    pub duration: i32,
}