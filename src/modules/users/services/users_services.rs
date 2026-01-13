use uuid::Uuid;
use crate::modules::users::dto::users_dto::{CreateUserDto, UserResponseDto};

pub async fn create_user(payload: CreateUserDto) -> UserResponseDto {
    
    let new_id = Uuid::new_v4();

    UserResponseDto {
        id: new_id,
        username: payload.username,
        email: payload.email,
        active: true,
    }
}