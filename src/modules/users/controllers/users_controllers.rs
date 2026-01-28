use axum::{
    http::StatusCode,
    Json,
};
use crate::modules::users::dto::users_dto::{CreateUserDto, UserResponseDto};
use crate::modules::users::services::users_services;
use crate::utils::respons::AppResponse;
use axum::{http::StatusCode, response::Response, Json};

pub async fn create(
    Json(payload): Json<CreateUserDto>, 
) -> (StatusCode, Json<UserResponseDto>) {
    
    let result = users_services::create_user(payload).await;

    (StatusCode::CREATED, Json(result))
}
