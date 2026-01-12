use axum::{routing::post, Router};
use crate::state::AppState;
use crate::modules::users::controllers::users_controllers;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(users_controllers::create))
}