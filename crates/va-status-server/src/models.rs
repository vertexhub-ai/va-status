use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "service_status", rename_all = "lowercase")]
pub enum ServiceStatus {
    Operational,
    Degraded,
    Outage,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus::Operational
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ServiceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Incident {
    pub id: Uuid,
    pub service_id: Uuid,
    pub status: ServiceStatus,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateService {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServiceStatus {
    pub status: ServiceStatus,
    pub description: Option<String>, // Description for the incident
}

#[derive(Debug, Serialize)]
pub struct ServiceWithIncidents {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ServiceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub incidents: Vec<Incident>,
}

#[derive(Debug, Serialize)]
pub struct StatusPage {
    pub services: Vec<ServiceWithIncidents>,
    pub last_updated: DateTime<Utc>,
}

impl StatusPage {
    pub fn new(services: Vec<ServiceWithIncidents>) -> Self {
        Self {
            services,
            last_updated: Utc::now(),
        }
    }
}
