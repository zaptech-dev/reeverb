use rapina::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Serialize, JsonSchema)]
pub struct TagResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct SetTestimonialTagsRequest {
    pub tag_ids: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct TestimonialTagsResponse {
    pub tags: Vec<TagResponse>,
}
