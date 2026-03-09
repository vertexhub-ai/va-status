use anyhow::Context;
use axum::{
    http::StatusCode,
    routing::{get, patch, post},
    Extension, Router,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

mod config;
mod db;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize tracing
    let filter = filter::Targets::new()
        .with_target("tower_http", Level::DEBUG)
        .with_target("va_status_server", Level::DEBUG)
        .with_target("sqlx", Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    info!("Starting va-status-server...");

    // Load configuration
    let configuration = config::get_configuration()
        .context("Failed to read configuration.")?;

    // Database connection pool
    let pool = db::get_connection_pool(&configuration.database)
        .await
        .context("Failed to connect to Postgres.")?;

    // Run database migrations
    db::run_migrations(&pool)
        .await
        .context("Failed to run database migrations.")?;

    // Build our application with a route
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/status", get(routes::get_all_services))
        .route("/services", post(routes::create_service))
        .route("/services/:id/status", patch(routes::update_service_status))
        .route("/services/:id/incidents", get(routes::get_service_incidents))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()) // For simplicity, allow all CORS
        .layer(Extension(pool));

    // Run our app with hyper
    let addr = SocketAddr::from((
        configuration.application.host.parse::<std::net::IpAddr>()?,
        configuration.application.port,
    ));

    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("Server failed: {}", e))?;

    info!("Server stopped.");

    Ok(())
}