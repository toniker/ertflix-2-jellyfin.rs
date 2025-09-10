use crate::config;
use chrono;
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace};
use uuid::Uuid;

#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResponse {
    user: User,
    server_id: String,
    access_token: String,
    session_info: SessionInfo
}

impl AuthenticationResponse {
    pub fn default(emby_authorization_header: EmbyAuthorizationHeader) -> Self {
        info!("Creating default authentication response");
        debug!("Initializing authentication response with default user");
        trace!("Authentication response creation completed");
        Self {
            user: User::default(),
            server_id: config::SERVER_ID.into(),
            access_token: Uuid::new_v4().to_string(),
            session_info: SessionInfo::from(emby_authorization_header),
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
        info!("Creating default system info response");
        debug!(
            "Setting up system info with server ID: {}",
            config::SERVER_ID
        );
        trace!("System info configured with local address: http://localhost:25860");

        let system_info = Self {
            local_address: "http://localhost:25860".into(),
            server_name: "Ertflix Adapter".into(),
            version: "10.8.0".into(),
            product_name: "Jellyfin Server".into(),
            operating_system: "Linux".into(),
            id: config::SERVER_ID.into(),
            startup_wizard_completed: true,
        };

        debug!("System info default configuration completed");
        system_info
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: String,
    pub server_id: String,
    pub id: String,
    pub has_password: bool,
    pub has_configured_password: bool,
    pub has_configured_easy_password: bool,
    pub enable_auto_login: bool,
    pub last_login_date: String,
    pub last_activity_date: String,
    pub configuration: Configuration,
    pub policy: Policy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    pub audio_language_preference: String,
    pub play_default_audio_track: bool,
    pub subtitle_language_preference: String,
    pub display_missing_episodes: bool,
    pub grouped_folders: Vec<String>,
    pub subtitle_mode: String,
    pub display_collections_view: bool,
    pub enable_local_password: bool,
    pub ordered_views: Vec<String>,
    pub latest_items_excludes: Vec<String>,
    pub my_media_excludes: Vec<String>,
    pub hide_played_in_latest: bool,
    pub remember_audio_selections: bool,
    pub remember_subtitle_selections: bool,
    pub enable_next_episode_auto_play: bool,
    pub cast_receiver_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    pub is_administrator: bool,
    pub is_hidden: bool,
    pub enable_collection_management: bool,
    pub enable_subtitle_management: bool,
    pub enable_lyric_management: bool,
    pub is_disabled: bool,
    pub blocked_tags: Vec<String>,
    pub allowed_tags: Vec<String>,
    pub enable_user_preference_access: bool,
    pub access_schedules: Vec<String>,
    pub block_unrated_items: Vec<String>,
    pub enable_remote_control_of_other_users: bool,
    pub enable_shared_device_control: bool,
    pub enable_remote_access: bool,
    pub enable_live_tv_management: bool,
    pub enable_live_tv_access: bool,
    pub enable_media_playback: bool,
    pub enable_audio_playback_transcoding: bool,
    pub enable_video_playback_transcoding: bool,
    pub enable_playback_remuxing: bool,
    pub force_remote_source_transcoding: bool,
    pub enable_content_deletion: bool,
    pub enable_content_deletion_from_folders: Vec<String>,
    pub enable_content_downloading: bool,
    pub enable_sync_transcoding: bool,
    pub enable_media_conversion: bool,
    pub enabled_devices: Vec<String>,
    pub enable_all_devices: bool,
    pub enabled_channels: Vec<String>,
    pub enable_all_channels: bool,
    pub enabled_folders: Vec<String>,
    pub enable_all_folders: bool,
    pub invalid_login_attempt_count: i32,
    pub login_attempts_before_lockout: i32,
    pub max_active_sessions: i32,
    pub enable_public_sharing: bool,
    pub blocked_media_folders: Vec<String>,
    pub blocked_channels: Vec<String>,
    pub remote_client_bitrate_limit: i32,
    pub authentication_provider_id: String,
    pub password_reset_provider_id: String,
    pub sync_play_access: String,
}

impl Default for User {
    fn default() -> Self {
        info!("Creating default user configuration");
        debug!("Setting up user with server ID: {}", config::SERVER_ID);
        trace!("User configured with administrative privileges");

        let timestamp = create_jellyfin_timestamp();
        
        let user = Self {
            name: "antonis".into(),
            server_id: config::SERVER_ID.into(),
            id: Uuid::new_v4().to_string(),
            has_password: true,
            has_configured_password: true,
            has_configured_easy_password: false,
            enable_auto_login: false,
            last_login_date: timestamp.clone(),
            last_activity_date: timestamp,
            configuration: Configuration::default(),
            policy: Policy::default(),
        };

        debug!("Default user configuration completed");
        user
    }
}

impl Default for Configuration {
    fn default() -> Self {
        info!("Creating default user configuration settings");
        debug!("Setting up default media preferences and display options");
        trace!("Configuration set with English audio/subtitle preferences");

        let config = Self {
            audio_language_preference: "eng".to_string(),
            play_default_audio_track: true,
            subtitle_language_preference: "eng".to_string(),
            display_missing_episodes: false,
            grouped_folders: vec![],
            subtitle_mode: "Always".to_string(),
            display_collections_view: false,
            enable_local_password: false,
            ordered_views: vec![],
            latest_items_excludes: vec![],
            my_media_excludes: vec![],
            hide_played_in_latest: true,
            remember_audio_selections: true,
            remember_subtitle_selections: true,
            enable_next_episode_auto_play: true,
            cast_receiver_id: String::new(),
        };

        debug!("Default user configuration settings completed");
        config
    }
}

impl Default for Policy {
    fn default() -> Self {
        info!("Creating default user policy settings");
        debug!("Setting up administrative policy with full permissions");
        trace!("Policy configured with transcoding and content management enabled");

        let policy = Self {
            is_administrator: true,
            is_hidden: false,
            enable_collection_management: false,
            enable_subtitle_management: false,
            enable_lyric_management: false,
            is_disabled: false,
            blocked_tags: vec![],
            allowed_tags: vec![],
            enable_user_preference_access: true,
            access_schedules: vec![],
            block_unrated_items: vec![],
            enable_remote_control_of_other_users: true,
            enable_shared_device_control: true,
            enable_remote_access: true,
            enable_live_tv_management: true,
            enable_live_tv_access: true,
            enable_media_playback: true,
            enable_audio_playback_transcoding: true,
            enable_video_playback_transcoding: true,
            enable_playback_remuxing: true,
            force_remote_source_transcoding: false,
            enable_content_deletion: true,
            enable_content_deletion_from_folders: vec![],
            enable_content_downloading: true,
            enable_sync_transcoding: true,
            enable_media_conversion: true,
            enabled_devices: vec![],
            enable_all_devices: true,
            enabled_channels: vec![],
            enable_all_channels: true,
            enabled_folders: vec![],
            enable_all_folders: true,
            invalid_login_attempt_count: 0,
            login_attempts_before_lockout: -1,
            max_active_sessions: 0,
            enable_public_sharing: true,
            blocked_media_folders: vec![],
            blocked_channels: vec![],
            remote_client_bitrate_limit: 0,
            authentication_provider_id:
                "Jellyfin.Server.Implementations.Users.DefaultAuthenticationProvider".to_string(),
            password_reset_provider_id:
                "Jellyfin.Server.Implementations.Users.DefaultPasswordResetProvider".to_string(),
            sync_play_access: "CreateAndJoinGroups".to_string(),
        };

        debug!("Default user policy settings completed");
        policy
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub play_state: PlayState,
    pub additional_users: Vec<String>,
    pub capabilities: Capabilities,
    pub remote_end_point: String,
    pub playable_media_types: Vec<String>,
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub client: String,
    pub last_activity_date: String,
    pub last_playback_check_in: String,
    pub device_name: String,
    pub device_id: String,
    pub application_version: String,
    pub is_active: bool,
    pub supports_media_control: bool,
    pub supports_remote_control: bool,
    pub now_playing_queue: Vec<String>,
    pub now_playing_queue_full_items: Vec<String>,
    pub has_custom_device_name: bool,
    pub server_id: String,
    pub supported_commands: Vec<String>,
}

impl Default for SessionInfo {
    fn default() -> Self {
        let timestamp = create_jellyfin_timestamp();
        
        Self {
            play_state: PlayState::default(),
            additional_users: vec![],
            capabilities: Capabilities::default(),
            remote_end_point: "".to_string(),
            playable_media_types: vec![],
            id: Uuid::new_v4().into(),
            user_id: config::USER_ID.into(),
            user_name: config::USERNAME.into(),
            client: "web".to_string(),
            last_activity_date: timestamp.clone(),
            last_playback_check_in: timestamp,
            device_name: "Mac".into(),
            device_id: Uuid::new_v4().into(),
            application_version: "v0.0.1".into(),
            is_active: false,
            supports_media_control: false,
            supports_remote_control: false,
            now_playing_queue: vec![],
            now_playing_queue_full_items: vec![],
            has_custom_device_name: false,
            server_id: config::SERVER_ID.into(),
            supported_commands: vec![],
        }
    }   
}

impl From<EmbyAuthorizationHeader> for SessionInfo {
    fn from(header: EmbyAuthorizationHeader) -> Self {
        let mut session_info = SessionInfo::default();
        session_info.device_name = header.device;
        session_info.device_id = header.device_id;
        session_info.client = header.client;
        session_info.application_version = header.version;
        session_info.id = Uuid::new_v4().to_string();
        session_info
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PlayState {
    pub can_seek: bool,
    pub is_paused: bool,
    pub is_muted: bool,
    pub repeat_mode: String,
    pub playback_order: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Capabilities {
    pub playable_media_types: Vec<String>,
    pub supported_commands: Vec<String>,
    pub supports_media_control: bool,
    pub supports_persistent_identifier: bool,
}

use std::str::FromStr;

#[derive(Debug)]
pub struct EmbyAuthorizationHeader {
    pub version: String,
    pub device: String,
    pub device_id: String,
    pub client: String,
}

impl FromStr for EmbyAuthorizationHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut version = String::new();
        let mut device = String::new();
        let mut device_id = String::new();
        let mut client = String::new();

        for part in s.split(',') {
            let mut kv = part.trim().splitn(2, '=');
            let key = kv.next().unwrap_or("").trim();
            let value = kv.next().unwrap_or("").trim().trim_matches('"');
            match key {
                "MediaBrowser Version" | "Version" => version = value.to_string(),
                "Device" => device = value.to_string(),
                "DeviceId" => device_id = value.to_string(),
                "Client" => client = value.to_string(),
                _ => {}
            }
        }

        Ok(EmbyAuthorizationHeader { version, device, device_id, client })
    }
}

fn create_jellyfin_timestamp() -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:07}Z",
        chrono::Utc::now().year(),
        chrono::Utc::now().month(),
        chrono::Utc::now().day(),
        chrono::Utc::now().hour(),
        chrono::Utc::now().minute(),
        chrono::Utc::now().second(),
        chrono::Utc::now().nanosecond() / 100
    )
}