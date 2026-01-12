use crate::modules::users::dto::users_dto::{CreateUserDto, UserResponseDto};

pub async fn create_user(payload: CreateUserDto) -> UserResponseDto {
    UserResponseDto {
        id: 1,
        username: payload.username,
        email: payload.email,
    }
}