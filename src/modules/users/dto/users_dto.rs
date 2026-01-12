use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub username: String,
    pub email: String,
}