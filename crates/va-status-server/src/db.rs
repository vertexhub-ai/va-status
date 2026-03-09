use sqlx::{
    Pool,
    migrate::Migrator,
    postgres::{PgPoolOptions, Postgres},
};
use tracing::info;

use crate::config::DatabaseSettings;

pub type PgPool = Pool<Postgres>;

pub async fn get_connection_pool(settings: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.connection_string())
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    info!("Running database migrations...");
    let m = Migrator::new(std::path::Path::new("./migrations")).await?;
    m.run(pool).await?;
    info!("Database migrations finished.");
    Ok(())
}
