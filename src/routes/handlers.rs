use actix_web::{web, HttpResponse, Responder};
use crate::services::media_service::{MediaService};
use crate::models::ertflix::{TVShow, Movie};

pub async fn handle_get_tv_shows(media_service: web::Data<MediaService>) -> impl Responder {
    match media_service.get_tv_shows().await {
        Ok(tv_shows) => HttpResponse::Ok().json(tv_shows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn handle_get_movies(media_service: web::Data<MediaService>) -> impl Responder {
    match media_service.get_movies().await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn handle_get_collections(media_service: web::Data<MediaService>) -> impl Responder {
    println!("Handling get collections request");
    match media_service.get_collections().await {
        Ok(collections) => HttpResponse::Ok().json(collections),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}