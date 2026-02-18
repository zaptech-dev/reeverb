use rapina::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthResponse {
    pub token: String,
    pub expires_in: u64,
    pub user: UserResponse,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}
