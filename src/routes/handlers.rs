use std::str::FromStr;
use crate::api::ertflix_client::ErtflixClient;
use crate::api::jellyfin_server;
use crate::models::jellyfin;
use crate::services::media_service::MediaService;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use tracing::{debug, error, info, trace, warn, instrument};
use crate::api::jellyfin_server::EmbyAuthorizationHeader;

pub async fn handle_get_collections<T: ErtflixClient>(media_service: web::Data<MediaService<T>>) -> impl Responder {
    info!("Handling request for collections");
    trace!("Starting collections retrieval process");

    match media_service.get_collections().await {
        Ok(collections_vec) => {
            info!("Successfully retrieved {} collections", collections_vec.len());
            debug!("Creating Jellyfin collections response");
            let response = jellyfin::Collections::new(collections_vec);
            trace!("Collections response prepared");
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            error!("Failed to retrieve collections: {}", e);
            warn!("Returning internal server error for collections request");
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn handle_get_tv_shows<T: ErtflixClient>(media_service: web::Data<MediaService<T>>) -> impl Responder {
    info!("Handling request for TV shows");
    trace!("Starting TV shows retrieval process");

    match media_service.get_tv_shows().await {
        Ok(tv_shows) => {
            info!("Successfully retrieved {} TV shows", tv_shows.len());
            debug!("Preparing TV shows JSON response");
            trace!("TV shows response ready");
            HttpResponse::Ok().json(tv_shows)
        },
        Err(e) => {
            error!("Failed to retrieve TV shows: {}", e);
            warn!("Returning internal server error for TV shows request");
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn handle_get_movies<T: ErtflixClient>(media_service: web::Data<MediaService<T>>) -> impl Responder {
    info!("Handling request for movies");
    trace!("Starting movies retrieval process");

    match media_service.get_movies().await {
        Ok(movies) => {
            info!("Successfully retrieved {} movies", movies.len());
            debug!("Preparing movies JSON response");
            trace!("Movies response ready");
            HttpResponse::Ok().json(movies)
        },
        Err(e) => {
            error!("Failed to retrieve movies: {}", e);
            warn!("Returning internal server error for movies request");
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn handle_authentication(req: HttpRequest) -> impl Responder {
    info!("Handling authentication request");

    debug!("Headers: {:#?}", req.headers());

    let emby_auth_header = req
        .headers()
        .get("x-emby-authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    match EmbyAuthorizationHeader::from_str(emby_auth_header) {
        Ok(authorization) => {
            HttpResponse::Ok().json(jellyfin_server::AuthenticationResponse::default(authorization))
        },
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid X-Emby-Authentication header")
        }
    }
}

#[instrument(level = "trace")]
pub async fn handle_get_system_info() -> impl Responder {
    info!("Handling system info request");
    debug!("Creating default system info response");
    trace!("System info response prepared");
    HttpResponse::Ok().json(jellyfin_server::SystemInfo::default())
}
