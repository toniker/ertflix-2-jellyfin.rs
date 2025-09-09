use actix_web::{App, HttpServer, web};

use crate::services::media_service;
mod api;
mod models;
mod routes;
mod services;
mod config;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let media_service = web::Data::new(media_service::MediaService::new(
        api::ertflix_client::ErtflixClient::new("https://api.ertflix.gr"),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(media_service.clone())
            .configure(routes::init_routes)
    })
    .bind("0.0.0.0:25860")?
    .run()
    .await
}
