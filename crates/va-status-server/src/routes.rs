use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        CreateService, Incident, Service, ServiceStatus, ServiceWithIncidents, StatusPage,
        UpdateServiceStatus,
    },
};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn create_service(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateService>,
) -> Result<Json<Service>, AppError> {
    let new_service = sqlx::query_as!(
        Service,
        r#"
        INSERT INTO services (id, name, description, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, description, status AS "status!: ServiceStatus", created_at, updated_at
        "#,
        Uuid::new_v4(),
        payload.name,
        payload.description,
        ServiceStatus::Operational as ServiceStatus,
        Utc::now(),
        Utc::now()
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(new_service))
}

pub async fn get_all_services(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<StatusPage>, AppError> {
    let services = sqlx::query_as!(
        Service,
        r#"SELECT id, name, description, status AS "status!: ServiceStatus", created_at, updated_at FROM services ORDER BY name"#
    )
    .fetch_all(&pool)
    .await?;

    let mut services_with_incidents: Vec<ServiceWithIncidents> = Vec::new();

    for service in services {
        let incidents = sqlx::query_as!(
            Incident,
            r#"SELECT id, service_id, status AS "status!: ServiceStatus", description, created_at FROM incidents WHERE service_id = $1 ORDER BY created_at DESC LIMIT 5"#,
            service.id
        )
        .fetch_all(&pool)
        .await?;

        services_with_incidents.push(ServiceWithIncidents {
            id: service.id,
            name: service.name,
            description: service.description,
            status: service.status,
            created_at: service.created_at,
            updated_at: service.updated_at,
            incidents,
        });
    }

    Ok(Json(StatusPage::new(services_with_incidents)))
}

pub async fn update_service_status(
    Extension(pool): Extension<PgPool>,
    Path(service_id): Path<Uuid>,
    Json(payload): Json<UpdateServiceStatus>,
) -> Result<Json<Service>, AppError> {
    let updated_service = sqlx::query_as!(
        Service,
        r#"
        UPDATE services
        SET status = $1, updated_at = $2
        WHERE id = $3
        RETURNING id, name, description, status AS "status!: ServiceStatus", created_at, updated_at
        "#,
        payload.status as ServiceStatus,
        Utc::now(),
        service_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::DbError(e),
    })?;

    if let Some(description) = payload.description {
        sqlx::query!(
            r#"INSERT INTO incidents (id, service_id, status, description, created_at) VALUES ($1, $2, $3, $4, $5)"#,
            Uuid::new_v4(),
            service_id,
            payload.status as ServiceStatus,
            description,
            Utc::now()
        ).execute(&pool).await?;
    }

    Ok(Json(updated_service))
}

pub async fn get_service_incidents(
    Extension(pool): Extension<PgPool>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Vec<Incident>>, AppError> {
    let incidents = sqlx::query_as!(
        Incident,
        r#"SELECT id, service_id, status AS "status!: ServiceStatus", description, created_at FROM incidents WHERE service_id = $1 ORDER BY created_at DESC"#,
        service_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(incidents))
}
