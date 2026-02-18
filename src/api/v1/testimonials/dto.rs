use rapina::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::api::v1::tags::dto::TagResponse;

#[derive(Deserialize, JsonSchema)]
pub struct CreateTestimonialRequest {
    pub author_name: String,
    #[serde(rename = "type")]
    pub testimonial_type: Option<String>,
    pub content: Option<String>,
    pub rating: Option<i16>,
    pub author_email: Option<String>,
    pub author_title: Option<String>,
    pub author_avatar_url: Option<String>,
    pub author_company: Option<String>,
    pub author_url: Option<String>,
    pub video_url: Option<String>,
    pub video_thumbnail_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub transcription: Option<String>,
    pub source: Option<String>,
    pub source_platform: Option<String>,
    pub source_url: Option<String>,
    pub source_id: Option<String>,
    pub sentiment: Option<String>,
    pub sentiment_score: Option<f32>,
    pub language: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateTestimonialRequest {
    #[serde(rename = "type")]
    pub testimonial_type: Option<String>,
    pub content: Option<String>,
    pub rating: Option<i16>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub author_title: Option<String>,
    pub author_avatar_url: Option<String>,
    pub author_company: Option<String>,
    pub author_url: Option<String>,
    pub video_url: Option<String>,
    pub video_thumbnail_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub transcription: Option<String>,
    pub source: Option<String>,
    pub source_platform: Option<String>,
    pub source_url: Option<String>,
    pub source_id: Option<String>,
    pub sentiment: Option<String>,
    pub sentiment_score: Option<f32>,
    pub language: Option<String>,
    pub is_approved: Option<bool>,
    pub is_featured: Option<bool>,
}

#[derive(Serialize, JsonSchema)]
pub struct TestimonialResponse {
    pub id: String,
    pub project_id: String,
    #[serde(rename = "type")]
    pub testimonial_type: String,
    pub content: Option<String>,
    pub rating: Option<i16>,
    pub author_name: String,
    pub author_email: Option<String>,
    pub author_title: Option<String>,
    pub author_avatar_url: Option<String>,
    pub author_company: Option<String>,
    pub author_url: Option<String>,
    pub video_url: Option<String>,
    pub video_thumbnail_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub transcription: Option<String>,
    pub source: Option<String>,
    pub source_platform: Option<String>,
    pub source_url: Option<String>,
    pub source_id: Option<String>,
    pub sentiment: Option<String>,
    pub sentiment_score: Option<f32>,
    pub language: Option<String>,
    pub is_approved: bool,
    pub is_featured: bool,
    pub tags: Vec<TagResponse>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ListTestimonialsQuery {
    pub is_approved: Option<bool>,
    pub is_featured: Option<bool>,
}
