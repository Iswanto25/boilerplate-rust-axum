use axum::{routing::get, Json};
use dotenvy::dotenv;

mod config;
mod db;
mod modules;
mod routes;
mod state;
mod utils;

use serde_json::json;
use state::AppState;
use tower_http::trace::TraceLayer;

async fn health_check_handler() -> Json<serde_json::Value> {
    tracing::debug!("Health check endpoint called");
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Load configuration from environment
    let config = config::Config::from_env()
        .expect("Failed to load configuration. Please check your environment variables.");

    // Initialize logging with config
    config::init_logging(&config.logging);

    tracing::info!("🚀 Starting Rust Boilerplate Server...");
    tracing::info!("🌍 Environment: {:?}", config.server.environment);

    // Initialize database connection (optional - uncomment when ready to use)
    // let db = config::init_database(&config.database)
    //     .await
    //     .expect("Failed to initialize database connection");
    //
    // Test database connection
    // config::test_connection(&db)
    //     .await
    //     .expect("Database connection test failed");

    let state = AppState {};

    // Build the application with routes
    let app = routes::routes::get_routes()
        .route("/health", get(health_check_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Get server address from config
    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    tracing::info!("✅ Server successfully started at http://{}", addr);
    tracing::info!(
        "📝 Logs are being written to: ./{}/{}",
        config.logging.directory,
        config.logging.file_name
    );

    axum::serve(listener, app).await.unwrap();
}
