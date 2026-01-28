use crate::state::AppState;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::modules::users::controllers::users_controllers;

fn user_routes() -> Router<AppState> {
    Router::new().route("/", post(users_controllers::create))
}

pub fn get_routes() -> Router<AppState> {
    Router::new().route("/", get(root_handler)).nest("/users", user_routes())
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message": "Hello, World!",
        "version": "1.0.0"
    }))
}
