use crate::modules::users::dto::users_dto::{CreateUserDto, UserResponseDto};
use uuid::Uuid;

pub async fn create_user(payload: CreateUserDto) -> Result<UserResponseDto, String> {
    if !payload.email.contains('@') {
        return Err("Invalid email format".to_string());
    }

    if payload.username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }

    let new_id = Uuid::new_v4();

    Ok(UserResponseDto { id: new_id, username: payload.username, email: payload.email, active: true })
}
