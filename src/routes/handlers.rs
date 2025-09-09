use actix_web::{web, HttpResponse, Responder};
use crate::models::jellyfin;
use crate::services::media_service::{MediaService};

pub async fn handle_get_tv_shows(media_service: web::Data<MediaService>) -> impl Responder {
    match media_service.get_tv_shows().await {
        Ok(ertflix_tv_shows) => HttpResponse::Ok().json(ertflix_tv_shows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn handle_get_movies(media_service: web::Data<MediaService>) -> impl Responder {
    match media_service.get_movies().await {
        Ok(ertflix_movies) => HttpResponse::Ok().json(ertflix_movies),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserViewsQuery {
    pub user_id: String,
}

pub async fn handle_get_collections(
    media_service: web::Data<MediaService>,
    query: web::Query<UserViewsQuery>,
) -> impl Responder {
    match media_service.get_collections(&query.user_id).await {
        Ok(collections_vec) => HttpResponse::Ok().json(jellyfin::Collections::new(collections_vec)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn handle_get_system_info() -> impl Responder {
    HttpResponse::Ok().json(jellyfin::SystemInfo::default())
}
