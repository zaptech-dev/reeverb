use rapina::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct CreateProjectRequest {
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
