use serde::{Deserialize, Serialize};
use log::{debug, info, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ertflix: ErtflixConfig,
    pub redis: RedisConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErtflixConfig {
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub connection_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub default_ttl_seconds: u64,
    pub movies_ttl_seconds: u64,
    pub tv_shows_ttl_seconds: u64,
    pub collections_ttl_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        info!("Creating default configuration");
        debug!("Setting up default ERTFLIX API URL: {}", ERTFLIX_API_URL);
        debug!("Setting up default Redis configuration");
        debug!("Setting up default cache TTL values");

        let config = Self {
            ertflix: ErtflixConfig {
                base_url: ERTFLIX_API_URL.to_string(),
            },
            redis: RedisConfig {
                url: "redis://127.0.0.1:6379".to_string(),
                connection_pool_size: 10,
            },
            cache: CacheConfig {
                default_ttl_seconds: 3600,     // 1 hour
                movies_ttl_seconds: 7200,      // 2 hours
                tv_shows_ttl_seconds: 3600,    // 1 hour
                collections_ttl_seconds: 1800, // 30 minutes
            },
        };

        trace!("Default configuration created with cache TTLs - default: {}s, movies: {}s, TV shows: {}s, collections: {}s",
               config.cache.default_ttl_seconds, config.cache.movies_ttl_seconds,
               config.cache.tv_shows_ttl_seconds, config.cache.collections_ttl_seconds);

        info!("Default configuration initialization completed");
        config
    }
}

pub const ERTFLIX_API_URL: &str = "https://api.ertflix.gr";
pub const TIMEOUT_SECONDS: u64 = 30; // Timeout for API requests
pub const SERVER_ID: &str = "optiplex-adapter"; // Replace with your actual server ID
pub const USER_ID: &str = "optiplex-user"; // Replace with your actual user ID
pub const USERNAME: &str = "antonis"; // Replace with your actual username