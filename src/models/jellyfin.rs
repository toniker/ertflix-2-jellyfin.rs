use crate::{config, models::ertflix};
use chrono::offset::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collection {
    pub name: String,
    pub server_id: String,
    pub id: String,
    pub etag: String,
    pub date_created: String,
    pub can_delete: bool,
    pub can_download: bool,
    pub sort_name: String,
    pub external_urls: Vec<String>,
    pub path: String,
    pub enable_media_source_display: bool,
    pub channel_id: Option<String>,
    pub taglines: Vec<String>,
    pub genres: Vec<String>,
    pub play_access: String,
    pub remote_trailers: Vec<String>,
    pub provider_ids: HashMap<String, String>,
    pub is_folder: bool,
    pub parent_id: String,
    #[serde(rename = "Type")]
    pub item_type: String,
    pub people: Vec<String>,
    pub studios: Vec<String>,
    pub genre_items: Vec<String>,
    pub local_trailer_count: i32,
    pub user_data: UserData,
    pub child_count: i32,
    pub special_feature_count: i32,
    pub display_preferences_id: String,
    pub tags: Vec<String>,
    pub primary_image_aspect_ratio: f64,
    pub collection_type: String,
    pub image_tags: ImageTags,
    pub backdrop_image_tags: Vec<String>,
    pub image_blur_hashes: ImageBlurHashes,
    pub location_type: String,
    pub media_type: String,
    pub locked_fields: Vec<String>,
    pub lock_data: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collections {
    items: Vec<Collection>,
    total_record_count: usize,
    start_index: i32,
}

impl Collections {
    pub fn new(items: Vec<Collection>) -> Self {
        Self {
            total_record_count: items.len(),
            items,
            start_index: 0,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    playback_position_ticks: i64,
    play_count: i32,
    is_favorite: bool,
    played: bool,
    key: String,
    item_id: String,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            playback_position_ticks: 0,
            play_count: 0,
            is_favorite: false,
            played: false,
            key: Uuid::new_v4().to_string(),
            item_id: "00000000000000000000000000000000".into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageTags {
    primary: String,
}

impl Default for ImageTags {
    fn default() -> Self {
        Self {
            primary: "00000000000000000000000000000000".into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageBlurHashes {
    primary: HashMap<String, String>,
}

impl Default for ImageBlurHashes {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("4183b69eb08fcd80b087bdf0cdd36c7c".into(), "000".into());
        Self { primary: map }
    }
}

impl Collection {
    pub fn from(ertflix_collection: ertflix::Collection) -> Self {
        let etag = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            &[
                ertflix_collection.id.as_bytes(),
                ertflix_collection.name.as_bytes(),
            ]
            .concat(),
        )
        .to_string();
        Self {
            name: ertflix_collection.name,
            server_id: config::SERVER_ID.into(),
            id: ertflix_collection.id,
            etag,
            date_created: Local::now().to_string(),
            can_delete: true,
            can_download: true,
            sort_name: "movies".into(),
            external_urls: vec![],
            path: "".into(),
            enable_media_source_display: false,
            channel_id: None,
            taglines: vec![],
            genres: vec![],
            play_access: "Full".into(),
            remote_trailers: vec![],
            provider_ids: Default::default(),
            is_folder: true,
            parent_id: "".into(),
            item_type: "CollectionFolder".into(),
            people: vec![],
            studios: vec![],
            genre_items: vec![],
            local_trailer_count: 0,
            user_data: UserData::default(),
            child_count: 0,
            special_feature_count: 0,
            display_preferences_id: "".into(),
            tags: vec![],
            primary_image_aspect_ratio: 0.0,
            collection_type: "".into(),
            image_tags: ImageTags::default(),
            backdrop_image_tags: vec![],
            image_blur_hashes: ImageBlurHashes::default(),
            location_type: "FileSystem".into(),
            media_type: "Unknown".into(),
            locked_fields: vec![],
            lock_data: false,
        }
    }
}
