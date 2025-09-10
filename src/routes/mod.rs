use actix_web::web;
use crate::api::ertflix_client::ErtflixClient;
use tracing::{debug, info, trace};

pub mod handlers;

pub fn init_routes<T: ErtflixClient + 'static>(cfg: &mut web::ServiceConfig) {
    info!("Initializing application routes");
    debug!("Configuring route handlers for ErtflixClient type");
    
    trace!("Registering /tv route for TV shows endpoint");
    cfg.route("/tv", web::get().to(handlers::handle_get_tv_shows::<T>));
    
    trace!("Registering /movies route for movies endpoint");
    cfg.route("/movies", web::get().to(handlers::handle_get_movies::<T>));
    
    // Infuse's first request to a Jellyfin server is to this endpoint, to provide a health check
    trace!("Registering /System/Info/Public route for system info endpoint");
    cfg.route(
        "/System/Info/Public",
        web::get().to(handlers::handle_get_system_info),
    );
    
    // Infuse's second request authenticates on a Jellyfin server
    trace!("Registering /Users/AuthenticateByName route for authentication endpoint");
    cfg.route(
        "/Users/AuthenticateByName",
        web::post().to(handlers::handle_authentication)
    );
    
    // Infuse requests collections from this endpoint
    trace!("Registering /UserViews route for collections endpoint");
    cfg.route(
        "/UserViews",
        web::get().to(handlers::handle_get_collections::<T>),
    );
    
    info!("All routes successfully registered");
    debug!("Route initialization completed");
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AuthenticationBody {
    pw: String,
    username: String,
    password: String
}
