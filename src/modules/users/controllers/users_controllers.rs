use crate::modules::users::dto::users_dto::CreateUserDto;
use crate::modules::users::services::users_services;
use crate::utils::respons::AppResponse;
use axum::{http::StatusCode, response::Response, Json};

pub async fn create(Json(payload): Json<CreateUserDto>) -> Response {
    let result = users_services::create_user(payload).await;

    match result {
        Ok(user) => AppResponse::success(user, "User created successfully", StatusCode::CREATED),
        Err(error_message) => AppResponse::error(error_message, "Failed to create user", StatusCode::BAD_REQUEST),
    }
}
