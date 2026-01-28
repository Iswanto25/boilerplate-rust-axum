use axum::{routing::get, Json};
use dotenvy::dotenv;

mod config;
mod db;
mod middleware;
mod modules;
mod routes;
mod state;
mod utils;

use axum::middleware as axum_middleware;
use serde_json::json;
use state::AppState;

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

    let config = config::Config::from_env().expect("Failed to load configuration. Please check your environment variables.");

    config::init_logging(&config.logging);

    tracing::info!("🚀 Starting Rust Boilerplate Server...");
    tracing::info!("🌍 Environment: {:?}", config.server.environment);

    let state = AppState {};

    let app = routes::routes::get_routes()
        .route("/health", get(health_check_handler))
        .layer(axum_middleware::from_fn(middleware::logger_middleware))
        .with_state(state);

    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    tracing::info!("✅ Server successfully started at http://{}", addr);
    tracing::info!("📝 Logs are being written to: ./{}/{}", config.logging.directory, config.logging.file_name);

    axum::serve(listener, app).await.unwrap();
}
