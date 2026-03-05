use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::models::Service;
use crate::error::AppError;

pub async fn list_services(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Service>>, AppError> {
    let services = sqlx::query_as!(
        Service,
        "SELECT id, name, status, created_at FROM services"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(services))
}