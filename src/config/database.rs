use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

use super::env::DatabaseConfig;

/// Initialize database connection with configuration
#[allow(dead_code)]
pub async fn init_database(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(&config.url);

    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    tracing::info!("🔌 Connecting to database...");
    let db = Database::connect(opt).await?;
    tracing::info!("✅ Database connection established");

    Ok(db)
}

/// Test database connection
#[allow(dead_code)]
pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    let _ = db.ping().await?;
    tracing::info!("✅ Database ping successful");
    Ok(())
}
