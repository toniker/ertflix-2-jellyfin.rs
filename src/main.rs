use actix_web::{web, App, HttpServer, middleware::Logger};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::ertflix_client::DefaultErtflixClient;
use crate::services::media_service;

mod api;
mod config;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber with environment-based filtering
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ertflix_2_jellyfin=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Ertflix to Jellyfin adapter server");
    info!("Binding to address: 0.0.0.0:25860");

    let media_service = web::Data::new(
        media_service::MediaService::<DefaultErtflixClient>::new(config::ERTFLIX_API_URL)
    );

    info!("Media service initialized with Ertflix API URL: {}", config::ERTFLIX_API_URL);

    let server_result = HttpServer::new(move || {
        info!("Configuring new app worker");
        App::new()
            .app_data(media_service.clone())
            .wrap(Logger::default()) // Add request logging middleware
            .wrap(tracing_actix_web::TracingLogger::default()) // Add tracing middleware
            .configure(routes::init_routes::<DefaultErtflixClient>)
    })
    .bind("0.0.0.0:25860");

    match server_result {
        Ok(server) => {
            info!("Server successfully bound to 0.0.0.0:25860");
            info!("Server starting...");
            server.run().await
        }
        Err(e) => {
            warn!("Failed to bind server to 0.0.0.0:25860: {}", e);
            Err(e)
        }
    }
}
