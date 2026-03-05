use axum::{routing::get, Router};
use sqlx::PgPool;
use std::net::TcpListener;
use crate::routes::{health_check, list_services};
use tower_http::trace::TraceLayer;

pub async fn run(listener: TcpListener, db_pool: PgPool) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/services", get(list_services))
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);

    axum::serve(listener, app).await.map_err(|e| e.into())
}