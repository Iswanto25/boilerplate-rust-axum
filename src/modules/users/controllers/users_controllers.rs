use axum::{Json, http::StatusCode};
use crate::modules::users::dto::users_dto::{CreateUserDto, UserResponseDto};
use crate::modules::users::services::users_services;

pub async fn create(
    Json(payload): Json<CreateUserDto>
) -> (StatusCode, Json<UserResponseDto>) {
    
    let result = users_services::create_user(payload).await;
    
    (StatusCode::CREATED, Json(result))
}