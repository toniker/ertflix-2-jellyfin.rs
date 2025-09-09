use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use crate::config;
use crate::models::ertflix;

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

#[derive(serde::Serialize)]
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
    pub provider_ids: std::collections::HashMap<String, String>,
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

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collections {
    items: Vec<Collection>,
    total_record_count: usize,
    start_index: i32,
}

impl Collections {
    pub fn new(items: Vec<Collection>) -> Self {
        let record_count = items.len();
        Collections {
            items,
            total_record_count: record_count,
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
        UserData {
            playback_position_ticks: 0,
            play_count: 0,
            is_favorite: false,
            played: false,
            key: Uuid::new_v4().to_string(),
            item_id: String::from("00000000000000000000000000000000"),
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
        ImageTags {
            primary: String::from("00000000000000000000000000000000"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageBlurHashes {
    primary: std::collections::HashMap<String, String>,
}

impl Default for ImageBlurHashes {
    fn default() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(
            String::from("4183b69eb08fcd80b087bdf0cdd36c7c"),
            String::from("000"),
        );
        Self {
            primary: map
        }
    }
}

impl Collection {
    pub fn from(ertflix_collection: ertflix::Collection) -> Self {
        let bytes = &[ertflix_collection.id.as_bytes(), ertflix_collection.name.as_bytes()].concat();
        let etag = Uuid::new_v5(&Uuid::NAMESPACE_URL, bytes).to_string();
        Self {
            name: ertflix_collection.name,
            server_id: String::from(config::SERVER_ID),
            id: ertflix_collection.id,
            etag,
            date_created: chrono::offset::Local::now().to_string(),
            can_delete: true,
            can_download: true,
            sort_name: "movies".to_string(),
            external_urls: vec![],
            path: "".to_string(),
            enable_media_source_display: false,
            channel_id: None,
            taglines: vec![],
            genres: vec![],
            play_access: "Full".to_string(),
            remote_trailers: vec![],
            provider_ids: Default::default(),
            is_folder: true,
            parent_id: "".to_string(),
            item_type: "CollectionFolder".to_string(),
            people: vec![],
            studios: vec![],
            genre_items: vec![],
            local_trailer_count: 0,
            user_data: UserData::default(),
            child_count: 0,
            special_feature_count: 0,
            display_preferences_id: "".to_string(),
            tags: vec![],
            primary_image_aspect_ratio: 0.0,
            collection_type: "".to_string(),
            image_tags: ImageTags::default(),
            backdrop_image_tags: vec![],
            image_blur_hashes: ImageBlurHashes::default(),
            location_type: "FileSystem".to_string(),
            media_type: "Unknown".to_string(),
            locked_fields: vec![],
            lock_data: false,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    local_address: String,
    server_name: String,
    version: String,
    product_name: String,
    operating_system: String,
    id: String,
    startup_wizard_completed: bool,
}

impl Default for SystemInfo {
    fn default() -> Self {
        SystemInfo {
            local_address: "http://localhost:25860".to_string(),
            server_name: "Ertflix Adapter".to_string(),
            version: "10.8.0".to_string(),
            product_name: "Jellyfin Server".to_string(),
            operating_system: "Linux".to_string(),
            id: String::from(config::SERVER_ID),
            startup_wizard_completed: true,
        }
    }
}
