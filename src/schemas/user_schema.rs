use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}