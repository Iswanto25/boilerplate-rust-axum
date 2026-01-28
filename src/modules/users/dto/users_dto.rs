use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub active: bool,
}
