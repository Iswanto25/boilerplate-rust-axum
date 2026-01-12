use dotenvy::dotenv;
use axum::{
    routing::get,
    Json,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    dotenv().ok();
    
    let file_appender = tracing_appender::rolling::daily("logs", "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
        )
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();
    
    tracing::info!("🚀 Starting Rust Boilerplate Server...");
    
    let state = AppState {};
    let app = routes::routes::get_routes()
        .route("/", get(|| async {
        Json(json!({
            "status": "success",
            "message": "Hello, World!",
            "version": "1.0.0"
        }))
    }))
    .route("/health", get(health_check_handler))
    .layer(TraceLayer::new_for_http())
    .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    let addr = format!("{}:{}", host, port);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));
    
    tracing::info!("✅ Server successfully started at http://{}", addr);
    tracing::info!("📝 Logs are being written to: ./logs/server.log");
    
    axum::serve(listener, app).await.unwrap();
}