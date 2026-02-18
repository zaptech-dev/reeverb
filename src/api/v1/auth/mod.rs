pub mod dto;
pub mod error;
pub mod handlers;

use handlers::*;
use rapina::prelude::*;

pub const PUBLIC_ROUTES: &[(&str, &str)] = &[
    ("POST", "/api/v1/auth/register"),
    ("POST", "/api/v1/auth/login"),
];

pub fn routes() -> Router {
    Router::new()
        .post("/register", register)
        .post("/login", login)
        .get("/me", me)
}
